
use lazy_static::lazy_static;

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::{net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4}, sync::{atomic::AtomicBool, Arc}, io, mem::{MaybeUninit, ManuallyDrop}, time::Duration, error::Error};

static PORT: u16 = 7982;

lazy_static! {
    pub static ref IPV4: IpAddr = Ipv4Addr::new(224, 0, 0, 69).into();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let is_finished = Arc::new(AtomicBool::new(false));
    let read_handle = is_finished.clone();
    let write_handle = is_finished.clone();

    let read_jh = tokio::spawn(async move {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
            socket.join_multicast_v4(&Ipv4Addr::new(224, 0, 0, 69), &Ipv4Addr::UNSPECIFIED).unwrap();
            socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, PORT).into()).unwrap();
            socket.set_read_timeout(Some(Duration::from_millis(200))).unwrap();
            let udp_socket = tokio::net::UdpSocket::from_std(socket.into()).unwrap();
    
            let mut buf: [u8; 50] = [0;50];
            while !read_handle.load(std::sync::atomic::Ordering::Relaxed) {
                match udp_socket.recv_from(&mut buf).await {
                    Ok(_) => {
                        let value = String::from_utf8_lossy(&buf);
                        println!("{value}");
                    },
                    _ => {
    
                    }
                }
            }
    });

    let write_jh = tokio::spawn(async move {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        socket.set_multicast_if_v4(&Ipv4Addr::UNSPECIFIED).unwrap();
        socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0).into()).unwrap();
        socket.set_write_timeout(Some(Duration::from_millis(200))).unwrap();

        let udp_socket = tokio::net::UdpSocket::from_std(socket.into()).unwrap();
        let mut ctr = 1;
        while !write_handle.load(std::sync::atomic::Ordering::Relaxed) {
            let to_send = format!("Jack has {} braincells", ctr);
            let bytes = to_send.as_bytes();
            udp_socket.send_to(&bytes, &SocketAddrV4::new(Ipv4Addr::new(224, 0, 0, 69), PORT)).await.unwrap();
            std::thread::sleep(Duration::from_secs(2));
            ctr += 1;
        }
    });


    loop {
        let mut str = String::new();
        let v = std::io::stdin().read_line(&mut str);
        match v {
            Ok(_) => {
                println!("read: {}", str);
                if str.starts_with("end") {
                    println!("Program will end");
                    is_finished.store(true, std::sync::atomic::Ordering::Relaxed);
                    break;
                }
            },
            _ => {}
        }
    }
    read_jh.await?;
    write_jh.await?;

    Ok(())
}
