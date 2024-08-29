use std::{fs, io};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
  // before start, clear the console
  print!("\x1B[2J\x1B[1;1H");

  println!("the project is open-sourced on GitHub: https://github.com/Clover-You/clean-maven-faild-product");
  println!("please enter the path to the Maven repository, e.g.: C:/user/.m2/repository");

  let mut target_path: String;

  loop {
    target_path = String::from("");

    io::stdin()
      .read_line(&mut target_path)
      .expect("field to read line");

    target_path = String::from(target_path.trim());

    match fs::read_dir(&target_path) {
      Err(err) => {
        println!("{}, please re-enter:", err);
      }
      Ok(_) => break,
    }
  }

  let dirs = WalkDir::new(target_path).into_iter().filter_map(Result::ok);
  let mut faild_pack_path: Vec<String> = Vec::new();

  for dir in dirs {
    let file_name = String::from(dir.file_name().to_string_lossy());

    clear_current_line();
    print_to_curr_console(file_name.as_str());

    if file_name.ends_with(".lastUpdated") {
      let parent = dir.path().parent().unwrap();
      let parent: String = String::from(parent.to_string_lossy());
      faild_pack_path.push(parent)
    }
  }

  // remove all faild pack of maven
  for folder in &faild_pack_path {
    fs::remove_dir_all(folder)?;
  }

  println!("\n\nsuccessful count {} !", faild_pack_path.len());

  Ok(())
}

fn print_to_curr_console(text: &str) {
  clear_current_line();
  print!("{}\r", text);
}

fn clear_current_line() {
  // ANSI escape code for clearing the current line
  print!("\x1B[2K");
  // Move the cursor back to the beginning of the line
  print!("\r");
}
