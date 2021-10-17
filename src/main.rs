use crossbeam::channel;
use std::future::Future;
use std::sync::Arc;

mod task;

// this struct represents our mini tokio instance
struct MiniTokio {
    // send end
    sender: channel::Sender<Arc<task::Task>>,
    // receiver end
    receiver: channel::Receiver<Arc<task::Task>>,
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
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        // embed the future inside our task type
        // then spawn this task.
        task::Task::spawn(future, &self.sender)
    }

    // our task/ future receiver
    // this method is blocking in it's parent and is the core executor
    // it's responsible for indefinitely listen on the receiver channel
    // for tasks (futures) and poll them on receive first basis
    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            task.poll();
        }
    }
}

fn main() {}
