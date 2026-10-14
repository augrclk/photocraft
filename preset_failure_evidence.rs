//! Native failed-preset-selection evidence for Brush Settings and the Brushes tab.
use egui_kittest::{
    Harness,
    kittest::{NodeT, Queryable},
};
use photocraft_engine::{
    Session,
    paint::{BrushPreset, BrushSettings, GrayTile, PresetCaptures, StoredTile, TipShape},
};
use photocraft_ui_egui::{PhotocraftApp, Services, state::Tool, theme};
use serde_json::json;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let phase = args.next().ok_or("phase")?;
    if phase != "before" && phase != "after" {
        return Err("phase".into());
    }
    let out = std::path::PathBuf::from(args.next().ok_or("output directory")?);
    let metadata = args.next().ok_or("metadata")?;
    let mut reports = Vec::new();
    for (case, kind, tab, had_current, valid, scale, size) in [
        ("grid-medium", theme::ThemeKind::ProMedium, 0, true, false, 1.0, [1200.0, 900.0]),
        ("grid-light", theme::ThemeKind::ProLight, 0, false, false, 1.0, [1200.0, 900.0]),
        ("picker-medium", theme::ThemeKind::ProMedium, 1, true, false, 1.0, [1200.0, 900.0]),
        ("picker-light", theme::ThemeKind::ProLight, 1, false, false, 1.0, [1200.0, 900.0]),
        ("grid-scale2", theme::ThemeKind::ProMedium, 0, true, false, 2.0, [900.0, 720.0]),
        ("valid-control", theme::ThemeKind::ProMedium, 0, true, true, 1.0, [1200.0, 900.0]),
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
        let working = BrushPreset {
            name: "Working tip".into(),
            brush: BrushSettings::default(),
            builtin: false,
            group: "Synthetic".into(),
            folder: Vec::new(),
            captures: Default::default(),
        };
        let name = if valid { "Available tip" } else { "Unavailable tip" };
        let tip = if valid {
            TipShape::Sampled(GrayTile::from_fn(16, 16, |_, _| 1.0))
        } else {
            TipShape::Stored(StoredTile {
                key: "unavailable-tip".into(),
                width: 16,
                height: 16,
                preview: Arc::new(GrayTile::from_fn(1, 1, |_, _| 1.0)),
                full: None,
            })
        };
        let chosen = BrushPreset {
            name: name.into(),
            brush: BrushSettings {
                tip,
                size: 73.0,
                hardness: 0.35,
                angle: 37.0,
                roundness: 0.45,
                spacing: 0.21,
                color: [1.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            captures: PresetCaptures { color: true, ..Default::default() },
            ..working.clone()
        };
        h.state_mut().session.tools.presets = vec![working, chosen];
        h.state_mut().session.tools.current_preset = None;
        h.state_mut().run("tools.setBrush", json!({"size":17.25,"opacity":0.37,"noise":true,"flipX":true,"spacingEnabled":false}))?;
        h.state_mut().run("tools.setColors", json!({"foreground":"#00ff00","background":"#0000ff"}))?;
        if had_current {
            h.state_mut().run("brush.presets.setCurrent", json!({"name":"Working tip"}))?;
        }
        h.state_mut().ui.panels.brush_settings = true;
        h.state_mut().ui.brush_tab = tab;
        h.state_mut().ui.brush_section = 0;
        h.run_steps(8);
        if h.state().ui.theme != kind || theme::Tokens::get(&h.ctx) != theme::Tokens::for_kind(kind) {
            return Err("wrong theme".into());
        }
        let before = h.state().session.tools.brush.clone();
        let current = h.state().session.tools.current_preset.clone();
        let presets = h.state().session.tools.presets.clone();
        let journal = h.state().session.journal.len();
        let rev = h.state().session.tools.brush_rev;
        let prefs = h.state().session.prefs().last_brush.clone();
        h.get_by_role_and_label(egui::accesskit::Role::Window, "Brush Settings").get_by_label(name).click();
        h.run_steps(6);
        let after = h.state().session.tools.brush.clone();
        let after_current = h.state().session.tools.current_preset.clone();
        let fg = h.state().session.tools.foreground;
        let calls = h.state().session.journal[journal..].to_vec();
        let expected_current = if valid || phase == "before" { Some(name.to_owned()) } else { current.clone() };
        let expected_fg = if !valid && phase == "before" && tab == 1 { [1.0, 0.0, 0.0, 1.0] } else { [0.0, 1.0, 0.0, 1.0] };
        let expected_calls = if valid {
            2
        } else if phase == "before" && tab == 0 {
            1
        } else {
            0
        };
        if after_current != expected_current
            || fg != expected_fg
            || calls.len() != expected_calls
            || h.state().session.tools.presets != presets
            || h.state().session.tools.background != [0.0, 0.0, 1.0, 1.0]
        {
            return Err(format!("{case}: wrong identity/color/journal/preset state").into());
        }
        let ids: Vec<&str> = calls.iter().map(|c| c.0.as_str()).collect();
        let expected_ids = if valid {
            if phase == "before" { vec!["brush.presets.setCurrent", "tools.setBrush"] } else { vec!["tools.setBrush", "brush.presets.setCurrent"] }
        } else if phase == "before" && tab == 0 {
            vec!["brush.presets.setCurrent"]
        } else {
            vec![]
        };
        if ids != expected_ids {
            return Err("wrong command route".into());
        }
        if !valid {
            if after != before || !h.state().ui.status.contains("no preset store") {
                return Err("wrong failed-pick state".into());
            }
            if phase == "after" && (h.state().session.tools.brush_rev != rev || h.state().session.prefs().last_brush != prefs) {
                return Err("failed pick changed revisions/persistence".into());
            }
        } else {
            let mut expected = before.clone();
            let p = presets.get(1).ok_or("preset")?;
            expected.tip = p.brush.tip.clone();
            expected.size = p.brush.size;
            expected.hardness = p.brush.hardness;
            expected.angle = p.brush.angle;
            expected.roundness = p.brush.roundness;
            expected.spacing = p.brush.spacing;
            if after != expected {
                return Err("valid tip changed unrelated fields".into());
            }
        }
        let update_enabled = if tab == 0 { Some(!h.get_by_label("Update the current brush with these settings").accesskit_node().is_disabled()) } else { None };
        if !valid && tab == 0 && update_enabled != Some(phase == "before" || had_current) {
            return Err("wrong Update availability".into());
        }
        let png = out.join(format!("preset-failure-{phase}-{case}.png"));
        h.render()?.save(&png)?;
        let status = h.state().ui.status.clone();
        let after_rev = h.state().session.tools.brush_rev;
        let retry_verified = if !valid {
            if let Some(p) = h.state_mut().session.tools.presets.get_mut(1) {
                p.brush.tip = TipShape::Sampled(GrayTile::from_fn(16, 16, |_, _| 1.0));
            }
            h.run_steps(24); // A separate click, outside the double-click window.
            h.get_by_role_and_label(egui::accesskit::Role::Window, "Brush Settings").get_by_label(name).click();
            h.run_steps(4);
            if h.state().session.tools.current_preset.as_deref() != Some(name) || h.state().session.tools.brush.size != 73.0 {
                return Err("available retry failed".into());
            }
            true
        } else {
            false
        };
        reports.push(json!({"case":case,"theme":kind.id(),"tab":tab,"had_current":had_current,"valid_control":valid,"scale":scale,"window":size,
            "before_current":current,"after_current":after_current,"before_foreground":[0.0,1.0,0.0,1.0],"after_foreground":fg,
            "before_size":before.size,"after_size":after.size,"brush_preserved_on_failure":!valid && before==after,"presets_unchanged":true,
            "journal_delta":calls.len(),"calls":calls,"before_brush_rev":rev,"after_brush_rev":after_rev,"update_enabled":update_enabled,
            "status":status,"available_retry_verified":retry_verified,"png":png.file_name().and_then(|s|s.to_str())}));
    }
    std::fs::write(metadata, serde_json::to_string_pretty(&reports)?)?;
    Ok(())
}
