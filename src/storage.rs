use crate::models::PageData;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// como用のデータディレクトリパス
pub fn get_como_data_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("ホームディレクトリが取得できません")?;
    Ok(home_dir.join("Library/Application Support/como"))
}

/// データディレクトリを作成（存在しない場合）
pub fn ensure_data_dir() -> Result<PathBuf> {
    let data_dir = get_como_data_dir()?;
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .with_context(|| format!("データディレクトリの作成に失敗しました: {:?}", data_dir))?;
    }
    Ok(data_dir)
}

/// ページファイルパスを取得
pub fn get_page_path(page_name: &str) -> Result<PathBuf> {
    let data_dir = ensure_data_dir()?;
    let filename = if page_name.ends_with(".json") {
        page_name.to_string()
    } else {
        format!("{}.json", page_name)
    };
    Ok(data_dir.join(filename))
}

/// ページデータを読み込み
pub fn load_page_data(page_name: &str) -> Result<PageData> {
    let page_path = get_page_path(page_name)?;
    
    if !page_path.exists() {
        // ファイルが存在しない場合は新しいページデータを作成
        return Ok(PageData::new());
    }

    let content = fs::read_to_string(&page_path)
        .with_context(|| format!("ページファイルの読み込みに失敗しました: {:?}", page_path))?;
    
    let page_data: PageData = serde_json::from_str(&content)
        .with_context(|| format!("JSONの解析に失敗しました: {:?}", page_path))?;
    
    Ok(page_data)
}

/// ページデータを保存
pub fn save_page_data(page_name: &str, page_data: &PageData) -> Result<()> {
    let page_path = get_page_path(page_name)?;
    
    let json_content = serde_json::to_string_pretty(page_data)
        .context("JSONのシリアライズに失敗しました")?;
    
    fs::write(&page_path, json_content)
        .with_context(|| format!("ページファイルの保存に失敗しました: {:?}", page_path))?;
    
    Ok(())
}

/// 利用可能なページ一覧を取得
pub fn list_pages() -> Result<Vec<String>> {
    let data_dir = ensure_data_dir()?;
    let mut pages = Vec::new();
    
    if data_dir.exists() {
        for entry in fs::read_dir(data_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        if let Some(stem) = path.file_stem() {
                            if let Some(page_name) = stem.to_str() {
                                pages.push(page_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    pages.sort();
    Ok(pages)
}

/// ページが存在するかチェック
pub fn page_exists(page_name: &str) -> Result<bool> {
    let page_path = get_page_path(page_name)?;
    Ok(page_path.exists())
}

/// ページを削除
pub fn delete_page(page_name: &str) -> Result<()> {
    let page_path = get_page_path(page_name)?;
    
    if page_path.exists() {
        fs::remove_file(&page_path)
            .with_context(|| format!("ページファイルの削除に失敗しました: {:?}", page_path))?;
    }
    
    Ok(())
}

/// 現在のページを管理する設定ファイル
pub fn get_current_page_config_path() -> Result<PathBuf> {
    let data_dir = ensure_data_dir()?;
    Ok(data_dir.join("current_page.txt"))
}

/// 現在のページ名を保存
pub fn save_current_page(page_name: &str) -> Result<()> {
    let config_path = get_current_page_config_path()?;
    fs::write(config_path, page_name)
        .context("現在のページ設定の保存に失敗しました")?;
    Ok(())
}

/// 現在のページ名を読み込み
pub fn load_current_page() -> Result<String> {
    let config_path = get_current_page_config_path()?;
    
    if config_path.exists() {
        let content = fs::read_to_string(config_path)
            .context("現在のページ設定の読み込みに失敗しました")?;
        Ok(content.trim().to_string())
    } else {
        // デフォルトページ
        Ok("default".to_string())
    }
}
