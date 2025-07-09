# AtCoderRustテンプレート

個人用です。
https://github.com/rust-lang-ja/atcoder-rust-base を、2025年7月にcargo generateで取得し改変しました。

## 動作環境
- Macbook Air(M2)

## 改変内容
- rust-toolchainのバージョン変更

## 使い方
```shell
# A問題を実行
cargo run --bin a

# A問題をテスト
cargo test --test a

# A問題のみビルド
cargo build --release --bin a

# 全問題ビルド
cargo build --release
```

コンテスト前にやること
```shell
cargo generate --git (このリポジトリのリンク) --branch ja```