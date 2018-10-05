extern crate colored;

use std::process::Command;
use colored::*;

pub struct SystemResult {
    pub stdout: String,
    pub stderr: String,
    pub status: i32
}

impl SystemResult {
    fn new(output: std::process::Output) -> SystemResult {
        let mut stdout: Vec<char> = std::str::from_utf8(&output.stdout[..]).unwrap().to_string().chars().collect();
        stdout.pop();
        let stdout: String = stdout.into_iter().collect();
        let mut stderr: Vec<char> = std::str::from_utf8(&output.stderr[..]).unwrap().to_string().chars().collect();
        stderr.pop();
        let stderr: String = stderr.into_iter().collect();
        let mut result = SystemResult {
            stdout: stdout,
            stderr: stderr,
            status: 0
        };
        if result.stderr != "" {
            result.status = 1
        }
        result
    }
}

pub fn my_eprint(msg: String) {
    let header = [
        "== ".red().to_string(),
        "[+]ERROR".red().bold().to_string(),
        " =====================".red().to_string()
    ].join("");
    println!("{}", header);
    println!("{}", msg);
    println!("{}", "=================================".red().to_string());
}

pub fn gen_systemresult(oput: std::process::Output) -> Result<SystemResult, SystemResult> {
    let system_result = SystemResult::new(oput);
    if system_result.status == 1 {
        return Err(system_result);
    }
    return Ok(system_result);
}

pub fn system_on_shell(command: &str) -> Result<SystemResult, SystemResult> {
    let oput = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect(format!("Failed to execute process: \"sh -c '{}'\"", command).as_str());
    return gen_systemresult(oput);
}

pub fn process_on_shell(command: &str) {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect(format!("Failed to execute process: \"sh -c '{}'\"", command).as_str());
    child.wait().expect(format!("Failed to execute process: \"sh -c '{}'\"", command).as_str());
}

pub fn system(command: &[&str]) -> Result<SystemResult, SystemResult> {
    let oput = Command::new(command[0])
        .args(&command[1..])
        .output()
        .expect(format!("Failed to execute process: \"sh -c '{}'\"", command.join(" ")).as_str());
    return gen_systemresult(oput);
}

pub fn process(command: &[&str]) {
    let mut child = Command::new(command[0])
        .args(&command[1..])
        .spawn()
        .expect(format!("Failed to execute process: \"sh -c '{}'\"", command.join(" ")).as_str());
    child.wait().expect(format!("Failed to execute process: \"sh -c '{}'\"", command.join(" ")).as_str());
}
