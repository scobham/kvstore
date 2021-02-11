// Exercise from https://www.youtube.com/watch?v=WnWGO-tLtLA&t=5167s
// use std::fmt::write;
// use std::collections;

use std::{collections::HashMap, fmt::write};
fn main() {
    // println!("Hello, world!");
    let mut arguments = std::env::args().skip(1); //let args: Args
    let key = arguments.next().unwrap();
    // let value = args.next().unwrap();
    let value = arguments.next().expect("Key was not there!");
    println!("The Key is {} and the Value is {}", key, value);
    // let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("kv.db", contents).unwrap();
    // let write_result = std::fs::write("kv.db", contents);
    // match write_result {
    //     Ok(()) => {

    //     }
    //     Err(e){

    //     }
    // }

    let mut database = Database::new().expect("Creating db failed");
    // Database::insert(database, key, value);
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    // map: std::collections::HashMap<String, String>,
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
        // let contents = match std::fs::read_to_string("kv.db"){
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Result::Err(error);
        //     }
        // };
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines(){
            // let (key, value) = line.split_once('\t').expect("Corrupt database");
            let mut chunks = line.splitn(2,'\t');
            let key = chunks.next().expect("No Key!");
            let value = chunks.next().expect("No Value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        // parse the string
        // populate our map
        Ok(Database {
            // map: map,
            map,
            flush: false,
        })
    }

    fn insert(&mut self, key: String, value: String){
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush{
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    println!("Do Flush called!");
    let mut contents = String::new();
    // for pairs in &self.map {
    for (key, value) in &database.map {
        // let kvpair = format!("{}\t{}\n", pairs.0, pairs.1);
        // let kvpair = format!("{}\t{}\n", key, value);
        // contents.push_str(&kvpair);
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');

    }
    std::fs::write("kv.db", contents)
    // todo!("Finish this method")
}