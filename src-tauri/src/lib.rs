// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mysql_cli;
mod editor_txt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn media(values: Vec<&str>)->f32{
    let vector_valores: Result<Vec<f32>, _> = values.iter().map(|&s| s.parse()).collect();
    let mut sum: f32 = 0.0;

    match vector_valores{
        Ok(vec) => { 
            for i in vec.iter(){
               sum += i 
            }
        },
        Err(e) => println!("{}",e),
    }

    sum = sum / values.len() as f32;
    
    sum
}

#[tauri::command]
fn editar_txt(){

    
   let text: String =  String::from("ESTE");
   let name: &str   =  "prueba_2.txt";
   let path: &str   =  "tmpo/texto";

   let salidatxt: Result<(), std::io::Error> = 
   editor_txt::edit_txt( name, path, text );
   // println!("{:?}", salidatxt);
}


fn create_header_script(schema:Vec<String>)->String{

    let array_cols_name = vec!["user_name", "address", "phone"];
    let bd = "bd_prueba";
    let table = "usuarios";
    let mut acm: usize = 0;

    let mut head_script = format!("INSERT INTO {}.{} ( ", bd, table); 

    for i in array_cols_name.iter(){
        acm += 1;
        head_script.push_str(i);
        if acm == array_cols_name.len(){ head_script.push_str("") }
        else{ head_script.push_str(",") }
    }

    head_script.push_str(" ) VALUES ");

    println!("{}", head_script);

    head_script

}

#[tauri::command]
fn rx_data(data: Vec<Vec<String>>){

    let mut cols_names: Vec<&str> = Vec::new();


       
    let mut script: String = create_header_script(data[0].clone());



    println!("{:?}", data);
} 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![rx_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
