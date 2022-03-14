//
use anyhow::Result;
use tokio::sync::mpsc;

mod app;
mod draw;
use self::app::app_center::App;


#[tokio::main]
async fn main() -> Result<()> {
    let (data_tx, mut data_rx) = mpsc::channel(1);
    let (signal_tx, signal_rx) = mpsc::channel(1);

    let mut app = App::new(data_tx, signal_rx);

    let data_receive_task = tokio::spawn(async move {
        app.send_data().await;
    });

    let ui_task = tokio::spawn(async move {
        'out: loop {
            'inner: loop {
                let now = std::time::Instant::now();

                while let Some(v) = data_rx.recv().await {
                    println!("data: {}", &v);

                    if now.elapsed() > std::time::Duration::from_secs(10) {
                        break 'inner;
                    }
                }
            }

            signal_tx.send(1).await;
            //send signal
        }
    });

    data_receive_task.await?;

    Ok(())
}
