# Claude Code Configuration

このファイルはClaude Codeがプロジェクトを理解するための設定ファイルです。

## プロジェクト概要

Rust製の環境設定値管理のライブラリ

## 開発環境

- 言語: Rust (edition 2024, MSRV 1.85.0, toolchain は stable)
- マネージャー: Cargo (ワークスペース構成)
- `make lint` には [cargo-machete](https://github.com/bnjbvr/cargo-machete) が必要 (`cargo install cargo-machete --locked`)
- `make udeps-all` には nightly ツールチェーンと [cargo-udeps](https://github.com/est31/cargo-udeps) が必要 (`rustup toolchain install nightly` と `cargo install cargo-udeps --locked`)

## よく使うコマンド

```bash
# ビルド
cargo build

# フォーマット
cargo fmt

# フォーマット/リント/未使用依存のチェック (cargo fmt --check + cargo clippy -- -D warnings + cargo machete)
make lint

# フォーマット/リント/未使用依存の自動修正 (cargo machete --fix + cargo fmt + cargo clippy --fix)
make lint-fix

# 全パッケージのテスト (ワークスペース外のテストパッケージ含む)
make test-all

# 全パッケージの未使用依存をコンパイルして厳密にチェック (nightly と cargo-udeps が必要)
make udeps-all

# 全パッケージのクリーン
make clean-all
```

## プロジェクト構造

```
packages
├── index    クレートのAPIを公開とドキュメント (portalenv)
├── core     コアになる部品の実装 (portalenv-core)
├── macro    proc-macroの実装 (portalenv-macro)
└── tests    ワークスペース外の結合テスト用パッケージ (テスト時の環境変数を分離するため個別のクレートとして配置)
```

## 開発時の規約

- コミットする前に必ず `make lint` と `make test-all` を実行する (CIも同じ内容を実行する)
- ソースコード (`.rs`、`Cargo.toml` など) のコメントは英語で書く
- コメントは必要最小限にする。コードを読めば分かる内容 (処理の言い換え、アサーションやエラーメッセージの繰り返しなど) は書かず、コードからは読み取れない理由や背景だけを書く
- rustのバージョンが上がった場合などに、trybuildのstderrの出力がズレたらTRYBUILD=overwriteで再生成する
- ドキュメントは、packages/index/src/docs/ 配下に配置されている。機能を変更したらドキュメントと README.md にも反映する
- テストは環境変数と `.env*` ファイルに依存するため、環境変数を書き換えるテストには `serial_test` の `#[serial]` を付ける
- `packages/tests/` にテストパッケージを追加したら、Makefile の `test-all` にも追記し、`Cargo.toml` に空の `[workspace]` を書く (親ディレクトリのワークスペースに誤って所属しないようにするため)
- バージョンは手動で変更しない。GitHub Release 時に CI がルート `Cargo.toml` の `#tag:release-version` 行を書き換えて crates.io に公開する
- `portalenv` クレートでは、再エクスポートとドキュメントのみ定義する
- `portalenv-core` クレートでは、コアな機能 (trait、struct, function など)を実装する
- `portalenv-macro` クレートでは、手続き型マクロを実装する (`rules/impl_macro.md` を参照)

## レビュー

レビューを依頼されたときは、`review-guidelines` スキルの観点に従う。
