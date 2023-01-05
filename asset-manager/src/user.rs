
use std::fs;
use std::path::Path;

use serde::{Serialize, Deserialize};
use crate::app::App;
use crate::types::{UserName, HashString};
use crate::models::AppEnv;
// use crate::errors::AppErrors;
// use std::fs;
// use std::path::Path;

use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct User {
    name: UserName,
    env: AppEnv,
    password_hash: String,
    pub app_state_hash: Option<HashString>
}

impl User {
    pub fn new(name: &str, password: &str, env: AppEnv) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt.hash_password(password.as_bytes(), &salt).unwrap().to_string();

        User {
            name: name.to_string(),
            env,
            password_hash,
            app_state_hash: None
        }
    }

    // pub fn open(name: &str, password: &str, env: AppEnv) -> User {
    //     let salt = SaltString::generate(&mut OsRng);
    //     let password_hash = Scrypt.hash_password(password.as_bytes(), &salt).unwrap().to_string();

    //     let user_file_path = format!("./files/{}/user_{}.json", &env, name);
    //     if Path::new(&user_file_path).exists() {
    //         println!("USERS FILE EXIST!");
    //         let content = fs::read_to_string(&user_file_path).expect("Error reading bond file");
    //         // json::parse(&content).unwrap()
    //         let result: User = serde_json::from_str(&content).unwrap();
    //         assert!(result.is_valid_password(password), "Invalid password.");
    //         result
    //     } else {
    //         // This is the default for ./ENV/users.json
    //         println!("CREATING NEW USERS FILE!");
    //         User::new(name, password, env)
    //     }
    // }

    // fn is_valid_password(&self, password: &str) -> bool {
    //     let parsed_hash = PasswordHash::new(&self.password_hash).unwrap();
    //     Scrypt.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    // }

    // pub fn new_app_state(&mut self, app: App, password: &str) {
    //     println!("HERE 1");
    //     assert!(self.is_valid_password(password), "Invalid password.");
    //     println!("HERE 2");
    //     assert!(self.app_state_hash.is_none(), "User already have a running app.");
    //     println!("HERE 3");
    //     self.app_state_hash = Some(app.get_state_hash(password));
    // }

    // pub fn save_app_state(&mut self, app: App, password: &str) {
    //     assert!(self.is_valid_password(password), "Invalid password.");
    //     assert!(self.app_state_hash.is_some(), "Create a new app state first.");
    //     self.app_state_hash = Some(app.get_state_hash(password));
    // }

    // pub fn load_app_state(&self, password: &str) -> App {
    //     assert!(self.is_valid_password(password), "Invalid password.");
    //     assert!(self.app_state_hash.is_some(), "Create a new app state first.");

    //     let mut app = App::new(self.clone());
    //     app.load_owner_state(password);
    //     app
    // }

    // pub fn save(&self) {
    //     let user_file_path = format!("./files/{}/user_{}.json", &self.env, self.name);

    //     std::fs::write(
    //         user_file_path,
    //         serde_json::to_string_pretty(&self).unwrap(),
    //     )
    //     .unwrap();
    // }

    // pub fn is_equal(&self, other: User) -> bool {
    //     self.name == other.name
    //         && self.env == other.env
    //         && self.password_hash == other.password_hash
    //         && self.app_state_hash == other.app_state_hash
    // }

    // fn import_app_state_hash(&mut self) {
    //     assert!(self.app_state_hash.is_none());

    //     let users_file_path = format!("./files/{}/users.json", &self.env);
    //     let users = if Path::new(&users_file_path).exists() {
    //         println!("USERS FILE EXIST!");
    //         let content = fs::read_to_string(&users_file_path).expect("Error reading bond file");
    //         json::parse(&content).unwrap()
    //     } else {
    //         // This is the default for ./ENV/users.json
    //         println!("CREATING NEW USERS FILE!");
    //         json::object! {
    //             "users": []
    //         }
    //     };
    //     for user in users["users"].members() {
    //         if user["name"] == self.name && user["env"] == self.env.to_string() {
    //             self.app_state_hash = user["app_state_hash"];
    //         }           
    //     }
    // }
}
