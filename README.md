# como

「como」はターミナルで動作する、キーボード操作のみで完結するCLI型のタスク管理アプリケーションです。  
MySQLやPythonのターミナル操作のような直感的なCLI体験を提供します。

## 特徴

- **CLI完結型**: ターミナルからコマンドラインで操作
- **ページ管理**: 複数のタスクリストをページごとに分類
- **ローカル保存**: JSONファイルでローカルにデータを保存
- **直感的操作**: シンプルで覚えやすいコマンド体系

## インストール

### 前提条件

- macOS（今後、WindowsやLinuxにも対応予定）
- Rust (Cargoが利用可能)

### ビルド方法

```bash
git clone <リポジトリURL>
cd cli-rust-como
cargo build --release
```

### システムへのインストール

```bash
cargo install --path .
```

## 使い方

### 基本的なタスク操作

```bash
# タスク一覧を表示
como list

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

タスクデータは以下のディレクトリに保存されます：

```text
~/Library/Application Support/como/
```

- 各ページのデータ: `{ページ名}.json`
- 現在のページ情報: `current_page.txt`

## コマンド一覧

| コマンド | 説明 |
|---------|------|
| `como list` | タスク一覧を表示 |
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

- WindowsやLinuxなどへの対応
- Web版の開発

