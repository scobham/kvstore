// Exercise from https://www.youtube.com/watch?v=WnWGO-tLtLA&t=5167s
// use std::fmt::write;
// use std::collections;

use std::collections::HashMap;
fn main() {
    // println!("Hello, world!");
    let mut args = std::env::args().skip(1); //let args: Args
    let key = args.next().unwrap();
    // let value = args.next().unwrap();
    let value = args.next().expect("Key was not there!");
    println!("The Key is {} and the Value is {}", key, value);
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();
    // let write_result = std::fs::write("kv.db", contents);
    // match write_result {
    //     Ok(()) => {

    //     }
    //     Err(e){

    //     }
    // }

    let database = Database::new().expect("Creating db failed");
}

struct Database {
    // map: std::collections::HashMap<String, String>,
    map: HashMap<String, String>,
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
            map: map,
        })
    }
}
