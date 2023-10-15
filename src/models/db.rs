use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DBConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub db_name: String,
}
