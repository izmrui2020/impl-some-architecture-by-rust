//
use tokio::sync::mpsc::Receiver;

pub struct SubCenter {
    receiver: Receiver<i128>,
    store: i128,
}

impl SubCenter {
    pub fn new(rx: Receiver<i128>) -> SubCenter {
        

        SubCenter {
            receiver: rx,
            store: 0,
        }
    }
    
    pub async fn on_data(&mut self) {
        while let Some(v) =  self.receiver.recv().await {
            println!("get data from data center and data = {}", &v);
            self.store = v;
        }
    }
}