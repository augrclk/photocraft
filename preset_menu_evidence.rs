//! Temporary evidence driver: real Brush Settings popup and shared modal, no UI shim.
use egui_kittest::{Harness, kittest::Queryable};
use photocraft_engine::Session;
use photocraft_ui_egui::{PhotocraftApp, Services, brush_preset_dialog, state::Tool, theme};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let phase = args.next().ok_or("expected before/after")?;
    if !matches!(phase.as_str(), "before" | "after") {
        return Err("expected before/after".into());
    }
    let out = std::path::PathBuf::from(args.next().ok_or("expected output directory")?);
    let metadata = std::path::PathBuf::from(args.next().ok_or("expected metadata file")?);
    std::fs::create_dir_all(&out)?;
    let mut reports = Vec::new();
    for (label, kind, tab, size, scale) in [
        ("medium", theme::ThemeKind::ProMedium, 0, [1200.0, 900.0], 1.0),
        ("brushes", theme::ThemeKind::ProMedium, 1, [1200.0, 900.0], 1.0),
        ("light", theme::ThemeKind::ProLight, 0, [1200.0, 900.0], 1.0),
        ("scale2", theme::ThemeKind::ProMedium, 0, [900.0, 720.0], 2.0),
    ] {
        let mut h = Harness::builder().with_size(egui::vec2(size[0], size[1])).with_pixels_per_point(scale).with_max_steps(64).wgpu().build_eframe(move |cc| {
            PhotocraftApp::setup_context(&cc.egui_ctx, kind);
            let mut app = PhotocraftApp::new(Session::new(), Services::default());
            app.ui.theme = kind;
            if let Some(rs) = cc.wgpu_render_state.as_ref() {
                app.set_wgpu(rs.clone());
            }
            app
        });
        h.run_steps(4);
        h.state_mut().run("prefs.set", json!({"path":"interface.theme", "value":kind.id()}))?;
        h.state_mut().run("prefs.set", json!({"path":"interface.language", "value":"en"}))?;
        h.state_mut().run("file.new", json!({"width":480, "height":360}))?;
        h.state_mut().run("tools.setBrush", json!({"preset":"Chalk", "size":47, "opacity":0.41, "flow":0.63, "mode":"Multiply"}))?;
        h.state_mut().ui.tool = Tool::Brush;
        h.state_mut().ui.panels.brush_settings = true;
        h.state_mut().ui.brush_tab = tab;
        h.run_steps(8);
        if h.state().ui.theme != kind || theme::Tokens::get(&h.ctx) != theme::Tokens::for_kind(kind) {
            return Err(format!("{label}: fixture theme did not apply").into());
        }
        let before = h.state().session.tools.presets.clone();
        let brush = h.state().session.tools.brush.clone();
        let journal = h.state().session.journal.len();
        let window = h.query_by_role_and_label(egui::accesskit::Role::Window, "Brush Settings").ok_or("missing Brush Settings window")?;
        window.query_by_label("Panel menu").ok_or("missing panel menu")?.click();
        h.run_steps(3);
        h.query_by_label("New Brush Preset…").ok_or("missing creation menu item")?.click();
        h.run_steps(16);
        let dialogs: Vec<_> = h.state().ui.dialogs.iter().filter(|d| brush_preset_dialog::owns(&d.fields)).map(|d| d.fields.clone()).collect();
        let now = h.state().session.tools.presets.clone();
        if phase == "before" {
            if now.len() != before.len() + 1 || !dialogs.is_empty() || h.state().session.journal.len() != journal + 1 {
                return Err(format!("{label}: baseline did not reproduce immediate saving").into());
            }
        } else if now != before || dialogs.len() != 1 || h.state().session.journal.len() != journal {
            return Err(format!("{label}: creation was not deferred to the modal").into());
        }
        if h.state().session.tools.brush != brush {
            return Err(format!("{label}: opening the menu changed the brush").into());
        }
        let image = out.join(format!("3477-preset-menu-{phase}-{label}.png"));
        h.render()?.save(&image)?;
        reports.push(json!({"case":label, "window":size, "scale":scale, "tab":tab, "theme":kind.id(),
            "before_presets":before.len(), "after_presets":now.len(), "new_preset_names":now.iter().filter(|p| !before.iter().any(|old|old.name==p.name)).map(|p|p.name.clone()).collect::<Vec<_>>(),
            "current_preset":h.state().session.tools.current_preset, "journal_delta":h.state().session.journal.len()-journal, "dialogs":dialogs, "brush_unchanged":true,
            "png":image.file_name().and_then(|n|n.to_str())}));
    }
    std::fs::write(metadata, serde_json::to_string_pretty(&reports)?)?;
    Ok(())
}
