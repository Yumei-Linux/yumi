use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub fn exec(command: String) -> Result<(), String> {
    let child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn();

    if let Err(err) = child {
        return Err(err.to_string());
    }

    let stdout = child.unwrap().stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}