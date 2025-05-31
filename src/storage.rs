use crate::todo::Todo;
use std::fs;

pub fn save_todos(todos: &Vec<Todo>) -> std::io::Result<()> {
    let json_string = serde_json::to_string_pretty(todos)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    fs::write("todos.json", json_string)?;
    println!("Todo'lar başarıyla kaydedildi.");
    Ok(())
}

pub fn load_todos() -> Vec<Todo> {
    match fs::read_to_string("todos.json") {
        Ok(json_string) => {
            match serde_json::from_str(&json_string) {
                Ok(todos) => {
                    println!("Todo'lar başarıyla yüklendi.");
                    todos
                }
                Err(e) => {
                    eprintln!("JSON'dan okuma hatası: {}", e);
                    Vec::new()
                }
            }
        }
        Err(_) => {
            // Dosya yoksa sessizce boş liste döndür
            Vec::new()
        }
    }
}