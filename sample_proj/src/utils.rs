use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://root:Password1234=@localhost:3306/test";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
