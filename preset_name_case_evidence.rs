//! Native evidence for a fresh preset name after switching tools.
use egui_kittest::{
    Harness,
    kittest::{NodeT, Queryable},
};
use photocraft_engine::Session;
use photocraft_ui_egui::{PhotocraftApp, Services, brush_preset_dialog, state::Tool, theme};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let phase = args.next().ok_or("phase")?;
    let out = std::path::PathBuf::from(args.next().ok_or("output directory")?);
    let metadata = args.next().ok_or("metadata path")?;
    let mut reports = Vec::new();
    for (label, kind, menu, tab, scale) in [
        ("menu-medium", theme::ThemeKind::ProMedium, true, 0, 1.0),
        ("picker-light", theme::ThemeKind::ProLight, false, 0, 1.0),
        ("menu-scale2", theme::ThemeKind::ProMedium, true, 0, 2.0),
        ("brushes-menu", theme::ThemeKind::ProMedium, true, 1, 1.0),
    ] {
        let mut h = Harness::builder().with_size(egui::vec2(1200.0, 900.0)).with_pixels_per_point(scale).with_max_steps(64).wgpu().build_eframe(move |cc| {
            PhotocraftApp::setup_context(&cc.egui_ctx, kind);
            let mut app = PhotocraftApp::new(Session::new(), Services::default());
            if let Some(rs) = cc.wgpu_render_state.as_ref() {
                app.set_wgpu(rs.clone());
            }
            app
        });
        h.run_steps(3);
        h.state_mut().run("prefs.set", json!({"path":"interface.theme", "value":kind.id()}))?;
        h.state_mut().run("prefs.set", json!({"path":"interface.language", "value":"en"}))?;
        h.state_mut().run("file.new", json!({"width":480,"height":360}))?;
        h.state_mut().ui.tool = Tool::Brush;
        h.run_steps(3);
        for (name, size) in [("brush 1", 31), ("BRUSH 2", 52)] {
            h.state_mut().run("tools.setBrush", json!({"size":size}))?;
            h.state_mut().run("brush.presets.save", json!({"name":name}))?;
        }
        h.state_mut().ui.tool = Tool::Eraser;
        h.run_steps(3);
        if h.state().session.tools.current_preset.is_some() {
            return Err("tool switch did not clear identity".into());
        }
        let presets = h.state().session.tools.presets.clone();
        let brush = h.state().session.tools.brush.clone();
        let journal = h.state().session.journal.len();
        if menu {
            h.state_mut().ui.panels.brush_settings = true;
            h.state_mut().ui.brush_tab = tab;
            h.run_steps(3);
            h.query_by_role_and_label(egui::accesskit::Role::Window, "Brush Settings").ok_or("window")?.query_by_label("Panel menu").ok_or("menu")?.click();
            h.run_steps(2);
            h.query_by_label("New Brush Preset…").ok_or("new action")?.click();
        } else {
            h.state_mut().ui.brush_picker = Some([300.0, 120.0]);
            h.run_steps(3);
            h.query_by_label("New Brush Preset").ok_or("new button")?.click();
        }
        h.run_steps(8);
        if h.state().ui.theme != kind || theme::Tokens::get(&h.ctx) != theme::Tokens::for_kind(kind) {
            return Err("wrong theme".into());
        }
        let dialogs: Vec<_> = h.state().ui.dialogs.iter().filter(|d| brush_preset_dialog::owns(&d.fields)).collect();
        if dialogs.len() != 1 {
            return Err("one preset dialog expected".into());
        }
        let name = dialogs.first().and_then(|d| d.fields.get("name")).and_then(|n| n.as_str()).ok_or("name")?.to_string();
        let disabled = h.query_by_label("OK").ok_or("OK")?.accesskit_node().is_disabled();
        let before = phase == "before";
        let duplicate_warning = h.query_by_label("A brush preset with this name already exists").is_some();
        if duplicate_warning != before {
            return Err("wrong duplicate warning".into());
        }
        if name != if before { "Brush 1" } else { "Brush 3" } || disabled != before {
            return Err(format!("{label}: unexpected name/availability").into());
        }
        if h.state().session.tools.presets != presets || h.state().session.tools.brush != brush || h.state().session.journal.len() != journal {
            return Err("opening mutated saved state".into());
        }
        let path = out.join(format!("preset-name-case-{phase}-{label}.png"));
        h.render()?.save(&path)?;
        h.query_by_label("OK").ok_or("OK")?.click();
        h.run_steps(4);
        let count = h.state().session.tools.presets.len();
        let delta = h.state().session.journal.len() - journal;
        if count != presets.len() + usize::from(!before)
            || delta != usize::from(!before)
            || h.state().session.tools.presets.get(..presets.len()) != Some(presets.as_slice())
        {
            return Err("incorrect save or existing preset changed".into());
        }
        reports.push(json!({"case":label,"theme":kind.id(),"menu":menu,"tab":tab,"scale":scale,"suggested_name":name,"ok_disabled":disabled,"duplicate_warning":duplicate_warning,"before_count":presets.len(),"after_count":count,"journal_delta":delta,"old_presets_unchanged":true,"current_preset":h.state().session.tools.current_preset,"png":path.file_name().and_then(|n|n.to_str())}));
    }
    std::fs::write(metadata, serde_json::to_string_pretty(&reports)?)?;
    Ok(())
}
