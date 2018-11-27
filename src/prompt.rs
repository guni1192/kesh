use std::env;

use colored::*;
use whoami;

pub fn print_prompt() {
    let current_path = env::current_dir().unwrap();
    let current_path = current_path.to_str().unwrap();

    println!(
        "({}@{}) [{}] [{}]",
        whoami::username().green(),
        whoami::hostname().green(),
        whoami::os().cyan(),
        current_path.blue()
    );
    print!("% ");
}
