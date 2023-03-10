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
