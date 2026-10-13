//! Temporary same-workload 24 MP benchmark; uses the legacy combined input in both revisions.
use photocraft_color::PixelFormat;
use photocraft_geom::Rect;
use photocraft_paint::{BrushSettings, StrokePoint, StrokeRenderer};
use photocraft_raster::Surface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = PixelFormat::RGBA8;
    let mut photo = Surface::with_default(format, &photocraft_raster::from_rgba(&format, [1.0; 4]));
    photo.fill_rect(Rect::new(0, 0, 6000, 4000), &photocraft_raster::from_rgba(&format, [0.8, 0.7, 0.6, 1.0]));
    let points: Vec<_> = (0..512)
        .map(|i| {
            let t = f64::from(i) / 511.0;
            StrokePoint {
                x: 500.0 + t * 5000.0,
                y: 2000.0 + (t * 8.0).sin() * 1200.0,
                tilt_x: 13.0 - 44.0 * t as f32,
                tilt_y: -27.0 + 69.0 * t as f32,
                pressure: 0.71,
                time: f64::from(i) * 8.0,
                ..Default::default()
            }
        })
        .collect();
    let mut reports = Vec::new();
    for enabled in [false, true] {
        let brush: BrushSettings = serde_json::from_value(serde_json::json!({
            "size": 64, "roundness": 0.47, "angle": -23, "spacing": 0.25,
            "pressureSize": false, "smoothing": {"enabled": false},
            "shapeDynamics": {"enabled": true, "brushProjection": true},
            "pose": {"enabled": enabled, "tiltX": 45, "tiltY": -18, "overrideTilt": true}
        }))?;
        let mut samples = Vec::new();
        let mut counts = Vec::new();
        let mut checksums = Vec::new();
        for _ in 0..6 {
            let mut surface = photo.clone();
            let start = std::time::Instant::now();
            let mut renderer = StrokeRenderer::new(&brush, Some(format), 1.0);
            for chunk in points.chunks(16) {
                renderer.push(chunk);
                renderer.composite(&photo, &mut surface, None, false, false);
            }
            renderer.finish();
            renderer.composite(&photo, &mut surface, None, false, false);
            let ms = start.elapsed().as_secs_f64() * 1000.0;
            let checksum: u64 = points.iter().flat_map(|p| surface.rgba(p.x.round() as i32, p.y.round() as i32)).map(|v| u64::from(v.to_bits())).sum();
            samples.push(ms);
            counts.push(renderer.dab_count());
            checksums.push(checksum);
        }
        let steady: Vec<_> = samples.iter().skip(1).copied().collect();
        let mut sorted = steady.clone();
        sorted.sort_by(f64::total_cmp);
        let median = sorted.get(2).copied().ok_or("missing steady samples")?;
        reports.push(serde_json::json!({"pose_enabled": enabled, "median_ms": median, "steady_ms": steady, "warmup_ms": samples.first(), "dab_counts": counts, "sample_checksums": checksums}));
    }
    println!("{}", serde_json::json!({"dimensions": [6000,4000], "pixels": 24000000, "format": "RGBA8", "points":512, "chunk_size":16, "runs":reports}));
    Ok(())
}
