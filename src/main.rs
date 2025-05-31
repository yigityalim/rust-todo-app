mod colors;
mod progress;
mod storage;
mod todo;

use colored::*;
use colors::*;
use progress::*;
use storage::{load_todos, save_todos};
use todo::{Todo, format_timestamp};
use uuid::Uuid;

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
                if todos.iter().any(|t| t.text == *text) {
                    error("❌ Bu isimde bir todo zaten var!");
                } else {
                    show_loading("Todo ekleniyor");
                    add_todo(&mut todos, text);
                    should_save = true;
                }
            } else {
                error("❌ Todo eklemek için metin girmeniz gerekiyor!");
            }
        }
        "done" => {
            if let Some(name) = args.get(2) {
                if mark_todo_done(&mut todos, name) {
                    should_save = true;
                }
            } else {
                error("❌ Todo ismini girmeniz gerekiyor!");
            }
        }
        "remove" => {
            if let Some(name) = args.get(2) {
                if remove_todo(&mut todos, name) {
                    should_save = true;
                }
            } else {
                error("❌ Todo ismini girmeniz gerekiyor!");
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
    println!("  cargo run list         - Todo'ları listele");
    println!("  cargo run add \"..\"   - Yeni todo ekle (benzersiz isim)");
    println!("  cargo run done <isim>  - Todo'yu tamamla");
    println!("  cargo run remove <isim> - Todo'yu sil");
    println!("  cargo run clear        - Tamamlananları temizle");
    println!("  cargo run stats        - İstatistik göster");
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
    println!(
        "{} {}",
        "📝 Toplam:".blue(),
        total.to_string().yellow().bold()
    );
    println!(
        "{} {}",
        "✅ Tamamlanan:".green(),
        completed.to_string().green().bold()
    );
    println!(
        "{} {}",
        "⏳ Bekleyen:".yellow(),
        pending.to_string().yellow().bold()
    );
    println!(
        "{} {}%",
        "📈 Tamamlanma oranı:".cyan(),
        format!("{:.1}", completion_rate).cyan().bold()
    );

    if completed > 0 {
        show_progress_bar(completed, "Tamamlanan todo'lar");
    }
}

fn list_todos(todos: &[Todo]) {
    if todos.is_empty() {
        warning("📝 Todo listesi boş!");
        return;
    }

    highlight("\n📝 TODO LİSTESİ");
    println!("{}", "═══════════════".cyan());

    for todo in todos {
        let status = if todo.done { "✅" } else { "⏳" };
        let created_date = format_timestamp(todo.created_at);

        if todo.done {
            let completed_date = todo
                .completed_at
                .map(|ts| format_timestamp(ts))
                .unwrap_or_else(|| "Bilinmiyor".to_string());

            println!("{} {} ", status, todo.text.green().bold().bright_green());
            todo_done(&format!("{} ({})", todo_id(todo.id), completed_date));
        } else {
            println!("{} {} ", status, todo.text.green().bold());
            todo_pending(&format!("{} ({})", todo_id(todo.id), created_date));
        }
    }
}

fn add_todo(todos: &mut Vec<Todo>, text: &str) {
    if todos.iter().any(|t| t.text == text) {
        error("❌ Bu isimde bir todo zaten var!");
        return;
    }

    todos.push(Todo::new(Uuid::new_v4(), text.to_string()));
    success(&format!("✅ Yeni todo eklendi: {}", text));
}

fn mark_todo_done(todos: &mut [Todo], text: &str) -> bool {
    for todo in todos.iter_mut() {
        if todo.text == text {
            if todo.done {
                warning("ℹ️ Bu todo zaten tamamlanmış.");
            } else {
                todo.mark_done();
                success(&format!("✅ Todo tamamlandı: {}", todo.text));
            }
            return true;
        }
    }
    error(&format!("❌ Bu isimde bir todo bulunamadı: {}", text));
    false
}

fn remove_todo(todos: &mut Vec<Todo>, text: &str) -> bool {
    if let Some(index) = todos.iter().position(|todo| todo.text == text) {
        let removed = todos.remove(index);
        warning(&format!("🗑️ Todo silindi: {}", removed.text));
        true
    } else {
        error(&format!("❌ Bu isimde bir todo bulunamadı: {}", text));
        false
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
        success(&format!(
            "🧹 {} tamamlanmış todo temizlendi!",
            removed_count
        ));
        true
    } else {
        info("ℹ️ Hiç tamamlanmış todo bulunamadı.");
        false
    }
}
