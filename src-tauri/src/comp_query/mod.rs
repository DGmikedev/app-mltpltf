use rand::{self, Rng};
use std::env;
use std::process::Command;
use std::process::exit;

pub fn get_token(lon: usize, chars_especials: bool )->String{

    // Generador de strings seguros!
    let mut clve: String = String::from("");
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    
    let characters: [&str;67] =[
    "A","B","C","D","E","F","G","H","I","J",
    "K","L","M","N","O","P","Q","R","S","T",
    "U","V","W","X","Y","Z","a","b","c","d",
    "e","f","g","h","i","j","k","l","m","n",
    "o","p","q","r","s","t","u","v","w","x",
    "y","z","0","1","2","3","4","5","6","7",
    "8","9","!","@","#","$","%"
    ];
    
    for _i in 0..= lon {

        if chars_especials{
            let indx = rng.random_range(0..=66);
        clve.push_str(characters[indx]);
        }else{
            let indx = rng.random_range(0..=52);
        clve.push_str(characters[indx]);
        }
    
    }
    
    clve
    
}


pub fn get_name()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let names: Vec<&str> = vec![
            "Juan", "James", "María", "John", "Maria", "Emily", "Juan", "Emily","Daniel", 
            "Carlos", "Michael", "Ana", "Michael", "Ana", "Sarah", "David", "Ashley",
            "Luis", "David", "Laura", "Christopher","Pedro", "Christopher", "Sofía", 
            "Sofia", "Jessica", "Michael", "Jessica", "Matthew", "Laura", "Ashley", "Amanda",
            "Emily", "Sarah", "Miguel", "Joshua", "Carlos", "David", "Isabel", "Megan",
    ];

    let random = rng.random_range(0..names.len());
    cadena.push_str(names[random]);
    cadena
}

pub fn get_mdl_lst_name()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let mdl_lst_name: Vec<&str> = vec![
            "Pérez", "Smith", "García", "Smith", "Rodríguez", "Johnson", "Hernández", "Thompson",
            "Williams", "Martínez", "Brown", "Martínez", "Jones", "González", "Davis","López",
            "Smith", "Miller", "Ramírez", "Martinez", "González", "Davis", "Martínez", "White",
            "Anderson", "Williams", "Rodriguez", "Jiménez", "Taylor", "Pérez", "Anderson", "Miller",
            "Hernández", "Thomas", "Jones", "Jackson", "García", "Jackson", "Sánchez","Lee",
            "Gómez", "Johnson", "López", "Johnson", "López", "Williams", "Martínez", "Brown",
            "Hernández", "Jones", "González", "Davis", "Sánchez", "Miller", "Johnson", "Wilson",
            "Torres", "Garcia", "Pérez", "Garcia", "Díaz", "Martinez", "Brown", "Martinez",
            "Cruz", "Hernandez", "Sánchez", "Thomas", "Mora", "Moore", "Davis", "White",
            "Fernández", "Martin", "Ramírez", "Moore", "Ramírez", "Lee", "Wilson", "Clark"
    ];  
    let random = rng.random_range(0..mdl_lst_name.len());
    cadena.push_str(mdl_lst_name[random]);
    cadena
}        

pub fn get_addres()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let address: Vec<&str> = vec![
            "Avenida Reforma 123, Ciudad de México, CDMX", "123 Main Street, New York, NY","789 Pine Avenue, Chicago, IL",
            "Calle Principal 123, Ciudad de México, México","123 Main St, Anytown, CA, USA","Av. Reforma 456, Guadalajara, México",
            "1234 Elm Street, Los Ángeles, CA", "456 Oak Road, Los Angeles, CA","123 Main St, Los Angeles, CA, USA",
            "456 Oak Ave, Springfield, IL, USA","Calle 5 de Febrero 456, Monterrey, Nuevo León", "789 Pine Ln, Riverside, TX, USA",
            "Carretera Panamericana 789, Guadalajara, Jalisco", "321 Maple Lane, Houston, TX","456 Oak Ave, New York, NY, USA",
            "101 Elm Rd, Greenville, NY, USA","Av. de los Insurgentes Sur 101, Ciudad de México, CDMX","123 Oak Avenue, Houston, TX",
            "654 Cedar Drive, Phoenix, AZ","Calle Hidalgo 789, Monterrey, México","202 Maple Dr, Sunnyvale, WA, USA",
            "987 Birch Street, Miami, FL","789 Pine Ln, Chicago, IL, USA", "Calle Hidalgo 789, Puebla, Puebla", "123 Elm Street, San Francisco, CA",
            "303 Birch St, Pleasantville, FL, USA", "Av. Juárez 101, Puebla, México", "404 Cedar Ave, Harmonyville, MI, USA",
           "Callejón del Sol 345, Monterrey, Nuevo León", "Boulevard Juárez 678, Tijuana, Baja California", "707 Sequoia Dr, Tranquil Town, OR, USA",
            "234 Pinecrest Road, Dallas, TX","101 Elm Rd, Houston, TX, USA","505 Willow Ln, Peaceful Pines, AZ, USA","890 Rosewood Avenue, Austin, TX","202 Maple Dr, Phoenix, AZ, USA",
            "567 Maple Lane, Seattle, WA","Calle Morelos 202, Tijuana, México","606 Redwood Rd, Serenity Springs, CO, USA","8th Avenue 234, New York, NY",
    ]; 
    let random = rng.random_range(0..address.len());
    cadena.push_str(address[random]);
    cadena
}        
    
pub fn get_movil()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let movil: Vec<&str> = vec![
        "555-1234",     "212-555-1234",  "525512345678",   "15551234567",
        "323-5678",     "323-555-2345",  "12135551212",    "15559876543",
        "818-9012",     "312-555-3456",  "523398765432",   "15551122334",
        "33-2233-4455", "713-555-4567",  "12125551212",    "15554455667",
        "55-1234-5678", "602-555-5678",  "528123456789",   "15557788990",
        "713-9087",     "305-555-6789",  "13125551212",    "15552233445",
        "222-3344",     "415-555-7890",  "522221234567",   "15555566778",
        "818-2345",     "214-555-8901",  "17135551212",    "15558899001",
        "664-3456",     "206-555-9012",  "526649876543",   "15553344556",
        "212-5678",     "512-555-0123",  "16025551212",    "15556677889",
    ];
    let random = rng.random_range(0..movil.len());
    cadena.push_str(movil[random]);
    cadena
}  

pub fn get_email()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let email: Vec<&str> = vec![
            "juan.perez@email.com", "maria.garcia@example.com", "james.smith@email.com", "john.smith@example.com",
            "maria.rodriguez@email.com", "juan.hernandez@example.com", "emily.johnson@email.com", "emily.williams@example.com",
            "carlos.martinez@email.com", "ana.martinez@example.com", "michael.brown@email.com", "michael.jones@example.com",
            "ana.gonzalez@email.com", "david.smith@example.com", "sarah.davis@email.com", "ashley.miller@example.com",
            "luis.ramirez@email.com", "laura.gonzalez@example.com", "david.martinez@email.com", "christopher.davis@example.com",
            "sofia.martinez@email.com", "michael.williams@example.com", "jessica.anderson@email.com", "jessica.rodriguez@example.com",
            "pedro.jimenez@email.com", "sofia.perez@example.com", "christopher.taylor@email.com", "matthew.anderson@example.com",
            "laura.hernandez@email.com", "emily.jones@example.com", "ashley.thomas@email.com", "sarah.jackson@example.com",
            "miguel.garcia@email.com", "carlos.sanchez@example.com", "joshua.jackson@email.com", "david.thompson@example.com",
            "isabel.lopez@email.com", "daniel.miller@example.com", "megan.white@email.com", "amanda.lee@example.com"
    ];
    let random = rng.random_range(0..email.len());
    cadena.push_str( &( get_token(5, false) + email[random] ) );
    cadena
}  

pub fn get_date(separador: String)->String{

    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let sep = separador;
    let year: Vec<&str> = vec![ "2020", "2021", "2022", "2023", "2024", "2025"];
    let mont: Vec<&str> = vec![ "01","02","03","04","05","06","07","08","09","10","11","12" ];
    let day:  Vec<&str> = vec![ "01","02","03","04","05","06","08","09","10","11","12",
                                "13","14","15","16","17","18","19","20","21","22","23",
                                "24","25","26","27","28","29","30" ];
    
    let mut random = rng.random_range(0..year.len());
    cadena.push_str(year[random]);
    cadena.push_str(&sep);

    random = rng.random_range(0..mont.len());
    cadena.push_str(mont[random]);
    cadena.push_str(&sep);

    if random == 1{
        random = rng.random_range(0..25);    
    }else{
        random = rng.random_range(0..day.len());
    }
    
    cadena.push_str(day[random]);

    cadena
}  

pub fn get_rand_words()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let word: Vec<&str> = vec!["sol", "estrella", "luna", "gato", "perro", "rojo", "flor", 
        "montaña", "playa", "viento", "piedra", "agua", "camino", "noche", "cielo", "amigo", 
        "familia", "feliz", "árbol", "río", "solución", "fuego", "café", "bici", "piedra", 
        "sueño", "música", "carro", "viaje", "paz", "salud", "luz", "libro", "puente", "ciudad", 
        "fútbol", "balón", "guitarra", "verde", "rojo", "amor", "nieve", "hielo", "flor", "viento", 
        "pasión", "sombra", "amigo", "brisa", "cielo", "mar", "risa"];
    let random = rng.random_range(0..word.len());
    cadena.push_str(word[random]);
    cadena
}

   
pub fn get_inserts(db: String, tabla:String, num_reg:usize){

    if num_reg < 200 {
        insertor(num_reg);
        println!("Insertando: {}", num_reg);
    }else{

        let mut exec_num =  num_reg as f32/200f32;

        exec_num = exec_num.floor();

        println!("Se insertarán {}", exec_num * 200f32);

        for i in 0 .. exec_num as usize{
            println!("Insertando: {} de {}", i * 200,  exec_num as f32 * 200f32);
            insertor(200);
        }

        println!("Insertando: {} de {}", exec_num as f32 * 200f32,  exec_num as f32 * 200f32);
    }

}