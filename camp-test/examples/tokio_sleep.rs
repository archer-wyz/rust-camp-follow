use std::time::Instant;
use tokio::{
    runtime::Runtime,
    time::{sleep, Duration},
};

fn main() -> std::io::Result<()> {
    let now = Instant::now();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(my_bg_task(i));
    }

    // Do something time-consuming while the background tasks execute.
    std::thread::sleep(Duration::from_millis(120));
    println!("Finished time-consuming task.");

    let rt = Runtime::new().unwrap();
    // Wait for all of them to complete.
    for handle in handles {
        rt.block_on(handle);
    }

    println!("总耗时：{} ms", now.elapsed().as_millis());
    Ok(())
}

async fn my_bg_task(i: u64) {
    let millis = 100;
    println!("Task {} sleeping for {} ms.", i, millis);
    sleep(Duration::from_millis(millis)).await;
    println!("Task {} stopping.", i);
}
