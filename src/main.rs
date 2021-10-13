use crossbeam::channel;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::time::Instant;
use std::sync::Mutex;

// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Task {
    // The `Mutex` is to make `Task` implement `Sync`. Only
    // one thread accesses `future` at any given time. The
    // `Mutex` is not required for correctness. Real Tokio
    // does not use a mutex here, but real Tokio has
    // more lines of code than can fit in a single tutorial
    // page.

    // TODO NEXT: FIND EXPLANATION FOR THIS TYPE
    tasks: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>
}

impl Task {
    fn spawn() {}
}

// our custom type which we will give a future trait
// so a type has a trait (how easy to understand :) !)
struct Delay {
    when: Instant,
}

impl Future for Delay {
    // dont go deep in for this right now
    type Output = &'static str;

    // poll function basically allows for an executor to poll this future 
    // towards further progress/ completion
    // it takes in a pinned generic mutable type and a context and returns
    // the future's status which is nothing but one of the states of the future's
    // finite state automata
    // context here is an anonymous lifetime
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// this struct represents our mini tokio instance
struct MiniTokio {
    // send end
    sender: channel::Sender<Arc<Task>>,
    // receiver end
    receiver: channel::Receiver<Arc<Task>>,
}

impl MiniTokio {
    // construct instance
    fn new() -> MiniTokio {
        let (s, r) = channel::unbounded();
        MiniTokio {
            sender: s,
            receiver: r,
        }
    }

    // mini-tokio instance spawns new tasks on the channel
    // which are received by the receiver and are concurrently polled on
    // so our SENDER IS spawn
    // and our RECEIVER IS run (defined below)
    fn spawn<F>(future: F)
    where
        F: Future<Output = ()> + Send,
    {
        // spawn
    }

    // our task/ future receiver
    // this method is blocking in it's parent and is the core executor
    // it's responsible for indefinitely listen on the receiver channel
    // for tasks (futures) and poll them on receive first basis
    fn run(&mut self) {
        while let Ok(task) = self.receiver.recv() {
            if task.poll().is_pending() {
                Poll::Pending;
            }
        }
    }
}

fn main() {}
