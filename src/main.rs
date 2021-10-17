// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

fn main() {
    let future = do_something(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}

async fn do_something() {
    /*
    Whereas calling a blocking function in a synchronous method would block the whole thread, 
    blocked Futures will yield control of the thread, allowing other Futures to run.
    */
    println!("hello, world!");
}
// The value returned by async fn is a Future. For anything to happen, 
// the Future needs to be run on an executor.