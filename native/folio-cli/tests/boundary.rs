use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn saved_plan_is_complete_and_existing_work_is_preserved() {
    let root = std::env::temp_dir().join(format!("folio-plan-cli-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let path = root.join("proof.json");
    let run = |count: &str| Command::new(env!("CARGO_BIN_EXE_folio-cli"))
        .args(["save-proof-plan", "12x15L", count]).arg(&path).output().unwrap();
    let invalid = run("40");
    assert_eq!(invalid.status.code(), Some(2));
    assert!(!path.exists());
    let first = run("39");
    assert!(first.status.success(), "{}", String::from_utf8_lossy(&first.stderr));
    let bytes = std::fs::read(&path).unwrap();
    let plan: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(plan["proofOnly"], true);
    assert_eq!(plan["units"].as_array().unwrap().len(), 41);
    let overwrite = run("1");
    assert_eq!(overwrite.status.code(), Some(2));
    assert!(overwrite.stdout.is_empty());
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    assert_eq!(std::fs::read_dir(&root).unwrap().count(), 1);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_dir(root).unwrap();
}

fn validate(input: &[u8]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_folio-cli"))
        .arg("validate-crop").stdin(Stdio::piped()).stdout(Stdio::piped())
        .stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn crop_boundary_rejects_bad_requests_without_success_output() {
    for input in [r#"{"x":0,"y":0,"w":1,"h":1,"extra":true}"#,
        r#"{"x":0.9,"y":0,"w":1,"h":1}"#, "not json"] {
        let output = validate(input.as_bytes());
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
    let valid = validate(br#"{"x":0.25,"y":0,"w":0.5,"h":1}"#);
    assert!(valid.status.success());
    let parsed: serde_json::Value = serde_json::from_slice(&valid.stdout).unwrap();
    assert_eq!(parsed["w"], 0.5);
}

#[test]
fn exact_profile_is_valid_json_at_process_boundary() {
    let output = Command::new(env!("CARGO_BIN_EXE_folio-cli"))
        .args(["inspect-profile", "12x15L", "39"]).output().unwrap();
    assert!(output.status.success());
    let p: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let units = p["units"].as_array().unwrap();
    assert_eq!(units.len(), 41);
    assert_eq!(p["physicalPages"], 80);
    assert_eq!(units[0]["widthPx"], 4394);
    assert_eq!(units[40]["widthPx"], 4394);
    assert!(units[1..40].iter().all(|u| u["widthPx"] == 8787));
}

#[test]
fn grid_request_runs_through_executable_and_rejects_unknown_fields() {
    let sources: Vec<_> = (0..3).map(|i| serde_json::json!({
        "sha256":format!("{i:064x}"), "width":30000, "height":30000, "faces":[]
    })).collect();
    let mut request = serde_json::json!({"profile":"12x15L", "spreads":1,
        "minimumPpi":200, "faceInsetMm":10, "sources":sources});
    let mut child = Command::new(env!("CARGO_BIN_EXE_folio-cli"))
        .arg("compose-grid").stdin(Stdio::piped()).stdout(Stdio::piped())
        .stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(request.to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let plan: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(plan["geometry"]["proofOnly"], true);
    assert_eq!(plan["geometry"]["units"][1]["widthPx"], 8787);
    assert_eq!(plan["images"][2][0]["sourceIndex"], 2);
    assert_eq!(plan["images"][1][0]["matrix"].as_array().unwrap().len(), 6);
    for valid in [true, false] {
        if !valid { request["unknown"] = true.into(); }
        let mut child = Command::new(env!("CARGO_BIN_EXE_folio-cli"))
            .arg("layout-grid").stdin(Stdio::piped()).stdout(Stdio::piped())
            .stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(request.to_string().as_bytes()).unwrap();
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.success(), valid);
        if valid {
            let pages: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(pages.as_array().unwrap().len(), 3);
            assert_eq!(pages[2][0]["source_index"], 2);
            assert!(pages[1][0]["rect"]["width"].as_f64().unwrap() > 700.0);
        } else { assert!(output.stdout.is_empty()); }
    }
}
