use std::process::Command;

use cmd::call;
use error::FatalError;
use cargo::core::MultiShell;

pub fn remote_get_url(remote: &str) -> Result<String, FatalError> {
    let output = try!(Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg(remote)
        .output());

    let url = try!(String::from_utf8(output.stdout));
    Ok(url)
}

pub fn init(dir: &str, shell: &mut MultiShell, dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["git", "init"], dir, shell, dry_run)
}

pub fn add_all(dir: &str, shell: &mut MultiShell, dry_run: bool) -> Result<bool, FatalError> {
    call(vec!["git", "add", "."], dir, shell, dry_run)
}

pub fn commit_all(dir: &str,
                  msg: &str,
                  sign: bool,
                  shell: &mut MultiShell,
                  dry_run: bool)
                  -> Result<bool, FatalError> {
    call(vec!["git", "commit", if sign { "-S" } else { "" }, "-am", msg],
         dir,
         shell,
         dry_run)
}

pub fn force_push(dir: &str,
                  remote: &str,
                  refspec: &str,
                  shell: &mut MultiShell,
                  dry_run: bool)
                  -> Result<bool, FatalError> {
    call(vec!["git", "push", "-f", remote, refspec],
         dir,
         shell,
         dry_run)
}
