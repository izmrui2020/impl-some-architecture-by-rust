//
use tokio::sync::mpsc::{self, Sender, Receiver};

pub struct App {
    send_tx: Sender<i128>,
    send_task: tokio::task::JoinHandle<()>,
    inner_rx: Receiver<i128>,
    signal_rx: Receiver<i128>,
    store: i128
}

impl App {
    pub fn new(to_draw_tx: Sender<i128>, signal_rx: Receiver<i128>) -> App {

        let (tx, rx) = mpsc::channel(1);

        let task = tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2));
            let data = 12;
            tx.send(data).await;
        });

        App {
            send_tx: to_draw_tx,
            send_task: task,
            signal_rx,
            inner_rx: rx,
            store: 0,
        }
    }

    pub async fn send_data(&mut self) {
        'outer: loop { 
            match self.inner_rx.recv().await {
            
            Some(v) => {
                self.store += v;
                if let Err(e) = self.send_tx.clone().send(self.store).await {
                    println!("failed send at data center to other: {}", &e);
                    break;
                }
                loop {
                    match self.signal_rx.try_recv() {
                        Ok(v) => break 'outer,
                        Err(e) => break,
                    }
                }
            }
            None => {}
            }
        }
    }
}