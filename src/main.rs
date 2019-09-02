use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use std::path::Path;

use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execve, fork, ForkResult};

mod buildin;
mod prompt;

static DEFAULT_PATH_ENV: &'static str = "PATH";

fn input_cmd() -> String {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Cannot read stdin");
    s.trim().parse().expect("Cannot parse trim str")
}

fn split_cmd(command: &str) -> Vec<CString> {
    let mut args = Vec::<CString>::new();
    let args_tmp: Vec<&str> = command.split_whitespace().collect();

    for i in 0..args_tmp.len() {
        let cmd = if i == 0 {
            CString::new(realpath_from_string(&args_tmp[i]))
        } else {
            CString::new(args_tmp[i])
        };
        let cmd = cmd.expect("Could not parse string to char string");
        args.push(cmd);
    }
    args
}

fn execv_wrapper(line: &str, path: CString) {
    let args = split_cmd(line);

    if args.len() == 0 {
        return;
    }

    if buildin::select_buildin(args.clone()) {
        return;
    }

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            match waitpid(child, None).expect("waitpid faild") {
                WaitStatus::Exited(_, _) => {}
                WaitStatus::Signaled(_, _, _) => {
                    // TODO: SIGKILL, SIGTERM
                }
                _ => eprintln!("Unexpected exit."),
            }
        }
        Ok(ForkResult::Child) => {
            execve(&args[0], &args, &[path])
                .unwrap_or_else(|e| panic!("Error while execve: {}", e));
        }
        Err(err) => eprintln!("Fork faild: {}", err),
    }
}

fn realpath_from_string(cmd: &str) -> String {
    match env::var_os(DEFAULT_PATH_ENV) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let cmd_full_path = format!("{}/{}", path.to_str().unwrap(), cmd);

                if Path::new(&cmd_full_path).exists() {
                    return cmd_full_path;
                }
            }
            cmd.to_string()
        }
        None => cmd.to_string(),
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
        execv_wrapper(&line, environ);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_realpath_from_path() {
        let v = vec!["/bin/bash", "/usr/bin/bash"];
        assert!(v
            .into_iter()
            .find(|&x| x == realpath_from_string("bash"))
            .is_some(),);
    }
}
