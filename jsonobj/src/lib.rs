use std::fs;
use serde_json;

pub struct JsonObj {
    pub data: serde_json::Value, 
}

impl JsonObj {
    pub fn open(path: &str) -> Self {
        let file = fs::File::open(path).unwrap();
        Self {
            data: serde_json::from_reader(file).unwrap(),
        }
    }

    pub fn write(&self, path: &str) {
        let file = fs::File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &self.data).unwrap();
    }

    pub fn disp(&self) {
        println!("{:?}", self.data);
    }
 
    /*
    pub fn set_str(&mut self, key: &str, value: &str) {
        let value = value.to_string();
        if self.is_exists(key) == true {
            println!("### true");
            *self.data.pointer_mut(key).unwrap() = serde_json::json!(value);
            self.disp();
        }
    }
    */
  
    /*
     * このままでは不十分なので何とかしたいが…。
     * 配列とかboolとか使えた方が良いだろうし～…。
     */

    /*
     * 既存のキーがあった場合にも特に問題が起きないので
     * こちらに統一しておいた方がシンプルに済みそう。
     */
    pub fn set_str(&mut self, key: &str, value: &str) {
        let value = value.to_string();
        let obj = self.data.as_object_mut().unwrap();
        obj.insert(key.to_string(), serde_json::json!(value));
    }

    pub fn set_i32(&mut self, key: &str, value: i32) {
        let obj = self.data.as_object_mut().unwrap();
        obj.insert(key.to_string(), serde_json::json!(value));
    }

    pub fn get_str(&self, key: &str) -> String {
        let buf = self.data.pointer(key).unwrap();
        buf.as_str().unwrap().to_string()
    }

    pub fn get_i32(&self, key: &str) -> i32 {
        let buf = self.data.pointer(key).unwrap();
        buf.as_i64().unwrap() as i32
    }

    pub fn is_exists(&self, key: &str) -> bool {
        let buf = self.data.pointer(key);
        if buf.is_some() {
            true
        } else {
            false
        }
    }
}


/*

    let jso = JsonObj::open("test.json");
    jso.disp();
    println!("name {:?}", jso.is_exists("/name"));
    println!("city {:?}", jso.is_exists("/city"));
    println!("{}", jso.get_str("/data/hello"));
    println!("{}", jso.get_i32("/age"));

    jso.set_str("name", "Jesie");
    jso.set_str("gender", "n/a");
    jso.set_i32("resporn", 5);
    jso.write("test2.json");
*/



