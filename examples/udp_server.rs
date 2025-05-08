use std::{env, net::SocketAddr, sync::Arc};
use tokio_uring::net::UdpSocket;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        panic!("Usage: {} <bind-address>", args[0]);
    }

    let addr: SocketAddr = args[1].parse().expect("Invalid address format");

    // 启动Tokio-uring运行时
    tokio_uring::start(async {
        let socket = UdpSocket::bind(addr).await.expect("Failed to bind socket");
        let socket = Arc::new(socket); // 使用Arc支持多线程处理（可选）

        println!("UDP server listening on {}", addr);

        // 主循环持续处理请求
        loop {
            let buf = vec![0u8; 1024]; // 每次循环分配新缓冲区
            let socket_clone = socket.clone();

            // 异步接收数据
            let (result, mut buf) = socket_clone.recv_from(buf).await;
            match result {
                Ok((read, client_addr)) => {
                    buf.truncate(read);
                    // println!("Received {} bytes from {}: {:?}", read, client_addr, &buf);

                    /***** 异步发送响应（示例：原样回传） *****/
                    tokio_uring::spawn(async move {
                        let send_result = socket_clone.send_to(buf, client_addr).await;

                        match send_result {
                            (Ok(size), buf) => {
                                // println!("Received {} bytes: {:?}", size, &buf[..size]);
                            }
                            (Err(e), _) => {
                                // eprintln!("Error occurred: {}", e);
                            }
                        }
                    });

                    /**** 同步回传 ****/
                    // let send_result = socket_clone.send_to(buf, client_addr).await;
                    // match send_result {
                    //     (Ok(size), buf) => {
                    //         // println!("Received {} bytes: {:?}", size, &buf[..size]);
                    //     }
                    //     (Err(e), _) => {
                    //         // eprintln!("Error occurred: {}", e);
                    //     }
                    // }
                }
                Err(e) => {
                    eprintln!("Receive error: {}", e);
                    continue; // 错误时继续运行
                }
            }
        }
    });
}
