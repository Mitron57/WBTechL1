use std::sync::mpsc;
use tokio::select;
use tokio::task::yield_now;
use tokio_util::sync::CancellationToken;

fn close_channel() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let sender = std::thread::spawn(move || {
        let mut i = 0;
        while tx1.send(i).is_ok() {
            i += 1;
        }
    });
    std::thread::spawn(move || {
        let mut count = 0;
        while let Ok(i) = rx.recv() {
            println!("Got {i}");
            count += 1;
            if count == 10 {
                // Because rx has been moved to the scope of the thread, so channel will be closed,
                // and send operation will be ended, so threads will be finished correctly
                return;
            }
        }
    });
    let _ = sender.join();
}

async fn cancellation_token() {
    let token = CancellationToken::new();
    let clone = token.clone();
    let task = tokio::spawn(async move {
        let mut i = 1u64;
        let reference = &mut i;
        loop {
            select! {
                _ = clone.cancelled(), if clone.is_cancelled() => {
                    println!("Cancelled on i: {i}");
                    return;
                },
                else => {
                    i += 1;
                }
            }
        }
    });
    let canceler = tokio::spawn(async move {
        yield_now().await;
        token.cancel();
    });
    let _ = tokio::join!(task, canceler);
}

#[tokio::main]
async fn main() {
    close_channel();
    cancellation_token().await;
}
