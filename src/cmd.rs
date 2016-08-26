use std::process::Command;

use error::FatalError;

fn do_call(command: Vec<&str>, path: Option<&str>, dry_run: bool) -> Result<bool, FatalError> {
    if dry_run {
        if path.is_some() {
            println!("cd {}", path.unwrap());
        }
        println!("{}", command.join(" "));
        if path.is_some() {
            println!("cd -");
        }
        return Ok(true);
    }
    let mut iter = command.iter();
    let cmd_name = iter.next().unwrap();

    let mut cmd = Command::new(cmd_name);

    if path.is_some() {
        cmd.current_dir(path.unwrap());
    }

    for arg in iter {
        if !arg.is_empty() {
            cmd.arg(arg);
        }
    }

    let mut child = try!(cmd.spawn().map_err(FatalError::from));
    let result = try!(child.wait().map_err(FatalError::from));

    Ok(result.success())
}

pub fn call(command: Vec<&str>, dry_run: bool) -> Result<bool, FatalError> {
    do_call(command, None, dry_run)
}

pub fn call_on_path(command: Vec<&str>, path: &str, dry_run: bool) -> Result<bool, FatalError> {
    do_call(command, Some(path), dry_run)
}
