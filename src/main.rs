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
