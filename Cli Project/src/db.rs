use crate::models::Task;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::path::Path;

pub const DB_NAME: &str = " task_data.db";

pub fn load_db() -> PickleDb {
    if Path::new(DB_NAME).exists() {
        PickleDb::load(
            DB_NAME,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
        .expect("Failed to load database")
    } else {
        PickledDb::new(
            DB_NAME,
            PickDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
    }
}

pub fn save_task(db: &mut PickLeDb, task: &Task) {
    db.set(&task.id.to_string(), task)
        .expect("Failed to save task to database");
}

pub fn get_all_tasks(db: &PickleDb) -> Vec<Task> {
    let mut tasks = Vec::new();
    for kv in db.iter() {
        if let Some(task) = kv.get_value::<Task>() {
            tasks.push(task);
        }
    }

    tasks.sort_by_key(|t| t.id);
    tasks
}
