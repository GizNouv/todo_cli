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
git clone https://github.com/giznouv/todo_cli.git
cd todo_cli
cargo build --release