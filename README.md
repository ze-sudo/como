# como

「como」はターミナルで動作する、キーボード操作のみで完結するCLI型のタスク管理アプリケーションです。  
MySQLやPythonのターミナル操作のような直感的なCLI体験を提供します。
新規機能やバグの修正などをしていただける方がいれば、issueを立てたり、ご連絡いただけると嬉しいです！

## 特徴

- **クロスプラットフォーム対応**: Windows、macOS、Linuxで動作
- **インタラクティブモード**: `como`と入力するだけで対話型シェルが起動
- **CLI完結型**: ターミナルからコマンドラインで操作
- **ページ管理**: 複数のタスクリストをページごとに管理
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

#### Rustによるビルド（推奨）

```bash
# GitHubから直接インストール
cargo install --git https://github.com/ze-sudo/como

# または、ローカルでビルド
git clone https://github.com/ze-sudo/como
cd cli-rust-como
cargo install --path .
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

### インタラクティブモード（推奨）

`como`とだけ入力すると、対話型シェルが起動します：

```bash
$ como
como - インタラクティブモード
終了するには 'quit', 'exit', ':q' または Ctrl+C を入力してください
コマンド一覧は 'help' を入力してください

=== default ページのタスク一覧 ===
[✓] 1 - 完了したタスク
[ ] 2 - 未完了のタスク

como(default)> add 新しいタスク
タスクを追加しました: [3] 新しいタスク

como(default)> check 3
タスク 3 を 完了 に設定しました

como(default)> list
=== default ページのタスク一覧 ===
[✓] 1 - 完了したタスク
[✓] 3 - 新しいタスク
[ ] 2 - 未完了のタスク

como(default)> quit
さようなら！
```

#### インタラクティブモードの特徴

- **プロンプト表示**: `como(ページ名)>` の形式で現在のページを表示
- **起動時にタスク一覧を自動表示**
- **コマンド履歴**: 上下キーで過去のコマンドを呼び出せる
- **行編集機能**: カーソル移動、削除などの編集が可能

#### インタラクティブモードの終了方法

- `quit` - 終了
- `exit` - 終了
- `:q` - 終了（Vimスタイル）
- `Ctrl+C` - 終了
- `Ctrl+D` - 終了

### CLIモード

従来通り、コマンドを直接指定して実行することも可能です：

```bash
# タスク一覧を表示（デフォルト：ステータス別ソート、完了→未完了）
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

### インタラクティブモード用コマンド

| コマンド | 説明 |
|---------|------|
| `list` | タスク一覧を表示 |
| `list -i` | ID順でタスク一覧を表示 |
| `list -u` | 未完了タスクのみ表示 |
| `add <タイトル>` | タスクを追加 |
| `edit <ID> <新タイトル>` | タスクを編集 |
| `delete <ID>` | タスクを削除 |
| `check <ID>` | チェック状態を切り替え |
| `page` | ページ一覧を表示 |
| `page <ページ名>` | ページを切り替え |
| `page create <ページ名>` | ページを作成 |
| `page delete <ページ名>` | ページを削除 |
| `help` | ヘルプを表示 |
| `quit` / `exit` / `:q` | 終了 |

### CLIモード用コマンド

| コマンド | 説明 |
|---------|------|
| `como` | インタラクティブモードを起動 |
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
├── main.rs      # エントリーポイント、インタラクティブモード
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

- ✅ **クロスプラットフォーム対応**: Windows、macOS、Linux対応完了
- ✅ **インタラクティブモード**: 対話型シェル実装完了
- **Web版開発**: ブラウザで動作するバージョン

## 貢献

新機能の提案やバグの修正などの貢献を歓迎します！

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

問題や提案があれば、[Issues](https://github.com/ze-sudo/como/issues)からお知らせください。