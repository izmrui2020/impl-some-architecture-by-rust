//
use anyhow::Result;
use tokio::sync::mpsc::{self, Sender, Receiver};

pub struct DataCenter {
    receive_task: tokio::task::JoinHandle<()>,
    to_draw_tx: Sender<i128>,
    data: i128,
    inner_rx: Receiver<i128>,
}

impl DataCenter {
    pub fn new(to_draw_tx: Sender<i128>) -> DataCenter {

        let (tx, rx) = mpsc::channel(1);

        let task = tokio::spawn(async move {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let data = 2_i128;
                println!("data: {}", &data);
                if let Err(e) = tx.send(data).await {
                    println!("failed at send error in data center");
                    break;
                }
            }
        });

        
        DataCenter {
            receive_task: task,
            to_draw_tx,
            data: 0,
            inner_rx: rx,
        }
    }

    pub async fn send_data(&mut self) {
        let now = std::time::Instant::now();

        while let Some(v) = self.inner_rx.recv().await {
            println!("receive side tx: {}", &v);
            self.data += v;

            if now.elapsed() > std::time::Duration::from_secs(3) {
                break;
            }

            println!("data center store: {}", self.data);

            if let Err(e) = self.to_draw_tx.clone().send(self.data).await {
                println!("failed send at data center to other: {}", &e);
                break;
            }
        }
    }
}