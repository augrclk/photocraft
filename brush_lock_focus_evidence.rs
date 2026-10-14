//! Native focus feedback and input evidence for the existing Brush Settings lock controls.
use egui::accesskit::Role;
use egui_kittest::{Harness, kittest::Queryable};
use photocraft_engine::Session;
use photocraft_ui_egui::{PhotocraftApp, Services, i18n, state::Tool, theme};
use serde_json::json;

fn has_ring(shape: &egui::Shape, target: egui::Rect, accent: egui::Color32) -> bool {
    match shape {
        egui::Shape::Rect(r) => r.rect == target && r.stroke == egui::Stroke::new(1.0, accent) && r.stroke_kind == egui::StrokeKind::Inside,
        egui::Shape::Vec(v) => v.iter().any(|s| has_ring(s, target, accent)),
        _ => false,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let phase = args.next().ok_or("phase")?;
    if phase != "before" && phase != "after" {
        return Err("phase must be before or after".into());
    }
    let out = std::path::PathBuf::from(args.next().ok_or("output directory")?);
    let metadata = args.next().ok_or("metadata path")?;
    let mut reports = Vec::new();
    for (case, kind, language, locked, scale, size) in [
        ("medium", theme::ThemeKind::ProMedium, "en", false, 1.0, [1200.0, 900.0]),
        ("light", theme::ThemeKind::ProLight, "en", true, 1.0, [1200.0, 900.0]),
        ("scale2", theme::ThemeKind::ProMedium, "en", false, 2.0, [900.0, 720.0]),
        ("turkish", theme::ThemeKind::ProMedium, "tr", true, 1.0, [1200.0, 900.0]),
    ] {
        let mut h = Harness::builder().with_size(egui::vec2(size[0], size[1])).with_pixels_per_point(scale).with_max_steps(64).wgpu().build_eframe(move |cc| {
            PhotocraftApp::setup_context(&cc.egui_ctx, kind);
            let mut app = PhotocraftApp::new(Session::new(), Services::default());
            if let Some(rs) = cc.wgpu_render_state.as_ref() {
                app.set_wgpu(rs.clone());
            }
            app
        });
        h.run_steps(3);
        h.state_mut().run("prefs.set", json!({"path":"interface.theme","value":kind.id()}))?;
        h.state_mut().run("prefs.set", json!({"path":"interface.language","value":language}))?;
        // Normal Tab navigation with the documented nondefault canvas shortcut binding.
        h.state_mut().session.prefs.edit(|prefs| prefs.shortcuts.insert("window.togglePanels".into(), String::new()));
        h.state_mut().run("file.new", json!({"width":480,"height":360}))?;
        h.state_mut().ui.tool = Tool::Brush;
        h.run_steps(3);
        h.state_mut().run(
            "tools.setBrush",
            json!({"size":17.25,"hardness":0.37,"noise":true,
            "locks":{"scattering":locked,"texture":true},"scattering":{"enabled":false,"scatter":{"jitter":1.25}}}),
        )?;
        h.state_mut().ui.panels.brush_settings = true;
        h.state_mut().ui.brush_tab = 0;
        h.state_mut().ui.brush_section = 0;
        h.hover_at(egui::pos2(0.0, 0.0));
        h.run_steps(8);
        let lang = i18n::Lang::from_code(language).ok_or("language")?;
        if i18n::current() != lang || h.state().ui.theme != kind || theme::Tokens::get(&h.ctx) != theme::Tokens::for_kind(kind) {
            return Err("wrong theme/language".into());
        }
        let label = format!("{}: {}", i18n::tr(lang, "Lock"), i18n::tr(lang, "Scattering"));
        let before = h.state().session.tools.brush.clone();
        let presets = h.state().session.tools.presets.clone();
        let journal = h.state().session.journal.len();
        let target = h.get_by_role_and_label(Role::Button, &label).rect();
        let accent = theme::Tokens::get(&h.ctx).accent;
        let ring = |h: &Harness<'_, PhotocraftApp>| h.output().shapes.iter().any(|s| has_ring(&s.shape, target, accent));
        if ring(&h) {
            return Err("idle lock has a focus ring".into());
        }
        let idle_png = if case == "medium" {
            let p = out.join(format!("brush-lock-focus-{phase}-idle.png"));
            h.render()?.save(&p)?;
            Some(p.file_name().and_then(|s| s.to_str()).ok_or("idle name")?.to_owned())
        } else {
            None
        };
        h.get_by_role_and_label(Role::Button, &label).focus();
        h.run_steps(2);
        if !h.get_by_role_and_label(Role::Button, &label).is_focused() {
            return Err("lock not focused".into());
        }
        let visible = ring(&h);
        if visible != (phase == "after") {
            return Err("unexpected focus indicator".into());
        }
        if h.state().session.tools.brush != before
            || h.state().session.tools.presets != presets
            || h.state().session.journal.len() != journal
            || h.state().ui.brush_section != 0
        {
            return Err("focus changed settings".into());
        }
        let p = out.join(format!("brush-lock-focus-{phase}-{case}.png"));
        h.render()?.save(&p)?;
        h.key_press(egui::Key::Space);
        h.run_steps(3);
        let mut expected = before.clone();
        expected.locks.scattering = !locked;
        if h.state().session.tools.brush != expected || h.state().session.tools.presets != presets || h.state().ui.brush_section != 0 || ring(&h) != visible {
            return Err("activation changes more than the lock or loses focus feedback".into());
        }
        let calls = &h.state().session.journal[journal..];
        if calls.len() != 1 || calls.first().is_none_or(|c| c.0 != "tools.setBrush" || c.1["brush"] != json!({"locks":{"scattering":!locked}})) {
            return Err("wrong keyboard command".into());
        }
        h.key_press(egui::Key::Tab);
        h.run_steps(3);
        if h.get_by_role_and_label(Role::Button, &label).is_focused()
            || ring(&h)
            || h.state().session.tools.brush != expected
            || h.state().session.journal.len() != journal + 1
        {
            return Err("Tab failed to move focus without editing".into());
        }
        reports.push(json!({"case":case,"theme":kind.id(),"language":language,"scale":scale,"window":size,"initially_locked":locked,
            "focus_name":label,"focused":true,"focus_ring_visible":visible,"focus_rect":[target.min.x,target.min.y,target.max.x,target.max.y],
            "idle_png":idle_png,"png":p.file_name().and_then(|s|s.to_str()),"focus_journal_delta":0,"activation_journal_delta":1,
            "other_fields_and_presets_unchanged":true,"section_unchanged":true,"space_keeps_focus_feedback":true,
            "tab_moves_focus_and_removes_indicator":true,"canvas_tab_binding":"cleared for navigation fixture only"}));
    }
    std::fs::write(metadata, serde_json::to_string_pretty(&reports)?)?;
    Ok(())
}
