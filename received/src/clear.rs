use std::process::Command;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .expect("failed to execute process");
    } else {
        Command::new("clear")
            .status()
            .expect("failed to execute process");
    }
}