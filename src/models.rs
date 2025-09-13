use serde::{Deserialize, Serialize};

/// タスクの構造体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}

/// ページデータの構造体（各ページのJSONファイルに対応）
#[derive(Debug, Serialize, Deserialize)]
pub struct PageData {
    pub last_id: u32,
    pub tasks: Vec<Task>,
}

impl PageData {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            tasks: Vec::new(),
        }
    }

    /// 新しいタスクIDを生成
    pub fn generate_next_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    /// IDでタスクを検索
    pub fn find_task_by_id(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    /// IDでタスクを検索（可変参照）
    pub fn find_task_by_id_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    /// タスクを追加
    pub fn add_task(&mut self, title: String) -> u32 {
        let id = self.generate_next_id();
        let task = Task::new(id, title);
        self.tasks.push(task);
        id
    }

    /// タスクを削除
    pub fn remove_task(&mut self, id: u32) -> bool {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for PageData {
    fn default() -> Self {
        Self::new()
    }
}
