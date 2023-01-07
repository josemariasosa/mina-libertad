// use std::str::FromStr;
// use serde::{Serialize, Deserialize};
// use std::fs;
// use std::collections::HashMap;
// use std::fmt;

mod asset;
mod models;
mod app;
mod internal;
mod transaction;
mod types;
mod user;
mod utils;
mod errors;

// use asset::Asset;

use crate::{user::User, models::AppEnv, app::App};

const ASSETS_FILEPATH: &str = "./files/dev/user.json";
const MARKET_FILEPATH: &str = "./files/dev/market.json";
const PRICE_SHEET_FILEPATH: &str = "./files/dev/naive_price_mxn.json";

// use aes_gcm::{
//     aead::{Aead, KeyInit, OsRng},
//     Aes256Gcm, Nonce // Or `Aes128Gcm`
// };

// // use json::{JsonValue, object};
// use scrypt::{
//     password_hash::{
//         rand_core::OsRng,
//         PasswordHash, PasswordHasher, PasswordVerifier, SaltString
//     },
//     Scrypt
// };

fn main() {
    println!("Asset Manager, welcome.");
    let user = User::new("TEST", "admin123", AppEnv::Dev);

    let mut app = App::new(user);
    app.import_user_file(ASSETS_FILEPATH);

    app.update_market(MARKET_FILEPATH);

    app.dashboard();

    // app.login(user);
    // assert!(user.is_valid_password("admin123"));
    // let app = App::new();
    // create_new_asset()
    // use aes_gcm::{
    //     aead::{Aead, KeyInit, OsRng},
    //     Aes256Gcm, Nonce // Or `Aes128Gcm`
    // };
    
    // let key = Aes256Gcm::generate_key(&mut OsRng);
    // let cipher = Aes256Gcm::new(&key);
    // let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    // let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref()).unwrap();
    // let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    // assert_eq!(&plaintext, b"plaintext message");

    // println!("{:?}", String::from_utf8(plaintext));

}

#[cfg(test)]
mod tests;