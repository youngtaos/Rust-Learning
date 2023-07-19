## cargo new 
使用cargo new 可以简单创建一个rust项目

## cargo.lock
此文件跟踪项目中依赖项的确切版本。

# 编译和运行

## cargo build
用 cargo build 构建了一个项目，并使用 ./target/debug/hello_cargo 运行它，
也可以使用 cargo run 来编译代码，然后运行生成的可执行文件

## cargo run
使用 cargo run 比记住运行 cargo build 然后使用整个二进制路径更方便，因此大多数开发人员使用 cargo run。

## cargo check
使用cargo check 比使用 cargo build 更快， 因为其会跳过生成可执行文件的步骤。
可以用来检查错误，确保项目可以编译成功。

## cargo build --release
使用cargo build --release可以编译项目并对其进行优化
此命令将在 target/relese 而不是 target/debug 文件夹中创建可执行文件。

