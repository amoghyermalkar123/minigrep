use crossbeam::channel;
use futures::task;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::boxed::Box;

// The `Mutex` is to make `Task` implement `Sync`. Only
// one thread accesses `future` at any given time. The
// `Mutex` is not required for correctness. Real Tokio
// does not use a mutex here, but real Tokio has
// more lines of code than can fit in a single tutorial
// page.
pub struct Task {
    // we use box because our futures go the heap
    // we wrap the heap memory inside a Pin type to ensure
    // memory validity and reliability.
    // finally we make the entire memory block thread-safe with
    // a mutex wrap
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

use futures::task::ArcWake;

impl ArcWake for Task {
    // TODO: the following schedule() call is a result of deref coercion
    // research and experiment more on this concept (convo in discord aswell :)
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl Task {
    // waking call? called when task/ future is awoken and ready to be polled?
    fn schedule(self: &Arc<Self>) {
        // re-send the task, this task is not new it was previously onboarded
        // TODO: research on the self.clone()
        self.executor.send(self.clone());
    }
}

impl Task {
    // send our tasks on the channel for the executor to start polling
    pub fn spawn<F>(future : F, sender_channel: &channel::Sender<Arc<Task>>) 
    // send trait is important because we take only those futures that have this trait
    // since it allows for passing it between threads safely
    where F : Future<Output = ()> + Send + 'static,
    {
        let new_task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender_channel.clone(),
        });
        // send our newly onboarded task to the channel
        let _ = sender_channel.send(new_task);
    }
    // poll poll's the underlying wrapped future
    // thread-safe polling
    // this poll itself is called by our executor
    pub fn poll(self: &Arc<Self>) {
        // the self.clone() actually clones `Task` type, the actual self
        // TODO: resolve this fucking confusion (is this deref coercion?)
        // waker is a handle for waking up a task.
        let waker = task::waker(self.clone());
        // Context provides contextual data present for every task
        // including a handle for waking up the task.
        let mut cx = Context::from_waker(&waker);

        // acquire the lock, unwrap the future from the heap memory for further usage
        let mut future = self.future.try_lock().unwrap();

        // borrow future as mutable
        let _ = future.as_mut().poll(&mut cx);
    }
}
