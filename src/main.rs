use std::{fs, io};
use walkdir::WalkDir;
fn main() -> io::Result<()> {
    println!("开源地址：https://github.com/Clover-You/clean-maven-faild-product");
    println!("请输入 maven 仓库路径, 例如: C:/user/.m2/repository");

    let mut target_path = String::new();

    io::stdin()
        .read_line(&mut target_path)
        .expect("field to read line");
    target_path = String::from(target_path.trim());
    fs::read_dir(&target_path).expect("目录不存在");

    let dirs = WalkDir::new(target_path).into_iter().filter_map(Result::ok);
    let mut faild_pack_path: Vec<String> = Vec::new();
    let mut prepath_length: usize = 0;

    for dir in dirs {
        let mut file_name = String::from(dir.file_name().to_string_lossy());
        let raw_file_name = file_name.clone();

        if file_name.len() < prepath_length {
            for _i in 0..(prepath_length - file_name.len()) {
                file_name.push_str(".")
            }
        }

        print!("{}\r", file_name);
        prepath_length = file_name.len();

        if raw_file_name.ends_with(".lastUpdated") {
            let parent = dir.path().parent().unwrap();
            let parent: String = String::from(parent.to_string_lossy());
            faild_pack_path.push(parent)
        }
    }

    for folder in &faild_pack_path {
        fs::remove_dir_all(folder)?;
    }

    println!();
    println!("successful count {} !", faild_pack_path.len());

    Ok(())
}
