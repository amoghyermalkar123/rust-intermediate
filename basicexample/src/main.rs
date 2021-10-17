
use std::{thread, time::Duration};
use futures::executor::block_on;

fn main() {
    block_on(start());
}

/*

Inside an async fn, you can use .await to wait for the completion of another type that 
implements the Future trait, such as the output of another async fn. Unlike block_on, 
.await doesn't block the current thread, but instead asynchronously waits for the future 
to complete, allowing other tasks to run if the future is currently unable to make progress.

*/

async fn start(){
    usual_operational_flow().await;
}

// this function represents a usual sequence of instructions
async fn usual_operational_flow() {
    println!("amogh");
    let fut = heavy_operations_that_may_take_time();
    println!("is a");
    println!("genius :)");
}

// this function represents any heavy operations
async fn heavy_operations_that_may_take_time() {
    thread::sleep(Duration::from_secs(10));
    println!("yermalkar")
}

/*
because futures are lazy af, zero-cost traits in addition to rust having no runtime,
there needs to be an executor implemented to handle all the futures because again, 
you have to keep this in mind that futures don't run/ progress until they are polled! 
*/

// from the tokio async docs :
/**
 * Unlike how futures are implemented in other languages, 
 * a Rust future does not represent a computation happening in the background, 
 * rather the Rust future is the computation itself. 
 * The owner of the future is responsible for advancing the computation 
 * by polling the future. This is done by calling Future::poll.
 */
