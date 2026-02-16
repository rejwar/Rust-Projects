use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub is_completed: bool,
    pub priority: Priority,
}

impl Task {
    pub fn new(id: usize, description: String, priority: Priority) -> Self {
        Task {
            id,
            description,
            is_complecated: false,
            priority,
        }
    }

    pub fn mark_done(&mut self) {
        self.is_completd = true;
    }
}
