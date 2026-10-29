use folio_core::CanveraSize;

fn profile_json(args: &[String]) -> Result<String, &'static str> {
    if args.len() != 3 || args[0] != "inspect-profile" {
        return Err("Usage: folio-cli inspect-profile <12x12|12x15L|12x18L> <spread-count:1..39>");
    }
    let size = match args[1].as_str() {
        "12x12" => CanveraSize::Square12,
        "12x15L" => CanveraSize::Landscape12x15,
        "12x18L" => CanveraSize::Landscape12x18,
        _ => return Err("Unknown profile; expected 12x12, 12x15L or 12x18L"),
    };
    let count = args[2].parse::<u32>().map_err(|_| "Spread count must be an integer")?;
    let plan = folio_core::export_plan::geometry_proof(size, count)
        .map_err(|_| "Product spread count must be 1..39")?;
    serde_json::to_string(&plan).map_err(|_| "Cannot serialize proof plan")
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("save-unit-proof") {
        use std::io::Read;
        if args.len()!=3 { eprintln!("Usage: folio-cli save-unit-proof <source-root> <new-jpeg-path> (JSON on stdin)"); std::process::exit(2); }
        let mut input = String::new();
        let result = std::io::stdin().take(1_048_577).read_to_string(&mut input)
            .map_err(|e|e.to_string()).and_then(|_| folio_core::raster::save_unit_json(
                &input,std::path::Path::new(&args[1]),std::path::Path::new(&args[2])));
        match result {
            Ok(assumed) => println!("{}",serde_json::json!({"saved":true,"proofOnly":true,"assumedSrgbSources":assumed})),
            Err(error) => { eprintln!("{error}"); std::process::exit(2); }
        }
        return;
    }
    if args.first().map(String::as_str) == Some("save-source-proof") {
        if args.len() != 7 {
            eprintln!("Usage: folio-cli save-source-proof <root> <relative-path> <sha256> <max-bytes> <max-pixels> <new-jpeg-path>");
            std::process::exit(2);
        }
        let result = (|| -> Result<bool, String> {
            let bytes = args[4].parse::<u64>().map_err(|e| e.to_string())?;
            let pixels = args[5].parse::<u64>().map_err(|e| e.to_string())?;
            let decoded = folio_core::decode::verified_oriented(
                std::path::Path::new(&args[1]), &args[2], &args[3], bytes, pixels)?;
            let (normalized, assumed) = folio_core::color::normalize_with_provenance(decoded)?;
            let rgb = normalized.pixels.as_rgb8().ok_or("Source proof currently requires opaque RGB8; alpha/depth conversion must be explicit")?;
            let icc = normalized.icc_profile.as_ref().ok_or("Missing normalized ICC")?;
            let jpeg = folio_core::jpeg_proof::encode(rgb, icc, 95)?;
            folio_core::publication::write_new(std::path::Path::new(&args[6]), &jpeg)
                .map_err(|e| e.to_string())?;
            Ok(assumed)
        })();
        match result {
            Ok(assumed) => println!("{}", serde_json::json!({"saved":true,"proofOnly":true,"assumedSrgb":assumed})),
            Err(error) => { eprintln!("{error}"); std::process::exit(2); }
        }
        return;
    }
    if matches!(args.first().map(String::as_str), Some("inspect-image" | "inspect-normalized-image" | "sample-normalized-image" | "sample-resized-image")) {
        let resize = args[0] == "sample-resized-image";
        if args.len() != if resize { 8 } else { 6 } {
            eprintln!("Usage: folio-cli inspect-image <root> <relative-path> <sha256> <max-bytes> <max-pixels>");
            std::process::exit(2);
        }
        let sample = args[0] == "sample-normalized-image" || resize;
        let normalized = args[0] != "inspect-image";
        let result = args[4].parse::<u64>().and_then(|bytes|
            args[5].parse::<u64>().map(|pixels| (bytes,pixels)))
            .map_err(|_| "Budgets must be positive integers".to_string())
            .and_then(|(bytes,pixels)| folio_core::decode::verified_oriented(
                std::path::Path::new(&args[1]), &args[2], &args[3], bytes,
                if sample { pixels.min(4096) } else { pixels }))
            .and_then(|image| if normalized { folio_core::color::normalize_with_provenance(image) } else { Ok((image, false)) })
            .and_then(|(mut image, assumed)| {
                if resize {
                    let width = args[6].parse::<u32>().map_err(|e| e.to_string())?;
                    let height = args[7].parse::<u32>().map_err(|e| e.to_string())?;
                    if u64::from(width)*u64::from(height) > 4096 { return Err("Diagnostic output exceeds 4096 pixels".into()); }
                    let rgb = image.pixels.as_rgb8().ok_or("Resize diagnostic requires RGB8")?;
                    image.pixels = folio_core::raster::resize_tile(rgb,width,height,64*1024*1024)?.into();
                }
                Ok((image, assumed))
            });
        match result {
            Ok((image, assumed_srgb)) => {
                let samples = if sample { folio_core::color::sample_values(&image) } else { serde_json::Value::Null };
                println!("{}", serde_json::json!({
                "width":image.pixels.width(), "height":image.pixels.height(),
                "colorType":format!("{:?}", image.pixels.color()),
                "iccBytes":image.icc_profile.as_ref().map_or(0, Vec::len),
                "orientationApplied":true, "colorNormalized":normalized,
                "samples": samples, "assumedSrgb": assumed_srgb
            })); },
            Err(error) => { eprintln!("{error}"); std::process::exit(2); }
        }
        return;
    }
    if args.first().map(String::as_str) == Some("save-proof-plan") {
        if args.len() != 4 {
            eprintln!("Usage: folio-cli save-proof-plan <12x12|12x15L|12x18L> <spread-count:1..39> <new-json-path>");
            std::process::exit(2);
        }
        let profile_args = vec!["inspect-profile".to_owned(), args[1].clone(), args[2].clone()];
        let result = profile_json(&profile_args).map_err(str::to_owned).and_then(|json| {
            folio_core::publication::write_new(std::path::Path::new(&args[3]), json.as_bytes())
                .map_err(|e| format!("Cannot publish proof plan: {e}"))
        });
        match result {
            Ok(()) => println!("{{\"saved\":true,\"proofOnly\":true}}"),
            Err(error) => { eprintln!("{error}"); std::process::exit(2); }
        }
        return;
    }
    if args.first().map(String::as_str) == Some("fingerprint") {
        if args.len() != 3 { eprintln!("Usage: folio-cli fingerprint <source-root> <relative-path>"); std::process::exit(2); }
        match folio_core::source_path::fingerprint(std::path::Path::new(&args[1]), &args[2]) {
            Ok(identity) => println!("{}", serde_json::to_string(&identity).unwrap()),
            Err(error) => { eprintln!("{error}"); std::process::exit(2); }
        }
        return;
    }
    if args == ["layout-grid"] || args == ["compose-grid"] {
        use std::io::Read;
        let mut input = String::new();
        let result = std::io::stdin().take(1_048_577).read_to_string(&mut input)
            .map_err(|e| e.to_string()).and_then(|_| {
                if args[0] == "compose-grid" { folio_core::grid::composition_json(&input) }
                else { folio_core::grid::layout_json(&input) }
            });
        match result {
            Ok(json) => println!("{json}"),
            Err(error) => { eprintln!("{error}"); std::process::exit(2); }
        }
        return;
    }
    if args == ["validate-crop"] {
        use std::io::Read;
        let mut input = String::new();
        let result = std::io::stdin().take(4097).read_to_string(&mut input)
            .map_err(|e| e.to_string())
            .and_then(|_| {
                if input.len() > 4096 { return Err("Crop input exceeds 4096 bytes".to_string()); }
                serde_json::from_str::<folio_core::crop::Crop>(&input).map_err(|e| e.to_string())
            });
        match result {
            Ok(crop) => println!("{}", serde_json::to_string(&crop).expect("validated finite crop")),
            Err(error) => { eprintln!("{error}"); std::process::exit(2); }
        }
        return;
    }
    match profile_json(&args) {
        Ok(json) => println!("{json}"),
        Err(message) => { eprintln!("{message}"); std::process::exit(2); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn args(values: &[&str]) -> Vec<String> { values.iter().map(|v| v.to_string()).collect() }
    #[test]
    fn invalid_arguments_do_not_produce_a_manifest() {
        for values in [vec![], vec!["inspect-profile", "unknown", "1"], vec!["inspect-profile", "12x15L", "40"], vec!["inspect-profile", "12x15L", "-1"], vec!["inspect-profile", "12x15L", "1", "extra"]] {
            assert!(profile_json(&args(&values)).is_err());
        }
    }
}
