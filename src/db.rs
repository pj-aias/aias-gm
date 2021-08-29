use crate::rbatis::executor::Executor;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use std::env;

#[crud_table(table_name:"publickeys")]
#[derive(Clone, Debug)]
pub struct PublicKey {
    pub id: Option<u8>,
    pub domains: Option<String>,
    pub pubkey: Option<String>,
    pub gm_id: Option<u8>,
}

#[crud_table(table_name:"members")]
#[derive(Clone, Debug)]
pub struct Member {
    pub id: Option<u8>,
    pub cert: Option<String>,
    pub domains: Option<String>,
    pub usk: Option<String>,
}

pub async fn init_db() -> Rbatis {
    let db_path = env::var("AIAS_OPENER_DB_PATH").unwrap_or("sqlite://aias.db".to_string());

    println!("{}", db_path);

    let rb = Rbatis::new();
    rb.link(&db_path).await.unwrap();

    rb.exec(
        "CREATE TABLE IF NOT EXISTS 
            publickeys(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                domains TEXT,
                pubkey TEXT,
                gm_id INTEGER
            )",
        &vec![],
    )
    .await
    .expect("Error creating table");

    rb.exec(
        "CREATE TABLE IF NOT EXISTS 
            members(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                domains TEXT,
                cert TEXT,
                usk TEXT
            )",
        &vec![],
    )
    .await
    .expect("Error creating");

    rb
}
