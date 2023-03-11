use std::error::Error as StdError;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

async fn make_request(url: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    println!("Response: {}", body);
    Ok(())
}

async fn run() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let task = make_request("https://jsonplaceholder.typicode.com/posts/8?_delay=3000");
    tokio::try_join!(task)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let (tx, _) = broadcast::channel(1);

    let mut task: Option<JoinHandle<Result<(), Box<dyn StdError + Send + Sync>>>> = None;
    loop {
        if task.is_none() {
            println!("Start!");
            task = Some(tokio::spawn(run()));
        }

        tokio::select! {
            result = task.as_mut().unwrap() => {
                match result {
                    Ok(_) => break,
                    Err(_) => break,
                }
            },
            _ = sleep(Duration::from_secs(1)) => {
                // keep it alive
                tx.send(()).ok();
            }
        }

        println!("Do other work ....");
    }
    println!("End!");
    Ok(())
}
