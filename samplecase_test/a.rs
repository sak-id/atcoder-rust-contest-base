#[path = "common.rs"]
mod common;

const BIN: &str = "./a";

const SAMPLE_JSON: &str = r#"
{
  "problem_url": "https://atcoder.jp/contests/abc156/tasks/abc156_a",
  "samples": [
    {
      "input": "3\n",
      "expected": "Yes\n"
    }
  ]
}
"#;

#[test]
fn samples() {
    common::run_json_samples(BIN, SAMPLE_JSON);
}
