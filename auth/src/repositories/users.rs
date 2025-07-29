use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

static GLOBAL_DB: Lazy<Mutex<Vec<User>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn add_record(record: User) -> Result<(), String> {
    let mut db = GLOBAL_DB.lock().unwrap();
    db.push(record);
    // Need to handle error case if the record cannot be added

    Ok(())
}

pub fn show_db() -> Vec<User> {
    let db = GLOBAL_DB.lock().unwrap();
    println!("DB actuelle : {:?}", *db);
    db.clone()
}

pub fn get_user_by_email(email: &str) -> Option<User> {
    let db = GLOBAL_DB.lock().unwrap();
    let found = db.iter().find(|user| user.email == email);
    found.cloned()
}
