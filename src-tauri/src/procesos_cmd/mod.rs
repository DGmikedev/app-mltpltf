use std::process::Command;
use std::process::exit;

/// EJECUTA PROCESOS DE CONSOLA Y DEVUELVE UN BOOLEAN CON 
/// EL ESTADO DE EJECUCIÓN 
/// ```
/// ARGUMENTOS
/// SE ENVIAN EN UN Vec<&str>
/// DONDE: [PROGRAMA, ARGUMNETO1, ARGUMNETO2, ARGUMNETO3 .....]
/// 
/// ejemplo:
/// 
/// vec!["mysql", "-u", "root", "-e", &path2exec];
/// 
/// donde: &path2exec => "C:/Users/NOM_PC/Desktop/DIRECTORIO1/PRJT1/rust_txt/script_to_exec.sql  -ruta donde se encuentra un script a ejecutar"
/// 
/// ```
/// 
pub fn ejecutar_proc(_args: Vec<&str>)->bool{

    let commando = Command::new(_args[0])
        .args([_args[1], _args[2], _args[3], _args[4]])
        .output()
        .expect("Falló al ejecutar el script SQL");

        // success?
        if commando.status.success() {
            return true
        } else {
            exit(1); // Salir con código de error 1
        }

}