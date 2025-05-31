use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: usize,
    text: String,
    done: bool,
}

impl Todo {
    fn list(&self) -> String {
        let status = if self.done { "✅" } else { "⏳" };
        format!("[{} ] {}: {}", status, self.id, self.text)
    }

    fn new(id: usize, text: String) -> Todo {
        Todo {
            id,
            text,
            done: false, // varsayılan olarak tamamlanmamış
        }
    }
}

fn save_todos(todos: &Vec<Todo>) -> std::io::Result<()> {
    match serde_json::to_string_pretty(todos) {
        Ok(json_string) => match fs::write("todos.json", json_string) {
            Ok(_) => {
                println!("Todo'lar başarıyla kaydedildi.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Dosyaya yazma hatası: {}", e);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Dosyaya yazma hatası",
                ))
            }
        },
        Err(e) => {
            eprintln!("JSON'a dönüştürme hatası: {}", e);
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "JSON'a dönüştürme hatası",
            ))
        }
    }
}

fn load_todos() -> Vec<Todo> {
    match fs::read_to_string("todos.json") {
        Ok(json_string) => {
            match serde_json::from_str(&json_string) {
                Ok(todos) => {
                    println!("Todo'lar başarıyla yüklendi.");
                    todos
                }
                Err(e) => {
                    eprintln!("JSON'dan okuma hatası: {}", e);
                    Vec::new() // hata durumunda boş bir liste döndür
                }
            }
        }
        Err(e) => {
            eprintln!("Dosyadan okuma hatası: {}", e);
            Vec::new() // hata durumunda boş bir liste döndür
        }
    }
}

pub fn main() {
    let mut todos: Vec<Todo> = load_todos();

    let args: Vec<String> = std::env::args().collect(); // komut satırı argümanlarını al

    if args.len() < 2 {
        println!("Kullanım:");
        println!("  cargo run list");
        println!("  cargo run add \"todo metni\"");
        println!("  cargo run done <id>");
        std::process::exit(1);
    }

    let command = &args[1]; // neden 1? çünkü ilk argüman programın adı.
    let mut should_save = false; // todo'lar değiştiğinde kaydetmek için

    match command.as_str() {
        "help" => {
            println!("Kullanım:");
            println!("  cargo run list");
            println!("  cargo run add \"todo metni\"");
            println!("  cargo run done <id>");
            println!("  cargo run remove <id>");
            std::process::exit(0);
        },
        "list" => {
            if todos.is_empty() {
                println!("Henüz todo eklenmemiş!");
                std::process::exit(0);
            }
            println!("Todo listesi:");
            for todo in &todos {
                println!("{}", todo.list());
            }
        }
        "add" => {
            if args.len() < 3 {
                println!("Todo eklemek için metin girmeniz gerekiyor!");
                println!("Kullanım: cargo run add \"todo metni\"");
            } else {
                let todo_text = &args[2];

                let new_id = if todos.is_empty() {
                    0
                } else {
                    todos.iter().map(|t| t.id).max().unwrap() + 1 // en yüksek ID'yi bul ve 1 artır
                };

                todos.push(Todo::new(new_id, todo_text.to_string()));
                println!("Yeni todo eklendi: {}", todos.last().unwrap().list());
                should_save = true;
            }
        }
        "done" => {
            if args.len() < 3 {
                println!("Todo ID'sini girmeniz gerekiyor!");
                println!("Kullanım: cargo run done <id>");
            } else {
                match args[2].parse::<usize>() {
                    Ok(id) => {
                        let mut found = false;

                        for todo in &mut todos {
                            if todo.id == id {
                                todo.done = true; // todo'yu tamamlandı olarak işaretle
                                found = true;
                                println!("Todo tamamlandı: {}", todo.list());

                                should_save = true;
                                break;
                            }
                        }

                        if !found {
                            println!("ID {} bulunamadı!", id);
                        }
                    }
                    Err(_) => println!("Geçersiz ID: {}", args[2]),
                }
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("Todo ID'sini girmeniz gerekiyor!");
                println!("Kullanım: cargo run remove <id>");
            } else {
                match args[2].parse::<usize>() {
                    Ok(id) => {
                        if let Some(index) = todos.iter().position(|todo| todo.id == id) {
                            let removed_todo = todos.remove(index);
                            println!("Todo silindi: {}", removed_todo.text);
                            should_save = true;
                        } else {
                            println!("ID {} bulunamadı!", id);
                        }
                    }
                    Err(_) => println!("Geçersiz ID: {}", args[2]),
                }
            }
        },
        "clear" => {
            let initial_count = todos.len();

            todos.retain(|todo| !todo.done); // tamamlanmış todo'ları sil
            let removed_count = initial_count - todos.len();

            if removed_count > 0 {
                println!("{} tamamlanmış todo temizlendi!", removed_count);
                should_save = true;
            } else {
                println!("Hiç tamamlanmış todo bulunamadı.");
            }
        }
        _ => {
            println!("Bilinmeyen komut: {}!", command);
            std::process::exit(1);
        }
    }

    if should_save {
        if let Err(e) = save_todos(&todos) {
            // if let: hata varsa işle
            eprintln!("Todo'ları kaydederken hata oluştu: {}", e);
        }
    }
}
