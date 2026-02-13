# AtCoderRustテンプレート

個人用です。
https://github.com/rust-lang-ja/atcoder-rust-base/tree/ja を、2025年7月にcargo generateで取得し改変しました。

## 動作確認環境
- Macbook Air(M2)

## 改変内容
- rust-toolchainのバージョン変更
- 各問題テンプレート：A問題~G問題までの解答ファイル・テストファイルを追加
  - A問題のみ自分の好みにカスタマイズしています
- テスト用シェルスクリプト：`./test [a-g]`で各問題のテストが可能

## 基本の使い方
```shell
# A問題を実行
cargo run --bin a

# A問題をテスト
cargo test --test a
##  または
./test.sh a

# A問題のみビルド
cargo build --release --bin a

# 全問題ビルド
cargo build --release
```

## コンテストでの使い方
テストケース等の取得はしません。
`samplecase_test/a.rs` などの `SAMPLE_JSON` に JSON 本文を貼り付けてください。

貼り付け形式（objectのみ）:
```json
{
  "problem_url": "https://atcoder.jp/contests/abc156/tasks/abc156_c",
  "samples": [
    {
      "input": "2\n1 4",
      "expected": "5\n"
    },
    {
      "input": "7\n14 14 2 13 56 2 37",
      "expected": "2354\n"
    }
  ]
}
```

判定ルール:
- `stdout` と `expected` は完全一致
- `stderr` は空であること
- 同一 `input + expected` ペアが重複した場合はテスト失敗

```shell
# ディレクトリを作成
cargo generate --git git@github.com:sak-id/atcoder-rust-contest-base.git

# 事前にビルドしておくといいらしい
cargo build --release

# a問題の解答コード、テストケースを入力した後にテストを行う
./test.sh a 

```

## 補足
`./test.sh a`はtest.shファイルへの実行権限が必要
```shell
chmod +x test.sh
```

サンプルケース
- A~G問題: 各1ケース（出力例: "Yes\\n"）
