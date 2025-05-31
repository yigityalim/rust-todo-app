
struct Todo {
    id: usize,
    text: String,
    done: bool
}

impl Todo {
    fn new(id: usize, text: String) -> Self {
        Todo { id, text, done: false }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}

fn main() {
    let mut todos = Vec::new();

    todos.push(Todo::new(1, "Learn Rust".to_string()));
    todos.push(Todo::new(2, "Build a project".to_string()));
    todos.push(Todo::new(3, "Contribute to open source".to_string()));

    // alternatif
    todos.push(Todo {
        id: 4,
        text: "Read Rust documentation".to_string(),
        done: false,
    });

    println!("\n📝 Todo Listesi:");

    for todo in &todos {
        let status = if todo.done { "✅" } else { "❌" };
        println!("{} [{} ] {}", todo.id, status, todo.text);
    }

    Todo::mark_done(&mut todos[1]); // 2. todo'yu tamamla

    println!("\nGüncellenmiş Todo Listesi:");
    for todo in &todos {
        let status = if todo.done { "✅" } else { "❌" };
        println!("{} [{} ] {}", todo.id, status, todo.text);
    }

    println!("\n📊 İstatistikler:");
    println!("Toplam todo sayısı: {}", todos.len());

    let done_count = todos.iter().filter(|todo| {
        todo.done
    }).count();
    println!("Tamamlanan todo sayısı: {}", done_count);
    let pending_count = todos.len() - done_count;
    println!("Bekleyen todo sayısı: {}", pending_count);
}