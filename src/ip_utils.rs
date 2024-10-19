use socket2::{Domain, Protocol, Socket, Type};
use std::mem::MaybeUninit;
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::net::{TcpStream, UdpSocket};
use tokio::time::timeout;

// ICMP Echo Request Type and Code
const ICMP_ECHO_REQUEST: u8 = 8;
const ICMP_ECHO_REPLY: u8 = 0;

fn checksum(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    let chunks = data.chunks_exact(2);
    for chunk in chunks {
        sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
    }
    if let Some(&byte) = data.chunks_exact(2).remainder().first() {
        sum += (byte as u32) << 8;
    }
    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

fn build_icmp_packet(sequence: u16) -> Vec<u8> {
    let mut packet = vec![0u8; 8]; // ICMP Header is 8 bytes
    packet[0] = ICMP_ECHO_REQUEST; // Type (Echo Request)
    packet[1] = 0; // Code
    packet[2] = 0; // Checksum (placeholder)
    packet[3] = 0;
    packet[4] = (sequence >> 8) as u8; // Identifier
    packet[5] = (sequence & 0xff) as u8;
    packet[6] = 0; // Sequence number
    packet[7] = sequence as u8;

    let checksum_val = checksum(&packet);
    packet[2] = (checksum_val >> 8) as u8;
    packet[3] = (checksum_val & 0xff) as u8;

    packet
}

pub async fn ping(target_ip: Ipv4Addr, timeout: Duration) -> Result<(), std::io::Error> {
    // Create a raw socket for ICMP (requires root/admin privileges)
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    // Set a timeout for the socket
    socket.set_read_timeout(Some(timeout))?;

    // Destination address
    let socket_addr = socket2::SockAddr::from(std::net::SocketAddrV4::new(target_ip, 0));

    // Build an ICMP echo request packet
    let icmp_packet = build_icmp_packet(1); // Sequence number 1

    // Send the ICMP packet
    socket.send_to(&icmp_packet, &socket_addr)?;

    // Buffer to receive the reply
    let mut buf = [const { MaybeUninit::<u8>::uninit() }; 32];

    // Receive the ICMP echo reply
    match socket.recv_from(&mut buf) {
        Ok((size, _)) => {
            // SAFETY: We know `size` bytes were written to the buffer, so we can safely assume these bytes are initialized
            let initialized_buf =
                unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, size) };

            if size >= 20 && initialized_buf[20] == ICMP_ECHO_REPLY {
                return Ok(());
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Received non-echo reply packet",
                ));
            }
        }
        Err(e) => {
            // Forward the error instead of printing it
            return Err(e);
        }
    }
}

pub async fn can_connect(
    ip: Ipv4Addr,
    port: u16,
    timeout_duration: Duration,
) -> Result<bool, std::io::Error> {
    // Define the socket address (IP + port)
    let address = SocketAddr::new(IpAddr::V4(ip), port);

    // Attempt to connect with a timeout
    let result = timeout(timeout_duration, TcpStream::connect(address)).await;

    match result {
        Ok(Ok(_)) => Ok(true), // Connection succeeded
        Ok(Err(e)) => Err(e),  // Connection failed
        Err(_) => Ok(false),   // Timeout error
    }
}

pub async fn get_local_ip() -> Result<IpAddr, std::io::Error>  {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect("8.8.8.8:80").await?;
    let socket_addr = socket.local_addr()?;
    return Ok(socket_addr.ip());
}

pub async fn to_ip_or_local(ip: Option<String>) -> String {
    // Check if the provided IP is Some and non-empty
    if let Some(t) = ip {
        if !t.is_empty() {
            return t; // Return the provided IP
        }
    }

    // If the provided IP is None or empty, get the local IP
    match get_local_ip().await {
        Ok(addr) => addr.to_string(), // Return the local address as a string
        Err(e) => {
            eprintln!("Failed to get local address: {}", e);
            String::from("127.0.0.1") // Return a default value if an error occurs
        }
    }
}