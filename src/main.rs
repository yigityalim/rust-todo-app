
pub struct Todo {
    id: usize,
    text: String,
    done: bool
}

impl Todo {
    // Yeni todo oluşturmak için helper function
    fn new(id: usize, text: String) -> Todo {
        Todo {
            id,
            text,
            done: false, // varsayılan olarak tamamlanmamış
        }
    }
}

pub fn main() {
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
        },
        "add" => {
            if args.len() < 3 { 
                println!("Todo eklemek için metin girmeniz gerekiyor!");
                println!("Kullanım: cargo run add \"todo metni\"");
            } else {
                let todo_text = &args[2];
                println!("Yeni todo eklendi: {}", todo_text);
            }
        },
        "done" => {
            if args.len() < 3 {
                println!("Todo ID'sini girmeniz gerekiyor!");
                println!("Kullanım: cargo run done <id>");
            } else {
                match args[2].parse::<usize>() {
                    Ok(id) => { println!("Todo {} tamamlandı!", id) },
                    Err(_) => println!("Geçersiz ID: {}", args[2]),
                }
            }
        },
        _ => {
            println!("Bilinmeyen komut: {}!", command);
            std::process::exit(1);
        }
    }
}