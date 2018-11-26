use std::env;
use std::ffi::CString;
use std::io::{self, Write};

use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execv, fork, ForkResult};

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

fn print_prompt() {
    print!(">> ");
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    loop {
        print_prompt();
        io::stdout().flush().unwrap();

        let s = input_cmd();
        let mut cmds = Vec::<CString>::new();
        split_cmd(s.clone(), &mut cmds);

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
                match execv(&cmds[0], &cmds) {
                    Ok(_) => {}
                    Err(_) => println!("Command Not Found: {}", s.clone()),
                };
            }
            Err(err) => eprintln!("Fork faild: {}", err),
        }
    }
}
