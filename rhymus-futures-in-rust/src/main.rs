fn main() {
    const NUM_MESSAGES: usize = 1000;

    let reciever = futures::executor::block_on(async {
        async_std::net::UdpSocket::bind("127.0.0.1:0")
            .await
            .unwrap()
    });
    let sender = futures::executor::block_on(async {
        async_std::net::UdpSocket::bind("127.0.0.1:0")
            .await
            .unwrap()
    });

    let sender_future = async {
        for _ in 0..NUM_MESSAGES {
            let _ = sender
                .send_to(b"Hello, World!", reciever.local_addr().unwrap())
                .await;
            futures_timer::Delay::new(std::time::Duration::from_millis(1)).await;
        }
    };

    let reciever_future = async {
        let mut buffer = [0; 256];
        let mut count = 0;
        for _ in 0..NUM_MESSAGES {
            let _ = reciever.recv_from(&mut buffer).await.unwrap();
            count += 1;
            println!("We recieved {} messages", count)
        }
    };

    futures::executor::block_on(async {
        futures::join!(sender_future, reciever_future);
    });
}
