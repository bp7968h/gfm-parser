use std::{env, process, fs, path::Path};

fn main() {
  let args:Option<String> = env::args().nth(1);

  match args {
    Some(arg) => {
      if let Ok(file_metadata) = fs::metadata(&arg) {
         assert!(file_metadata.is_file());

         if is_markdown_file(&arg) {
           match fs::read_to_string(&arg) {
             Ok(content) => {
               println!("Content\n: {}", content);
             },
             Err(_) => {
               eprintln!("Error: failed to read file content");
               process::exit(1);
             }
           } 
         } else {
           eprintln!("Error: markdown file expected");
           process::exit(1);
         }
      } else {
        eprintln!("Error: file '{}' not found", arg);
        process::exit(1);
      }
    },
    None => {
      eprintln!("Usage: gfm-parser <some.md>");
      process::exit(1);
    }
  }
}

fn is_markdown_file(file_path: &str) -> bool {
  match Path::new(file_path).extension().and_then(|ext| ext.to_str()) {
    Some(ext) => matches!(ext.to_lowercase().as_str(), "md" | "markdown" | "mdown"),
    None => false
  }
}
