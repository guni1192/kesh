use std::env;
use std::ffi::CString;
use std::io::{self, Write};

use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execve, fork, ForkResult};
use colored::*;

fn input_cmd() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("hoge");
    s.trim().parse().expect("foo")
}

fn split_cmd(command: String, cmds: &mut Vec<CString>) {
    let iter = command.split_whitespace();
    for x in iter {
        let c = CString::new(x).expect("Could not parse string to char string");
        cmds.push(c);
    }
}


fn execv_wrapper(line: String, path: CString) {
    let mut cmds = Vec::<CString>::new();
    split_cmd(line.clone(), &mut cmds);

    println!("{:?}", cmds.clone());

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            match waitpid(child, None).expect("waitpid faild") {
                WaitStatus::Exited(_, _) => {}
                WaitStatus::Signaled(_, _, _) => {}
                _ => eprintln!("Unexpected exit."),
            }
        }
        Ok(ForkResult::Child) => {
            match execve(&cmds[0], &cmds, &[path]) {
                Ok(_) => {}
                Err(_) => println!("Command Not Found: {:?}", cmds.clone()),
            };
        }
        Err(err) => eprintln!("Fork faild: {}", err),
    }
}

fn print_prompt() {
    let current_path = env::current_dir().unwrap();
    let current_path = current_path.to_str().unwrap();
    println!("{}", current_path.blue());
    print!(">> ");
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    loop {
        print_prompt();
        io::stdout().flush().unwrap();

        let line = input_cmd();
        let path = env::var("PATH").expect("Environment value not found:");
        let environ = format!("PATH={}", path);
        let environ = CString::new(environ).expect("Failed to convert string to char string: ");
        execv_wrapper(line, environ);
    }
}
