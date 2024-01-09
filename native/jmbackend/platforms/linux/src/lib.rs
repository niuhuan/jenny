use std::process::exit;
use std::time::Duration;

use serde_json::to_string;
use warp::Filter;

pub use jmbackend::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DartQuery {
    pub method: String,
    pub params: String,
}

#[no_mangle]
pub unsafe extern "C" fn init_http_server() {
    let ping = warp::get()
        .and(warp::path("ping"))
        .map(|| "pong".to_owned());
    let invoke = warp::post()
        .and(warp::path("invoke"))
        .and(warp::body::json())
        .map(invoke);
    std::thread::spawn(move || {
        jmbackend::RUNTIME.block_on(warp::serve(ping.or(invoke)).run(([127, 0, 0, 1], 52764)))
    });
    jmbackend::RUNTIME.block_on(test_startup());
}

fn invoke(dq: DartQuery) -> String {
    jmbackend::RUNTIME.block_on(jmbackend::invoke_async(to_string(&dq).unwrap().as_str()))
}

async fn test_startup() {
    for i in 0..6 {
        if i == 5 {
            exit(2);
        }
        std::thread::sleep(Duration::new(1, 0));
        match reqwest::get("http://127.0.0.1:52764/ping").await {
            Ok(req) => match req.text().await {
                Ok(txt) => {
                    println!("OK : {}", txt);
                    break;
                }
                Err(err) => println!("ERR : {}", err),
            },
            Err(err) => {
                println!("ERR : {}", err)
            }
        }
    }
}
