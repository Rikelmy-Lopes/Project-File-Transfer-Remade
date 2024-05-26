use std::net::IpAddr;

use rocket::{get, routes, Config, Shutdown};

include!("../utils/os.rs");
include!("../fs/fs.rs");

#[get("/down?<password>")]
fn down(password: &str, shutdown: Shutdown) -> &'static str {
    if password == "8910" {
        shutdown.notify();
    }
    "ShoutDown"
}

#[get("/entries")]
fn get_entries() -> String {
    let files = get_dir_entries();

    return serde_json::to_string(&files).unwrap();
}

#[tauri::command]
pub async fn start_server(port: u16) -> &'static str {
    let ip = get_current_ip();

    let config = Config {
        address: Result::expect(ip.parse::<IpAddr>(), "Error converting string to Address!"),
        port: port,
        ..Config::debug_default()
    };

    rocket::custom(&config)
        .mount("/", routes![down, get_entries])
        .launch()
        .await
        .expect("Failed to start the server!!!");

    return "Started";
}
