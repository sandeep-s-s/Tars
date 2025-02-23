use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
extern crate dirs;

use dotenv::dotenv;
use log::warn;
use std::env;

use std::fs;
use std::path::Path;

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.tars/tars.db"
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let _env = env::var("TARS_ENV");

    match _env {
        Ok(_env) => {
            let database_url = &env::var("DATABASE_URL").unwrap();

            SqliteConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", &database_url))
        }
        Err(_) => {
            warn!("Application env not set");

            if !db_file_exists() {
                create_db_file();
            }

            let home_dir = dirs::home_dir().expect("Could not find home directory");
            let db_path = home_dir.join(".tars/tars.db"); 
            let database_url = db_path.display().to_string();

            SqliteConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", &database_url))
        }
    }
}
