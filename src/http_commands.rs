use reqwest::Method;
use tokio::fs::File;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::str::{self, FromStr};
use serde_derive::Deserialize;
use serde_json;
use reqwest::{Client, header::{HeaderMap, HeaderName, HeaderValue}};
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct HttpRequestConfig {
    method: Option<String>,
    url: Option<String>,
    headers: Option<Value>,
    body: Option<Value>
}

async fn read_config_file(file_path: &str) -> Result<HttpRequestConfig, String> {
    let mut file = File::open(file_path).await.map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.map_err(|e| format!("Failed to read file: {}", e))?;
    let config: HttpRequestConfig = serde_json::from_str(&contents).map_err(|e| format!("Failed to deserialize JSON: {}", e))?;
    Ok(config)
}

// Main function to send HTTP requests
pub async fn http_request(method: Option<String>, url: Option<String>, config_path: Option<String>) {
    let mut config: HttpRequestConfig = HttpRequestConfig {
        method: None,
        url: None,
        headers: None,
        body: None,
    };

    // Read configuration from file if provided
    if let Some(c) = config_path {
        if !c.is_empty() {
            match read_config_file(&c).await {
                Ok(cfg) => config = cfg,
                Err(e) => {
                    eprintln!("Error reading config: {}", e);
                    return; // Exit early if the config fails
                }
            }
        }
    }

    // Set the URL from input if provided
    if let Some(u) = url {
        if !u.is_empty() {
            config.url = Some(u);
        }
    }

    if let Some(m) = method {
        if !m.is_empty() {
            config.method = Some(m);
        }
    }

    if let Some(url) = config.url {
        let method = match config.method {
            Some(method) => match Method::from_str(&method) {
                Ok(m) => m,
                Err(_) => {
                    eprintln!("Invalid HTTP method: {}", method);
                    return;
                }
            },
            None => {
                eprintln!("No valid HTTP method provided.");
                return;
            }
        };

        // Initialize the request builder
        let mut builder = Client::new().request(method, &url);

        // Add headers if provided
        if let Some(header_map) = config.headers {
            if let Some(headers_object) = header_map.as_object() {
                let mut headers = HeaderMap::new();
                for (key, value) in headers_object {
                    if let Some(value_str) = value.as_str() {
                        if let Ok(header_key) = HeaderName::from_str(key) {
                            if let Ok(header_value) = HeaderValue::from_str(value_str) {
                                headers.insert(header_key, header_value);
                            } else {
                                eprintln!("Invalid header value for '{}': {}", key, value_str);
                            }
                        } else {
                            eprintln!("Invalid header key: {}", key);
                        }
                    } else {
                        eprintln!("Header value for '{}' is not a string: {:?}", key, value);
                    }
                }
                builder = builder.headers(headers);
            } else {
                eprintln!("Headers are not in an object format.");
            }
        }

        // Add the body if it's provided
        if let Some(body) = config.body {
            // Serialize the body as a JSON string
            match serde_json::to_string(&body) {
                Ok(body_str) => {
                    builder = builder.body(body_str);
                }
                Err(e) => {
                    eprintln!("Failed to serialize body: {}", e);
                    return;
                }
            }
        }

        // Send the request and handle the response
        match builder.send().await {
            Ok(response) => handle_response(response).await,
            Err(e) => {
                eprintln!("Http request failed: {}", e);
            }
        }
    } else {
        eprintln!("No valid configuration or URL provided.");
    }
}

// Function to handle HTTP response
async fn handle_response(response: reqwest::Response) {
    if response.status().is_success() {
        match response.text().await {
            Ok(text) => println!("{}", text),
            Err(e) => eprintln!("Failed to read response: {}", e),
        }
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }
}

pub async fn http_serve(port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on http://{}", addr);

    // Accept incoming connections in a loop
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_client(&mut stream).await;
        });
    }
}

async fn handle_client(stream: &mut tokio::net::TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer).await {
        Ok(0) => return, // Connection closed
        Ok(_) => {
            if let Ok(request) = str::from_utf8(&buffer) {
                println!("Received request: {}", request);
                let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
                
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response: {}", e);
                }

                if let Err(e) = stream.flush().await {
                    eprintln!("Failed to flush stream: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }

    // Close the connection
    if let Err(e) = stream.shutdown().await {
        eprintln!("Failed to shutdown stream: {}", e);
    }
}
