pub fn create_header_insert_script(schema:Vec<String>, head_data: Vec<String>)->String{

    let array_cols_name = schema; 
    let bd: &String = &head_data[0];
    let table: &String = &head_data[1];
    let mut acm: usize = 0;

    let mut head_script = format!("INSERT INTO {}.{} ( ", bd, table); 

    for i in array_cols_name.iter(){
        acm += 1;
        head_script.push_str(i);
        if acm == array_cols_name.len(){ head_script.push_str("") }
        else{ head_script.push_str(",") }
    }

    head_script.push_str(" ) VALUES ");

    head_script

}