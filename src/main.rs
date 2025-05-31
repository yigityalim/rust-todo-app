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

pub fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    todos.push(Todo::new(1, "Örnek todo".to_string()));
    todos.push(Todo::new(2, "Başka bir todo".to_string()));

    let args: Vec<String> = std::env::args().collect(); // komut satırı argümanlarını al

    if args.len() < 2 {
        println!("Kullanım:");
        println!("  cargo run list");
        println!("  cargo run add \"todo metni\"");
        println!("  cargo run done <id>");
        std::process::exit(1);
    }

    let command = &args[1]; // neden 1? çünkü ilk argüman programın adı.

    match command.as_str() {
        "list" => {
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
                let new_id = todos.len() + 1;
                todos.push(Todo::new(new_id, todo_text.to_string()));
                println!("Yeni todo eklendi: {} (ID: {})", todo_text, new_id);
                for todo in &todos {
                    println!("{}", todo.list());
                }
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
        _ => {
            println!("Bilinmeyen komut: {}!", command);
            std::process::exit(1);
        }
    }
}
