//! Temporary synthetic ABR fixture generator for the Brush Pose axis investigation.
use photocraft_psd::{abr::write_v6, descriptor::{Descriptor, Value}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args_os().nth(1).map(std::path::PathBuf::from).ok_or("output directory required")?;
    std::fs::create_dir_all(&out)?;
    for (name, x, y) in [("none", false, false), ("x-only", true, false), ("y-only", false, true), ("both", true, true)] {
        let preset = Descriptor::new("brushPreset")
            .with("Nm  ", Value::Text(photocraft_psd::descriptor::UnicodeString::new_nul(&format!("Pose {name}"))))
            .with("Brsh", Value::Descriptor(Descriptor::new("computedBrush")
                .with("Dmtr", Value::UnitFloat { unit: *b"#Pxl", value: 47.0 })
                .with("Angl", Value::UnitFloat { unit: *b"#Ang", value: -23.0 })
                .with("Rndn", Value::UnitFloat { unit: *b"#Prc", value: 47.0 })))
            .with("useBrushPose", Value::Boolean(true))
            .with("brushPoseTiltX", Value::UnitFloat { unit: *b"#Prc", value: 50.0 })
            .with("brushPoseTiltY", Value::UnitFloat { unit: *b"#Prc", value: -20.0 })
            .with("overridePoseTiltX", Value::Boolean(x))
            .with("overridePoseTiltY", Value::Boolean(y))
            .with("useTipDynamics", Value::Boolean(true))
            .with("brushProjection", Value::Boolean(true));
        let bytes = write_v6(1, &[], &[], &[preset], false)?;
        std::fs::write(out.join(format!("{name}.abr")), bytes)?;
    }
    Ok(())
}
