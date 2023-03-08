/*
use glob::glob;
//use std::path::Path;

fn print_typename<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
*/

//use binaryfile::{BinaryWriter, BinaryReader};
//use sjis;
//use pathobj::PathObj;
//use globobj::globobj;
//use jsonobj::JsonObj;

use database::{open, Database};
use serialobj::SerialComm;

//use std::process;

fn main() {

    let port = serialobj::get_port();

    let mut sp = SerialComm::new(&port, 9600).unwrap();
    sp.set_logpath("log.txt");
    sp.wait_for("login:");
    sp.write("admin\n");
    sp.wait_for("Password:");
    sp.write("admin\n");

    let ret = sp.wait_for("(WA2021>|Login incorrect)");
    
    if ret == "Login incorrect" {
        println!("INCORRECT !!!!!");
    } else {
        sp.write("terminal length 0\n");
        sp.wait_for("WA2021>");
        sp.write("show run\n");
        sp.wait_for("WA2021>");
    }
    sp.set_logpath("");

    /*
    let mut con = open("_DATA/test.db").unwrap();
    let db = Database::fetch(&con, "select * from test_table");
    for row in db {
        println!("> {}({})", row["name"], row["age"]);
    }
    
    //let trans = transaction(&mut con).unwrap(); 
    let trans = con.transaction().unwrap();
    for count in 0..100 {
        let sql = format!("insert into test_table (name, age) values ('Bob-{}', {})", count, count);
        trans.execute(&sql, []).unwrap();
    }
    trans.commit().unwrap();
    */

    /*
    let trans = con.transaction().unwrap(); 
    let sql = "create table if not exists test (x integer, y integer)";
    trans.execute(&sql, []).unwrap();
    trans.commit().unwrap();
    */

    /*
    let mut jso = JsonObj::open("_DATA/test.json");
    jso.disp();
    println!("name {:?}", jso.is_exists("/name"));
    println!("city {:?}", jso.is_exists("/city"));
    println!("{}", jso.get_str("/data/hello"));
    println!("{}", jso.get_i32("/age"));
   
    jso.set_str("name", "Jesie");
    jso.set_str("gender", "n/a");
    jso.set_i32("resporn", 5);
    jso.write("test2.json");


    for target in globobj("./**/Cargo.toml") {
        let mut path = PathObj::new();
        path.push(&target);
        path.push("aaa");
        path.join(vec!["abc", "def", "ghi.png"]);
        println!("{}", path.parent());
        println!("{}", path.file_name());
        println!("{}", path.extension());
        println!("{}", PathObj::getcwd());
    }
    */
    //let bytes = include_bytes!("../../_DATA/test2.json");
    //println!("{:?}", String::from_utf8_lossy(bytes));

    /*
    //let s = sjis::encode("日本語");
    let s = "日本語".to_string();
    let s = s.into_bytes();
    let s3 = String::from_utf8(s);
    match s3 {
        Ok(_o) => println!("{:?}", _o),
        Err(_e) => println!("errorrrr!!!  {}", _e),
    }
    */

    /*
    let s = "日本語".to_string();
    let s = s.into_bytes();
    println!("1>> {:?}", sjis::is_sjis(&s));

    let s = sjis::encode("日本語");
    println!("2>> {:?}", sjis::is_sjis(&s));


    let br = BinaryReader::open("_DATA/test.txt").unwrap();
    for v in br {
        println!("{}", sjis::decode(v.unwrap()) );
    }

    let dat = sjis::encode("ハロー\nRust hello!!!");
    //let dat: Vec<u8> = vec![0x64, 0x65, 0x66];

    let mut bw = BinaryWriter::new("_DATA/hello.txt").unwrap();
    bw.write(&dat).unwrap();
    //bw.write(&s2).unwrap();
    */
}
