## How to import

```Bash
$ cat Cargo.toml
...
[dependencies]
renert = { git = "https://github.com/miyagaw61/renert" }
$ cat src/main.rs
extern crate renert;
use renert::*;
...
```

## Usage

### structs

- SystemResult

result struct for system_on_shell function and system function

```Rust
pub struct SystemResult {
    pub stdout: String,
    pub stderr: String,
}
```

### macros

- debug / d

print data of variables

```
@param {$(tt)*} some variables or some tuples of prefix-text and variable.
(e.g.) var01
(e.g.) var01, var02
(e.g.) ("old var02", var02)
(e.g.) var01, ("new var02", var02), var03
```

- err_print / e

print err message

```
@param {$(tt)*} format string
(e.g.) "This is err message"
(e.g.) "Error: {} is not found", file_name
```

- err_println / eln

err_print with return

- input 

This is made by tanakh.
Check [here](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8) for details.
diff: Don't lock stdin

### traits

#### VecUtils

This is a utils trait for Vec<T>.

- npop

```
@param {usize} number of elements
@return {Result<Vec<T>, String>} elements poped from back
```

- nget

```
@param {usize} number of elements
@return {Result<Vec<T>, String>} elements geted from back
```

- is_valid_range

```
@param {usize} index of start
@param {usize} index of end
@return {bool} valid or invalid
```

- pop_range

```
@param {usize} index of start
@param {usize} index of end
@return {Result<Vec<T>, String>} poped elements
```

- get_range

```
@param {usize} index of start
@param {usize} index of end
@return {Result<Vec<T>, String>} geted elements
```

- mul

```
@param {usize} number of multiplication
@return {Vec<T>} new Vec
```

#### StrUtils

This is a utils trait for String.
This is almost the same as VecUtils.

### functions

- system_on_shell

execute command on new shell process and get result

```
@param {&str} command
@return {Result<SystemResult, SystemResult>}
```

- process_on_shell

execute command on new shell process

```
@param {&str} command
```

- system

create new process and get result

```
@param {&[&str]} command (space splited)
@return {Result<SystemResult, SystemResult>}
```

- process

create new process

```
@param {&[&str]} command (space splited)
```

- my_open

```
@param {&str} file name
@param {&str} flags
    r: read
    w: write
    c: create
    a: append
    (e.g.) "rw"
    (e.g.) "wa"
    (e.g.) "rwca"
@return {Result<std::fs::File, String>}
```
