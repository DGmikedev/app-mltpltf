// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs::File;
use std::io::Error;
use std::env;

mod get_fker_data;
mod editor_txt;
mod mkr_fke_data;
mod make_script_sql;
mod procesos_cmd;

#[tauri::command]
fn rx_data(datos: Vec<Vec<String>>)->String{

    
// Se parsea el numero de rgistro deseados desde los datos de entrada  en la poscicion datos[0][2].  
    let num_de_registros = datos[0][2].
        parse::<usize>().
        expect("Ha ocurrido un error");

    let mut cols_names: Vec<String> = Vec::new();

    // Se recaban los nombres de las columnas para hacer el insert en el paso 3
        for i in 1..datos.len(){ cols_names.push(datos[i][0].clone()) }


     // SE RECIBEN LOS DATOS  Y SE CREAN LOS DIRECTORIOS Y EL ARCHIVO 

        let path: String =  format!("{}\\{}\\", datos[0][0], datos[0][1]); 
        let name: String =  format!("{}.sql", datos[0][1]);
        let full_name: String = format!("{path}{name}");

    // 1) =>  Se crea el directorio
        let path_directory: bool =  editor_txt::create_directory(path.clone());

        // RETURN SI FALLA LA CREACIÓN DEL DIRECTORIO
        if !&path_directory { println!("No se pudo crear el directorio indicado: {}", &path) }

        // Se clonan los paths de directorios y documento
        //  cambiar en servidor y revisar las diagonales para LINUX o WINDOWS

        let current_dir: std::path::PathBuf = env::current_dir().expect("No se pudo obtener el directorio actual");
        
        let localpath = current_dir.to_string_lossy().into_owned();

        let mut path2exec = format!("{}\\{}{}", localpath, path.clone(), name.clone());

    // 2) => se crea el archvio
        let document: Result<File, Error> = editor_txt::create_file(path.clone(), name);

        // RETURN SI FALLA LA CREACIÓN DEL ARCHIVO
        match &document{
            Ok(v) => println!("Archivo creado"),
            _ => println!("Error al crear el archivo indicado: {}", &full_name),
        }

    // 3) => Se Inicia el insert

    // Se obtiene el Header - nombre de tabla insert into (columns)
        let header_script = 
        make_script_sql::create_header_insert_script(cols_names, datos[0].clone());
    
    // Se  inserta en el documento
        let _ = 
        editor_txt::insert_txt_by_ln(full_name.clone(),header_script.clone());

    // 4) => Se generan los rows a insertar

        for i in 0..num_de_registros{

            let mut row = format!("(");

            for i in 1..datos.len(){
                row.push_str("\'");
                row.push_str(&get_fker_data::get_fake_data(&datos[i][1], &datos[i][2], &datos[i][3]));
                if i == datos.len() - 1 {
                    row.push_str("\'");
                }else{
                    row.push_str("\',");
                }
            }
            if i == num_de_registros - 1{ 
                row.push_str(");"); 
            }else{
                row.push_str("),");
            }
            let _ = editor_txt::insert_txt_by_ln(full_name.clone(),row.clone());
        }
    // 5) => Ya creado el script se manda ejecutar en CLI de MySQL

        path2exec = format!("source {}",path2exec);  // se adciona el la bandera "source" para indicar que es un script y su path
        
        let args = vec!["mysql", "-u", "root", "-e", &path2exec];

        if procesos_cmd::ejecutar_proc(args){
            println!("INSERCIÓN COMPLETADA");
        }else{
            panic!("NO SE PUDO COMPLETAR LA INSERCIÓN")
        }
        println!("{} - {}",full_name, &path2exec);

        return "Inserción efectuada con exito".to_string();

} 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![rx_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
