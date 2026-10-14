//! Native evidence driver for the existing Brush Settings size rows (#3825).
use egui_kittest::{
    Harness,
    kittest::{NodeT, Queryable},
};
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
    for (label, kind, section, sampled, changed, scale) in [
        ("computed", theme::ThemeKind::ProMedium, 0, false, true, 1.0),
        ("original", theme::ThemeKind::ProMedium, 0, true, false, 1.0),
        ("dual-computed-light", theme::ThemeKind::ProLight, 4, false, true, 1.0),
        ("dual-changed-scale2", theme::ThemeKind::ProMedium, 4, true, true, 2.0),
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
        let mut b = BrushSettings::default();
        b.dual_brush.enabled = true;
        let tip = if sampled { TipShape::Sampled(GrayTile::from_fn(20, 12, |x, y| ((x + y) % 2) as f32)) } else { TipShape::Round };
        let size = if changed { 37.25 } else { 20.0 };
        if section == 0 {
            b.tip = tip;
            b.size = size;
        } else {
            b.dual_brush.tip = tip;
            b.dual_brush.size = size;
        }
        h.state_mut().run("tools.setBrush", json!({"brush":b}))?;
        h.state_mut().ui.panels.brush_settings = true;
        h.state_mut().ui.brush_tab = 0;
        h.state_mut().ui.brush_section = section;
        let journal = h.state().session.journal.len();
        let brush = h.state().session.tools.brush.clone();
        h.run_steps(8);
        if h.state().ui.theme != kind || theme::Tokens::get(&h.ctx) != theme::Tokens::for_kind(kind) {
            return Err("fixture theme mismatch".into());
        }
        let button = h.query_by_label("Restore original size");
        let visible = button.is_some();
        let disabled = button.as_ref().map(|n| n.accesskit_node().is_disabled());
        let rect = button.as_ref().map(|n| { let r = n.rect(); [r.min.x, r.min.y, r.max.x, r.max.y] });
        if phase == "before" {
            if visible != sampled || disabled == Some(true) {
                return Err(format!("{label}: baseline changed").into());
            }
        } else if !visible || disabled != Some(!sampled || !changed) {
            return Err(format!("{label}: wrong reset availability").into());
        }
        if h.state().session.tools.brush != brush || h.state().session.journal.len() != journal {
            return Err("idle UI changed the brush".into());
        }
        let path = out.join(format!("3825-size-restore-{phase}-{label}.png"));
        drop(button);
        h.render()?.save(&path)?;
        if let Some(button) = h.query_by_label("Restore original size") {
            button.click();
        }
        h.run_steps(4);
        let now = &h.state().session.tools.brush;
        let final_size = if section == 0 { now.size } else { now.dual_brush.size };
        let delta = h.state().session.journal.len() - journal;
        if final_size != if sampled { 20.0 } else { size } || delta != usize::from(sampled && changed) {
            return Err(format!("{label}: wrong click effect").into());
        }
        reports.push(json!({"case":label,"theme":kind.id(),"section":section,"scale":scale,"sampled":sampled,"size":size,"restore_visible":visible,"restore_disabled":disabled,"rect":rect,"idle_brush_unchanged":true,"click_size":final_size,"click_journal_delta":delta,"png":path.file_name().and_then(|n|n.to_str())}));
    }
    std::fs::write(metadata, serde_json::to_string_pretty(&reports)?)?;
    Ok(())
}
