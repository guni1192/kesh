use std::ffi::CString;
use std::path::Path;

use nix::unistd::chdir;

pub fn select_buildin(args: Vec<CString>) -> bool {
    let cmd = args[0].clone();
    let cmd = cmd.to_str().unwrap();
    match cmd {
        "cd" => {
            do_chdir(args);
            return true;
        }
        _ => return false,
    }
}

pub fn do_chdir(args: Vec<CString>) {
    let cd_path = if args.len() > 1 {
        Path::new(args[1].to_str().unwrap())
    } else {
        Path::new(env!("HOME"))
    };

    chdir(cd_path).unwrap_or_else(|e| eprintln!("failed to chdir: {}", e));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_select_buildin() {
        let cd_command = {
            let mut c = Vec::<CString>::new();
            c.push(CString::new("cd").unwrap());
            c
        };
        assert!(select_buildin(cd_command));
    }

    #[test]
    fn test_do_chdir() {
        let cd_command = {
            let mut c = Vec::<CString>::new();
            c.push(CString::new("cd").unwrap());
            c
        };
        do_chdir(cd_command);
        let pwd = std::env::current_dir().unwrap();
        let home_dir = Path::new(env!("HOME"));
        assert_eq!(pwd, home_dir)
    }
}
