
# immutable
在Rust中，变量默认是不可变的
`let immutable_variables = 1`

# mutable
也可以创建可变的变量
`let mut mutable_variables = 2`

# 输入输出

## 输入

### io
可以通过`use std::io`引入io库，使用`io::Stdin`获取用户的输入
也可以不用`use std::io`引入io库，而是直接使用`std::io::Stdin`，来获取用户的输入

### expect
`read_line` 将用户输入的任何内容放入我们传递给它的字符串中，但它也返回一个 Result 值。
这个Result 结果是一个枚举类型的值 `Result<usize, Error>`
如果 Result 的此实例是 Err 值，expect 将导致程序崩溃并显示您作为参数传递给expect 的消息。
如果 read_line 方法返回 Err，则可能是来自底层操作系统的错误的结果。
如果不调用`expect`,将会收到警告：
```
warning: unused `Result` that must be used
  --> src/main.rs:11:5
   |
11 | /     io::stdin()
12 | |         .read_line(&mut guess);
   | |______________________________^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
11 |     let _ = io::stdin()
   |     +++++++

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
```

## &
& 表示该参数是一个引用，提供了一种让代码的多个部分访问一条数据的方法，而无需多次将该数据复制到内存中。 
引用是一项复杂的功能，Rust 的主要优点之一是使用引用的安全性和易用性。 您不需要知道很多细节来完成这个程序。 现在，您需要知道的是，与变量一样，默认情况下引用是不可变的。 因此，您需要编写 &mut Guess 而不是 &Guess 来使其可变。

## 输出
`println!("You guessed: {guess}");`
大括号 {} 组是一个占位符。 
打印变量值时，变量名称可以放在大括号内。 

```
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);

```
打印表达式求值结果时，将空大括号放在格式字符串中，然后在格式字符串后跟上以逗号分隔的表达式列表，以相同的顺序在每个空大括号占位符中打印。 调用 println! 中打印变量和表达式的结果为`x = 5 and y + 2 = 12`

