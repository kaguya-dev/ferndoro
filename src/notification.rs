use std::process::Command;

pub struct Notifier;

impl Notifier {
    pub fn notify(title: &str, message: &str) {
        // Output terminal bell
        print!("\x07");

        // Try notify-send on Linux
        let _ = Command::new("notify-send")
            .arg("-a")
            .arg("FernDoro")
            .arg("-i")
            .arg("alarm")
            .arg("-u")
            .arg("normal")
            .arg(title)
            .arg(message)
            .spawn();
    }
}
