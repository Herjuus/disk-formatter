use std::process::Command;

fn clear_screen() {
    #[cfg(target_os = "windows")]
    let _ = Command::new("cls").status();
    #[cfg(target_os = "macos")]
    let _ = Command::new("clear").status();
}
    
pub fn init() {
    clear_screen();
    
    println!("");
    println!("  _____ _____  _____ _  __  ______ ____  _____  __  __       _______ _______ ______ _____  ");
    println!(" |  __ \\_   _|/ ____| |/ / |  ____/ __ \\|  __ \\|  \\/  |   /\\|__   __|__   __|  ____|  __ \\ ");
    println!(" | |  | || | | (___ | ' /  | |__ | |  | | |__) | \\  / |  /  \\  | |     | |  | |__  | |__) |");
    println!(" | |  | || |  \\___ \\|  <   |  __|| |  | |  _  /| |\\/| | / /\\ \\ | |     | |  |  __| |  _  / ");
    println!(" | |__| || |_ ____) | . \\  | |   | |__| | | \\ \\| |  | |/ ____ \\| |     | |  | |____| | \\ \\ ");
    println!(" |_____/_____|_____/|_|\\_\\ |_|   \\____/|_|  \\_\\_|  |_|/_/    \\_\\_|     |_|  |______|_|  \\_\\");
    println!("  By Herjus");
    println!("");
}