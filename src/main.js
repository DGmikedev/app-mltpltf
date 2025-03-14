const { invoke } = window.__TAURI__.core; // jalo el winodw
//const { invoke } = window.__TAURI__.tauri;

let array_datos = Array();
let cant_INI = document.querySelector("#cant_INI");
let cant_FIN = document.querySelector("#cant_FIN");
let select_data =  document.querySelector("#select_data");
let btn_add = document.querySelector("#btn_add");

cant_INI.disabled = true;
cant_FIN.disabled = true;

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

btn_add.addEventListener('click', ()=>{
  if( select_data.value == "N/A" ){
    window.alert("Selecciona un tipo de dato")
    return;
  } 

  array_datos.push([name_col.value, select_data.value, cant_INI.value,cant_FIN.value])

  invoke("rx_data", {
    data:array_datos
  }).then((res)=>{
    console.log(res)
  })
  console.log( array_datos)
  
});




/*

*/

/*

value="NAM">NAME 
value="NMF">NAME 
value="APP">MIDDLE 
value="ADD">ADDRESS 
value="MAI">EMAIL 
value="PHO">PHONE 
value="CON">COUNTRY 
value="CNT">CONTINENT 
value="SLF">SELL 
value="SLI">SELL 

value="TIN">TINYINT
value="SMA">SMALLINT
value="INT">INT
value="BIG">BIGINT
value="FLO">FLOAT
value="DOU">DOUBLE
value="DEC">DECIMAL
value="NUM">NUMERIC
value="CHA">CHAR
value="VAR">VARCHAR
value="TEX">TEXT
value="DAT">DATE
value="TIM">TIME
value="DAT">DATETIME
value="TMS">TIMESTAMP
value="YEA">YEAR
value="BIN">BINARY
value="VAR">VARBINARY
value="BOO">BOOLEAN
value="JSO">JSON
value="ENU">ENUM
value="SET">SET
*/


/*
let numero = [
  [ "TINYINT",  "1 byte" ],
  [ "SMALLINT", "2 bytes" ],
  [ "INT",      "4 bytes" ],
  [ "BIGINT",   "8 bytes" ]
];
   
let floats = [
  [ "FLOAT",   "4 bytes"],
  [ "DOUBLE",  "8 bytes"],
  [ "DECIMAL", "(p, s) variable"],
  [ "NUMERIC", "(p, s) variable"],
];

let _string = [
  ["CHAR(n)", 	 "n bytes"],
  ["VARCHAR(n)", "n bytes"],
  ["TEXT" , 	   "variable"],
];

let fech_hrs = [
  ["DATE",	"3 bytes"],	                   // '2025-03-13'
  ["TIME",	"3 bytes"],	                   // '14:30:00'
  ["DATETIME",	"8 bytes"],	               // '2025-03-13 14:30:00'
  ["TIMESTAMP",	"8 bytes"],	               // '2025-03-13 14:30:00'
  ["YEAR",	"1 byte"],	                   //  2025
];

let binarios = [
  ["BINARY(n)", "n bytes"],
  ["VARBINARY(n)", "n bytes"],
];

let booleanos = ["BOOLEAN", "1 byte"];

let variable = [
  ["JSON", "variable"],
  ["ENUM", "variable"],
  ["SET", "variable"],
];
console.log(numero);

*/






/*

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
*/