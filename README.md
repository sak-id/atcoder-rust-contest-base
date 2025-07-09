# AtCoderRustテンプレート

個人用です。
https://github.com/rust-lang-ja/atcoder-rust-base/tree/ja を、2025年7月にcargo generateで取得し改変しました。

## 動作環境
- Macbook Air(M2)

## 改変内容
- rust-toolchainのバージョン変更
- 各問題テンプレート：A問題~G問題までの解答ファイル・テストファイルを追加
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
`samplecase_test`ディレクトリ以下のファイルの該当の箇所にコピペしてください。

```shell
# ディレクトリを作成
cargo generate --git (このリポジトリのリンク)

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