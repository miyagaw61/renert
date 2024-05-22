#![allow(unused_imports)]

extern crate byteorder;
extern crate colored;
extern crate find_folder;

pub use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian};
pub use colored::*;
pub use std::cmp;
pub use std::collections::{HashMap, VecDeque};
pub use std::fs::OpenOptions;
pub use std::io::{stderr, stdin, stdout, BufReader, BufWriter, Read, Write};
pub use std::process::{exit, Command};

#[macro_export]
macro_rules! debug_one {
    (($t:expr, $e:expr)) => {
        #[cfg(debug_assertions)]
        {
            let mut err = stderr();
            writeln!(err, "\x1B[33m{}\x1B[0m = {:?}", $t, $e).unwrap()
        }
    };
    ($e:expr) => {
        #[cfg(debug_assertions)]
        {
            let e = stringify!($e);
            let mut err = stderr();
            writeln!(err, "\x1B[33m{}\x1B[0m = {:?}", e, $e).unwrap()
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($tt:tt),*) => {
        #[cfg(debug_assertions)]
        $({
            debug_one!($tt);
        })*
    };
}

#[macro_export]
macro_rules! d {
    ($($tt:tt),*) => {
        #[cfg(debug_assertions)]
        $({
            debug!($tt);
        })*
    };
}

#[macro_export]
macro_rules! err_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let mut err = stderr();
            let e = format!($($arg)*);
            write!(err, "{}", e.green().bold().to_string()).unwrap()
        }
    };
}

#[macro_export]
macro_rules! e {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            err_print!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! err_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let mut err = stderr();
            let e = format!($($arg)*);
            writeln!(err, "{}", e.green().bold().to_string()).unwrap()
        }
    };
}

#[macro_export]
macro_rules! eln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            err_println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! stdin {
    () => {{
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
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

#[macro_export]
macro_rules! test {
    (name = $name:ident, $($input:expr => $output:expr),* $(,)*) => (
        #[test]
        fn $name() {
            $(
                assert_eq!(solve($input.to_string()), $output);
            )*
        }
    );
    ($($input:expr => $output:expr),* $(,)*) => (
        #[test]
        fn solve_test() {
            $(
                assert_eq!(solve($input.to_string()), $output);
            )*
        }
    )
}

macro_rules! to_T {
    ($v: expr, $e: expr, $n: ident, $t: ty) => {{
        let s: &[u8] = &$v[..];
        let mut _n: $t = 0;
        let endian = $e;
        match endian {
            "native" => {
                _n = NativeEndian::$n(s);
            }
            "little" => {
                _n = LittleEndian::$n(s);
            }
            "big" => {
                _n = BigEndian::$n(s);
            }
            _ => {
                return Err("".to_string());
            }
        }
        Ok(_n)
    }};
}

#[derive(Debug)]
pub struct SystemResult {
    pub stdout: String,
    pub stderr: String,
}

impl SystemResult {
    fn new(output: std::process::Output) -> Result<SystemResult, SystemResult> {
        let mut stdout: Vec<char> = std::str::from_utf8(&output.stdout[..])
            .unwrap()
            .to_string()
            .chars()
            .collect();
        stdout.pop();
        let stdout: String = stdout.into_iter().collect();
        let mut stderr: Vec<char> = std::str::from_utf8(&output.stderr[..])
            .unwrap()
            .to_string()
            .chars()
            .collect();
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
            stderr: format!("Failed to excute process: {}", e),
        };
        return system_result;
    }
}

pub fn my_eprint(msg: String) {
    let header = [
        "== ".red().to_string(),
        "[+]ERROR".red().bold().to_string(),
        " =====================".red().to_string(),
    ]
    .join("");
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
        Err(e) => return Err(SystemResult::from(e)),
    }
}

pub fn process_on_shell(command: &str) {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect(format!("Failed to execute process: \"sh -c '{}'\"", command).as_str());
    child
        .wait()
        .expect(format!("Failed to execute process: \"sh -c '{}'\"", command).as_str());
}

pub fn system(command: &[&str]) -> Result<SystemResult, SystemResult> {
    let oput = Command::new(command[0])
        .args(&command[1..])
        .output()
        .map_err(|e| format!("{}: \"{}\"", e.to_string(), command.join(" ")));
    match oput {
        Ok(oput) => return SystemResult::new(oput),
        Err(e) => return Err(SystemResult::from(e)),
    }
}

pub fn process(command: &[&str]) {
    let mut child = Command::new(command[0]).args(&command[1..]).spawn().expect(
        format!(
            "Failed to execute process: \"sh -c '{}'\"",
            command.join(" ")
        )
        .as_str(),
    );
    child.wait().expect(
        format!(
            "Failed to execute process: \"sh -c '{}'\"",
            command.join(" ")
        )
        .as_str(),
    );
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

pub fn bytes_mul(bytes: &[u8], n: i32) -> std::vec::Vec<u8> {
    let mut res = vec![];
    for _ in 0..n {
        for b in bytes {
            res.push(*b);
        }
    }
    return res;
}

pub trait VecUtils<T: Clone> {
    fn npop(&mut self, n: usize) -> Result<Vec<T>, String>;
    fn nget(&self, n: usize) -> Result<Vec<T>, String>;
    fn is_valid_range(&self, idx_a: usize, idx_b: usize) -> bool;
    fn get_range(&self, idx_a: usize, idx_b: usize) -> Result<Vec<T>, String>;
    fn pop_range(&mut self, idx_a: usize, idx_b: usize) -> Result<Vec<T>, String>;
    fn mul(&self, n: usize) -> Vec<T>;
}

pub trait StrUtils {
    fn npop(&mut self, n: usize) -> Result<String, String>;
    fn nget(&self, n: usize) -> Result<String, String>;
    fn is_valid_range(&self, idx_a: usize, idx_b: usize) -> bool;
    fn get_range(&self, idx_a: usize, idx_b: usize) -> Result<String, String>;
    fn pop_range(&mut self, idx_a: usize, idx_b: usize) -> Result<String, String>;
    fn mul(&self, n: usize) -> String;
}

pub trait BytesUtils {
    fn to_u32(&self, endian: &str) -> Result<u32, String>;
    fn to_u64(&self, endian: &str) -> Result<u64, String>;
    fn to_u128(&self, endian: &str) -> Result<u128, String>;
}

impl<T: Clone> VecUtils<T> for Vec<T> {
    fn npop(&mut self, n: usize) -> Result<Vec<T>, String> {
        let len = self.len();
        if n > len {
            return Err("too large num".to_string());
        }
        let mut res: Vec<T> = Vec::new();
        for _ in 0..n {
            let x = self.pop().ok_or("".to_string())?;
            res.push(x);
        }
        res.reverse();
        Ok(res)
    }

    fn nget(&self, n: usize) -> Result<Vec<T>, String> {
        let len = self.len();
        if n > len {
            return Err("too large num".to_string());
        }
        let mut res: Vec<T> = Vec::new();
        for i in 0..n {
            res.push(self[len - 1 - i].clone());
        }
        Ok(res)
    }

    fn is_valid_range(&self, idx_a: usize, idx_b: usize) -> bool {
        let mut can = true;
        let self_len = self.len();
        if self_len < idx_a {
            can = false;
        }
        if self_len < idx_b {
            can = false;
        }
        can
    }

    fn get_range(&self, idx_a: usize, idx_b: usize) -> Result<Vec<T>, String> {
        if !self.is_valid_range(idx_a, idx_b) {
            return Err("invalid range".to_string());
        }
        let mut res: Vec<T> = Vec::new();
        for (i, x) in self.into_iter().enumerate() {
            let i = i;
            if i < idx_a {
                continue;
            }
            if idx_b <= i {
                break;
            }
            res.push(x.clone());
        }
        Ok(res)
    }

    fn pop_range(&mut self, idx_a: usize, idx_b: usize) -> Result<Vec<T>, String> {
        if !self.is_valid_range(idx_a, idx_b) {
            return Err("invalid range".to_string());
        }
        let mut res: Vec<T> = Vec::new();
        let mut pop_idxs: Vec<usize> = Vec::new();
        for (i, x) in self.iter().enumerate() {
            let i = i;
            if i < idx_a {
                continue;
            }
            if idx_b <= i {
                break;
            }
            res.push(x.clone());
            pop_idxs.push(i);
        }
        for _ in 0..pop_idxs.len() {
            match pop_idxs.pop() {
                Some(idx) => {
                    self.remove(idx as usize);
                }
                None => {}
            }
        }
        Ok(res)
    }

    fn mul(&self, n: usize) -> Vec<T> {
        let mut res: Vec<T> = Vec::new();
        for _ in 0..n {
            for x in self {
                res.push(x.clone());
            }
        }
        res
    }
}

impl BytesUtils for Vec<u8> {
    fn to_u32(&self, endian: &str) -> Result<u32, String> {
        to_T!(&self, endian, read_u32, u32)
    }
    fn to_u64(&self, endian: &str) -> Result<u64, String> {
        to_T!(&self, endian, read_u64, u64)
    }
    fn to_u128(&self, endian: &str) -> Result<u128, String> {
        to_T!(&self, endian, read_u128, u128)
    }
}

impl StrUtils for String {
    fn npop(&mut self, n: usize) -> Result<String, String> {
        let mut self_chars: Vec<char> = self.chars().collect();
        match self_chars.npop(n) {
            Ok(poped) => {
                *self = self_chars.into_iter().collect::<String>();
                Ok(poped.into_iter().collect::<String>())
            }
            Err(e) => Err(e),
        }
    }

    fn nget(&self, n: usize) -> Result<String, String> {
        let self_chars: Vec<char> = self.chars().collect();
        match self_chars.nget(n) {
            Ok(v) => Ok(v.into_iter().collect::<String>()),
            Err(e) => Err(e),
        }
    }

    fn is_valid_range(&self, idx_a: usize, idx_b: usize) -> bool {
        let self_chars: Vec<char> = self.chars().collect();
        self_chars.is_valid_range(idx_a, idx_b)
    }

    fn get_range(&self, idx_a: usize, idx_b: usize) -> Result<String, String> {
        let self_chars: Vec<char> = self.chars().collect();
        match self_chars.get_range(idx_a, idx_b) {
            Ok(v) => Ok(v.into_iter().collect::<String>()),
            Err(e) => Err(e),
        }
    }

    fn pop_range(&mut self, idx_a: usize, idx_b: usize) -> Result<String, String> {
        let mut self_chars: Vec<char> = self.chars().collect();
        match self_chars.pop_range(idx_a, idx_b) {
            Ok(poped) => {
                *self = self_chars.into_iter().collect::<String>();
                Ok(poped.into_iter().collect::<String>())
            }
            Err(e) => Err(e),
        }
    }

    fn mul(&self, n: usize) -> String {
        let mut res: String = String::new();
        for _ in 0..n {
            res.push_str(self);
        }
        res
    }
}

pub fn search_dir(
    dirname: &str,
    kids_depth: u8,
    parents_depth: u8,
) -> Result<std::path::PathBuf, find_folder::Error> {
    let dir = find_folder::Search::KidsThenParents(kids_depth, parents_depth).for_folder(dirname);
    dir
}
