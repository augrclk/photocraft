//! Native preset-confirmation evidence with automation-supplied invalid settings.
use egui_kittest::{Harness, kittest::Queryable};
use photocraft_engine::Session;
use photocraft_ui_egui::{PhotocraftApp, Services, brush_preset_dialog, control, state::Tool, theme};
use serde_json::{Value, json};

fn call(app: &mut PhotocraftApp, ctx: &egui::Context, method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let (request, _receiver) = control::ControlRequest::new(method, params);
    match control::handle(app, ctx, &request) {
        control::Outcome::Done(reply) => Ok(reply),
        _ => Err("expected immediate control reply".into()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let phase = args.next().ok_or("phase")?;
    if phase != "before" && phase != "after" {
        return Err("phase".into());
    }
    let out = std::path::PathBuf::from(args.next().ok_or("output directory")?);
    let metadata = args.next().ok_or("metadata")?;
    let mut reports = Vec::new();
    for (case, kind, defect, scale) in [
        ("size-medium", theme::ThemeKind::ProMedium, "size", 1.0),
        ("scatter-light", theme::ThemeKind::ProLight, "scatter", 1.0),
        ("duplicate-medium", theme::ThemeKind::ProMedium, "duplicate", 1.0),
        ("size-scale2", theme::ThemeKind::ProMedium, "size", 2.0),
        ("valid-control", theme::ThemeKind::ProMedium, "valid", 1.0),
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
        h.state_mut().run("prefs.set", json!({"path":"interface.theme","value":kind.id()}))?;
        h.state_mut().run("prefs.set", json!({"path":"interface.language","value":"en"}))?;
        h.state_mut().run("file.new", json!({"width":480,"height":360}))?;
        h.state_mut().ui.tool = Tool::Brush;
        h.run_steps(3);
        h.state_mut().run("tools.setBrush", json!({"size":17.25,"noise":true,"opacity":0.37}))?;
        h.state_mut().run("tools.setColors", json!({"foreground":"#00ff00","background":"#0000ff"}))?;
        h.state_mut().run("brush.presets.save", json!({"name":"Working preset"}))?;
        h.state_mut().ui.panels.brush_settings = true;
        h.state_mut().ui.brush_tab = 0;
        h.run_steps(5);
        h.get_by_role_and_label(egui::accesskit::Role::Window, "Brush Settings").get_by_label("Panel menu").click();
        h.run_steps(2);
        h.get_by_label("New Brush Preset…").click();
        h.run_steps(6);
        if h.state().ui.theme != kind || theme::Tokens::get(&h.ctx) != theme::Tokens::for_kind(kind) {
            return Err("theme".into());
        }
        let id = h.state().ui.dialogs.iter().find(|d| brush_preset_dialog::owns(&d.fields)).ok_or("dialog")?.id;
        let ctx = h.ctx.clone();
        let name = if defect == "duplicate" { "working PRESET" } else { "Agent preset" };
        let patch = match defect {
            "size" => json!({"size":5001.0}),
            "scatter" => json!({"size":17.25,"scattering":{"enabled":true,"scatter":{"jitter":11.0}}}),
            _ => json!({"size":17.25}),
        };
        for (field, value) in [("name", json!(name)), ("brush", patch.clone()), ("toolSettings", json!(true)), ("color", json!(true))] {
            let response = call(h.state_mut(), &ctx, "ui.dialog.set", json!({"dialog":id,"field":field,"value":value}))?;
            if response["ok"] != true {
                return Err("setting form fields failed".into());
            }
        }
        h.run_steps(4);
        let fields = h.state().ui.dialogs.iter().find(|d| d.id == id).ok_or("dialog")?.fields.clone();
        let presets = h.state().session.tools.presets.clone();
        let brush = h.state().session.tools.brush.clone();
        let current = h.state().session.tools.current_preset.clone();
        let rev = (h.state().session.tools.brush_rev, h.state().session.tools.brush_saved_rev, h.state().session.tools.presets_rev);
        let prefs = h.state().session.prefs().last_brush.clone();
        let journal = h.state().session.journal.len();
        let mut reply = Value::Null;
        if defect == "duplicate" {
            reply = call(h.state_mut(), &ctx, "ui.dialog.confirm", json!({"dialog":id}))?;
            if reply["ok"] != false {
                return Err("duplicate must fail on both revisions".into());
            }
        } else {
            h.get_by_label("OK").click();
        }
        h.run_steps(6);
        let failed = phase == "after" && defect != "valid";
        let accepted = defect == "valid" || (phase == "before" && defect != "duplicate");
        let open = h.state().ui.dialogs.iter().find(|d| d.id == id);
        let expected_open = phase == "after" && defect != "valid";
        if open.is_some() != expected_open {
            return Err(format!("{case}: incorrect dialog retention").into());
        }
        let retained_fields = open.is_some_and(|d| d.fields == fields);
        if expected_open && !retained_fields {
            return Err("form fields changed on failure".into());
        }
        let calls = h.state().session.journal[journal..].to_vec();
        if calls.len() != usize::from(accepted)
            || h.state().session.tools.brush != brush
            || h.state().session.tools.foreground != [0.0, 1.0, 0.0, 1.0]
            || h.state().session.tools.background != [0.0, 0.0, 1.0, 1.0]
        {
            return Err("wrong brush/color/command state".into());
        }
        if accepted {
            if calls.first().map(|c| c.0.as_str()) != Some("brush.presets.save")
                || h.state().session.tools.presets.len() != presets.len() + 1
                || h.state().session.tools.presets.get(..presets.len()) != Some(presets.as_slice())
                || h.state().session.tools.current_preset.as_deref() != Some(name)
            {
                return Err("wrong accepted save".into());
            }
        } else if h.state().session.tools.presets != presets
            || h.state().session.tools.current_preset != current
            || rev != (h.state().session.tools.brush_rev, h.state().session.tools.brush_saved_rev, h.state().session.tools.presets_rev)
            || h.state().session.prefs().last_brush != prefs
        {
            return Err("rejection mutated stored/working state".into());
        }
        if failed && defect == "size" && !h.state().ui.status.contains("at most 5000") {
            return Err("missing size error".into());
        }
        if failed && defect == "scatter" && !h.state().ui.status.contains("scatter amount") {
            return Err("missing scatter error".into());
        }
        let png = out.join(format!("preset-bounds-{phase}-{case}.png"));
        h.render()?.save(&png)?;
        let after_current = h.state().session.tools.current_preset.clone();
        let after_count = h.state().session.tools.presets.len();
        let status = h.state().ui.status.clone();
        let mut old_created_preset_is_unselectable = false;
        if phase == "before" && (defect == "size" || defect == "scatter") {
            old_created_preset_is_unselectable = h.state_mut().run("tools.setBrush", json!({"preset":name})).is_err();
            if !old_created_preset_is_unselectable {
                return Err("baseline should have stored an unusable preset".into());
            }
        }
        let mut retry_verified = false;
        if failed {
            for (field, value) in [("name", json!("Corrected preset")), ("brush", json!({"size":17.25}))] {
                if call(h.state_mut(), &ctx, "ui.dialog.set", json!({"dialog":id,"field":field,"value":value}))?["ok"] != true {
                    return Err("retry field update".into());
                }
            }
            h.run_steps(3);
            h.get_by_label("OK").click();
            h.run_steps(4);
            if h.state().ui.dialogs.iter().any(|d| d.id == id)
                || h.state().session.tools.presets.len() != presets.len() + 1
                || h.state().session.journal.len() != journal + 1
            {
                return Err("retry failed".into());
            }
            h.state_mut().run("tools.setBrush", json!({"preset":"Corrected preset"}))?;
            retry_verified = true;
        }
        reports.push(json!({"case":case,"theme":kind.id(),"scale":scale,"defect":defect,"brush_patch_supplied_via_ui_dialog_set":patch,"dialog_open":expected_open,"fields_retained":retained_fields,"before_current":current,"after_current":after_current,"before_count":presets.len(),"after_count":after_count,"journal_delta":calls.len(),"calls":calls,"status":status,"control_reply":reply,"working_brush_preserved":true,"old_presets_preserved":true,"old_created_preset_is_unselectable":old_created_preset_is_unselectable,"retry_verified":retry_verified,"png":png.file_name().and_then(|p|p.to_str())}));
    }
    std::fs::write(metadata, serde_json::to_string_pretty(&reports)?)?;
    Ok(())
}
