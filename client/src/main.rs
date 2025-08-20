use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use std::{
    net::{ToSocketAddrs, UdpSocket},
    thread::{sleep, spawn},
    time::Duration,
};

#[repr(C)]
struct Handshaker {
    identifier: i32,
    version: i32,
    operation_id: i32,
}

impl Handshaker {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(12);
        buf.write_i32::<LittleEndian>(self.identifier)?;
        buf.write_i32::<LittleEndian>(self.version)?;
        buf.write_i32::<LittleEndian>(self.operation_id)?;
        Ok(buf)
    }
}

#[repr(C)]
struct HandshakerResponse {
    car_name: [u8; 50],
    driver_name: [u8; 50],
    identifier: i32,
    version: i32,
    track_name: [u8; 50],
    track_config: [u8; 50],
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // --- TELEMETRY LISTENER ---
    spawn(|| {
        let server_addr = "127.0.0.1:5000";
        tracing::info!("Listening on UDP port {server_addr} ...");
        let telemetry_socket = UdpSocket::bind(server_addr).unwrap();

        let mut buf = [0u8; 2048];
        loop {
            let size = telemetry_socket.recv(&mut buf).unwrap();
            tracing::warn!("Received {} bytes", size);
            tracing::warn!("{:02x?}", &buf[..size]);

            // Optionally: send ACK back to plugin
            // telemetry_socket.send_to(&buf[..size], "127.0.0.1:5000").unwrap();
        }
    });

    let server_addr = "127.0.0.1:9600";
    tracing::info!("Connecting to server at {server_addr} ...");
    let server_addr = server_addr
        .to_socket_addrs()?
        .next()
        .expect("Invalid address");
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(server_addr)?;

    let handshake = Handshaker {
        identifier: 1,
        version: 1,
        operation_id: 0,
    };
    let buf = handshake.to_bytes()?;

    socket.send(&buf)?;
    tracing::info!("Handshake sent.");

    sleep(Duration::new(60, 0));

    Ok(())
}
