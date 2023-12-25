use lazy_static::lazy_static;

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::{
    error::Error,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

static PORT: u16 = 7982;

lazy_static! {
    pub static ref IPV4: IpAddr = Ipv4Addr::new(224, 0, 0, 69).into();
}

#[cfg(windows)]
fn bind_socket_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    let addr = SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), addr.port());
    socket.bind(&socket2::SockAddr::from(addr))
}

#[cfg(unix)]
fn bind_socket_multicast(socket: &Socket, addr: &SocketAddr) -> io::Result<()> {
    socket.bind(&socket2::SockAddr::from(*addr))
}

fn main() -> Result<(), Box<dyn Error + Send>> {
    let is_finished = Arc::new(AtomicBool::new(false));
    let read_handle = is_finished.clone();
    let read_print_handle = is_finished.clone();
    let write_handle = is_finished.clone();
    let reader_ready = Arc::new(AtomicBool::new(false));
    let rr = reader_ready.clone();
    let writer_ready = Arc::new(AtomicBool::new(false));
    let wr = writer_ready.clone();

    let (sender, receiver) = std::sync::mpsc::channel::<String>();
    let read_jh = std::thread::spawn(move || {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        socket
            .join_multicast_v4(&Ipv4Addr::new(224, 0, 0, 69), &Ipv4Addr::UNSPECIFIED)
            .unwrap();
        let sa = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 69)), PORT);
        bind_socket_multicast(&socket, &sa).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_millis(200)))
            .unwrap();
        let udp_socket = std::net::UdpSocket::from(socket);
        reader_ready.store(true, std::sync::atomic::Ordering::Relaxed);

        let mut buf: [u8; 50] = [0; 50];
        while !read_handle.load(std::sync::atomic::Ordering::Relaxed) {
            match udp_socket.recv_from(&mut buf) {
                Ok(_) => {
                    let value = String::from_utf8_lossy(&buf);
                    sender.send(value.into_owned()).unwrap();
                }
                _ => {}
            }
        }
    });

    let read_print_jh = std::thread::spawn(move || {
        while !read_print_handle.load(std::sync::atomic::Ordering::Relaxed) {
            match receiver.try_recv() {
                Ok(v) => {
                    println!("Received: {v}");
                }
                Err(_) => {}
            }
        }
    });

    let (sender, receiver) = std::sync::mpsc::channel::<String>();
    let write_jh = std::thread::spawn(move || {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        socket.set_multicast_if_v4(&Ipv4Addr::UNSPECIFIED).unwrap();
        socket
            .bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0).into())
            .unwrap();
        socket
            .set_write_timeout(Some(Duration::from_millis(200)))
            .unwrap();
        let udp_socket: std::net::UdpSocket = std::net::UdpSocket::from(socket);
        writer_ready.store(true, std::sync::atomic::Ordering::Relaxed);

        while !write_handle.load(std::sync::atomic::Ordering::Relaxed) {
            let send_value;
            let mut val: String = "".to_owned();
            match receiver.try_recv() {
                Ok(v) => {
                    send_value = true;
                    val = v;
                }
                Err(_) => {
                    send_value = false;
                }
            }

            if send_value {
                let bytes = val.as_bytes();
                udp_socket
                    .send_to(
                        &bytes,
                        &SocketAddrV4::new(Ipv4Addr::new(224, 0, 0, 69), PORT),
                    ).unwrap();
            }
        }
    });

    println!("Starting loop");
    loop {
        let mut str = String::new();
        let v = std::io::stdin().read_line(&mut str);
        match v {
            Ok(_) => {
                if str.starts_with("end") {
                    println!("Program will end");
                    is_finished.store(true, std::sync::atomic::Ordering::Relaxed);
                    break;
                } else {
                    sender.send(str).unwrap();
                }
            }
            _ => {}
        }
    }
    read_jh.join();
    read_print_jh.join();
    write_jh.join();

    Ok(())
}
