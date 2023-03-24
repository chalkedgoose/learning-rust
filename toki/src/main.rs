use log::Level;
use tokio::{io::AsyncReadExt, time};

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

async fn sleeper() {
    log::info!("Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading some snap data");
    let mut f = tokio::fs::File::open("snap.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read snap {} bytes", contents.len());

    tokio::task::spawn_blocking(move || {
        log::info!("Computing fib(40)");
        fib(40);
        log::info!("Done Computing fib(40)");
    }).await.unwrap();
}

async fn run() {
    tokio::join!(
        sleeper(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
    );
}
 
#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("Took {:?} seconds", end - start)
}
