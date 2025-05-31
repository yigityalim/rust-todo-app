// 

use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

pub fn show_loading(message: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message(message.to_string());

    // Kısa bir loading simülasyonu
    for _ in 0..10 {
        pb.tick();
        thread::sleep(Duration::from_millis(50));
    }

    pb.finish_with_message("✅ Tamamlandı!");
}

pub fn show_progress_bar(total: usize, message: &str) {
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ")
    );
    pb.set_message(message.to_string());

    for i in 0..total {
        pb.set_position(i as u64 + 1);
        thread::sleep(Duration::from_millis(100)); // Gerçek işlem simülasyonu
    }

    pb.finish_with_message("✅ İşlem tamamlandı!");
}