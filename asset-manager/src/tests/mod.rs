use crate::{user::User, models::AppEnv, app::App};
use crate::models::AssetType;

#[test]
fn create_and_load_state_hash() {
    let mut user = User::new("TESTUSER", "admin123", AppEnv::Dev);

    let mut app = App::new(user.clone());

    let fund_name = String::from("liberty");
    app.create_fund(fund_name.clone());

    let fund = app.get_fund(&fund_name);

    let asset_type = AssetType::Bitcoin { utxo_hash: "hash123".to_string(), sats: 100_000_000 };
    app.create_new_asset(fund.clone(), asset_type);

    let asset = app.get_asset(0);

    println!("NICE: {:?}", asset);

    user.new_app_state(app.clone(), "admin123");

    println!("USER: {:?}", user);

    let asset_type = AssetType::Bitcoin { utxo_hash: "hash456".to_string(), sats: 250_000 };
    app.create_new_asset(fund, asset_type);    

    user.save_app_state(app.clone(), "admin123");

    user.save();

    let new_user = User::open("TESTUSER", "admin123", AppEnv::Dev);
    let new_app = new_user.load_app_state("admin123");

    assert!(new_user.is_equal(user));
    assert!(new_app.is_equal(app));

    // let users = app.import_user_app_state_hash();

    // app.login(user);
    // assert!(user.is_valid_password("admin123"));
    // let app = App::new();
    // create_new_asset()
}