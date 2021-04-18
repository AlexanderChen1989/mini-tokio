use mini_tokio::*;
use std::time::{Duration};

fn main() {
    let mt = MiniTokio::new();
    
}

// Main entry point. A mini-tokio instance is created and a few tasks are
// spawned. Our mini-tokio implementation only supports spawning tasks and
// setting delays.
fn test() {
    // Create the mini-tokio instance.
    let mini_tokio = MiniTokio::new();

    // Spawn the root task. All other tasks are spawned from the context of this
    // root task. No work happens until `mini_tokio.run()` is called.
    mini_tokio.spawn(async {
        // Spawn a task
        spawn(async {
            // Wait for a little bit of time so that "world" is printed after
            // "hello"
            delay(Duration::from_millis(100)).await;
            println!("world");
        });

        // Spawn a second task
        spawn(async {
            println!("hello");
        });

        // We haven't implemented executor shutdown, so force the process to exit.
        delay(Duration::from_millis(200)).await;
        std::process::exit(0);
    });

    // Start the mini-tokio executor loop. Scheduled tasks are received and
    // executed.
    mini_tokio.run();
}
