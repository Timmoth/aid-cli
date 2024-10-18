use reqwest;

pub async fn http_get_request(url: String) {
    match reqwest::get(url).await {
        Ok(s) => println!("{}", s.text().await.unwrap()),
        Err(e) => {
            eprintln!("Http request failed: {}", e);
            return;
        }
    };
}