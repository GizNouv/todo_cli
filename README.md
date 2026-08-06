# 📋 Todo CLI - Rust Learning Project

> A simple command-line todo list manager built to learn Rust 🦀

[![Rust](https://img.shields.io/badge/rust-1.97+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🎯 Purpose

This project was created as a **learning exercise** to explore Rust fundamentals through a practical project. It's not meant for production use – just for fun and education!

**What I learned:**
- ✅ Rust syntax and ownership model
- ✅ Structs, enums, and pattern matching
- ✅ File I/O and JSON serialization with `serde`
- ✅ Command-line argument parsing
- ✅ The borrow checker (we fought, we won 🏆)

---

## 🚀 Features

- ➕ **Add** tasks with a description
- 📋 **List** all tasks with their status
- ✅ **Mark** tasks as done
- 🗑️ **Remove** tasks by ID
- 💾 **Persistent storage** – tasks saved to `tasks.json`

---

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.97+)

### Build from source

```bash
git clone https://github.com/GizNouv/todo_cli.git
cd todo_cli
cargo build --release
```
The binary will be at target/release todo_cli.exe (Windows) or target/release/todo_cli (Linux/macOS).

---

## 🎮 Usage
### Add a new task
todo add "Buy groceries"

### List all tasks
todo list

### Mark task #2 as done
todo done 2

### Remove task #1
todo remove 1

### Show help
todo

## Example session
```bash
$ todo add "Learn Rust"
✅ Task added! (ID: 1)

$ todo add "Build CLI app"
✅ Task added! (ID: 2)

$ todo list
📋 Your tasks:
  ⬜ [1] Learn Rust
  ⬜ [2] Build CLI app

$ todo done 1
✅ Task 1 marked as done!

$ todo list
📋 Your tasks:
  ✅ [1] Learn Rust
  ⬜ [2] Build CLI app

$ todo remove 2
🗑️ Task 2 removed!

$ todo list
📋 Your tasks:
  ✅ [1] Learn Rust
```
---

## 📁 Project Structure

todo_cli/
├── src/
│   └── main.rs          # All the code (single-file for simplicity)
├── Cargo.toml           # Dependencies and project metadata
├── Cargo.lock           # Locked dependency versions
├── tasks.json           # Auto-generated storage file
├── .gitignore           # Files to exclude from Git
└── README.md            # This file

---

## 🧠 Key Concepts Explored
Concept	| Implementation
Ownership |	String vs &str, borrowing (&), moving
Structs | Task and TodoList
Serialization |	serde_json for JSON persistence
Pattern Matching | match for CLI routing, if let for Option handling
Error Handling | expect() for simplicity (learning focus)

---

## 🛠️ Dependencies
- serde – Serialization framework
- serde_json – JSON support

---

## 🤝 Contributing
This is a personal learning project, so I'm not accepting contributions. But feel free to fork it and make it your own!

---

## 📄 License
MIT License – feel free to use this code for your own learning!

---

## 🙏 Acknowledgments
- [The Rust Book](https://doc.rust-lang.org/book/ch00-00-introduction.html) – The best resource for learning Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) – Practical examples
- [Serde](https://serde.rs/) Documentation – For making JSON handling a breeze