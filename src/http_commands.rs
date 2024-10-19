use reqwest;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::str;

pub async fn http_get_request(url: String) {
    match reqwest::get(url).await {
        Ok(s) => println!("{}", s.text().await.unwrap()),
        Err(e) => {
            eprintln!("Http request failed: {}", e);
            return;
        }
    };
}

pub async fn http_serve(port: u16) {
    let addr = format!("127.0.0.1:{}", port);
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

// Handler function to process client requests
async fn handle_client(stream: &mut tokio::net::TcpStream) {
    let mut buffer = [0; 1024]; // Buffer for storing client data
    match stream.read(&mut buffer).await {
        Ok(0) => return, // Connection closed
        Ok(_) => {
            // Convert the buffer to a string to see the request
            if let Ok(request) = str::from_utf8(&buffer) {
                println!("Received request: {}", request);

                // Simple HTTP response with "Hello, World!"
                let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
                
                // Write the response to the client
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Failed to send response: {}", e);
                }

                // Flush the stream to ensure all data is sent
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
