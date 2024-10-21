use reqwest::Method;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_derive::Deserialize;
use serde_json::{from_str, Value};
use reqwest::{Client, header::{HeaderMap, HeaderName, HeaderValue}};
use core::str;
use std::str::FromStr;
use tokio::net::TcpListener;

#[derive(Deserialize, Debug)]
struct HttpRequestConfig {
    method: Option<String>,
    url: Option<String>,
    headers: Option<Value>,
    body: Option<Value>,
}

async fn read_config_file(file_path: &str) -> Result<HttpRequestConfig, String> {
    let mut file = File::open(file_path).await.map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.map_err(|e| format!("Failed to read file: {}", e))?;
    serde_json::from_str(&contents).map_err(|e| format!("Failed to deserialize JSON: {}", e))
}

// Helper function to build request headers
fn build_headers(header_map: Option<Value>) -> Result<HeaderMap, String> {
    let mut headers = HeaderMap::new();
    if let Some(headers_object) = header_map.and_then(|h| h.as_object().cloned()) {
        for (key, value) in headers_object {
            let header_key = HeaderName::from_str(&key).map_err(|_| format!("Invalid header key: {}", key))?;
            let value_str = value.as_str().ok_or(format!("Header value for '{}' is not a string", key))?;
            let header_value = HeaderValue::from_str(value_str).map_err(|_| format!("Invalid header value for '{}': {}", key, value_str))?;
            headers.insert(header_key, header_value);
        }
    }
    Ok(headers)
}

// Helper function to handle the request method and URL logic
fn parse_method_and_url(config: &mut HttpRequestConfig, method: Option<String>, url: Option<String>) -> Result<(Method, String), String> {
    if let Some(u) = url {
        if !u.is_empty() {
            config.url = Some(u);
        }
    }

    let url = config.url.clone().ok_or("No valid URL provided.".to_string())?;

    if let Some(m) = method {
        if !m.is_empty() {
            config.method = Some(m);
        }
    }

    let method = config.method.clone().ok_or("No valid HTTP method provided.".to_string())?;
    let method = Method::from_str(&method).map_err(|_| format!("Invalid HTTP method: {}", method))?;

    Ok((method, url))
}

pub async fn http_request(
    method: Option<String>, 
    url: Option<String>, 
    config_path: Option<String>,
    output: Option<String>
) {
    let mut config = HttpRequestConfig {
        method: None,
        url: None,
        headers: None,
        body: None,
    };

    // Load configuration from file, if provided
    if let Some(c) = config_path {
        if let Err(e) = read_config_file(&c).await.map(|cfg| config = cfg) {
            eprintln!("Error reading config: {}", e);
            return;
        }
    }

    // Parse the method and URL
    let (method, url) = match parse_method_and_url(&mut config, method, url) {
        Ok((method, url)) => (method, url),
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Build the request
    let client = Client::new();
    let mut builder = client.request(method, &url);

    // Add headers if provided
    if let Ok(headers) = build_headers(config.headers) {
        builder = builder.headers(headers);
    } else {
        eprintln!("Error building headers");
        return;
    }

    // Add body if provided
    if let Some(body) = config.body {
        if let Ok(body_str) = serde_json::to_string(&body) {
            builder = builder.body(body_str);
        } else {
            eprintln!("Failed to serialize body");
            return;
        }
    }

    // Send the request and handle the response
    match builder.send().await {
        Ok(response) => handle_response(response, output).await,
        Err(e) => eprintln!("Http request failed: {}", e),
    }
}

// Helper function to handle HTTP response
async fn handle_response(response: reqwest::Response, output: Option<String>) {
    let status = response.status();
    if status.is_success() {
        if let Some(output) = output {
            if let Ok(bytes) = response.bytes().await {
                if let Ok(mut file) = File::create(&output).await {
                    let _ = file.write_all(&bytes).await;
                    println!("Downloaded file to: {}", output);
                } else {
                    eprintln!("Failed to write the output file.");
                }
            }
        } else {
            if let Ok(text) = response.text().await {
                let payload = from_str(&text).unwrap_or(Value::Null);
                let json_output = serde_json::to_string_pretty(&payload).unwrap();
                println!("{}", json_output);
            } else {
                eprintln!("Failed to read response.");
            }
        }
    } else {
        eprintln!("Request failed with status: {}", status);

        // Attempt to read and print the error body if available
        if let Ok(error_body) = response.text().await {
            let payload = from_str(&error_body).unwrap_or(Value::Null);
            let json_output = serde_json::to_string_pretty(&payload).unwrap();
            println!("{}", json_output);
        } else {
            eprintln!("Failed to read the error response body.");
        }
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
