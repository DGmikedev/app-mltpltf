use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::io::Error;

pub fn edit_txt(name: &str, path: &str, text:String)->io::Result<()>{

    let mut dir_path:PathBuf          = PathBuf::from(path);
    let setdir: Result<(), Error> = fs::create_dir_all(&dir_path);
    
    match setdir{
        Ok(ok) => dir_path.push(name),
        _ => println!("Error al general archivo [{:?}]", dir_path),
    }

    let doc = File::create(dir_path);
    let _ = doc?.write_all(text.as_bytes());

    Ok(())
} 

