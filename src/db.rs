use crate::rbatis::executor::Executor;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use std::env;

#[crud_table(table_name:"pubkeys")]
#[derive(Clone, Debug)]
pub struct Pubkey {
    pub id: Option<u32>,
    pub openers: Option<String>,
    pub pubkey: Option<String>,
}

pub async fn init_db() -> Rbatis {
    let db_path = env::var("AIAS_OPENER_DB_PATH").unwrap_or("sqlite://aias.db".to_string());

    println!("{}", db_path);

    let rb = Rbatis::new();
    rb.link(&db_path).await.unwrap();

    rb.exec(
        "CREATE TABLE IF NOT EXISTS 
            pubkeys(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                openers TEXT,
                pubkey TEXT
            )",
        &vec![],
    )
    .await
    .expect("Error creating");

    rb
}
