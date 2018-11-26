use std::env;
use std::ffi::CString;
use std::io::{self, Write};

use nix::unistd::execv;

fn input_cmd() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("hoge");
    s.trim().parse().expect("foo")
}

fn print_prompt() {
    print!(">> ");
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    loop {
        print_prompt();
        io::stdout().flush().unwrap();

        let s = input_cmd();
        let cmd = CString::new(s.clone()).expect("Could not parse string to char string");

        // execv(&cmd, &[cmd.clone()]).expect(&format!("Command Not Found: {}", s.clone()));
        match execv(&cmd, &[cmd.clone()]) {
            Ok(_) => {}
            Err(_) => println!("Command Not Found: {}", s.clone()),
        };
    }
}
