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
