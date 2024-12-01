use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
extern crate dirs;

use dotenv::dotenv;
use std::env;

use std::fs;
use std::path::Path;

// Check if a database file exists, and create one if it does not.
// pub fn init() {
// }

// Create the database file.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.arya/arya.db"
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let _env = env::var("ARYA_ENV");

    match _env {
        Ok(_env) => {
            let database_url = &env::var("DATABASE_URL").unwrap();

            SqliteConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", &database_url))
        }
        Err(_) => {
            println!("no ARYA_ENV");

            if !db_file_exists() {
                create_db_file();
            }

            let home_dir = dirs::home_dir().expect("Could not find home directory");
            let db_path = home_dir.join(".arya/arya.db"); // Specify your database file name
            let database_url = db_path.display().to_string();
            // format!("sqlite:{}", db_path.display());
            // let database_url = dirs::home_dir();

            // database_url.join(".arya/arya.db");

            // let database_url = database_url.to_str().clone().unwrap();

            SqliteConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", &database_url))
        }
    }
}
