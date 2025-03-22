const { invoke } = window.__TAURI__.core; // jalo el winodw
//const { invoke } = window.__TAURI__.tauri;

let array_datos = Array();
let cant_INI = document.querySelector("#cant_INI");
let cant_FIN = document.querySelector("#cant_FIN");
let select_data =  document.querySelector("#select_data");
let btn_add = document.querySelector("#btn_add");
let btn_insert = document.querySelector("#btn_insert");
let table_rows =  document.querySelector("#table_rows");
let bd_on = document.querySelector("#bd_on");
let base_de_datos = document.querySelector("#base_de_datos");
let tabla = document.querySelector("#tabla");
let num_rows = document.querySelector("#num_rows");

bd_on.checked = false;
cant_INI.disabled = true;
cant_FIN.disabled = true;   
let all_right =  false;

select_data.addEventListener('change',()=>{
  if( select_data.value == "VAR" || select_data.value == "VRB" ||
      select_data.value == "CHA" || select_data.value == "DEC" || 
      select_data.value == "BIN" || select_data.value == "NUM"  )
  {
        if( select_data.value == "DEC" || select_data.value == "NUM" ){
            cant_INI.disabled = false;
            cant_FIN.disabled = false;
        }else{ cant_INI.disabled = false }
  }else{
    cant_INI.disabled = true;
    cant_FIN.disabled = true;
    cant_INI.value = "0";
    cant_FIN.value = "0";
  }
});

bd_on.addEventListener('change', ()=>{
  if (bd_on.checked){

    if(base_de_datos.value.length == 0 || tabla.value.length == 0){
      window.alert("Inserte los datos de la base de datos!");
      bd_on.checked = false;
      return;
    }
    
    array_datos.push([ base_de_datos.value, tabla.value, "0"]);
    base_de_datos.disabled = true;
    tabla.disabled = true;
    all_right = true;

  }else{

    all_right = false;
    base_de_datos.disabled = false;
    tabla.disabled = false;

  }

});

btn_add.addEventListener('click', ()=>{
  if( select_data.value == "N/A" || name_col.value.length == 0){
    window.alert("Selecciona un tipo de dato o nombre de columna")
    return;
  } 
  array_datos.push([name_col.value, select_data.value, cant_INI.value,cant_FIN.value])

   // Obtén la opción seleccionada
  let tipo_nom = select_data.options[select_data.selectedIndex];
  let fila = document.createElement('tr');
  let td_nom = document.createElement('td');
  let td_tipe = document.createElement('td');
  let td_lon = document.createElement('td');

  td_nom.textContent=name_col.value;
  td_tipe.textContent=tipo_nom.text;
  td_lon.textContent = cant_INI.value + " - " +cant_FIN.value;

  fila.appendChild(td_nom);
  fila.appendChild(td_tipe);
  fila.appendChild(td_lon);

  table_rows.appendChild(fila);
  all_right = true;
  // console.log(array_datos);

});

btn_insert.addEventListener('click', ()=>{
  if(all_right ==  true){
    array_datos[0][2] = num_rows.value;
    console.log({datos_a_enviar: array_datos})
    // return 
    invoke("rx_data", {  datos:array_datos })
    .then((res)=>{  window.alert("Inserción completada!")  })
  }else{
    window.alert("Ajuste los datos de la base de datos y active el check correspondiete");
    return;
  }
  
});
