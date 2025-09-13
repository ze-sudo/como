pub mod models;
pub mod storage;
pub mod cli;

use anyhow::{anyhow, Result};
use models::{PageData, Task};
use storage::*;

/// タスク管理の中核となる構造体
pub struct TaskManager {
    current_page: String,
}

impl TaskManager {
    pub fn new() -> Result<Self> {
        let current_page = load_current_page().unwrap_or_else(|_| "default".to_string());
        Ok(Self { current_page })
    }

    /// 現在のページ名を取得
    pub fn get_current_page(&self) -> &str {
        &self.current_page
    }

    /// ページを切り替え
    pub fn switch_page(&mut self, page_name: &str) -> Result<()> {
        self.current_page = page_name.to_string();
        save_current_page(page_name)?;
        Ok(())
    }

    /// 現在のページのタスク一覧を取得
    pub fn list_tasks(&self) -> Result<Vec<Task>> {
        let page_data = load_page_data(&self.current_page)?;
        Ok(page_data.tasks)
    }

    /// タスクを追加
    pub fn add_task(&self, title: String) -> Result<u32> {
        let mut page_data = load_page_data(&self.current_page)?;
        let id = page_data.add_task(title);
        save_page_data(&self.current_page, &page_data)?;
        Ok(id)
    }

    /// タスクを編集
    pub fn edit_task(&self, id: u32, new_title: String) -> Result<()> {
        let mut page_data = load_page_data(&self.current_page)?;
        
        if let Some(task) = page_data.find_task_by_id_mut(id) {
            task.title = new_title;
            save_page_data(&self.current_page, &page_data)?;
            Ok(())
        } else {
            Err(anyhow!("ID {} のタスクが見つかりません", id))
        }
    }

    /// タスクを削除
    pub fn delete_task(&self, id: u32) -> Result<()> {
        let mut page_data = load_page_data(&self.current_page)?;
        
        if page_data.remove_task(id) {
            save_page_data(&self.current_page, &page_data)?;
            Ok(())
        } else {
            Err(anyhow!("ID {} のタスクが見つかりません", id))
        }
    }

    /// タスクのチェック状態を切り替え
    pub fn toggle_task(&self, id: u32) -> Result<bool> {
        let mut page_data = load_page_data(&self.current_page)?;
        
        if let Some(task) = page_data.find_task_by_id_mut(id) {
            task.completed = !task.completed;
            let completed = task.completed;
            save_page_data(&self.current_page, &page_data)?;
            Ok(completed)
        } else {
            Err(anyhow!("ID {} のタスクが見つかりません", id))
        }
    }

    /// ページ一覧を取得（現在のページを明示）
    pub fn list_pages(&self) -> Result<Vec<(String, bool)>> {
        let pages = list_pages()?;
        let current = &self.current_page;
        
        let mut result = Vec::new();
        for page in pages {
            let is_current = page == *current;
            result.push((page, is_current));
        }
        
        // 現在のページが一覧にない場合は追加
        if !result.iter().any(|(name, _)| name == current) {
            result.push((current.clone(), true));
        }
        
        result.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(result)
    }

    /// ページを作成
    pub fn create_page(&mut self, page_name: &str) -> Result<()> {
        if page_exists(page_name)? {
            return Err(anyhow!("ページ '{}' は既に存在します", page_name));
        }
        
        // 空のページデータを作成・保存
        let page_data = PageData::new();
        save_page_data(page_name, &page_data)?;
        
        // 作成したページに切り替え
        self.switch_page(page_name)?;
        
        Ok(())
    }

    /// ページを削除
    pub fn delete_page(&mut self, page_name: &str) -> Result<()> {
        if !page_exists(page_name)? {
            return Err(anyhow!("ページ '{}' が見つかりません", page_name));
        }
        
        if page_name == self.current_page {
            return Err(anyhow!("現在のページは削除できません"));
        }
        
        delete_page(page_name)?;
        Ok(())
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            current_page: "default".to_string(),
        })
    }
}
