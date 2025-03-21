use std::env::join_paths;
use std::{env, fs};
use std::fs::File;
use std::io::Read;
use serde_json::Value;

fn main() {
    read_json(String::from("303.json"));
}

fn read_json(fileName: String) -> std::io::Result<()>{
    let cur_dir = env::current_dir()?;
    println!("{}", cur_dir.display());
    let file_path =cur_dir.join("xjtj_report").join(fileName);
    println!("{}", file_path.display());
    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let v: Value = serde_json::from_str(&contents)?;

    println!("{}", v);
    Ok(())
}
