use std::collections::HashMap;

use error_chain::error_chain;
use serde::{Serialize, Deserialize};

use reqwest::header::{CONTENT_TYPE, ORIGIN, HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_ENCODING, CACHE_CONTROL, CONNECTION, HOST, UPGRADE, UPGRADE_INSECURE_REQUESTS, ACCEPT_LANGUAGE};

use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

// #[derive(Serialize, Deserialize, Debug)]
// struct GETAPIResponse {
//     origin: String,
// }

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

async fn run_retries() {
    // Retry up to 3 times with increasing intervals between attempts.
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    client
        .get("https://truelayer.com")
        .header("foo", "bar")
        .send()
        .await
        .unwrap();
}


#[tokio::main]
async fn main() -> Result<()> {

    // let mut headers = HeaderMap::new();
    // headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    // headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    // headers.insert(ORIGIN, HeaderValue::from_static("null"));
    // headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    // headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    // headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"));
    // headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    // headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    // headers.insert(HOST, HeaderValue::from_static("worldtimeapi.org"));
    // headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    // headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36"));

    // let client = reqwest::Client::new();
    // let res = client.get("https://timeapi.io/api/Time/current/zone")
    //     .send()
    //     .await?
    //     .json().await?;


    const URL_WORLDTIMEAPI: &str = "http://worldtimeapi.org/api/timezone/Europe/Berlin";
    const URL_TIMEAPI: &str = "https://timeapi.io/api/Time/current/zone";
        
    
    // let client = reqwest::Client::new();
    // let res = client
    //     .get(URL_WORLDTIMEAPI)
    //     .send()
    //     .await
    //     .expect("failed to get response").json().await?;
    //     // .text()
    //     // .await
    //     // .expect("failed to get payload");

    // println!("{:?}", res);


    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .get(URL_WORLDTIMEAPI)
        .header("foo", "bar")
        .send()
        .await
        .unwrap();


    println!("{:?}", res);

    Ok(())
}