use colored::Colorize;

pub fn success(text: &str) {
    println!("{}", text.green());
}

pub fn error(text: &str) {
    println!("{}", text.red());
}

pub fn info(text: &str) {
    println!("{}", text.blue());
}

pub fn warning(text: &str) {
    println!("{}", text.yellow());
}

pub fn highlight(text: &str) {
    println!("{}", text.cyan().bold());
}

// Todo durumları için özel renkler
pub fn todo_done(text: &str) {
    println!("{}", text.green().strikethrough());
}

pub fn todo_pending(text: &str) {
    println!("{}", text.white());
}

pub fn todo_id(id: uuid::Uuid) -> String {
    format!("{}", format!("[{}]", id).strikethrough().bold())
}