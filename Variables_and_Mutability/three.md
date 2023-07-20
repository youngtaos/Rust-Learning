# scalar and compound

请记住，Rust 是一种静态类型语言, 这意味着它必须在编译时知道所有变量的类型

## Scalar Types
Rust 有四种主要标量类型：整数、浮点数、布尔值和字符。

### Integer Types
u32 类型。 此类型声明表明与其关联的值应该是占用 32 位空间的无符号整数（有符号整数类型以 i 而不是 u 开头）

| Length      | Signed |  Unsigned |
| ----------- | ----------- |    ----------- |
| 8-bit     | i8      |   u8       |
| 16-bit   | i16        |    u16        |
|32-bit	|i32    |u32|
|64-bit	|i64	|u64|
|128-bit	|i128	|u128|
|arch	|isize	|usize|

每个变量可以是有符号的或无符号的，并且具有明确的大小。 有符号和无符号是指数字是否可能为负数，换句话说，数字是否需要带有符号（有符号），或者它是否只能为正数。  

有符号数使用二进制补码表示形式存储。

1. 有符号变量可以存储从 $-(2^n-1)$ 到 $(2^n/2)-1$（含）的数字，其中 n 是变体使用的位数。 因此，i8 可以存储从 $-2^7$ 到$2^7-1$ 的数字，相当于 -128 到 127。

2. 无符号变量可以存储从 0 到 $2^n-1$ 的数字，因此 u8 可以存储从 0 到 $2^8-1$ 的数字，相当于 0 到 255。


此外， isize 和 usize 类型取决于程序运行所在计算机的体系结构，在表中表示为“arch”：如果使用 64 位体系结构，则为 64 位；如果使用 32 位体系结构，则为 32 位。

### Floating-Point Types

###  The Boolean Type

### The Character Type

```
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```


## Compound Types
复合类型可以将多个值分组为一种类型。 Rust 有两种原始复合类型：元组和数组。

### The Tuple Type

```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
元组是将具有多种类型的多个值组合在一起形成一个复合类型的通用方法。 元组有固定的长度：一旦声明，它们的大小就不能改变。


 为了从元组中获取单个值，可以使用模式匹配来解构元组值：
```
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

我们还可以通过使用句点 (.) 后跟要访问的值的索引来直接访问元组元素:
```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### The Array Type
拥有多个值的集合的另一种方法是使用数组。 与元组不同，数组的每个元素必须具有相同的类型。 与其他一些语言中的数组不同，Rust 中的数组具有固定长度。