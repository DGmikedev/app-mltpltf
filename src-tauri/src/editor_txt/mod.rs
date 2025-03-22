// use std::os::windows::fs::OpenOptionsExt;
use std::path::{ PathBuf, Path };
use std::fs::create_dir_all;
use std::fs::{ File, OpenOptions };
use std::io::Error;
use std::io::{BufWriter, Write};


pub fn create_directory(path:String)->bool{

    let mut _path: PathBuf = PathBuf::from(path);
    let dir: Result<(), Error> = create_dir_all(&_path);
    match dir {
        Ok(v) => return true,
        _ =>  return false,
    }
}

pub fn create_file(path: String, name: String)->Result<File, Error>{

    let path_full: String = format!("{}{}",path, name);
    let doc: Result<File, Error> = File::create(path_full);
    match doc{
        Ok(v) => return Ok(v),
        _ => return doc,
    }
}

pub fn insert_txt_by_ln(document: String, line: String) -> std::io::Result<()> { 

    let path = Path::new(&document);
    let _file = OpenOptions::new()
    .create(false)
    .append(true)
    .open(path)?;
    let mut writer = BufWriter::new(_file);
    writeln!(writer, "{}", line)?;
    Ok(())
}
