## Rust入门
#### 安装方式
```shell
# macos或linux
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
# 其他方式
参见：https://www.rust-lang.org/tools/install
```
#### 查看本地文档
```shell
rustup doc
```
#### hello world
```rust
// main.rs
// 执行步骤
// 1. rustc main.rs  => 生成可执行文件main
// 2. ./main

// fn function_name {} 定义main函数
// main特殊函数，可执行程序入口
fn main() {
    // println! 对rust宏println的调用
    println!("Hello world!");
    // ; 结尾表示是语句 语句没有返回值
    // 无;结尾表示表达式 表达式有返回值
    // 编译器发现表达式后有分号结尾时，在编译期间会自动修改代码，它会在分号的后面加上一个小括号()。单独的小括号是一个特殊的值，表示什么也不做
    // 用于声明或定义的代码都是语句
}

```
#### cargo
```
rust的构建及包管理工具，包含在rust官方安装包中
创建一个项目（使用cargo）
cargo new project_name
cargo init
构建
cargo build 会在target/debug/下生成可执行程序
运行
cargo run  构建并运行
校验是否可编译
cargo check 比cargo build快，跳过了生产可以执行程序的过程
cargo build --release 编译并优化 输出文件在target/release目录
```
#### 编码规范
 * 源代码文件名以_分隔，hello_world.rs
 * 代码缩紧风格 四个空格 而不是tab

## 编程练习-猜数字游戏
#### 语法
```rust
use std::io; // 导包 std标准库
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // 声明 可变变量 guess，默认变量是不可变的，需要mut标识可变
    // String::new() String 类型由标准库提供，是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。::表明new是String的一个方法
    let mut guess = String::new();  
    // io::stdin <==> std::io::stdin
    io::stdin()
        // & 引用符号
        // 引用和变量一样，默认是 不可变 的，需要mut标识为可变
        .read_line(&mut guess) // 将控制台读取到的数据 追加 到guess
        .expect("Failed to read line");
    // {} 是个占位符
    println!("You guessed: {guess}");
}
```
```rust
// 生成随机数
use rand::Rng; // 导入Rng接口实现
fn main() {
    println!("Please input your number");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number is: {secret_number}");
}

```
```rust
use rand::Rng; // 导入Rng接口实现
use std::io;
fn main() {
    println!("Please input your number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    /// while (true) <==> loop
    while (true) {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed read your guess");
        // 声明变量并指定类型
        // i32、u32、i64、u64，默认是i32
        let number: i32 = guess.trim().parse().expect("输入错误！");
        match number.cmp(&secret_number) {
            // Ordering 枚举
            std::cmp::Ordering::Equal => {
                println!("Your win!");
                // 跳出循环
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Less => println!("Too small"),
        }
    }
}

```
```yaml
# Cargo.toml
[dependencies]
# "0.8.5" 是^0.8.5的简写，表示>=0.8.5并且<=0.9.0
rand = "0.8.5"
```
## 常用概念
* 变量 默认是不可变的 let number = 5; 可通过mut修饰为可变 let mut number = 5;
* 常量 const NUMBER: i32 = 5; 默认不可变，不需要mut修饰；必须指定类型；通常用大写命名下划线分隔；可以是表达式 const NUMBER: i32=60*60; 编译时由编译器计算；
* 静态量 static N: i32=60*60; 静态量全局共享，不内联，每一个值只有一个实例，并且位于内存中的固定位置；
* 数据类型，rust是静态类型语言，编译时必须知道所有变量的类型；
* 标量类型（scalar） integers、floating-point numbers、boolean、characters

| length | signed | unsigned |
| ---- | ---- | ---- |
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

| Number literal | Example |
| - | - |
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b111_110 |
| Byte(u8 only) | b'A' |

* integer溢出 debug模式编译时，rust会进行check是否溢出，release模式下会回check溢出，对于u8，256 => 0 257=> 1
* 浮点类型 floating-point f32、f64（默认），
* boolean 一字节 true、false 
* 字符 character 四字节 unicode 
* 组合类型 元组tuples、数组arrays  let tup: (i32,f64) = (500, 6.4); println!("{}", tup.0)
* array 数组 固定长度 let a = [1,2,3]; let b:[i32,5]=[1,2,3,4,5]; let c = [3;5]; // [3,3,3,3,3]; 有下标越界panic
### 函数
* 命名规范 见名知意 小写字母、下划线分隔单词 
```rust
fn main() {
    let res = print_value(1);
    println!("{res}")
}
// 方法签名中必须声明参数类型
// 返回值类型 可省略 表示返回 unit ()
fn print_value(x: i32) -> i32 {
    // 函数体 由多个语句组成并以表达式结尾
    // 函数的定义也是语句
    // rust是基于表达式的语言
    // 语句 是执行指令 没有返回值
    // 表达式 有返回值  函数调用 宏调用都是表达式
    // 表达式可以复制给变量，语句不能
    println!("The value is:{}", x);
    // 默认返回最后一个表达式，如需提前返回可用 return
    1
}

```
### 流程控制
* if  
```rust fn main() {
    let x = 50;
    // 判断条件必须是bool
    if x < 50 {
        println!("too small")
    } else if x == 50 {
        println!("ok")
    } else {
        println!("too large")
    }
    // if 是个表达式 可以赋值给变量
    let res = if x == 50 { "win" } else { "failed" };
    println!("res:{res}")
}
```
* loop、while、for
```rs
// loop
fn main() {
    let res = loop {
        // 如果没有退出语句会一直执行
        // 可以通过ctrl+c终止
        println!("again!");
        // 退出循环 可以带有返回值 默认是unit () break带返回参数需要和loop一起使用
        break 1;
        // 跳过剩余代码，直接开始下个迭代
        // continue;
    };
    // loop 也是一个表达式，可通过break返回执行结果
    println!("{res}");
    // 使用标签label区分内外层loop
    'loop1: loop {
        println!("loop");
        loop {
            break 'loop1;
        }
    }
}

```
```rs
// while
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
}

```
```rs
fn main() {
    let number = [1, 2, 3];
    for num in number {
        println!("{num}")
    }
}

```
## 理解所属权概念
所属权是rust特有的概念，它使内存安全得到保证及时没有内存回收
所属权rust程序管理内存的规则集合，rust通过编译器校验，如果违反规则，则不能编译。运行时并不会影响运行速度。
* 理解堆和栈的区别，所属权的目的是管理堆上的数据
* rust中每一个value都有一个所属者
* 同一时间只有一个所属者
* 当所属者超过范围scope，该value将会被删除

```rust
// 所属权介绍
fn main() {
    // string存储在堆上
    // ""括起来的是字符串常量，不可变的，内容是编译时确定的,编译到可执行程序 硬编码
    // 类型 &str
    let mut number: &str = "hello";
    println!("{number}");
    number = "world";
    println!("{number}");

    // string类型，分配在堆上，编译时不知道其内容
    // 一旦运行超过变量范围，rust会替我们调用drop函数，string的拥有者归还内存，内存自动归还内存分配器
    let mut val = String::from("hello");
    val.push('!');
    println!("{val}");

    // string在堆上分配，有三个属性，指向堆空间的指针、len、capacity
    // 当重新赋值是，栈上有val、val1，对应堆上同一块内存，此时val将不在有效，val转移到val1
    let val1 = val;
    println!("{val1}");

    // clone 实现深拷贝
    let mut val2 = val1.clone();
    val2.push('!');
    println!("{val2}");

    // 编译时确定size的类型如 i32 变量分配在栈上, copy成本低，没有浅拷贝很深拷贝的区分，实现了copy接口，赋值时，所属权不再是move，而是copy
    let x = 5;
    let y = x;
    println!("x={x},y={y}")

    // copy 和 clone 接口的区别
    // Copy 如果一个类型 impl 了 Copy trait，意味着任何时候，我们可以通过简单的内存拷贝(C语言的按位拷贝memcpy)实现该类型的复制，而不会产生任何问题。一旦一个类型实现了 Copy trait，那么它在变量绑定、函数参数传递、函数返回值传递等场景下，它都是 copy 语义，而不再是默认的 move 语义。
    // 并不是所有的类型都可以实现Copy trait。Rust规定，对于自定义类型，只有所有的成员都实现了 Copy trait，这个类型才有资格实现 Copy trait
    // 常见的数字类型、bool类型、共享借用指针&，都是具有 Copy 属性的类型。而 Box、Vec、可写借用指针&mut 等类型都是不具备 Copy 属性的类型。
    // 对于数组类型，如果它内部的元素类型是Copy，那么这个数组也是Copy类型。
    // 对于tuple类型，如果它的每一个元素都是Copy类型，那么这个tuple会自动实现Copy trait。
    // 对于struct和enum类型，不会自动实现Copy trait。而且只有当struct和enum内部每个元素都是Copy类型的时候，编译器才允许我们针对此类型实现Copy trait。
    // 我们可以认为，Rust中只有 POD(C++语言中的Plain Old Data) 类型才有资格实现Copy trait。在Rust中，如果一个类型只包含POD数据类型的成员，没有指针类型的成员，并且没有自定义析构函数（实现Drop trait），那它就是POD类型。比如整数、浮点数、只包含POD类型的数组等，都属于POD类型。而Box、 String、 Vec等，不能按 bit 位拷贝的类型，都不属于POD类型。但是，反过来讲，并不是所有的POD类型都应该实现Copy trait。
    // rust禁止实现了drop方法的类型，再实现copy接口

    // clone 方法一般用于“基于语义的复制”操作。所以，它做什么事情，跟具体类型的作用息息相关。比如对于 Box 类型，clone 就是执行的“深拷贝”，而对于 Rc 类型，clone 做的事情就是把引用计数值加1。
}
```
```rs
// 引用
fn main() {
    // 函数调用和返回值的规则 同 赋值一样
    let s = String::from("hello world");
    op_string(s);
    // println!("{s}"); s所属权转移 无法再使用

    let x = 5;
    print_i32(x); // i32通过copy，x变量仍能使用

    let mut st = String::from("你好");
    println!("{st}");
    rc_string(&mut st);
    println!("{st}");
    let r1 = &st;
    let r2 = &st;
    println!("rs={r1},r2={r2}");

    // 可变类型的引用 不能同时借给两个及两个以上变量
    // 在编译期间防止并发修改的风险
    // let r2 = &mut st;
    // println!("rs={r1},r2={r2}");
    // 不会出现野指针，编译器会确保当有引用时，变量不失效

    // 函数内部不能返回函数内变量的引用
    // fn error_eg()->&String {
    //     let s = String::from("hello");
    //     return &s;
    // }
    // let err = error_eg();

    // 可以返回对象，从而实现所属权的转移
    let ok = ok_eg();
    fn ok_eg() -> String {
        let s = String::from("hello");
        return s;
    }
    println!("{ok}")
}

fn op_string(s: String) {
    println!("{s}");
}

fn print_i32(val: i32) {
    println!("{val}");
}

// 引用 有类型 没有所属权
fn rc_string(s: &mut String) {
    (*s).push('!'); // <==> s.push(ch);
    println!("{s}");
}

```
```rs
// slice
fn main() {
    // slice 切片 一种引用 没有所属权
    // 允许你引用一个集合中连续的部分，而不是整个集合

    let mut s = String::from("hello world!");
    println!("{}", first_word(&s));

    let hello = &s[0..5];
    let world = &s[6..11];
    // s.clear();   clean 会用到可变引用，而后面还在用不可变引用
    println!("{hello},{world}");
}
// 返回第一个单词的长度
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

```
```rs
fn main() {
    // &str是string slices
    // s类型是&str，是一个切片指向二进制的特殊位置
    let s = "hello world";
    // s类型是&str，是一个切片指向二进制的特殊位置
    println!("{}", test_s(&s[..]));
    println!("{}", test_s(s))
}
fn test_s(s: &str) -> &str {
    &s[..]
}
```

## 结构体 structs
```rs
// 结构体的声明于使用
use std::fmt::{self, Display};

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Helen"),
        email: String::from("xx@qq.com"),
        sign_in_count: 1,
    };
    user1.t(); //方法调用
    println!("{user1}"); // 打印结构体 需要实现 Display接口

    let user2 = User {
        active: false,
        ..user1 // 其余字段自动赋值 必须放在最后
    };
    // = 赋值之后 不能再使用 user1，因为user的username为string类型，已转移到user2
    println!("{}", user2);

    // 赋值 时 重新制定username、email，其他字段时基础类型 实现了copy，不会发生所属权的转移
    let user3 = User {
        username: String::from("jj"),
        email: String::from("x"),
        ..user2
    };
    println!("{}", user3);
    println!("{}", user2);

    println!("{}", build_user(String::from("Qin")));

    // 可能会出现，结构体存储的引用同时被其他对象引用，lifttime会确保结构体引用的数据是有效的 同结构体对象一样
}
fn build_user(username: String) -> User {
    User {
        active: true,
        username, // 参数名和字段名完全一致时可这样简写
        email: String::from(""),
        sign_in_count: 1,
    }
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// 为结构体实现接口
impl fmt::Display for User {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "username:{}, active:{}", self.username, self.active)
    }
}
// 为结构体添加自定义方法
impl User {
    // 方法必须有一个&self参数
    fn t(&self) {
        println!("hello")
    }
}

```
```rust
use std::fmt::Display;

fn main() {
    let c = Color(1, 2, 3);
    println!("{c}")
}

// tuple struce 元组结构体
struct Color(i32, i32, i32);
impl Display for Color {
    // &self 可读引用
    // &mut self 可修改引用
    // self 发生所属权转移
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}
// 空的结构体
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
struct None;

```
```rs
// debug 接口使用
use std::fmt::Display;

fn main() {
    let c = Color(1, 2, 3);
    println!("{:?}", c);
    println!("{:#?}", c) // :? 告诉println!使用debug模式的输出
}

// tuple struce 元组结构体
// 实现 debug接口 或手动实现debug
#[derive(Debug)]
struct Color(i32, i32, i32);
// impl Display for Color {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{},{},{}", self.0, self.1, self.2)
//     }
// }
// 空的结构体
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
struct None;

```
```rs
// dbg
use std::fmt::Display;

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    // dbg!(rect1);  会发生所属权转移 所以用引用
    println!("{:?}", rect1) // print使用的就是引用
}

// output：
// [src/main.rs:6] 30 * scale = 60
// [src/main.rs:9] &rect1 = Rectangle {
//     width: 60,
//     height: 50,
// }
// Rectangle { width: 60, height: 50 }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

```
```rs
// 结构体关联函数
use std::fmt::Display;

fn main() {
    let square = Rectangle::square(10);
    println!("square width:{}, height:{}", square.width, square.height)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // 关联函数，不是方法，没有self参数，一般用于new等场景
    // self返回值，是Rectangle的别名
    // 需要使用 :: 语法调用
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

```

## 枚举与模式匹配
