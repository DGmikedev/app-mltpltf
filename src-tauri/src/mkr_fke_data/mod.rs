use rand::{ self, Rng };


pub fn get_json()->String{

    let mut jsn = 
        format!(" {{ \"nombre\":\"{}\", \"direccion\": \"{}\", \"telefono\": \"{}\", \"email\": \"{}\" }}",
                get_name(), get_addres(), get_movil(), get_email() );
    return jsn
 
}

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO BINARY
/// /// ```
/// ARGUMENTO
/// arg -  NUMERO DE CARACTERES REQUERIDO EN EL BINARY
/// ```
pub fn get_binary(len: usize)->String{

    let mut bin = String::from("");
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let alpha:[&str;15] = [ "a", "e", "i", "o", "u", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9" ];

            for i in 0..len { 
                bin.push_str(alpha[rng.random_range(0..alpha.len())]); 
            }
            return bin
}

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO BOOLEAN 
 pub fn get_boolean()->String{

    let mut rng: rand::prelude::ThreadRng = rand::rng();

    if rng.random_range(0..=1) > 0 
    { let ser = format!("true");  ser 

    }else {  let ser = format!("false");  ser  }
}
/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO YEAR, DATE, DATETIME, TIME
/// ```
/// OPCIONES
/// arg - 0 - DATETIME  YYYY-MM-DD HH:MM:SS
/// arg - 1 - TIME      HH:MM:SS
/// arg - 2 - YEAR      YYYY
/// arg - 3 - DATE      YYYY-MM-DD
/// ```
pub fn get_full_date(tipo: usize)->String{

    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");

    // let sep = separador;
    let year: Vec<&str> = vec![ "2020", "2021", "2022", "2023", "2024", "2025"];
    let mont: Vec<&str> = vec![ "01","02","03","04","05","06","07","08","09","10","11","12" ];
    let day:  Vec<&str> = vec![ "01","02","03","04","05","06","07","08","09","10","11","12",
                                "13","14","15","16","17","18","19","20","21","22","23",
                                "24","25","26","27","28","29","30" ];

    match tipo{
        0 => {  // DATETIME  YYYY-MM-DD HH:MM:SS
            cadena = format!("{} {}",
                get_full_date(3),      // YYYY-MM-DD
                get_full_date(1));     // HH:MM:SS
            return cadena
        },
        1 =>{ // time HH:MM:SS
            cadena = format!("{}:{}:{}", 
                day[rng.random_range(0..24)],   // Hours
                rng.random_range(10..60),       // Minutes
                rng.random_range(10..60)        // Seconds 
            );
            return cadena
        },
        2 => { // year
            let random = rng.random_range(0..year.len());
            cadena.push_str(year[random]);
            return cadena
        },
        3 => { // full date
            let mut random = rng.random_range(0..year.len());
            cadena.push_str(year[random]);
            cadena.push_str("-");
        
            random = rng.random_range(0..mont.len());
            cadena.push_str(mont[random]);
            cadena.push_str("-");
        
            if random == 1{ random = rng.random_range(0..25)
            }else{ random = rng.random_range(0..day.len()) }
            cadena.push_str(day[random]);
            cadena
        }
        _ => return "x".to_string()
    }
}   

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL VARCHAR, VAR, TEXT <br>
/// ```
/// OPCIONES
/// arg - num - Tamaño del string que genera  
/// ```
pub fn get_varchar_txt(num: usize)->String{

    get_token(num, false)

}

 /// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO NUMERIC O DECIMAL <br>
/// ```
/// OPCIONES
/// opc- num  cantidad de numeros de la cifra entera
/// opc- dec  cantidad de numeros de la cifra decimal
/// ```

pub fn get_num_dec(num: usize, dec: usize)->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut str = format!("");
    let mut str_dec = format!("");
    for i in 0 ..num{
        str.push_str( &rng.random_range(0 .. 9).to_string() )
    }
    for i in 0 ..dec{
        str_dec.push_str( &rng.random_range(0 .. 9).to_string() )
    }
    let resp = format!("{}.{}",str, str_dec);
    resp
}

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO FLOAT O DOUBLE <br>
/// DE RANGO DESDE -1.7^308 HASTA 1.7^308
/// ```
/// OPCIONES
/// opc- 0 "FLO">FLOAT    -3.4E+38   3.4E+38
/// opc- 1 "DOU">DOUBLE   -1.7E+308  1.7E+308
/// ```
/// 
pub fn get_double_or_float(flo_num: usize)->String{

    let mut rng: rand::prelude::ThreadRng = rand::rng();
    match flo_num{
        0 =>{
            let indx = rng.random_range(-3.4f32.powf(37.5)..3.4f32.powf(38.0));
            return indx.to_string()
        },
        1 =>{
            let indx  = rng.random_range(-1.7f64.powf(308.0)..1.7f64.powf(308.0));
            return indx.to_string()
        },
        _ => return format!("x")
    }
}

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO INT <br>
/// DE RANGO DESDE 0 HASTA 2^64 - 2 
/// ```
/// OPCIONES
/// opc- 1 "TIN">TINYINT    1 byte   -128         0  127         max 255
/// opc- 2 "SMA">SMALLINT   2 byte  -32768        0  32767       max 65535
/// opc- 3 "MED">MEDIUMINT  3 byte  -8388608      0  8388607     max 16777215
/// opc- 4 "INT">INT        4 byte  -2147483648   0  2147483647  max 4294967295
/// opc- 5 "BIG">BIGINT     8 byte  -2^63 	      0  2^63 - 1    max 2^64 - 1
/// ```
/// 
pub fn get_int_usize(tipo: usize)-> String{

    let mut rng: rand::prelude::ThreadRng = rand::rng();
    match tipo{
        1 => { let indx: i32 = rng.random_range(-128..127);  
               return indx.to_string();    },
        2 => { let indx: i32 = rng.random_range(-32768..32767); 
               return indx.to_string();    },
        3 => { let indx: i32 = rng.random_range(-8388608..8388607); 
               return indx.to_string();    },
        4 => { let indx: i64 = rng.random_range(-2147483648..2147483647); 
               return indx.to_string();    },
        5 => { let indx: i64 = rng.random_range(-2i64.pow(60)..2i64.pow(60) - 2); 
               return indx.to_string();    },
        _ => format!("x"),
    }
}


/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO TOKEN
/// ```
/// OPCIONES
/// arg - lon             usize - LONGITUD DEL TOKEN
/// arg - chars_especials bool  - HABILITA Y DE DESHABILITA EL USO DE CARCTERES ESPECIALES EN EL TOKEN
/// 
/// ```
pub fn get_token(lon: usize, chars_especials: bool )->String{

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

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO NOMBRE
pub fn get_name()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let names: Vec<&str> = vec![
            "Juan", "James", "María", "John", "Maria", "Emily", "Juan", "Emily","Daniel", 
            "Carlos", "Michael", "Ana", "Michael", "Ana", "Sarah", "David", "Ashley",
            "Luis", "David", "Laura", "Christopher","Pedro", "Christopher", "Sofía", "Tatiana",
            "Sofia", "Jessica", "Michael", "Jessica", "Matthew", "Laura", "Ashley", "Amanda",
            "Emily", "Sarah", "Miguel", "Joshua", "Carlos", "David", "Isabel", "Megan",
            "Akira", "Mei","Hiroshi","Yumi","Suki","Taro","Jiro","Ling","Kenji","Sakura",
            "Iván","Anastasia","Dmitri","Ekaterina","Alexéi","Natalia","Vladimir","Olga","Mijaíl"
    ];

    let random = rng.random_range(0..names.len());
    cadena.push_str(names[random]);
    cadena
}

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO APELLIDO
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
            "Cruz", "Hernandez", "Sánchez", "Thomas", "Mora", "Moore", "Davis", "White", "Vasiliev",
            "Fernández", "Martin", "Ramírez", "Moore", "Ramírez", "Lee", "Wilson", "Clark",
            "Wang","Kim","Li","Zhang","Tanaka","Yamamoto","Lee","Sato","Cho","Park","Morozov",
            "Ivánov","Petrova","Smirnov","Romanov","Kuznetsov","Fedorov","Mikhailov","Alexandrov"
    ];  
    let random = rng.random_range(0..mdl_lst_name.len());
    cadena.push_str(mdl_lst_name[random]);
    cadena
}

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO DIRECCIÓN
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


/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO MOVIL
pub fn get_movil()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let movil: Vec<&str> = vec![
        "5551234",     "2125551234",  "525512345678",   "15551234567",
        "3235678",     "3235552345",  "12135551212",    "15559876543",
        "8189012",     "3125553456",  "523398765432",   "15551122334",
        "3322334455",  "7135554567",  "12125551212",    "15554455667",
        "5512345678",  "6025555678",  "528123456789",   "15557788990",
        "7139087",     "3055556789",  "13125551212",    "15552233445",
        "2223344",     "4155557890",  "522221234567",   "15555566778",
        "8182345",     "2145558901",  "17135551212",    "15558899001",
        "6643456",     "2065559012",  "526649876543",   "15553344556",
        "2125678",     "5125550123",  "16025551212",    "15556677889"
    ];
    let random = rng.random_range(0..movil.len());
    cadena.push_str(movil[random]);
    cadena
}  

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UN VALOR DEL TIPO E-MAIL
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

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA UNA PALBRA AL AZAR
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


/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA EL NOMBRE DE UN PAIS AL AZAR
pub fn get_countries()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let countrie: Vec<&str> = vec!["Venezuela","Chile","Ecuador","España",
    "Colombia","Argentina","Italia","Alemania","Reino Unido","Grecia",
    "Países Bajos","México","Brasil","Portugal","Polonia","Suecia","Perú",
    "Canadá"];
    let random = rng.random_range(0..countrie.len());
    cadena.push_str(countrie[random]);
    cadena
}

/// RETORNA UN VALOR DEL TIPO STRING <br>
/// QUE REPRESENTA EL NOMBRE DE UN CONTINENTE AL AZAR
pub fn get_continents()->String{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut cadena:String = String::from("");
    let continent: Vec<&str> = vec![
        "asia", "europa", "america", "africa", "ociania"
    ];
    let random = rng.random_range(0..continent.len());
    cadena.push_str(continent[random]);
    cadena
}





