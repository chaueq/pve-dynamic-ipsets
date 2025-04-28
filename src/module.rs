use std::{sync::mpsc::Sender, thread};

pub struct Module<ReqType> {
    handle: thread::JoinHandle<()>,
    sender: Sender<ReqType>
}

impl<ReqType> Module<ReqType> {
    pub fn new(handle: thread::JoinHandle<()>, sender: Sender<ReqType>) -> Self {
        Self {
            handle,
            sender
        }
    }

    pub fn join(self) {
        let _ = self.handle.join();
    }

    pub fn send(&self, msg: ReqType) {
        self.sender.send(msg).unwrap();
    }
}