# como

「como」はターミナルで動作する、キーボード操作のみで完結するCLI型のタスク管理アプリケーションです。  
MySQLやPythonのターミナル操作のような直感的なCLI体験を提供します。
新規機能やバグの修正などをしていただける方がいれば、issueを立てたり、ご連絡いただけると嬉しいです！

## 特徴

- **クロスプラットフォーム対応**: Windows、macOS、Linuxで動作
- **CLI完結型**: ターミナルからコマンドラインで操作
- **ページ管理**: 複数のタスクリストをページごとに## 今後の予定

- ✅ **クロスプラットフォーム対応**: Windows、macOS、Linux対応完了
- 🔄 **パッケージマネージャー対応**: Homebrew、Winget、apt/yum などの各プラットフォームパッケージマネージャーでのインストール対応
- 📦 **自動リリース**: GitHub Actionsによる自動ビルド・リリース
- 🌐 **Web版の開発**: ブラウザで動作するWeb版
- 📱 **モバイル対応**: iOS/Android向けのモバイルアプリ
- 🔧 **プラグインシステム**: 機能拡張のためのプラグインアーキテクチャ
- 🎨 **テーマ・カスタマイゼーション**: カラーテーマや表示形式のカスタマイズカル保存**: JSONファイルでローカルにデータを保存
- **直感的操作**: シンプルで覚えやすいコマンド体系
- **高速表示オプション**: ID順ソート、ステータス別ソート、未完了フィルタ

## サポートプラットフォーム

- **Windows**: Windows 10以降
- **macOS**: macOS 10.15 (Catalina)以降
- **Linux**: Ubuntu 18.04以降、CentOS 7以降、その他主要ディストリビューション

## インストール

### 対応プラットフォーム

Comoは以下のプラットフォームで動作確認済みです：

| プラットフォーム | アーキテクチャ | 対応状況 | 推奨インストール方法 |
|----------------|--------------|----------|-------------------|
| **macOS** | Intel (x86_64) | ✅ 完全対応 | cargo install |
| **macOS** | Apple Silicon (ARM64) | ✅ 完全対応 | cargo install |
| **Windows** | x86_64 | ✅ 完全対応 | バイナリダウンロード |
| **Linux** | x86_64 | ✅ 完全対応 | cargo install |

### インストール方法

#### 1. Rustによるビルド（推奨）

```bash
# GitHubから直接インストール
cargo install --git https://github.com/ze-sudo/como

# または、ローカルでビルド
git clone https://github.com/ze-sudo/como
cd cli-rust-como
cargo install --path .
```

#### 2. バイナリダウンロード

各プラットフォーム用のプリビルドバイナリを[Releases](https://github.com/ze-sudo/como/releases)からダウンロード可能です：

- **Windows**: `como-x86_64-pc-windows-msvc.exe`
- **macOS (Intel)**: `como-x86_64-apple-darwin`
- **macOS (Apple Silicon)**: `como-aarch64-apple-darwin`
- **Linux**: `como-x86_64-unknown-linux-gnu`

```bash
# ダウンロード後の設定例 (macOS/Linux)
chmod +x como-*
sudo mv como-* /usr/local/bin/como

# パス確認
como --version
```

### 前提条件

#### Rustでビルドする場合

- **Rust**: 1.70以降
- **Cargo**: Rust付属版

#### バイナリ使用の場合

- 前提条件なし（各OSのネイティブバイナリ）

### ビルド方法

開発者向け：

```bash
# プロジェクトをクローン
git clone https://github.com/ze-sudo/como
cd cli-rust-como

# デバッグビルド
cargo build

# リリースビルド
cargo build --release

# ローカルテスト実行
./test-local.sh

# クロスプラットフォームビルド（要設定）
cargo build --target x86_64-pc-windows-msvc      # Windows
cargo build --target x86_64-unknown-linux-gnu    # Linux
```

### システムへのインストール

```bash
cargo install --path .
```

## 使い方

### 基本的なタスク操作

```bash
# タスク一覧を表示（デフォルト：ステータス別ソート）
como list

# ID順でタスク一覧を表示
como list -i
como list --id

# ステータス別ソート（デフォルトと同じ）
como list -s
como list --status

# 未完了タスクのみ表示
como list -u
como list --unchecked

# オプションの組み合わせ使用
como list -i -u    # ID順で未完了のみ
como list --id --unchecked

# タスクを追加
como add "企画書を作成する"

# タスクを編集
como edit 1 "企画書を完成させる"

# タスクを削除
como delete 1

# タスクのチェック状態を切り替え
como check 1
```

### ページ管理

```bash
# ページ一覧を表示（現在のページは*で表示）
como page

# 新しいページを作成
como page create work

# ページを切り替え
como page work

# ページを削除
como page delete work
```

## データ保存先

タスクデータは各OS固有のディレクトリに保存されます：

### Windows

```text
%APPDATA%\como\
```

### macOS

```text
~/Library/Application Support/como/
```

### Linux

```text
~/.config/como/
# または $XDG_CONFIG_HOME/como/
```

各プラットフォームで以下のファイルが作成されます：

- 各ページのデータ: `{ページ名}.json`
- 現在のページ情報: `current_page.txt`

## コマンド一覧

| コマンド | 説明 |
|---------|------|
| `como list` | タスク一覧を表示（デフォルト：ステータス別ソート） |
| `como list -i` / `como list --id` | ID順でタスク一覧を表示 |
| `como list -s` / `como list --status` | ステータス別ソート（デフォルトと同じ） |
| `como list -u` / `como list --unchecked` | 未完了タスクのみ表示 |
| `como add <タイトル>` | タスクを追加 |
| `como edit <ID> <新タイトル>` | タスクを編集 |
| `como delete <ID>` | タスクを削除 |
| `como check <ID>` | チェック状態を切り替え |
| `como page` | ページ一覧を表示 |
| `como page <ページ名>` | ページを切り替え |
| `como page create <ページ名>` | ページを作成 |
| `como page delete <ページ名>` | ページを削除 |

## データ構造

### タスク (Task)

```json
{
  "id": 1,
  "title": "タスクのタイトル",
  "completed": false
}
```

### ページデータ (PageData)

```json
{
  "last_id": 2,
  "tasks": [
    {
      "id": 1,
      "title": "最初のタスク",
      "completed": true
    },
    {
      "id": 2,
      "title": "二番目のタスク",
      "completed": false
    }
  ]
}
```

## 開発

### プロジェクト構造

```text
src/
├── main.rs      # エントリーポイント
├── lib.rs       # コアロジック
├── models.rs    # データ構造定義
├── storage.rs   # ファイル操作
└── cli.rs       # CLI定義
```

### テスト実行

```bash
cargo test
```

### フォーマット

```bash
cargo fmt
```

### リント

```bash
cargo clippy
```

## ライセンス

MIT License

## 今後の予定

- ✅ **Web版開発**:


## 貢献

新機能の提案やバグの修正などの貢献を歓迎します！

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

問題や提案があれば、[Issues](https://github.com/ze-sudo/como/issues)からお知らせください。

