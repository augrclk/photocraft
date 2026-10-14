//! Native regression evidence: primary tip picking honors the saved size-capture choice.
use egui_kittest::{Harness, kittest::Queryable};
use photocraft_engine::{
    Session,
    paint::{BrushSettings, GrayTile, TipShape},
};
use photocraft_ui_egui::{PhotocraftApp, Services, state::Tool, theme};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let phase = args.next().ok_or("phase")?;
    let out = std::path::PathBuf::from(args.next().ok_or("output directory")?);
    let metadata = args.next().ok_or("metadata path")?;
    let mut reports = Vec::new();
    for (label, kind, capture, scale, size) in [
        ("off-medium", theme::ThemeKind::ProMedium, false, 1.0, [1200.0, 900.0]),
        ("off-light", theme::ThemeKind::ProLight, false, 1.0, [1200.0, 900.0]),
        ("off-scale2", theme::ThemeKind::ProMedium, false, 2.0, [900.0, 720.0]),
        ("on-control", theme::ThemeKind::ProMedium, true, 1.0, [1200.0, 900.0]),
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
        h.state_mut().run("prefs.set", json!({"path":"interface.language","value":"en"}))?;
        h.state_mut().run("file.new", json!({"width":480,"height":360}))?;
        h.state_mut().ui.tool = Tool::Brush;
        h.run_steps(3);
        h.state_mut().session.tools.presets.clear();
        let name = if capture { "Capture size on" } else { "Capture size off" };
        let saved = BrushSettings {
            size: 73.0,
            hardness: 0.35,
            angle: 37.0,
            roundness: 0.45,
            spacing: 0.21,
            tip: TipShape::Sampled(GrayTile { width: 2, height: 2, data: vec![0, 65535, 65535, 0].into() }),
            ..Default::default()
        };
        h.state_mut().run("brush.presets.save", json!({"name":name,"brush":&saved,"captureSize":capture,"toolSettings":false}))?;
        h.state_mut().run(
            "tools.setBrush",
            json!({"size":17.25,"hardness":0.9,"angle":-29,"roundness":0.85,"spacing":0.11,
            "flipX":true,"spacingEnabled":false,"noise":true,"locks":{"noise":true},"opacity":0.41,
            "shapeDynamics":{"enabled":true,"size":{"control":"penTilt","minimum":0.37}}}),
        )?;
        h.state_mut().ui.panels.brush_settings = true;
        h.state_mut().ui.brush_tab = 0;
        h.state_mut().ui.brush_section = 0;
        h.run_steps(8);
        if h.state().ui.theme != kind || theme::Tokens::get(&h.ctx) != theme::Tokens::for_kind(kind) {
            return Err("wrong theme".into());
        }
        let before = h.state().session.tools.brush.clone();
        let presets = h.state().session.tools.presets.clone();
        let preset = presets.first().ok_or("preset")?;
        if preset.captures.size != capture {
            return Err("wrong capture flag".into());
        }
        let model_size = preset.picked_over(&before).size;
        let journal = h.state().session.journal.len();
        h.query_by_role_and_label(egui::accesskit::Role::Window, "Brush Settings").ok_or("window")?.query_by_label(name).ok_or("grid cell")?.click();
        h.run_steps(6);
        let after = h.state().session.tools.brush.clone();
        let observed = if phase == "before" { 73.0 } else { model_size };
        let mut expected = before.clone();
        expected.tip = saved.tip.clone();
        expected.size = observed;
        expected.hardness = saved.hardness;
        expected.angle = saved.angle;
        expected.roundness = saved.roundness;
        expected.spacing = saved.spacing;
        if after != expected || h.state().session.tools.presets != presets {
            return Err(format!("{label}: wrong brush or saved preset changed").into());
        }
        let calls = &h.state().session.journal[journal..];
        if calls.len() != 2 || calls.first().is_none_or(|c| c.0 != "brush.presets.setCurrent") || calls.last().is_none_or(|c| c.0 != "tools.setBrush") {
            return Err("wrong command route".into());
        }
        let patch = calls.last().map(|c| c.1["brush"].clone()).ok_or("patch")?;
        if (phase == "after" && !capture) && patch.get("size").is_some() {
            return Err("uncaptured size leaked into command".into());
        }
        let path = out.join(format!("capture-size-grid-{phase}-{label}.png"));
        h.render()?.save(&path)?;
        reports.push(json!({"case":label,"theme":kind.id(),"capture_size":capture,"scale":scale,"window":size,
            "before_size":before.size,"saved_size":saved.size,"after_size":after.size,"model_expected_size":model_size,
            "patch":patch,"journal_delta":2,"source_preset_unchanged":true,"other_fields_preserved":true,
            "current_preset":h.state().session.tools.current_preset,"png":path.file_name().and_then(|n|n.to_str())}));
    }
    std::fs::write(metadata, serde_json::to_string_pretty(&reports)?)?;
    Ok(())
}
