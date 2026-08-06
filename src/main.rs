use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    text: String,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

fn get_todo_path() -> PathBuf {
    // Get the current executable's directory
    let mut path = env::current_exe().unwrap();
    path.pop(); // Remove the executable name
    path.push("tasks.json");
    path
}

fn load_todo_list() -> TodoList {
    let path = get_todo_path();

    // Check if the file exists
    if path.exists() {
        // Read the file contents
        let data = fs::read_to_string(&path).expect("Failed to read tasks file");
        // Parse JSON into TodoList
        serde_json::from_str(&data).expect("Failed to parse tasks JSON")
    } else {
        // Return an empty TodoList
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
}

fn save_todo_list(todo_list: &TodoList) {
    let path = get_todo_path();
    // Convert TodoList to JSON string (pretty-printed)
    let data = serde_json::to_string_pretty(todo_list).expect("Failed to serialize tasks");
    // Write to file
    fs::write(&path, data).expect("Failed to write tasks file");
}

fn print_usage() {
    println!("Todo CLI - Manage your tasks");
    println!();
    println!("Usage: todo <command> [options]");
    println!();
    println!("Commands:");
    println!("  add <text>     Add a new task");
    println!("  list           List all tasks");
    println!("  done <id>      Mark a task as done");
    println!("  remove <id>    Remove a task");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Error: Task text required");
                println!("Usage: todo add \"Task description\"");
                return;
            }
            let task_text = &args[2];
            
            // Load existing tasks
            let mut todo_list = load_todo_list();
            
            // Create new task
            let task = Task {
                id: todo_list.next_id,
                text: task_text.clone(),
                done: false,
            };
            
            // Add to list and increment next_id
            todo_list.tasks.push(task);
            todo_list.next_id += 1;
            
            // Save back to file
            save_todo_list(&todo_list);
            
            println!("✅ Task added! (ID: {})", todo_list.next_id - 1);
        }
        "list" => {
            let todo_list = load_todo_list();
            
            if todo_list.tasks.is_empty() {
                println!("📋 No tasks yet. Add one with 'todo add \"Task description\"'");
                return;
            }
            
            println!("📋 Your tasks:");
            for task in &todo_list.tasks {
                let status = if task.done { "✅" } else { "⬜" };
                println!("  {} [{}] {}", status, task.id, task.text);
            }
        }
        "done" => {
            if args.len() < 3 {
                println!("Error: Task ID required");
                println!("Usage: todo done <id>");
                return;
            }
            let id: u32 = args[2].parse().expect("Please enter a valid number");
            
            let mut todo_list = load_todo_list();
            
            // Find the task by ID and mark it as done
            if let Some(task) = todo_list.tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_todo_list(&todo_list);
                println!("✅ Task {} marked as done!", id);
            } else {
                println!("❌ Task with ID {} not found", id);
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("Error: Task ID required");
                println!("Usage: todo remove <id>");
                return;
            }
            let id: u32 = args[2].parse().expect("Please enter a valid number");
            
            let mut todo_list = load_todo_list();
            
            // Find the task by ID and remove it
            let initial_len = todo_list.tasks.len();
            todo_list.tasks.retain(|task| task.id != id);
            
            if todo_list.tasks.len() < initial_len {
                save_todo_list(&todo_list);
                println!("🗑️ Task {} removed!", id);
            } else {
                println!("❌ Task with ID {} not found", id);
            }
        }
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }
}
