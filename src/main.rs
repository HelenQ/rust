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
