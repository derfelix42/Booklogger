// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Debug)]
struct Book {
  id: u32,
  title: String,
  author: String,
  year: String,
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_books])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_books() -> Vec<String> {
  println!("Getting books...");
  // Represents a book with its properties.

  // A collection of books.
  //
  // This vector stores instances of the `Book` struct.
  let books = vec!["The Great Gatsby".to_string(), "The Old Man and the Sea".to_string(), "The Alchemist".to_string()];
  books
}