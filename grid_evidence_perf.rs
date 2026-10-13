//! Temporary evidence: Brush Settings UI work on a 24 MP document, no canvas presentation.
use egui_kittest::Harness;
use photocraft_engine::{
    Session,
    paint::{BrushPreset, BrushSettings},
};
use photocraft_ui_egui::{PhotocraftApp, Services, brush_panel, brush_preview, theme};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reports = Vec::new();
    for count in [64, 2048, 8192] {
        let mut session = Session::new();
        session.execute("file.new", serde_json::json!({"width": 6000, "height": 4000}))?;
        session.tools.presets = (0..count)
            .map(|i| BrushPreset {
                name: format!("Grid tip {i:04}"),
                brush: BrushSettings { size: 47.0, ..Default::default() },
                builtin: false,
                group: "Synthetic grid".into(),
                folder: Vec::new(),
                captures: Default::default(),
            })
            .collect();
        session.tools.current_preset = Some("Grid tip 0007".into());
        let mut app = PhotocraftApp::new(session, Services::default());
        app.ui.panels.brush_settings = true;
        app.ui.brush_section = 0;
        let mut ready = false;
        let mut h = Harness::builder().with_size(egui::vec2(1200.0, 900.0)).build_ui_state(
            move |ui, app: &mut PhotocraftApp| {
                if !ready {
                    PhotocraftApp::setup_context(ui.ctx(), theme::ThemeKind::ProMedium);
                    ready = true;
                    return;
                }
                brush_panel::window(app, ui.ctx());
            },
            app,
        );
        h.run_steps(12);
        let doc = &h.state().session.active().ok_or("missing benchmark document")?.doc;
        if (doc.size.width, doc.size.height) != (6000, 4000) || h.state().session.tools.presets.len() != count {
            return Err("invalid benchmark fixture".into());
        }
        let before = h.state().session.tools.brush.clone();
        let journal = h.state().session.journal.len();
        let thumbnails = brush_preview::render_count(&h.ctx);
        let mut ms = Vec::new();
        for _ in 0..60 {
            let start = std::time::Instant::now();
            h.run_steps(1);
            ms.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        if h.state().session.tools.brush != before || h.state().session.journal.len() != journal || brush_preview::render_count(&h.ctx) != thumbnails {
            return Err("an idle benchmark frame changed state or rerendered thumbnails".into());
        }
        ms.sort_by(f64::total_cmp);
        reports.push(serde_json::json!({"presets": count, "frames": 60, "median_ms": (ms.get(29).ok_or("missing median")? + ms.get(30).ok_or("missing median")?) * 0.5, "p95_ms": ms.get(56).ok_or("missing p95")?, "cached_previews": thumbnails}));
    }
    println!(
        "{}",
        serde_json::json!({"document": [6000,4000], "scope": "Brush Settings egui CPU frame and cached thumbnails; excludes canvas composition and GPU presentation", "warmup_frames": 12, "runs": reports})
    );
    Ok(())
}
