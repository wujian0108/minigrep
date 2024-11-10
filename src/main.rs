use std::env;
use std::fs;
fn main() {
 // 获取当前工作目录
 let current_dir = env::current_dir().expect("无法获取当前目录");
 
 // 打印当前工作目录
 println!("当前工作目录是: {:?}", current_dir);
    let args=env::args().collect::<Vec<String>>();
    let query=&args[1];
    let file_path=&args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}