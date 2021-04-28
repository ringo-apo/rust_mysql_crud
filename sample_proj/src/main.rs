use std::io;
use crate::utils::establish_connection;
use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use chrono::{NaiveDateTime};
use diesel::sql_query;
use diesel::prelude::*;

mod utils;

type DB = diesel::mysql::Mysql;

fn main() {
    println!("menu sample");

    loop {
        println!("menu 1:Select, 2:Insert, 3:Delete, 4:Update 5:end");

        let mut menus = String::new();
        io::stdin().read_line(&mut menus).expect("failed to read line");

        let menus: u32 = menus.trim().parse().expect("Input number");

        match menus {
            1 => {
                     println!("You selected Select");
                     select();
                 },

            2 => {
                     println!("You selected Insert");
                     insert();
                 }

            3 => {
                     println!("You selected Delete");
                     delete();
                 },

            4 => {
                     println!("You selected Update");
                     update();
                 },


            5 => {
                    println!("You selected end");
                    break;
                 },
            _   => println!("You bat input"),
        }
    }
}

#[derive(Debug)]
pub struct Memos {
        id: i32,
        name: String,
        comment: String,
        time: NaiveDateTime,

}

impl QueryableByName<DB> for Memos {
        fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(row: &R,) -> diesel::deserialize::Result<Self> {
            Ok(Memos {
                         id:      row.get("id")?,
                         name:    row.get("name")?,
                         comment: row.get("comment")?,
                         time:    row.get::<diesel::mysql::types::Datetime, _>("time")?,
                     }
              )
        }
}

fn select() {

        let connection: MysqlConnection = establish_connection();

        let memos: Vec<Memos> = sql_query("SELECT id, name, comment, time FROM memos",).load(&connection).unwrap();

        for uu in memos.iter(){
                println!("{}\t{}\t{}\t{}", uu.id, uu.name, uu.comment, uu.time);
        }
}

fn insert() {
    println!("Input name");
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("failed to read line");
    let name: String = name.trim().parse().expect("Error");

    println!("Input comment");
    let mut comment = String::new();

    io::stdin().read_line(&mut comment).expect("failed to read line");
    let comment: String = comment.trim().parse().expect("Error");

    let connection: MysqlConnection = establish_connection();

    let sql_str = ("insert into memos (name, comment, time) values ('").to_string() + &name.to_string() + "', '" + &comment.to_string() + "', NOW());";

    println!("{}", sql_str);
    connection.execute(&sql_str).unwrap();

}

fn delete (){
    println!("Input id");
    let mut id = String::new();

    io::stdin().read_line(&mut id).expect("failed to read line");
    let id: String = id.trim().parse().expect("Error");

    let connection: MysqlConnection = establish_connection();
    let sql_str = ("delete from memos where id=").to_string() + &id.to_string() + ";";

    println!("{}", sql_str);
    connection.execute(&sql_str).unwrap();

}

fn update (){
    println!("Input id");
    let mut id = String::new();

    io::stdin().read_line(&mut id).expect("failed to read line");
    let id: String = id.trim().parse().expect("Error");

    println!("Input comment");
    let mut comment = String::new();

    io::stdin().read_line(&mut comment).expect("failed to read line");
    let comment: String = comment.trim().parse().expect("Error");

    let connection: MysqlConnection = establish_connection();
    let sql_str = ("update memos set comment='").to_string() + &comment.to_string() + "' , time = NOW() where id=" + &id.to_string() + ";";

    println!("{}",sql_str);
    connection.execute(&sql_str).unwrap();
}

