mod db;
mod models;

use chrono::prelude::*;
use db::init::init_db;
use models::db::DBConfig;
use mysql::prelude::*;
use mysql::*;
use std::fs::File;
use std::io::Read;
use std::string::String;
use toml;

fn main() {
    let file_path = "config.yaml";
    let mut file = File::open(file_path).unwrap();
    let mut config_string = String::new();
    file.read_to_string(&mut config_string).unwrap();
    let db_config: DBConfig = serde_yaml::from_str(&config_string).unwrap();
    println!("{:?}", db_config);
    let mut conn = init_db(
        &db_config.user,
        &db_config.password,
        &db_config.host,
        &db_config.db_name,
    );
    conn.query_iter("select * from likes")
        .unwrap()
        .for_each(|row| {
            let r: (i32, i32, i32) = from_row(row.unwrap());
            println!("{:?}, {:?}, {:?}", r.0, r.1, r.2)
        });
}
