use std::process::Command;
use std::process::exit;




/*
pub fn mysql_cli_start()->String{

    
    let commando = Command::new("mysql")
            .arg("-u")
            .arg("root")
            .arg("-e")
            .arg("SELECT COUNT(*) as \"\" FROM pltfrm_laravel.users")
            .output()
            .expect("Falló al ejecutar el comando");

            if commando.status.success(){
                // println!("{:?}", &commando.stdout);
                let mensaje = String::from_utf8_lossy(&commando.stdout);

                return mensaje.to_string() //"salida ok".to_string();
                
            }else{
                 // Si ocurrió un error, mostramos el error
                 eprintln!("Error al ejecutar el comando MySQL:");
                let mensaje_fallo = String::from_utf8_lossy(&commando.stderr);
                return "salida XX".to_string();
                 // exit(1); // Salir con código de error 1
            }
           
}
*/




