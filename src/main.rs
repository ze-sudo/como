use clap::Parser;
use cli_rust_como::cli::{Cli, Commands, InteractiveCli, InteractiveCommands, PageCommands};
use cli_rust_como::TaskManager;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() {
    let cli = Cli::parse();

    let mut task_manager = match TaskManager::new() {
        Ok(tm) => tm,
        Err(e) => {
            eprintln!("エラー: タスクマネージャーの初期化に失敗しました: {e}");
            std::process::exit(1);
        }
    };

    // コマンドが指定されていない場合はインタラクティブモードを起動
    let result = match cli.command {
        None => run_interactive_mode(&mut task_manager),
        Some(command) => run_command(&mut task_manager, command),
    };

    if let Err(e) = result {
        eprintln!("エラー: {e}");
        std::process::exit(1);
    }
}

fn run_command(task_manager: &mut TaskManager, command: Commands) -> Result<(), anyhow::Error> {
    match command {
        Commands::List {
            sort_by_id,
            sort_by_status,
            unchecked_only,
        } => list_tasks(task_manager, sort_by_id, sort_by_status, unchecked_only),
        Commands::Add { title } => add_task(task_manager, title),
        Commands::Edit { id, title } => edit_task(task_manager, id, title),
        Commands::Delete { id } => delete_task(task_manager, id),
        Commands::Check { id } => check_task(task_manager, id),
        Commands::Page { name, action } => handle_page_command(task_manager, name, action),
    }
}

fn run_interactive_mode(task_manager: &mut TaskManager) -> Result<(), anyhow::Error> {
    println!("como - インタラクティブモード");
    println!("終了するには 'quit', 'exit', ':q' または Ctrl+C を入力してください");
    println!("コマンド一覧は 'help' を入力してください");
    println!();
    // 起動時にタスク一覧を表示
    let _ = list_tasks(task_manager, false, false, false);
    println!();

    let mut rl = DefaultEditor::new()?;

    loop {
        let prompt = format!("como({})> ", task_manager.get_current_page());

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line);

                // 入力をパース
                let args = match shellwords::split(line) {
                    Ok(args) => args,
                    Err(e) => {
                        eprintln!("パースエラー: {e}");
                        continue;
                    }
                };

                if args.is_empty() {
                    continue;
                }

                // clapでパース
                match InteractiveCli::try_parse_from(&args) {
                    Ok(parsed) => match parsed.command {
                        InteractiveCommands::Quit | InteractiveCommands::Exit => {
                            println!("さようなら！");
                            break;
                        }
                        InteractiveCommands::List {
                            sort_by_id,
                            sort_by_status,
                            unchecked_only,
                        } => {
                            if let Err(e) =
                                list_tasks(task_manager, sort_by_id, sort_by_status, unchecked_only)
                            {
                                eprintln!("エラー: {e}");
                            }
                        }
                        InteractiveCommands::Add { title } => {
                            if let Err(e) = add_task(task_manager, title) {
                                eprintln!("エラー: {e}");
                            }
                        }
                        InteractiveCommands::Edit { id, title } => {
                            if let Err(e) = edit_task(task_manager, id, title) {
                                eprintln!("エラー: {e}");
                            }
                        }
                        InteractiveCommands::Delete { id } => {
                            if let Err(e) = delete_task(task_manager, id) {
                                eprintln!("エラー: {e}");
                            }
                        }
                        InteractiveCommands::Check { id } => {
                            if let Err(e) = check_task(task_manager, id) {
                                eprintln!("エラー: {e}");
                            }
                        }
                        InteractiveCommands::Page { name, action } => {
                            if let Err(e) = handle_page_command(task_manager, name, action) {
                                eprintln!("エラー: {e}");
                            }
                        }
                    },
                    Err(e) => {
                        // ヘルプや使用方法エラーの場合は表示
                        eprintln!("{e}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl+C が押されました。終了します...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl+D が押されました。終了します...");
                break;
            }
            Err(err) => {
                eprintln!("エラー: {err}");
                break;
            }
        }
    }

    Ok(())
}

fn list_tasks(
    task_manager: &TaskManager,
    sort_by_id: bool,
    _sort_by_status: bool,
    unchecked_only: bool,
) -> Result<(), anyhow::Error> {
    let mut tasks = task_manager.list_tasks()?;

    println!(
        "=== {} ページのタスク一覧 ===",
        task_manager.get_current_page()
    );

    if tasks.is_empty() {
        println!("タスクがありません");
        return Ok(());
    }

    // フィルタリング：未完了のみ表示
    if unchecked_only {
        tasks.retain(|task| !task.completed);
    }

    // ソート処理（優先順位: -i > -s > デフォルト）
    if sort_by_id {
        // ID順ソート
        tasks.sort_by_key(|task| task.id);
    } else {
        // デフォルトまたは-s指定時：ステータス別ソート（完了→未完了）
        tasks.sort_by_key(|task| (!task.completed, task.id));
    }

    if tasks.is_empty() {
        println!("表示するタスクがありません");
        return Ok(());
    }

    // シンプルな表示形式
    for task in &tasks {
        let status = if task.completed { "✓" } else { " " };
        println!("[{}] {} - {}", status, task.id, task.title);
    }

    Ok(())
}

fn add_task(task_manager: &TaskManager, title: String) -> Result<(), anyhow::Error> {
    let id = task_manager.add_task(title.clone())?;
    println!("タスクを追加しました: [{id}] {title}");
    Ok(())
}

fn edit_task(task_manager: &TaskManager, id: u32, new_title: String) -> Result<(), anyhow::Error> {
    task_manager.edit_task(id, new_title.clone())?;
    println!("タスク {id} を編集しました: {new_title}");
    Ok(())
}

fn delete_task(task_manager: &TaskManager, id: u32) -> Result<(), anyhow::Error> {
    task_manager.delete_task(id)?;
    println!("タスク {id} を削除しました");
    Ok(())
}

fn check_task(task_manager: &TaskManager, id: u32) -> Result<(), anyhow::Error> {
    let completed = task_manager.toggle_task(id)?;
    let status = if completed { "完了" } else { "未完了" };
    println!("タスク {id} を {status} に設定しました");
    Ok(())
}

fn handle_page_command(
    task_manager: &mut TaskManager,
    name: Option<String>,
    action: Option<PageCommands>,
) -> Result<(), anyhow::Error> {
    // ページ名が指定されていて、アクションが指定されていない場合はページ切り替え
    if let Some(page_name) = name {
        if action.is_none() {
            task_manager.switch_page(&page_name)?;
            println!("ページ '{page_name}' に切り替えました");
            return Ok(());
        }
    }

    match action {
        None | Some(PageCommands::List) => {
            let pages = task_manager.list_pages()?;

            println!("=== ページ一覧 ===");

            if pages.is_empty() {
                println!("ページがありません");
                return Ok(());
            }

            for (page_name, is_current) in pages {
                let marker = if is_current { " *" } else { "  " };
                println!("{marker} {page_name}");
            }
        }
        Some(PageCommands::Create { name }) => {
            task_manager.create_page(&name)?;
            println!("ページ '{name}' を作成し、切り替えました");
        }
        Some(PageCommands::Delete { name }) => {
            task_manager.delete_page(&name)?;
            println!("ページ '{name}' を削除しました");
        }
        Some(PageCommands::Switch { name }) => {
            task_manager.switch_page(&name)?;
            println!("ページ '{name}' に切り替えました");
        }
    }

    Ok(())
}
