mod todo;
mod storage;
mod colors;
mod progress;

use todo::{Todo, format_timestamp};
use storage::{save_todos, load_todos};
use colors::*;
use progress::*;
use colored::*;

fn main() {
    println!("{}", "\n\n\n📝 TODO CLI UYGULAMASI".cyan().bold());
    println!("{}", "═══════════════════════".cyan());

    let mut todos: Vec<Todo> = load_todos();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        std::process::exit(1);
    }

    let command = &args[1];
    let mut should_save = false;

    match command.as_str() {
        "help" => print_help(),
        "stats" => show_stats(&todos),
        "list" => list_todos(&todos),
        "add" => {
            if let Some(text) = args.get(2) {
                show_loading("Todo ekleniyor");
                add_todo(&mut todos, text);
                should_save = true;
            } else {
                error("❌ Todo eklemek için metin girmeniz gerekiyor!");
            }
        }
        "done" => {
            if let Some(id_str) = args.get(2) {
                if mark_todo_done(&mut todos, id_str) {
                    should_save = true;
                }
            } else {
                error("❌ Todo ID'sini girmeniz gerekiyor!");
            }
        }
        "remove" => {
            if let Some(id_str) = args.get(2) {
                if remove_todo(&mut todos, id_str) {
                    should_save = true;
                }
            } else {
                error("❌ Todo ID'sini girmeniz gerekiyor!");
            }
        }
        "clear" => {
            if clear_completed(&mut todos) {
                should_save = true;
            }
        }
        _ => {
            error(&format!("❌ Bilinmeyen komut: {}!", command));
            std::process::exit(1);
        }
    }

    if should_save {
        show_loading("Todo'lar kaydediliyor");
        if let Err(e) = save_todos(&todos) {
            error(&format!("❌ Todo'ları kaydederken hata oluştu: {}", e));
        }
    }
}

fn print_help() {
    highlight("\n\n\n📝 TODO CLI UYGULAMASI - YARDIM");
    println!();
    info("🚀 Kullanım:");
    println!("  cargo run list      - Todo'ları listele");
    println!("  cargo run add \"..\" - Yeni todo ekle");
    println!("  cargo run done <id> - Todo'yu tamamla");
    println!("  cargo run remove <id> - Todo'yu sil");
    println!("  cargo run clear     - Tamamlananları temizle");
    println!("  cargo run stats     - İstatistik göster");
}

fn show_stats(todos: &[Todo]) {
    if todos.is_empty() {
        warning("📊 İstatistik bulunamadı - liste boş!");
        return;
    }

    let total = todos.len();
    let completed = todos.iter().filter(|todo| todo.done).count();
    let pending = total - completed;
    let completion_rate = (completed as f64 / total as f64) * 100.0;

    highlight("📊 TODO İSTATİSTİKLERİ");
    println!("{} {}", "📝 Toplam:".blue(), total.to_string().yellow().bold());
    println!("{} {}", "✅ Tamamlanan:".green(), completed.to_string().green().bold());
    println!("{} {}", "⏳ Bekleyen:".yellow(), pending.to_string().yellow().bold());
    println!("{} {}%", "📈 Tamamlanma oranı:".cyan(), format!("{:.1}", completion_rate).cyan().bold());

    // Progress bar ile tamamlanma oranını göster
    if completed > 0 {
        show_progress_bar(completed, "Tamamlanan todo'lar");
    }
}

fn list_todos(todos: &[Todo]) {
    if todos.is_empty() {
        warning("📝 Todo listesi boş!");
        return;
    }

    highlight("📝 TODO LİSTESİ");
    println!("{}", "═══════════════".cyan());

    for todo in todos {
        let status = if todo.done { "✅" } else { "⏳" };
        let created_date = format_timestamp(todo.created_at);
        let id_colored = todo_id(todo.id);

        if todo.done {
            let completed_date = todo
                .completed_at
                .map(|ts| format_timestamp(ts))
                .unwrap_or_else(|| "Bilinmiyor".to_string());

            println!(
                "{} {} {} {}",
                status,
                id_colored,
                todo.text.green().strikethrough(),
                format!("({})", completed_date).dimmed()
            );
        } else {
            println!(
                "{} {} {} {}",
                status,
                id_colored,
                todo.text.white(),
                format!("({})", created_date).dimmed()
            );
        }
    }
}

fn add_todo(todos: &mut Vec<Todo>, text: &str) {
    let new_id = if todos.is_empty() {
        0
    } else {
        todos.iter().map(|t| t.id).max().unwrap() + 1
    };

    todos.push(Todo::new(new_id, text.to_string()));
    success(&format!("✅ Yeni todo eklendi: {}", text));
}

fn mark_todo_done(todos: &mut [Todo], id_str: &str) -> bool {
    match id_str.parse::<usize>() {
        Ok(id) => {
            for todo in todos.iter_mut() {
                if todo.id == id {
                    todo.mark_done();
                    success(&format!("✅ Todo tamamlandı: {}", todo.text));
                    return true;
                }
            }
            error(&format!("❌ ID {} bulunamadı!", id));
            false
        }
        Err(_) => {
            error(&format!("❌ Geçersiz ID: {}", id_str));
            false
        }
    }
}

fn remove_todo(todos: &mut Vec<Todo>, id_str: &str) -> bool {
    match id_str.parse::<usize>() {
        Ok(id) => {
            if let Some(index) = todos.iter().position(|todo| todo.id == id) {
                let removed_todo = todos.remove(index);
                warning(&format!("🗑️ Todo silindi: {}", removed_todo.text));
                true
            } else {
                error(&format!("❌ ID {} bulunamadı!", id));
                false
            }
        }
        Err(_) => {
            error(&format!("❌ Geçersiz ID: {}", id_str));
            false
        }
    }
}

fn clear_completed(todos: &mut Vec<Todo>) -> bool {
    let initial_count = todos.len();
    let completed_count = todos.iter().filter(|todo| todo.done).count();

    if completed_count > 0 {
        show_progress_bar(completed_count, "Tamamlanan todo'lar temizleniyor");
    }

    todos.retain(|todo| !todo.done);
    let removed_count = initial_count - todos.len();

    if removed_count > 0 {
        success(&format!("🧹 {} tamamlanmış todo temizlendi!", removed_count));
        true
    } else {
        info("ℹ️ Hiç tamamlanmış todo bulunamadı.");
        false
    }
}