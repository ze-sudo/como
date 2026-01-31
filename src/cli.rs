use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "como")]
#[command(about = "A simple CLI task manager")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// タスク一覧を表示
    List {
        /// ID順でソート
        #[arg(short = 'i', long = "id")]
        sort_by_id: bool,
        /// ステータス別ソート（デフォルト）
        #[arg(short = 's', long = "status")]
        sort_by_status: bool,
        /// 未完了タスクのみ表示
        #[arg(short = 'u', long = "unchecked")]
        unchecked_only: bool,
    },
    /// 新しいタスクを追加
    Add {
        /// タスクのタイトル
        title: String,
    },
    /// タスクを編集
    Edit {
        /// タスクID
        id: u32,
        /// 新しいタイトル
        title: String,
    },
    /// タスクを削除
    Delete {
        /// タスクID
        id: u32,
    },
    /// タスクのチェック状態を切り替え
    Check {
        /// タスクID
        id: u32,
    },
    /// ページ管理
    Page {
        /// ページ名（指定時は切り替え、未指定時は一覧表示）
        name: Option<String>,
        #[command(subcommand)]
        action: Option<PageCommands>,
    },
}

#[derive(Subcommand)]
pub enum PageCommands {
    /// ページ一覧を表示（引数なしの場合のデフォルト動作）
    List,
    /// ページを作成
    Create {
        /// ページ名
        name: String,
    },
    /// ページを削除
    Delete {
        /// ページ名
        name: String,
    },
    /// ページを切り替え
    Switch {
        /// ページ名
        name: String,
    },
}
