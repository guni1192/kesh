use std::ffi::CString;
use std::path::Path;

use nix::unistd::chdir;

pub fn select_buildin(cmds: Vec<CString>) -> bool {
    let cmd = cmds[0].clone();
    let cmd = cmd.to_str().unwrap();
    match cmd {
        "cd" => {
            do_chdir(cmds);
            return true;
        }
        _ => return false,
    }
}

pub fn do_chdir(cmds: Vec<CString>) {
    let mut cd_path = Path::new("/");

    if cmds.len() > 1 {
        cd_path = Path::new(cmds[1].to_str().unwrap());
    }

    match chdir(cd_path) {
        Ok(_) => {}
        Err(err) => eprintln!("Faild not chdir(2): {}", err),
    }
}
