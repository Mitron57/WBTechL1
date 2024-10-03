use tokio::signal::ctrl_c;
use flume::bounded;
use tokio::select;
use tokio_util::sync::CancellationToken;

// Вариант 2: вместо CancellationToken использовать второй 
// канал для отправки уведомления о закрытии программы

#[tokio::main]
async fn main() {
    let (tx, rx) = bounded(10);
    let cpus = std::thread::available_parallelism().unwrap().get();  // Get available CPUs
    let mut pool = Vec::with_capacity(cpus);  // Worker pool
    let token = CancellationToken::new(); //Synchronization
    for i in 0..cpus {
        let rx = rx.clone();
        let token = token.clone();
        pool.push(tokio::spawn(async move {
            loop {
                //God bless golang
                select! {
                    _ = token.cancelled() => {
                        println!("Thread {i} cancelled");
                        return;
                    },
                    Ok(msg) = rx.recv_async() => {
                        println!("Thread {i} received message: {}", msg);
                    }
                }
            }
        }));
    }
    tokio::spawn(async move {
        let mut i = 0;
        loop {
            select! {
                // This kinda buggy: it can be required for several signals sent to the process,
                // dunno, maybe macOS problem?
                _ = ctrl_c() => {
                    println!("Received Ctrl+C, shutting down...");
                    token.cancel();
                    return;
                },
                Ok(_) = tx.send_async(i) => {
                    i += 1;
                }
            }
        }
    });
    //Await them
    for worker in pool {
        if let Err(err) = worker.await {
            eprintln!("Error in worker task: {err}");
        }
    }
}