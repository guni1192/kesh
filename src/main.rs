use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use std::path::Path;

use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execve, fork, ForkResult};

mod prompt;

fn input_cmd() -> String {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Cannot read stdin");
    s.trim().parse().expect("Cannot parse trim str")
}

fn split_cmd(command: String, cmds: &mut Vec<CString>) {
    let mut iter = command.split_whitespace();

    let c = match iter.next() {
        Some(cmd) => cmd.to_string(),
        None => return,
    };

    let cmd = realpath_from_string(c);
    let cmd = CString::new(cmd).expect("Could not parse string to char string");
    cmds.push(cmd);

    for x in iter {
        let c = CString::new(x).expect("Could not parse string to char string");
        cmds.push(c);
    }
}

fn execv_wrapper(line: String, path: CString) {
    let mut cmds = Vec::<CString>::new();
    split_cmd(line.clone(), &mut cmds);

    if cmds.len() == 0 {
        return;
    }

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
                Err(_) => eprintln!("{:?} not found.", cmds[0].clone()),
            };
        }
        Err(err) => eprintln!("Fork faild: {}", err),
    }
}

fn realpath_from_string(cmd: String) -> String {
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let cmd_full_path = format!("{}/{}", path.to_str().unwrap(), cmd);

                if Path::new(&cmd_full_path).exists() {
                    return cmd_full_path;
                }
            }
            cmd
        }
        None => cmd,
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    loop {
        prompt::print_prompt();
        io::stdout().flush().unwrap();

        let line = input_cmd();
        let path = env::var("PATH").expect("Environment value not found:");
        let environ = format!("PATH={}", path);
        let environ = CString::new(environ).expect("Failed to convert string to char string: ");
        execv_wrapper(line, environ);
    }
}

#[test]
fn test_realpath_from_path() {
    assert_eq!(
        realpath_from_string("bash".to_string()),
        "/bin/bash".to_string()
    );
}
