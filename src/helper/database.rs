use chrono::NaiveDate;
use std::fs;
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::result::Result;
use serde_json::json;
use crate::helper::trace::trace_logs;
use std::sync::{Arc, Mutex};
use std::{ptr::addr_of, sync::mpsc, thread};
use tokio::time::{interval, Duration};

use once_cell::sync::Lazy;

// ------------ ALL STRUCTURE ------------

#[derive(Debug, FromRow, Clone)]
pub struct USERS {
    pub user_uuid: String,
    pub user_username: String,
    pub user_email: String,
    pub user_cookie: String,
}

impl USERS {
    pub fn default() -> USERS {
        USERS {
            user_uuid: "".to_string(),
            user_username: "".to_string(),
            user_email: "".to_string(),
            user_cookie: "".to_string(),
        }
    }

    pub async fn get_user_by_cookie(user_cookie:String) -> Vec<USERS> {
        
        // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
        let lock_result = unsafe { DB_CLIENT.lock() };
    
        if lock_result.is_err() {
            // kill script
            trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
            std::process::exit(1);
        }
    
        // check if need to create new client
        if lock_result.unwrap().is_none() {
            new_client().await;
        }
    
        // perform database operations
        let db_client = unsafe { DB_CLIENT.lock().unwrap() };
    
        let db_client = db_client.as_ref();

        let mut risks: Vec<USERS> = Vec::new();

        if let Some(pool) = db_client {
            let mut conn = pool.get_conn().unwrap();
            let query = format!("SELECT user_uuid, user_username, user_email, user_cookie FROM users WHERE user_cookie = '{}'", user_cookie);

            let result = conn.query_map(query, |(user_uuid, user_username, user_email, user_cookie): (String, String, String, String)| {
                USERS {
                    user_uuid,
                    user_username,
                    user_email,
                    user_cookie,
                }
            });

            // check how many rows are returned
            match result {
                Ok(fetched_risks) => {
                    for risk in fetched_risks {
                        risks.push(risk);
                    }
                },
                Err(_) => {
                    return risks;
                }
            }

            return risks;
        }

        println!("No database connection");
        return risks;
    }

    pub async fn create_user(user_email:String, user_hashed_password: String) {
        // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
        let lock_result = unsafe { DB_CLIENT.lock() };
    
        if lock_result.is_err() {
            // kill script
            trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
            std::process::exit(1);
        }
    
        // check if need to create new client
        if lock_result.unwrap().is_none() {
            new_client().await;
        }
    
        // perform database operations
        let db_client = unsafe { DB_CLIENT.lock().unwrap() };
    
        let db_client = db_client.as_ref();
    
        if let Some(pool) = db_client {
            let mut conn = pool.get_conn().unwrap();
    
            let user_uuid = Uuid::new_v4().to_string();
            let user_username = user_email.split("@").collect::<Vec<&str>>()[0].to_string();
            let user_cookie = "".to_string();
    
            let query = format!("INSERT INTO users (user_uuid, user_username, user_email, user_password, user_cookie) VALUES ('{}', '{}', '{}', '{}', '{}')", user_uuid, user_username, user_email, user_hashed_password, user_cookie);
    
            let result = conn.query_drop(query);
    
            match result {
                Ok(_) => {
                    return;
                },
                Err(_) => {
                    return;
                }
            }
        }
    
        println!("No database connection");
        return;
    }

    pub async fn login_user(user_email:String, user_hashed_password: String) -> bool {
        // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
        let lock_result = unsafe { DB_CLIENT.lock() };
    
        if lock_result.is_err() {
            // kill script
            trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
            std::process::exit(1);
        }
    
        // check if need to create new client
        if lock_result.unwrap().is_none() {
            new_client().await;
        }
    
        // perform database operations
        let db_client = unsafe { DB_CLIENT.lock().unwrap() };
    
        let db_client = db_client.as_ref();
    
        if let Some(pool) = db_client {
            let mut conn = pool.get_conn().unwrap();
    
            let query = format!("SELECT user_uuid, user_username, user_email, user_cookie FROM users WHERE user_email = '{}' AND user_password = '{}'", user_email, user_hashed_password);
    
            let result = conn.query_map(query, |(user_uuid, user_username, user_email, user_cookie): (String, String, String, String)| {
                USERS {
                    user_uuid,
                    user_username,
                    user_email,
                    user_cookie,
                }
            });
    
            // check how many rows are returned
            match result {
                Ok(fetched_risks) => {
                    if fetched_risks.len() > 0 {
                        return true;
                    }
                },
                Err(_) => {
                    return false;
                }
            }
    
            return false;
        }
    
        println!("No database connection");
        return false;
    }

    pub async fn generate_cookie(user_email:String) -> Uuid {
        // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
        let lock_result = unsafe { DB_CLIENT.lock() };
    
        if lock_result.is_err() {
            // kill script
            trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
            std::process::exit(1);
        }
    
        // check if need to create new client
        if lock_result.unwrap().is_none() {
            new_client().await;
        }
    
        // perform database operations
        let db_client = unsafe { DB_CLIENT.lock().unwrap() };
    
        let db_client = db_client.as_ref();
    
        if let Some(pool) = db_client {
            let mut conn = pool.get_conn().unwrap();
    
            let user_cookie = Uuid::new_v4();
    
            let query = format!("UPDATE users SET user_cookie = '{}' WHERE user_email = '{}'", user_cookie.to_string(), user_email);
    
            let result = conn.query_drop(query);
    
            match result {
                Ok(_) => {
                    return user_cookie;
                },
                Err(_) => {
                    return Uuid::nil();
                }
            }
        }
    
        println!("No database connection");
        return Uuid::nil();
    }
}


// ------------ DATABASE SYSTEM ------------

static mut DB_CLIENT: Lazy<Arc<Mutex<Option<mysql::Pool>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

async fn new_client() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            periodic_database().await;
            tx.send(()).unwrap(); // Signal that work is done
        });
    });

    reset_database().await
}

async fn periodic_database() {
    let mut interval = interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        reset_database().await;
    }
}

async fn reset_database() {

    let mut h = "127.0.0.1";

    let config = fs::read_to_string("config/default.json").unwrap();
    let config: serde_json::Value = serde_json::from_str(config.as_str()).unwrap();

    let port: u16 = config.get("db_port").unwrap().as_u64().unwrap() as u16;
    let host:String = config.get("db_host").unwrap().as_str().unwrap().to_owned();
    let db_username:String = config.get("db_username").unwrap().as_str().unwrap().to_owned();
    let db_password:String = config.get("db_password").unwrap().as_str().unwrap().to_owned();

    // check if process arg --prod is used
    if std::env::args().any(|arg| arg == "--prod") {
        h = host.as_str();
    }

    // Define MySQL connection options
    let opts = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(h))
        .tcp_port(port)
        .db_name(Some("phantom"))
        .user(Some(db_username))
        .pass(Some(db_password));

    // hcekc if DB_CLIENT.lock().unwrap().is_none() return any poison error
    if mysql::Pool::new(opts.clone()).is_err() {
        return ;
    }

    // Create a new MySQL connection pool
    let pool = mysql::Pool::new(opts).unwrap();

    unsafe {
        let mut db_client = DB_CLIENT.lock().unwrap();
        *db_client = Some(pool);
    }

}

pub async fn check_db_is_up() -> bool {

    reset_database().await;

    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    if db_client.is_none() {
        return false;
    }

    let db_client = db_client.as_ref().unwrap();

    let mut conn = db_client.get_conn().unwrap();

    let query = "SELECT 1";

    let result = conn.query_map(query, |_: i32| {
        ()
    });

    match result {
        Ok(_) => {
            return true;
        },
        Err(_) => {
            return false;
        }
    }
}

// ------------ DATABASE FUNCTIONS ------------

// ------------ DATABASE UTILS ------------
pub async fn check_if_table_exist(table_name:String) -> bool {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
        std::process::exit(1);
    }

    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();

        let query = format!("SELECT table_name FROM information_schema.tables WHERE table_name = '{}' AND table_schema = 'phantom' LIMIT 1", table_name);

        let result = conn.query_map(query, |table_name: String| {
            table_name
        });

        // check how many rows are returned
        match result {
            Ok(fetched_table) => {
                if fetched_table.len() > 0 {
                    return true;
                }
            },
            Err(_) => {
                return false;
            }
        }

        return false;
    }

    println!("No database connection");
    return false;
}

pub async fn create_table(table_name:String, column:Vec<serde_json::Value>) {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
        std::process::exit(1);
    }

    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();

        let mut query = format!("CREATE TABLE {} (", table_name);

        for (i, col) in column.iter().enumerate() {
            if i == column.len() - 1 {
                query.push_str(&format!("{} {})", col["name"], col["type"]));
            } else {
                query.push_str(&format!("{} {}, ", col["name"], col["type"]));
            }
        }

        query = query.replace("\"", "");

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}

pub async fn check_column_exist(table_name:String, column_name:String) -> bool {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
        std::process::exit(1);
    }

    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();

        let query = format!("SELECT column_name FROM information_schema.columns WHERE table_name = '{}' AND column_name = '{}'  AND database = 'phantom' LIMIT 1", table_name, column_name);

        let result = conn.query_map(query, |(column_name): (String)| {
            column_name
        });

        // check how many rows are returned
        match result {
            Ok(fetched_column) => {
                if fetched_column.len() > 0 {
                    return true;
                }
            },
            Err(_) => {
                return false;
            }
        }

        return false;
    }

    println!("No database connection");
    return false;
}

pub async fn add_column(table_name:String, column_name:String, column_type:String) {
    // check if DB_CLIENT.lock().unwrap().is_none() return any poison error
    let lock_result = unsafe { DB_CLIENT.lock() };

    if lock_result.is_err() {
        // kill script
        trace_logs("Error: DB_CLIENT.lock().unwrap() is_none() return any poison".to_owned());
        std::process::exit(1);
    }

    // check if need to create new client
    if lock_result.unwrap().is_none() {
        new_client().await;
    }

    // perform database operations
    let db_client = unsafe { DB_CLIENT.lock().unwrap() };

    let db_client = db_client.as_ref();

    if let Some(pool) = db_client {
        let mut conn = pool.get_conn().unwrap();

        let query = format!("ALTER TABLE {} ADD COLUMN {} {}", table_name, column_name, column_type);

        let result = conn.query_drop(query);

        match result {
            Ok(_) => {
                return;
            },
            Err(_) => {
                return;
            }
        }
    }

    println!("No database connection");
    return;
}








