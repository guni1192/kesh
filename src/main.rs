use std::env;
use std::ffi::CString;

use nix::unistd::execv;

pub fn input_cmd() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("hoge");
    s.trim().parse().expect("foo")
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let s = input_cmd();
    let cmd = CString::new(s.clone()).expect("Could not parse string to char string");

    execv(&cmd, &[cmd.clone()]).expect(&format!("Command Not Found: {}", s.clone()));
}
