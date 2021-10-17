use crossbeam::channel;
use futures::task::{self, ArcWake};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

// impl Future for Delay {
//     type Output = &'static str;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
//         -> Poll<&'static str>
//     {
//         if Instant::now() >= self.when {
//             println!("Hello world");
//             Poll::Ready("done")
//         } else {
//             // we signalled the waker inline. Doing so will result in the future
//             // being immediately re-scheduled, executed again, and probably not
//             // be ready to complete.
//             cx.waker().wake_by_ref();
//             // Notice that you are allowed to signal the waker more often than
//             // necessary. In this particular case, we signal the waker even
//             // though we are not ready to continue the operation at all.
//             // There is nothing wrong with this besides some wasted CPU cycles.
//             // However, this particular implementation will result in a
//             // busy loop.
//             Poll::Pending
//             // run this program and see the htop command for the cpu load
//         }
//     }
// }

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("future complete");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread.
            match thread::Builder::new().spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            }) {
                Ok(_) => println!("not fail"),
                Err(_) => println!("fail"),
            }

            Poll::Pending
        }
    }
}

struct Task {
    // The `Mutex` is to make `Task` implement `Sync`. Only
    // one thread accesses `future` at any given time. The
    // `Mutex` is not required for correctness. Real Tokio
    // does not use a mutex here, but real Tokio has
    // more lines of code than can fit in a single tutorial
    // page.
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        (*arc_self).schedule();
    }
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }
}

impl Task {
    fn poll(self: Arc<Self>) {
        // Create a waker from the `Task` instance. This
        // uses the `ArcWake` impl from above.
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // No other thread ever tries to lock the future
        let mut future = self.future.try_lock().unwrap();

        // Poll the future
        let _ = future.as_mut().poll(&mut cx);
    }

    // Spawns a new task with the given future.
    //
    // Initializes a new Task harness containing the given future and pushes it
    // onto `sender`. The receiver half of the channel will get the task and
    // execute it.
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }

}

// impl MiniTokio {
//     fn new() -> MiniTokio {
//         MiniTokio {
//             tasks: VecDeque::new(),
//         }
//     }
//     /// Spawn a future onto the mini-tokio instance.
//     fn spawn<F>(&mut self, future: F)
//     where
//         F: Future<Output = ()> + Send + 'static,
//     {
//         self.tasks.push_back(Box::pin(future));
//     }
//     fn run(&mut self) {
//         let waker = task::noop_waker();
//         let mut cx = Context::from_waker(&waker);
//         // we are taking a task from the queue front
//         // polling it and see it's state
//         // if its pending push it to the back of the queue
//         while let Some(mut task) = self.tasks.pop_front() {
//             if task.as_mut().poll(&mut cx).is_pending() {
//                 self.tasks.push_back(task);
//             }
//         }
//     }
// }

// We want the executor to only run tasks when they are woken, and to do this,
// Mini Tokio will provide its own waker. When the waker is invoked, its associated
// task is queued to be executed. Mini-Tokio passes this waker to the future when
// it polls the future.

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }

    /// Initialize a new mini-tokio instance.
    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();

        MiniTokio { scheduled, sender }
    }

    /// Spawn a future onto the mini-tokio instance.
    ///
    /// The given future is wrapped with the `Task` harness and pushed into the
    /// `scheduled` queue. The future will be executed when `run` is called.
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        // this is possible see playground #1
        Task::spawn(future, &self.sender);
    }
}

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_secs(10);
        let future = Delay { when };

        let out = future.await;
        println!("got future complete in main thread")
    });

    println!("future spawned");
    mini_tokio.run();
    println!("above is a blocking call")
}
