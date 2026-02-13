use std::collections::HashMap;

use cli_test_dir::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SampleFile {
    #[serde(default)]
    problem_url: Option<String>,
    samples: Vec<SampleCase>,
}

#[derive(Debug, Deserialize)]
struct SampleCase {
    input: String,
    expected: String,
}

pub fn run_json_samples(bin: &str, json: &str) {
    let sample_file: SampleFile = serde_json::from_str(json)
        .unwrap_or_else(|err| panic!("failed to parse sample JSON for {}: {}", bin, err));

    if sample_file.samples.is_empty() {
        panic!("samples must not be empty for {}", bin);
    }

    let problem_url = sample_file.problem_url.as_deref().unwrap_or("problem_url not set");
    let mut seen_pairs: HashMap<(String, String), usize> = HashMap::new();

    for (index, sample) in sample_file.samples.iter().enumerate() {
        let key = (sample.input.clone(), sample.expected.clone());
        if let Some(first_index) = seen_pairs.insert(key, index) {
            panic!(
                "duplicate sample pair detected for {} at samples[{}] (first at samples[{}])",
                bin, index, first_index
            );
        }

        let testdir = TestDir::new(bin, "");
        let output = testdir
            .cmd()
            .output_with_stdin(&sample.input)
            .tee_output()
            .expect_success();

        assert_eq!(
            output.stdout_str(),
            sample.expected.as_str(),
            "stdout mismatch for {} sample[{}] ({})",
            bin,
            index,
            problem_url,
        );
        assert!(
            output.stderr_str().is_empty(),
            "stderr is not empty for {} sample[{}] ({}): {}",
            bin,
            index,
            problem_url,
            output.stderr_str(),
        );
    }
}
