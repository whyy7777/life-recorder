use std::fmt::format;
use mysql::*;

pub fn init_db(user: &str, password: &str, host: &str, db_name: &str) -> PooledConn {
    let url = format!("{}{}{}{}{}{}{}{}", "mysql://", user, ":", password, "@", host, ":3306/", db_name);
    let pool = Pool::new(url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();
    conn
}