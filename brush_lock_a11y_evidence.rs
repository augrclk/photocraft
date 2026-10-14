//! Native visual and AccessKit evidence for Brush Settings section locks.
use egui::accesskit::{Role, Toggled};
use egui_kittest::{
    Harness,
    kittest::{NodeT, Queryable},
};
use photocraft_engine::Session;
use photocraft_ui_egui::{PhotocraftApp, Services, brush_panel::SECTIONS, i18n, state::Tool, theme};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let phase = args.next().ok_or("phase")?;
    let output = std::path::PathBuf::from(args.next().ok_or("output directory")?);
    let metadata = args.next().ok_or("metadata path")?;
    let mut reports = Vec::new();
    for (case, kind, language, initially_locked, scale, size) in [
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
        h.state_mut().run("file.new", json!({"width":480,"height":360}))?;
        h.state_mut().ui.tool = Tool::Brush;
        h.run_steps(3);
        h.state_mut().run(
            "tools.setBrush",
            json!({"size":17.25,"hardness":0.37,"spacing":0.23,
            "noise":true,"locks":{"scattering":initially_locked,"texture":true},
            "scattering":{"enabled":false,"scatter":{"jitter":1.25}},
            "shapeDynamics":{"enabled":false,"size":{"control":"penTilt","minimum":0.37}}}),
        )?;
        h.state_mut().ui.panels.brush_settings = true;
        h.state_mut().ui.brush_tab = 0;
        h.state_mut().ui.brush_section = 0;
        h.run_steps(8);
        let lang = i18n::Lang::from_code(language).ok_or("language")?;
        if i18n::current() != lang || h.state().ui.theme != kind || theme::Tokens::get(&h.ctx) != theme::Tokens::for_kind(kind) {
            return Err("wrong language or theme".into());
        }
        let before = h.state().session.tools.brush.clone();
        let presets = h.state().session.tools.presets.clone();
        let journal = h.state().session.journal.len();
        let window_name = i18n::tr(lang, "Brush Settings");
        let row = h.get_by_role_and_label(Role::Window, window_name).get_by_label(i18n::tr(lang, "Scattering")).rect();
        let target = egui::pos2(row.right() - 11.0, row.center().y);
        h.hover_at(target);
        h.run_steps(1);
        h.drag_at(target);
        h.run_steps(1);
        h.drop_at(target);
        h.run_steps(3);
        h.hover_at(egui::pos2(0.0, 0.0));
        h.run_steps(2);
        let mut expected = before.clone();
        expected.locks.scattering = !initially_locked;
        if h.state().session.tools.brush != expected || h.state().session.tools.presets != presets || h.state().ui.brush_section != 0 {
            return Err("pointer changes something besides the lock".into());
        }
        let calls = &h.state().session.journal[journal..];
        if calls.len() != 1 || calls.first().is_none_or(|c| c.0 != "tools.setBrush" || c.1["brush"] != json!({"locks":{"scattering":!initially_locked}})) {
            return Err("incorrect pointer command route".into());
        }
        let mut nodes = Vec::new();
        for &(name, _) in SECTIONS.iter().skip(1) {
            let label = format!("{}: {}", i18n::tr(lang, "Lock"), i18n::tr(lang, name));
            if let Some(node) = h.query_by_role_and_label(Role::Button, &label) {
                let locked = node.accesskit_node().toggled().ok_or("missing toggle state")?;
                let expected_locked = match name {
                    "Scattering" => !initially_locked,
                    "Texture" => true,
                    _ => false,
                };
                if (locked == Toggled::True) != expected_locked || node.accesskit_node().is_disabled() {
                    return Err("incorrect reported lock state".into());
                }
                nodes.push(json!({"name":label,"role":"button","locked":locked==Toggled::True,"disabled":node.accesskit_node().is_disabled()}));
            }
        }
        if nodes.len() != if phase == "before" { 0 } else { 12 } {
            return Err("unexpected named lock count".into());
        }
        let png = output.join(format!("brush-lock-a11y-{phase}-{case}.png"));
        h.render()?.save(&png)?;
        let label = format!("{}: {}", i18n::tr(lang, "Lock"), i18n::tr(lang, "Scattering"));
        let keyboard_checked = phase == "after";
        if keyboard_checked {
            h.get_by_role_and_label(Role::Button, &label).focus();
            h.run_steps(2);
            let count = h.state().session.journal.len();
            h.key_press(egui::Key::Space);
            h.run_steps(3);
            if h.state().session.tools.brush != before || h.state().ui.brush_section != 0 || h.state().session.tools.presets != presets {
                return Err("keyboard changes something besides the lock".into());
            }
            let calls = &h.state().session.journal[count..];
            if calls.len() != 1 || calls.first().is_none_or(|c| c.0 != "tools.setBrush" || c.1["brush"] != json!({"locks":{"scattering":initially_locked}})) {
                return Err("incorrect keyboard command route".into());
            }
        }
        reports.push(json!({"case":case,"theme":kind.id(),"language":language,"scale":scale,"window":size,
            "initially_locked":initially_locked,"after_pointer_locked":!initially_locked,"named_lock_count":nodes.len(),"nodes":nodes,
            "pointer_journal_delta":1,"presets_and_other_brush_fields_unchanged":true,"section_unchanged":true,
            "keyboard_checked":keyboard_checked,"png":png.file_name().and_then(|s|s.to_str())}));
    }
    std::fs::write(metadata, serde_json::to_string_pretty(&reports)?)?;
    Ok(())
}
