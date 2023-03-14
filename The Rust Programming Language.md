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
```rust
// 定义枚举
fn main() {
    // 枚举使用
    let black = Color::BLACK;
    println!("black:{:?}", black);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    home.print()
}

// 枚举声明
#[derive(Debug)]
enum Color {
    BLACK,
    RED,
    BLUE,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
impl IpAddr {
    fn print(&self) {
        println!("{:?}", self)
    }
}

```
``` rust
// option
fn main() {
    let val: Option<String> = Some(String::from("hello world"));
    if val.is_some() {
        // unwrap内部参数是self，会转移/消耗所属权
        // println!("{}", val.unwrap())
    }
    // match 支持 字符常量、变量名、通配符等
    match val {
        None => {
            println!("none");
        }
        // string类型，在Some是会转移所属权
        Some(x) => {
            println!("{x}")
        }
    }

    let x = 9;
    let x1 = match x {
        1 => 1,
        5 => 100,
        _ => 0, // 其他默认分支 放在选项最后
    };
    println!("{x1}")
}

```
```rust
// if let
fn main() {
    let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("{max}");
    } else {
        println!("none")
    }
    // 等价于
    match config_max {
        Some(x) => println!("{x}"),
        _ => println!("none"),
    }
    // if let 等价于 if let match中的一个分支 =后面的是match的patten
}

```

## 包、模块
* packages cargo的功能，使你可以构建、测试和分享包
* crates 一系列模块的组合，可以生产lib和可执行程序
* modules 和 use 控制组织、作用域、路径隐私
* paths 命名 包、函数的命名
#### crate
- 编译器一次编译的最小数量的代码 可以包含模块、或其他已编译文件定义的模块
- crate 分为二进制、library两种类型
  - 二进制 可执行的 包含main入口   src/main.rs是二进制crate的根 其他crate可放置于src/bin目录下 每个文件是独立的可执行程序
  - library 没有main函数，不会编译成可执行文件，而是定义函数等被其他项目共享使用的   src/lib.rs是library crate的根 每个package最多只有一个library crate
- packages 有一个或多个crate组成。一个package包含一个Cargo.toml文件描述如何构建crates。
- module 模块内的代码 默认是私有的 为了公有访问 pub mod 代替mod 对于模块内的内容同样需要pub

```rust
// main.rs
// src/main.rs 和 src/lib.rs 作为crate的根
use crate::garden::vegetables::Asparagus; // 引用其他mod的内容
pub mod garden; // 声明模块查找当前目录下 graden.rs 或者  graden 目录下的 mod.rs
fn main() {
    let plnt = Asparagus {};
    println!("I'm growing {:?}!", plnt);
}

// garden/garden.rs
pub mod vegetables; // 声明子模块

// garden/vegetables/vegetables.rs
#[derive(Debug)]
pub struct Asparagus {}

```

#### paths
```rust
// main.rs
// 外部module的引用
use rust::front_of_house as foh; // as 别名
// 当前crate的引用，以crate开头
use crate::garden::vegetables::Asparagus;
pub mod garden;
fn main() {
    front_of_house::hosting::add_to_waitlist();
}

// lib.rs
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {
            // 通过suer使用父模块的内容
            super::hosting::add_to_waitlist();
        }
        fn serve_order() {}
        fn take_payment() {}
    }
    // 父模块不能使用子模块的私有方法
    // 子模块可以使用祖先（父模块、父模块的父模块、、、）模块的私有方法
    // pub修饰struct时，属性仍然是默认私有 需要pub修饰field
    // pub修饰enum时，枚举类型都是public
}
pub use crate::front_of_house::hosting;// 重新将hosting导入到当前包  其他文件可通过 rust::hosting::add_to_waitlist(); 使用


// use使用方式
// 1. use std::cmp::Ordering;
//    use std::io;
// 2. use std::{cmp::Ordering, io};
// 3. use std::io::{self, Write};
// 4. use std::collections::*;
```

## 集合
```rust
// vector 数组
// string 字符集合
// hashmap
fn main() {
    // 声明数组 : Vec<i32> 类型可省略，编译器可以根据上下文推测
    // vec是包含 指针、容量、长度 的元组  其中指针指向一块连续空间，当容量==长度，会触发扩容
    // 初始化是容量0
    let mut v: Vec<i32> = Vec::new();
    // push需要对数组操作，需要是可变变量 mut
    // 扩容机制 max(max(cap*2, required(oldlen+need)), 元素大小（所占用字节数）=1时取8、<1024取4，>1024取1)
    v.push(1);
    v.push(2);
    println!("v len:{}", v.len());

    // 使用宏vec!创建数组
    let v = vec![1, 2, 3];
    println!("v len:{}", v.len());
    // for in 后面是变量引用
    // 因为vector分配在堆上，每个元素对应地址 &i32
    for i in &v {
        println!("{}", i);
    }
    let second = v[2];
    println!("v[2]={}", second);

    let o2 = v.get(2);
    println!("v[2]={}", o2.unwrap());
    // v.push(6);  此处是不允许的，因为o2是immutable，push一个元素可能需要重新分配空间，将原数据copy到新空间，o2的内存可能会被回收

    match o2 {
        Some(x) => println!("v[2]={}", x),
        None => println!("v[2]=none"),
    }
    if let Some(x) = o2 {
        println!("v[2]={}", x);
    }
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        // *解引用
        *i += 5;
        // *i = *i + 5;
    }
    for i in &mut v {
        // v.pop(); for 循环期间 禁止插入或删除元素
        println!("{i}");
    }
}

```

```rs
// vector enum
fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(i64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(30),
        SpreadsheetCell::Text(String::from("300")),
    ];
    for item in &row {
        println!("{:?}", item);
    }
}

```

```rs
// string
fn main() {
    // String 是可以增长的、可变的、有所属权的、UTF-8编码的string类型
    // &str string slice类型
    // string类型，底层是一个bytes类型的vector
    let mut s = String::new();

    let s: &str = "hello world";

    let s = s.to_string();
    let mut s1 = String::from("hello ");
    let s2 = "world";
    s1.push_str(s2); // string slice不需要所属权
    println!("{s1}");
    println!("{s2}");

    let s1 = "hello";
    let s2 = "hello";
    // "hello" 字符串字面常量 分配在Read Only Memory区 所以s1、s2指向同一个区域
    // str类型包含两个属性 指针、长度 固定长度 不同于string 分配在堆上 有三个属性 指针、容量、长度
    // 不能直接操作str所以s1是引用类型
    assert_eq!(*s1, *s2);

    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = format!("{s1}{s2}");
    println!("{s3}");
    let s3 = s1 + &s2; // s1 被消耗 之后不能再使用 + 是调用fn add(self, s: &str) -> String {方法 另外编译器将&string转成&str &s2[..]
    println!("{s3}");

    let ch = "中国";
    println!("{}", &ch[0..3]); // 中国所占字节数为6，[0,3)表示中

    // chars
    for item in ch.chars() {
        print!("{item}")
        // 输出 中国
    }
    println!();
    for item in ch.bytes() {
        println!("{item}")
    }
}

```
```rust
// hash map
use std::collections::HashMap;

// hash map
fn main() {
    let mut h = HashMap::new();
    h.insert("1", 1);
    println!("key:1, v:{}", h.get("1").copied().unwrap());
    println!("key:1, v:{}", h.get("1").unwrap());
    // 遍历map
    for (k, v) in &h {
        println!("{k}:{v}")
    }

    let filed_name = String::from("key");
    let field_value = String::from("value");
    let mut m = HashMap::new();
    m.insert(filed_name, field_value);
    // println!("{filed_name}"); //此处编译错误，因为对于实现了copy接口的类型，如i32，会将值copy到map中，而对于string有所属的类型，会讲value转移到map内
    // 如果&filed_name将引用传入map，则原field_name不受影响
    m.insert(String::from("key"), String::from("key")); // 覆盖
    println!("{:?}", m);

    // entry 返回对应的entry，如果entry存在 返回value的可变引用，如果entry不存在，插入key及对应的value，并返回新value的可变引用
    println!(
        "{:?}",
        m.entry(String::from("key1"))
            .or_insert(String::from("value"))
    );
    println!("{:?}", m);

}

```

## 错误
* 可恢复错误 Result<T,E>
* 不可恢复错误 panic!宏，当程序遇到不可恢复错误是终止执行

#### panic!
默认情况下，当panic发生时，rust会回退栈调用，并清除方法数据，这个操作有很多工作量，因此，rust允许立马退出，不清除直接退出。可以通过Cargo.toml配置
```toml
<!-- release模式下生效 -->
[profile.release]
panic = 'abort'
<!-- 
cargo run --bin rust --release
[1]    16764 abort      cargo run --bin rust --release 
-->
```
```rust
use std::{
    fs::File,
    io::{self, ErrorKind},
};

fn main() {
    let greet_file = File::open("Cargo1.toml");
    match greet_file {
        Ok(file) => println!("{:?}", file),
        Err(e) => println!("{e}"), // No such file or directory (os error 2)
    }

    let greeting_file = File::open("Cargo.toml").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Cargo.toml").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
        // unwrap 如果result的value是ok，返回ok内部的value，如果 是 err，会调用panic
        // expect panic的提示
    });

    // ？
    if let error = is_exist() {
        println!("{:?}", error);
    }
}

fn is_exist() -> Result<File, io::Error> {
    // ? 如果结果是ok，正常执行，如果结果是err，函数直接返回err提前退出
    // ? 仅适用于result、option、其他实现了FromResidual的类型
    let greeting_file = File::open("Cargo.yaml")?;
    Ok(greeting_file)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

```

```rs
use std::error::Error;
use std::fs::File;

// Box<dyn Error> 是一个接口对象  =》 任何类型的error
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

```

## 范型、接口、声明周期
* 范型
    ```rs
    fn main() {
        let number_list = vec![2, 3, 55, 6, 6, 0];
        println!("{:?}", largest(&number_list));

        let p = Point { x: 5, y: 5 };
        println!("{:?}", p.x_i32());
    }

    // 范型示例 PartialOrd接口，可用于<>比较
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if largest < item {
                largest = item
            }
        }
        largest
    }

    // 结构体范型
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // 方法仅作用在i32类型的参数上
    impl Point<i32, i32> {
        fn x_i32(&self) -> &i32 {
            &self.x
        }
    }

    enum Result<T, E> {
        OK(T),
        Err(E),
    }
    // let integer = Some(5);
    // let float = Some(5.0);
    // 当rust编译以上代码时，会识别到两种Option<T>,i32和f64,编译器会扩展成两个具体类型的定义，用具体类型替换范型
    // 因此 范型不会影响程序的运行速度

    ```
* 接口 traits 定义共享行为
    ```rust
    use std::fmt::Debug;

    fn main() {
        let play = Dog {};
        play.play();

        // 为vec实现play接口
        let v: Vec<String> = vec![];
        play_action_2(&play);
    }
    // 将接口作为参数限定
    fn play_action(play: &impl Play) {
        play.play();
    }
    // 语法糖
    fn play_action_1<T: Play>(play: &T) {
        play.play();
    }
    fn play_action_2<T: Play + Debug>(play: &T) {
        play.play();
    }
    // 继承多接口示例
    fn play_action_3(play: &(impl Play + Debug)) {
        play.play();
    }
    // 更加清晰的一种方式
    fn play_action_4<T>(play: &T)
    where
        T: Play + Debug,
    {
        play.play();
    }
    pub trait Play {
        fn play(&self) {
            // 接口方法默认实现
            println!("balabala...")
        }
    }

    // 可以为外部类型实现本crate内的接口
    impl<T> Play for Vec<T> {}

    #[derive(Debug)]
    struct Dog {}
    // 接口实现
    // 不能为外部类型实现外部接口 比如 为 vec实现Display
    impl Play for Dog {
        fn play(&self) {
            println!("wang wang awng...")
        }
    }

    // 如果可以为外部类型实现外部接口 可能会导致两个crate同时为同一类型实现同一个接口，此时将会出现问题

    ```
    ```rust
    use std::fmt::{Debug, Display};

    fn main() {
        let x = return_debug1();
        println!("{:?}", x);

        let a = Animal { kind: Dog {} };
        a.info();
        println!("{}", a);
    }

    // 返回接口的实现类
    fn return_debug() -> impl Debug {
        vec![0]
    }

    #[derive(Debug)]
    struct Dog {}
    // impl ToString for Dog {
    //     fn to_string(&self) -> String {
    //         String::from("wang wang wang...")
    //     }
    // }
    // 实现了display接口的类型，会自动实现tostring
    impl Display for Dog {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            println!("{}", "wang wang wang...");
            // println!("{}", self.to_string());// 在display中调用to_string会报错，递归了
            Result::Ok(())
        }
    }
    #[derive(Debug)]
    struct Cat {}
    // impl trait仅能在返回类型是一种的情况下使用
    // 如果a、b同时实现接口Debug，return_debug可能返回a或者b，此时 impl Debug不能编译
    // fn return_debug1() -> impl Debug {
    //     if true {
    //         return Dog {};
    //     } else {
    //         return Cat {};
    //     }
    // }
    // dyn 关键字用于突出显示在关联的 Trait 上对方法的调用是动态调度的。 要以这种方式使用 trait，它必须是对象安全的。
    // 与泛型参数或 impl Trait 不同，编译器不知道要传递的具体类型。即，类型为 erased。 因此，dyn Trait 引用包含 two 指针。 一个指针指向该数据 (例如，结构体的实例)。 另一个指针指向方法名称为函数指针的 map (称为虚拟方法表或 vtable)。
    // 在运行时，当需要在 dyn Trait 上调用方法时，将查询 vtable 以获取函数指针，然后调用该函数指针。
    // dyn 可实现返回对象的动态化
    fn return_debug1() -> Box<dyn Debug> {
        if false {
            return Box::new(Dog {});
        } else {
            return Box::new(Cat {});
        }
    }

    #[derive(Debug)]
    struct Animal<T> {
        kind: T,
    }
    // 仅为实现类Display接口的范型类型实现接口
    // 指定条件下实现接口
    impl<T: Display> Animal<T> {
        fn info(&self) {
            println!("{}", self.kind);
        }
    }
    impl<T: ToString> Display for Animal<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            println!("{}", self.kind.to_string());
            Result::Ok(())
        }
    }

    ```
* lifetimes
    ```rust
    fn main() {
        let r;
        {
            let x = 5;
            // 编译不通过，x引用的生命周期短于r的生命周期
            // borrowed value does not live long enough
            // 之后x回收后，r的引用将无效，会导致问题，所以编译器不允许
            r = &x;
        }
        println!("r:{}", r);
    }

    ```
    * The Borrow Checker
      - 比较作用域决定borrow是否有效
      ```rs
      // compile error
        fn main() {
            let string1 = String::from("abcd");
            let string2 = "xyz";
            let result = longest(string1.as_str(), string2);
            println!("{result}")
        }

        // &str是string的切片引用
        // 该函数编译不通过，函数返回一个借用类型的值，但是函数并没有说明是从x或者y借用的
        fn longest(x: &str, y: &str) -> &str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

      ```
      - 生命周期参数必须以'开始
      ```rs
        <!-- &i32
        &'a i32
        &'a mut i32 -->

        fn main() {
            let string1 = String::from("abcd");
            let result;
            {
                let string2 = "xyz";
                result = longest(string1.as_str(), string2);
            }
            println!("{result}");

            // 静态生命周期
            // 该字符串存储在程序二进制中，全局有效
            let s: &'static str = "hello world";
        }

        // 表示返回值引用有效，只要传入的俩参数有效，并保证只要'a有效，参数x，y就有效，返回值也有效
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        // 结构体生命周期标注
        struct A<'a> {
            part: &'a str,
        }

        // 部分场景下，rust会自动识别lifetime，可省略生命周期标注 'a
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
        // 当没有显式生命周期标注是，编译器使用一下三种方式识别引用的lifetime，如果仍识别不了这编译报错，第一种适用输入参数，第二、第三适用于返回类型，规则适用于fn函数、impl方法
        // 1. 编译器分配一个生命周期参数给每一个引用类型的参数 ，foo<'a>(x: &'a i32); foo<'a,'b>(x: &'a i32, y:&'b i32);
        // 2. 如果参数只有一个，则该生命周期赋予给返回类型 foo<'a>(x:&'a i32)->&'a i32
        // 3. 如果有多个输入参数，其中一个是&self或&mut self，因为是方法，则self的生命周期被赋予给所有返回值

        use std::fmt::Display;
        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {ann}");
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

      ```

## 测试
* 使用cargo test运行测试用例，默认运行所有类型的测试用例，bin crate、lib crate；--bin bin_name运行指定可执行程序的测试用例；--lib运行lib crate的测试用例；
* cargo test默认并行运行测试用例，cargo test --help 或者 cargo test -- --help展示help；
* cargo test -- --test-threads=1 串行执行测试用例
* cargo test test_name 执行具体测试用例；
* cargo test -- --ignored 只执行有忽略注解的用例
* cargo test -- --include-ignored 执行所有用例 包括忽略的
* 单元测试 通常和代码放在一块，创建一个`tests`模块包含测试函数，并对mod添加cfg[test]; 私有函数也能测试
* 集成测试 创建tests目录和src同级，tests目录下每个文件都是一个独立的crate，不需要cfg[test];
```rust
#[cfg(test)]
mod tests {
    use crate::add_two;

    #[test] // 表示是一个测试方法
    fn add_work() {
        assert_eq!(3, add_two(1, 2))
    }
    #[test]
    #[should_panic(expected = "panic")] // panic 并且包含expected认为通过
    fn panic_work() {
        panic!("panic test");
    }
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    // 0 measured性能测试
}

```
```rust
// 使用Result返回测试结果
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_work() -> Result<(), String> {
        // 返回Result
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("error，expect 2+2=4"))
        }
    }
}
```
```rs
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "reason"] // 忽略执行
    fn it_work() -> Result<(), String> {
        // 返回Result
        if 2 + 2 == 4 {
            println!("tttttttttttttttt"); // 控制台的输出不会体现在测试结果中，可以通过--show-output控制打印
            Ok(())
        } else {
            Err(String::from("error，expect 2+2=4"))
        }
    }
}

```

## i/o项目实践
```rs
use std::{env, fs};

fn main() {
    // 使用方式
    // cargo run -- searchstring example-filename.txt
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    let contents = fs::read_to_string(file_path).expect("should have been able to read file");

    for item in contents.lines() {
        if item.contains(query) {
            println!("{}", item)
        }
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    // 第一个参数是可执行程序路径 args[0]="target/debug/minigrep"
    if args.len() != 3 {
        panic!("使用方式minigrep searchstring example_filename.txt");
    }
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}

```

## 函数式编程 迭代和闭包
函数式编程风格：通常包括使用函数作为参数或返回值，或者将函数赋值给变量

* 闭包：匿名函数，当前作用域变量的绑定
```rust
use std::default;

fn main() {
    let opt: Option<i32> = Option::None;

    // 普通的函数传递
    println!("{}", opt.unwrap_or_else(defult_value));

    // 闭包
    // || function_name
    // 其中||中间是闭包需要的参数
    // 定义闭包 并赋值给变量
    let default_closure = || -> i32 { 100 };
    println!("{}", opt.unwrap_or_else(default_closure));

    // 将函数赋值给变量
    let default_closure = defult_value;
    println!("{}", opt.unwrap_or_else(default_closure));

    // 闭包调用函数
    let default_closure = || -> i32 { defult_value() };
    println!("{}", opt.unwrap_or_else(default_closure));

    // 闭包类型推断
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // 此处编译不通过，调用类型不一致
    println!();
    // 闭包对变量的借用规则，不可变借用，可变借用，所属权转移
    // 不可变借用
    let list = vec![1, 2, 3];
    println!("Before defining closure:{:?}", list);
    let only_borrows = || println!("From closure:{:?}", list);
    println!("Befor calling closure:{:?}", list);
    only_borrows();
    println!("After calling closure:{:?}", list);
    println!();
    // 可变借用
    let mut list = vec![1, 2, 3];
    println!("Before defining closure:{:?}", list);
    let mut borrows_mutably = || list.push(4);
    // println!("Befor calling closure:{:?}", list); // 当borrows_mutably定义时，closure持有一个list的可变引用，直到borrows_mutably调用结束后，对list的借用结束，才能继续调用list
    borrows_mutably();
    println!("After calling closure:{:?}", list);
    println!();
    // 所属权转移
    let list = vec![1, 2, 3];
    println!("Before defining closure:{:?}", list);
    let borrows_move = move || println!("After calling closure:{:?}", list);
    // println!("Before calling closure:{:?}", list); // 通过move对绑定的变量进行了所属权转移，此处无法在使用list
    borrows_move();
    // println!("After calling closure:{:?}", list);
}

fn defult_value() -> i32 {
    100
}

```
* Fn接口 (如果不需要将上下文中的变量关联到闭包中，可用传递函数代替)
    * FnOnce 适用于闭包只被调用一次，所有的闭包至少实现一个FnOnce接口，unwrap_or_else
    * FnMut 适用于闭包没有将关联的变量转移到闭包外，但是可能会改变关联的变量，可以被多次调用, sort_by_key
    * Fn 适用于闭包，没有将关联变量移动到闭包外，并且不修改关联变量，可被调用多次，适用于并发调用
* iterators
    ```rust
    fn main() {
        // pub trait Iterator {
        //     type Item;

        //     fn next(&mut self) -> Option<Self::Item>;

        //     // methods with default implementations elided
        // }

        let v1 = vec![1, 2, 3];

        // 迭代
        let mut v1_iter = v1.iter();

        // for val in v1_iter {
        //     println!("Got: {val}");
        // }

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    ```
## cargo和crates.io
* profile
    * dev 默认 cargo build
    * release cargo build --release
    * 默认配置在Cargo.toml文件中。通过`[profile.*]`定制配置或覆盖默认配置
        ```toml
        [profile.dev]
        opt-level = 0
        ```
    
* 发布crate到crates.io
    * 添加文档注释 会生成 html文档 使用 `///` 注释 正常markdown语法
    * cargo doc 在target目录下生成html文档
    * cargo doc --open 在浏览器打开文档
    * cargo test --doc 会测试`///`中的代码块
    * `//!`添加文档 通常放在root文件，作为整个模块的注视
    * 配置crates.io账号
        * cargo login api_key 通常key保存在～/.cargo/credentials
    * 添加配置Cargo.toml
        * 唯一的包名
            ```toml
            [package]
            package_name
            version = "0.1.0"
            edition = "2023"
            description = "_"
            license = 'MIT OR Apache-2.0'
            ```
    * cargo publish  // 发布时永久的，version不能被覆盖，代码不能被删除
    * 废弃版本 cargo yank --vers 0.1.0 防止新的项目在引用，该版本不会出现在新的Cargo.lock中，原来的不受影响
    * cargo yank --vers 0.1.0 --undo // 撤销yank
* cargo工作空间 通过 cargo run -p package 指定运行哪个包
    * Cargo.toml 
        ```toml
        [workspace]
        members = [
            "adder",
            "add_one",
        ]
        ```
    * adder
        * Cargo.tomal
            ```tomal
            [dependencies]
            add_one = { path = "../add_one" }
            ```
        * src
            * main.rs
                ```rs
                use add_one;

                fn main() {
                    let num = 10;
                    println!(
                        "Hello, world! {num} plus one is {}!",
                        add_one::add_one(num)
                    );
                }
                ```
    * add_one
            * Cargo.tomal
        * src
            * lib.rs
                ```rs
                pub fn add_one(x: i32) -> i32 {
                    x + 1
                }
                ```
    * target
* cargo install package_name 安装二进制文件到`~/.cargo/bin`

## 指针

  指针，包含内存地址的变量，在rust中，引用是一种普通的指针，通过`&`表示，并从借用指向的数据

  smart pointers，是一个结构体，类似一个指针，但是还有其他额外的元数据及能力。rust提供了多种smart指针，其功能超过不通的引用，其中包含 引用计数指针（该类型的指针能够允许有多个owner，当没有owner时，清理数据）

  smart指针通常使用结构体实现，不同于传统的结构体，smart指针实现了deref和drop接口。其中deref结构，允许该类型的实例像普通引用一样操作。drop接口，允许当smart指针超过作用域时实现自定义操作。

* `Box<T>` 实现了Deref、Drop接口
    - 使用`Box<T>`指向堆上的数据
        使用场景
        * 大小在编译时不确定的类型、上下文需要确定具体大小
        * 大数据所属权转移，而不发生copy的情况
        * 当你想拥有一个数据，只在意是否实现了某一接口，而不关心具体类型
    - 自定义Box，实现Deref接口
        ```rust
        use std::ops::Deref;

        fn main() {
            let b = Box::new(5);
            // 当b被清理时，其指向的堆空间也会被清理
            println!("b={b}");
            assert_eq!(5, *b);

            // 自定义box
            let b = MyBox::new(5);
            // 当b被清理时，其指向的堆空间也会被清理
            println!("b={:?}", b);
            assert_eq!(5, *b);
            assert_eq!(5, *(b.deref()));
        }

        #[derive(Debug)]
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        // 没有deref接口的实现，编译器只能堆&引用进行解引用
        impl<T> Deref for MyBox<T> {
            // 定义一个关联类型为Deref接口
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        ```
    - 解引用协变，将一个实现了deref接口的引用转换成另一个引用类型。例如，将&String转换成&str
    - from &T to &U when T: Deref<Target=U>
    - from &mut T to &mut U when T: Deref<Target=U>
    - from &mut T to &U when T: Deref<Target=U>
    - 实现Drop接口
        ```rs
        fn main() {
            // 变量删除顺序 与 创建顺序相反
            let s1 = S {
                data: String::from("s1"),
            };
            let s2 = S {
                data: String::from("s2"),
            };
            // 不允许显示调用s1.drop()
            drop(s1);
        }
        struct S {
            data: String,
        }
        impl Drop for S {
            fn drop(&mut self) {
                println!("{}", self.data);
            }
        }

        ```
* Rc<T>  reference counting类型引用，能够拥有多个所属权ownership
    - 通常一个数据仅属于一个owner，但是有些时候，比如在图结构中，一个点同时被多个边引用，节点不应该被回收，除非所有边都不在引用该节点
    - 当引用为0时，数据被清理
    - 分配在堆上的数据，为多个owner，编译时不能确定谁最后完成对数据的使用
    - 在单线程中使用，多线程中需要特殊处理，见并发模块
    ```rs
    use crate::List::{Cons, Nil};
    use std::rc::Rc;
    fn main() {
        // 初始化1
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        // Rc::clone 不进行深拷贝，只是增加引用计数
        let b = Cons(3, Rc::clone(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("a:reference count:{}", Rc::strong_count(&a));
        }
        //
        println!("a:reference count:{}", Rc::strong_count(&a));
        println!("a:reference count:{}", Rc::weak_count(&a));
    }
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    ```
    - RefCell和内部可变规则，内部可变规则允许你修改数据，即使引用是不可变的。为了修改数据，内部使用了unsafe代码。unsafe代码指示我们会手动check规则，而不是依赖编译器
    - Box<T>借用规则是在编译时，而RefCell<T>是在运行时，编译时如果违反借用规则，将编译错误，而运行时违反规则会panic并退出，同Rc<T>类似，RefCell仅适用于单线程场景。
    - Box允许可变和不可变借用通过编译时检查，Rc也是在编译时检查是不可变引用，RefCell是运行时检查，可以是可变也可以是不可变
    - 内部可变规则，可变的借用 从 不可变的值，
        ```rs
        use std::cell::RefCell;

        fn main() {
            let v = RefCell::new(S1 { value: 1 });
            let mut v1 = v.borrow_mut();
            // let mut v2 = v.borrow_mut(); // 不运行多于一个引用 thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:6:17
            v1.value = 2;
            println!("{:?}", v1);
        }

        #[derive(Debug)]
        struct S1 {
            value: i32,
        }
        ```
* Ref<T>和RefMut<T> 通过RefCell<T>访问，强制使用运行时borrowing规则，而不是编译时规则
    - 由于引用存在循环引用，可能会导致内存泄漏。
    - 使用Weak<T>防止循环引用。弱引用不影响数据的清理，当对应的强引用为0时，弱引用关系被打破。Rc::downgrade会返回一个弱引用，每次调用downgrade弱引用计数会加1. 使用场景，树结构，父节点对子节点强引用，子节点对父节点弱引用。

## 并发

  - Rust标准库使用1:1的线程模型，一个rust线程对应一个操作系统线程，可通过其他包实现其他模型。
  - thread::spawn(closure) 创建线程
  - thread::sleep、thread::spawn(closure).join().unwrap()在当前线程等待spawn线程完成
    ```rs
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];
        // 使用move转移变量所属权
        let handle = thread::spawn(move || {
            println!("{:?}", v);
        });
        handle.join().unwrap();
    }

    ```
  - 使用Message在线程中传递数据
    - mpsc::channel() mpsc表示多个生产者一个消费者。该函数返回一个元组(transmitter, receiver)。一旦将消息放入channel，将不在拥有消息，所属权被转移
        ```rs
        use std::{sync::mpsc, thread};

        fn main() {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let val = String::from("hi");
                tx.send(val).unwrap();
                // println!("{val}"); // error[E0382]: borrow of moved value: `val`
            });
            let received = rx.recv().unwrap();
            println!("{received}")
        }

        ```
        ```rs
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        fn main() {
            let (tx, rx) = mpsc::channel();
            // let tx1 = tx.clone(); 复制多个生产者
            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });
            // 接收方逐个等待接收
            // 迭代接收，当channel closed，迭代结束
            for received in rx {
                println!("Got: {received}");
            }
        }

        ```
    - 共享状态 
        - mutex 互斥锁 运行同一时间仅一个线程访问数据。
            ```rs
            use std::sync::Mutex;

            fn main() {
                let mux = Mutex::new(5);
                {
                    // lock 阻塞方法 不一定会获取到，所以通过unwrap如果失败会panic
                    let mut num = mux.lock().unwrap();
                    *num = 6;
                    // lock返回的是smart引用 所以失去作用域，锁将会被释放
                }
                println!("{:?}", mux)
            }

            ```
        - 多线程 使用`Arc<T>`
            ```rs
            use std::{
                rc::Rc,
                sync::{Arc, Mutex},
                thread,
            };

            fn main() {
                // Rc并不是线程安全的引用，多线程需要使用 atomic reference counting Arc<T>
                let counter = Arc::new(Mutex::new(0));
                let mut handles = vec![];

                for _ in 0..10 {
                    let counter = Arc::clone(&counter);
                    let handle = thread::spawn(move || {
                        let mut num = counter.lock().unwrap();
                        *num += 1;
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
                println!("Result:{}", counter.lock().unwrap());
            }

            ```
        - 通过Send、Sync接口扩展并发能力
            - Send接口可以是变量的所属权在多线程之间转移。几乎所有的类型都实现了send接口，除了Rc<T>。只包含send类型的类型会自动实现send接口
            - Sync允许多线程访问 任何类型T是Sync的，如果&T(不可变引用)是send的，意味着引用可以被安全的send到其他线程，基本类型都是sync的，如果只有sync的类型组成一个新类型，新类型也是sync的，Rc<T>不是sync

## 面向对象

- 面向对象语言的特点

    - 对象包含数据和行为
    - 封装，隐藏实现细节
    - 继承

## 模式与匹配

- 支持类型，字符字面量、数组、枚举、结构体、元组、变量、通配符、占位符
    ```rust
    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        // _ 匹配任何内容，会忽略value，通常作为最后一个分支
    }
    ```
    ```rs
    fn main() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{top}");
        }

        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{value} is at index {index}");
        }
    }
    ```
    ```rs
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    fn main() {
        let point = (3, 5);
        print_coordinates(&point);
    }

    ```
    ```rust
    fn main() {
        let x = 1;
        // matching literals
        match x {
            1 => println!("one"),
            _ => println!("other"),
        }

        let x = Some(5);
        let y = 10;
        // matching named variables
        match x {
            Some(50) => println!("got 50"),
            // 新作用域的y 代表匹配的值
            Some(y) => println!("matched, y={}", y),
            _ => println!("default case:{:?}", x),
        }
        println!("at the end: x = {:?}, y = {y}", x);

        let x = 1;
        match x {
            // 多匹配
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }

        // ..=语法
        let x = 5;
        match x {
            // 1,2,3,4,5都会匹配搭配1..5分支
            // 1｜2｜3｜4｜5
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }

        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        // 结构体分解
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        match p {
            Point { x, y: 0 } => println!("on the x axis at {x}"),
            Point { x: 0, y } => println!("on the y axis at {y}"),
            Point { x, y } => {
                println!("on neither axis:({x},{y})")
            }
        }
        match p {
            // 使用..忽略剩余的
            Point { x, .. } => println!("x:{}", x),
        }
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x dir {x}, in the y dir {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            // 嵌套分解，..忽略连续内容
            Message::ChangeColor(Color::Hsv(r, .., b)) => {
                println!("Change color to red {r}, green _, and blue {b}")
            }
            _ => (),
        }
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }

    ```
    ```rs
    fn main() {
        // match guard 使用额外的if语句
        let num = Some(4);
        match num {
            Some(x) if x % 2 == 0 => println!("the number is even"),
            Some(x) => println!("the number {x} is odd"),
            None => (),
        }
    }

    ```
    ```rs
    fn main() {
        // @ Bindings 创建一个变量，同时测试@后的条件是否匹配
        enum Message {
            Hello { id: i32 },
        }
        let msg = Message::Hello { id: 11 };
        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {id_variable}"),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Some other id: {id}"),
        }
    }

    ```
## 高级功能

  - Unsafe 
    - 解引用一个原始的指针
        ```rust
        fn main() {
            // smart引用和原始引用的区别
            // 原始引用
            // 不保证指向的是有效内存
            // 允许为null
            // 没有实现任何自动清理接口
            // 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
            let mut num = 5;
            // 使用as将不可变和可变引用转换成相应的原始指针类型。
            // 默认编译器不允许 可变引用和不可变引用同时指向一个区域
            let r1 = &num as *const i32;
            let r2 = &mut num as *mut i32;

            // 不能够解引用原始指针
            // let address = 0x012345usize;
            // let r = address as *const i32;
            // println!("{:?}", *r);

            unsafe {
                let address = r2;
                let r = address as *const i32;
                println!("{:?}", *r);
            }
        }

        ```
    - 调用unsafe函数或方法
        ```rust
        fn main() {
            unsafe {
                // 调用unsafe方法需要unsafe声明
                danngerous();
            }
        }

        unsafe fn danngerous() {}

        ```
        ```rust
        use std::slice;

        fn main() {
            let mut v = vec![1, 2, 3, 4, 5, 6];

            let r = &mut v[..];

            let (a, b) = r.split_at_mut(3);

            assert_eq!(a, &mut [1, 2, 3]);
            assert_eq!(b, &mut [4, 5, 6]);
        }

        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();

            assert!(mid <= len);
            // values借用两次 编译失败
            // (&mut values[..mid], &mut values[mid..])

            let ptr = values.as_mut_ptr();

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
        ```
        ```rs
        let address = 0x01234usize;
        let r = address as *mut i32;
        // 非法访问
        let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
        println!("{:?}", values);
        ```
        ```rust
        fn main() {
            unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }
        }
        // 引入外部函数
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        // 其他语言调用rust函数
        #[no_mangle] // 告诉编译器不要mangle方法名
        pub extern "C" fn call_from_c() {
            println!("just called a Rust function from C");
        }

        ```
    - 访问或修改可变静态变量
        ```rust
        static HELLO_WORLD: &str = "Hello World";
        static mut COUNTER: u32 = 0;

        fn main() {
            println!("value is :{}", HELLO_WORLD);
            // 常量和不可变静态变量
            // 静态变量在内存中有固定的地址  可以说可变类型 使用unsafe修改
            // constants 使用时复制

            add_to_count(3);
            unsafe {
                println!("counter: {COUNTER}");
            }
        }
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        ```
    - 实现unsafe接口 使用unsafe实现一个unsafe接口，一个unsafe接口至少有个一个方法包含编译器无法严重的不变式；例如：一个类型中既包含实现send、sync的类型，同时包含未实现send、sync的类型，编译器验证时，可以通过unsafe标记手动检查
    - 访问unnios属性
        - union和结构体struct相似，但是使用时 只声明一个属性。参考C语言的union
  - 高级接口 关联类型、默认类型参数、全限定语法、子接口、newtype模式
    ```rust
    fn main() {
        let mut counter = Counter::new(0);
        println!("{:?}", counter.next().unwrap_or(-1));
        println!("{:?}", counter.next().unwrap_or(-1));
        println!("{:?}", counter.next().unwrap_or(-1));
        println!("{:?}", counter.next().unwrap_or(-1));
        println!("{:?}", counter.next().unwrap_or(-1));
        println!("{:?}", counter.next().unwrap_or(-1));
    }

    pub trait Iterator {
        // 接口类型占位符 在实现中必须指明具体类型，不需要关心具体实现 直到接口具体实现
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    struct Counter {
        count: i32,
    }
    impl Counter {
        fn new(count: i32) -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    ```
    ```rs
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    // Rhs默认类型
    // trait Add<Rhs=Self> {
    //     type Output;  // 关联类型

    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }
    // 通过Add接口实现+运算符的重载
    impl Add<Point> for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn main() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    ```
    ```rs
    fn main() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        // 默认优先调用该类型的实现
        person.fly();
    }
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    ```
    ```rs
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    fn main() {
        // 优先使用当前类型的实现
        println!("A baby dog is called a {}", Dog::baby_name());
        // 全限定名使用接口的方法实现
        // <Type as Trait>::function(receiver_if_method, next_arg, ...);
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    ```
    - 子接口
        ```rust
        use std::fmt;

        // 子接口限定  实现OutlinePrint接口 依赖display接口
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }
        struct Point {
            x: i32,
            y: i32,
        }

        impl OutlinePrint for Point {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        fn main() {}

        ```

    - newtype
        ```rs
        use std::fmt;

        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        fn main() {
            let w = Wrapper(vec![String::from("hello"), String::from("world")]);
            println!("w = {w}");
        }

        ```
        ```rs
        use core::panicking::panic;

        fn main() {
            // 别名
            type Kilometers = i32;
            let x: i32 = 5;
            let y: Kilometers = 5;
            println!("x + y = {}", x + y);

            //
        }

        // ! 从不返回，在不返回任何内容的时候的返回值
        fn bar() -> ! {
            panic!("expr")
        }

        ```
  - 高级函数和闭包 函数指针，返回值闭包
    ```rs
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // 不同于闭包closure，f是一个类型 而不是 接口
    // 函数指针实现了 Fn、FnMut、FnOnce，所以可以当做闭包使用
    // 函数指针 fn 类型 fn(i32) -> i32
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    fn tos(i: &i32) -> String {
        i.to_string()
    }
    fn main() {
        let answer = do_twice(add_one, 5);

        println!("The answer is: {answer}");

        let list_of_numbers = vec![1, 2, 3];
        // tos 用函数代替闭包
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(tos).collect();
        println!("{:?}", list_of_strings);

        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

        // 闭包是以接口的形式呈现，返回值不能是闭包，需要是你要发挥的接口
        // 闭包不是具体类型 不能作为返回值
    }
    enum Status {
        Value(u32),
        Stop,
    }

    // 不能编译 不知道返回内容的大小size
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    // 可以编译
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    ```
  - 宏 https://veykril.github.io/tlborm
    - 三种方式

## web server
