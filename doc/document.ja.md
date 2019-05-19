## import方法

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

## 使い方

### 構造体

- SystemResult

system_on_shell関数とsystem関数の結果を格納する構造体

```Rust
pub struct SystemResult {
    pub stdout: String,
    pub stderr: String,
}
```

### マクロ

- debug / d

変数のデバッグ

```
@param {$(tt)*} 一つ以上の変数または一つ以上のプレフィックステキストと変数のタプル
(e.g.) var01
(e.g.) var01, var02
(e.g.) ("old var02", var02)
(e.g.) var01, ("new var02", var02), var03
```

- err_print / e

stderrに文字列を出力する

```
@param {$(tt)*} format string
(e.g.) "This is err message"
(e.g.) "Error: {} is not found", file_name
```

- err_println / eln

改行が自動で付加されるerr_print関数

- input 

tanakhさん作の標準入力マクロ。
詳しくは[こちら](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8)まで。
diff: stdinをロックしない

### トレイト

#### VecUtils

Vec<T>のための便利トレイト

- npop

```
@param {usize} 取得する要素の数
@return {Result<Vec<T>, String>} 末尾から取得された要素のベクタ (破壊的)
```

- nget

```
@param {usize} 取得する要素の数
@return {Result<Vec<T>, String>} 末尾から取得された要素のベクタ (非破壊的)
```

- is_valid_range

```
@param {usize} 開始インデックス
@param {usize} 終了インデックス
@return {bool} 適切か否か
```

- pop_range

```
@param {usize} 開始インデックス
@param {usize} 終了インデックス
@return {Result<Vec<T>, String>} 指定した範囲のベクタ (破壊的)
```

- get_range

```
@param {usize} 開始インデックス
@param {usize} 終了インデックス
@return {Result<Vec<T>, String>} 指定した範囲のベクタ (非破壊的)
```

- mul

```
@param {usize} 繰り返す数
@return {Vec<T>} 指定回数繰り返されたベクタ
```

#### StrUtils

Stringのための便利トレイト。
VecUtilsとほとんど同じ。

### functions

- system_on_shell

新しいシェルプロセスを立ち上げ、その中でコマンドを実行し、結果を取得する

```
@param {&str} command
@return {Result<SystemResult, SystemResult>}
```

- process_on_shell

新しいシェルプロセスを立ち上げ、その中でコマンドを実行する

```
@param {&str} command
```

- system

新しいプロセスとしてコマンドを実行し、結果を取得する

```
@param {&[&str]} command (space splited)
@return {Result<SystemResult, SystemResult>}
```

- process

新しいプロセスとしてコマンドを実行する

```
@param {&[&str]} command (space splited)
```

- my_open

```
@param {&str} ファイル名
@param {&str} フラグ
    r: read
    w: write
    c: create
    a: append
    (e.g.) "rw"
    (e.g.) "wa"
    (e.g.) "rwca"
@return {Result<std::fs::File, String>}
```
