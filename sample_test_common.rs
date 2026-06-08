#[path = "samplecase_common.rs"]
mod samplecase_common;

use std::fs;
use std::path::PathBuf;

fn current_problem() -> String {
    let exe_path = std::env::current_exe().expect("failed to get current test executable path");
    let file_stem = exe_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("failed to get current test executable name");

    file_stem
        .split_once('-')
        .map_or(file_stem, |(name, _)| name)
        .to_owned()
}

fn sample_json_path(problem: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(problem)
        .join("sample.json")
}

#[test]
fn samples() {
    let problem = current_problem();
    let sample_json_path = sample_json_path(&problem);
    let sample_json = fs::read_to_string(&sample_json_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {}", sample_json_path.display(), err));

    samplecase_common::run_json_samples(&format!("./{}", problem), &sample_json);
}
