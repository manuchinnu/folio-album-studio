use folio_core::{dimensions, interior_units, CanveraSize, InteriorUnit};

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
    let units = interior_units(count).map_err(|_| "Product spread count must be 1..39")?;
    let template = size.template();
    let entries: Vec<String> = units.iter().enumerate().map(|(i, unit)| {
        let pixels = dimensions(template, *unit);
        let kind = match unit {
            InteriorUnit::FirstSingle => "firstSingle",
            InteriorUnit::Spread => "spread",
            InteriorUnit::LastSingle => "lastSingle",
        };
        format!("{{\"index\":{},\"kind\":\"{}\",\"widthPx\":{},\"heightPx\":{}}}", i+1, kind, pixels.width, pixels.height)
    }).collect();
    // All strings below are fixed literals or allowlisted profile names.
    Ok(format!("{{\"profile\":\"{}\",\"dpi\":300,\"physicalPages\":{},\"structureProvisional\":true,\"productionCoverAllowed\":false,\"units\":[{}]}}", args[1], count*2+2, entries.join(",")))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("fingerprint") {
        if args.len() != 3 { eprintln!("Usage: folio-cli fingerprint <source-root> <relative-path>"); std::process::exit(2); }
        match folio_core::source_path::fingerprint(std::path::Path::new(&args[1]), &args[2]) {
            Ok(identity) => println!("{}", serde_json::to_string(&identity).unwrap()),
            Err(error) => { eprintln!("{error}"); std::process::exit(2); }
        }
        return;
    }
    if args == ["layout-grid"] {
        use std::io::Read;
        let mut input = String::new();
        let result = std::io::stdin().take(1_048_577).read_to_string(&mut input)
            .map_err(|e| e.to_string()).and_then(|_| folio_core::grid::layout_json(&input));
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
