use crate::db;
use crate::models::{Priority, Task};
use pickledb::PickleDb;

pub fn add_task(description: String, priority: Priority) {
    let mut db = db::load_db();

    let all_tasks = db::get_all_tasks(&db);

    let next_id = all_tasks.last().map(|t| t.id + 1).unwrap_or(1);

    let new_task = Task::new(next_id, description, priority);

    db::save_task(&mut db, &new_task);

    println!("✅ Task added successfully! ID: {}", next_id);
}

pub fn mark_task_done(id: usize) {
    let mut db = db::load_db();
    let id_str = id.to_string();

    if let Some(mut task) = db.get::<Task>(&id_str) {
        task.mark_done();
        db.set(&id_str, &task).expect("Failed to update task");
        println!("🎯 Task {} marked as completed!", id);
    } else {
        println!("❌ Error: Task with ID {} not found!", id);
    }
}

pub fn list_tasks() {
    let db = db::load_db();
    let tasks = db::get_all_tasks(&db);

    if tasks.is_empty() {
        println!("📭 Your TODO list is empty!");
        return;
    }

    println!("{:-<40}", "");
    println!("{:<5} | {:<20} | {:<10}", "ID", "Description", "Status");
    println!("{:-<40}", "");

    for task in tasks {
        let status = if task.is_completed {
            "✅ Done"
        } else {
            "⏳ Pending"
        };
        println!("{:<5} | {:<20} | {:<10}", task.id, task.description, status);
    }
    println!("{:-<40}", "");
}
