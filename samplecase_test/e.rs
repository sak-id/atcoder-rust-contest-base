#[path = "common.rs"]
mod common;

const BIN: &str = "./e";

const SAMPLE_JSON: &str = r#"
{
  "problem_url": "https://atcoder.jp/contests/abc000/tasks/abc000_e",
  "samples": [
    {
      "input": "",
      "expected": "Yes\n"
    }
  ]
}
"#;

#[test]
fn samples() {
    common::run_json_samples(BIN, SAMPLE_JSON);
}
