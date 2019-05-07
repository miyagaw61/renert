extern crate colored;

use std::fs::OpenOptions;
use std::process::Command;
use colored::*;

#[macro_export]
macro_rules! d {
    ($t:expr, $($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), stderr());
            writeln!(err, "{} = {:?}", $t.yellow().to_string(), $e).unwrap()
        })*
    };
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), stderr());
            writeln!(err, "{} = {:?}", e.yellow().to_string(), $e).unwrap()
        })*
    };
}

// ref: tanakh <https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8>
// diff: Don't lock stdin
#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

pub struct SystemResult {
    pub stdout: String,
    pub stderr: String,
}

impl SystemResult {
    fn new(output: std::process::Output) -> Result<SystemResult, SystemResult> {
        let mut stdout: Vec<char> = std::str::from_utf8(&output.stdout[..]).unwrap().to_string().chars().collect();
        stdout.pop();
        let stdout: String = stdout.into_iter().collect();
        let mut stderr: Vec<char> = std::str::from_utf8(&output.stderr[..]).unwrap().to_string().chars().collect();
        stderr.pop();
        let stderr: String = stderr.into_iter().collect();
        let result = SystemResult {
            stdout: stdout,
            stderr: stderr,
        };
        if result.stderr != "" {
            return Err(result);
        }
        return Ok(result);
    }
}

impl From<String> for SystemResult {
    fn from(e: String) -> SystemResult {
        let system_result = SystemResult {
            stdout: "".to_string(),
            stderr: format!("Failed to excute process: {}", e)
        };
        return system_result;
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

pub fn system_on_shell(command: &str) -> Result<SystemResult, SystemResult> {
    let oput = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| format!("{}: \"{}\"", e.to_string(), command));
    match oput {
        Ok(oput) => return SystemResult::new(oput),
        Err(e) => return Err(SystemResult::from(e))
    }
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
        .map_err(|e| format!("{}: \"{}\"", e.to_string(), command.join(" ")));
    match oput {
        Ok(oput) => return SystemResult::new(oput),
        Err(e) => return Err(SystemResult::from(e))
    }
}

pub fn process(command: &[&str]) {
    let mut child = Command::new(command[0])
        .args(&command[1..])
        .spawn()
        .expect(format!("Failed to execute process: \"sh -c '{}'\"", command.join(" ")).as_str());
    child.wait().expect(format!("Failed to execute process: \"sh -c '{}'\"", command.join(" ")).as_str());
}

pub fn my_open(filename: &str, flag: &str) -> Result<std::fs::File, String> {
    let mut op = OpenOptions::new();
    if flag.contains("r") {
        op.read(true);
    }
    if flag.contains("w") {
        op.write(true);
    }
    if flag.contains("c") {
        op.create(true);
    }
    if flag.contains("a") {
        op.append(true);
    }
    return op.open(filename).map_err(|e| e.to_string());
}

pub fn str_mul(s: &str, n: i32) -> String {
    let mut res: String = "".to_string();
    for _ in 0..n {
        for c in s.chars() {
            res.push(c);
        }
    }
    return res;
}

pub fn bytes_mul(bytes: &[u8], n: i32) -> std::vec::Vec<u8> {
    let mut res = vec![];
    for _ in 0..n {
        for b in bytes {
            res.push(*b);
        }
    }
    return res;
}
