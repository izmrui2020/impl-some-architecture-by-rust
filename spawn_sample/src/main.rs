fn main() {
    println!("Hello, world!");
}

// tokio = {version = "1.15.0", features = ["sync", "rt", "macros"]}
// async-trait = "0.1.52"
// anyhow = "1.0.52"
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;


#[async_trait]
pub trait Interface {
    fn create(&mut self);
    // ここにasync           ここに'staticが必要
    async fn target_method(&'static mut self) -> Result<()>;
}

pub trait Fuga {
    fn hogehoge(&self);
    fn update(&mut self) -> Result<()>;
    fn down(&mut self) -> Result<()>;
}

pub struct Hoge<T>
where
    T: Fuga + Send + Default + 'static,
{
    store: HashMap<String, Box<T>>,
}

// tokio::spawn()内で"?"が使えなくて、面倒なので、外部に関数を切り出しています。
async fn target_process_async<T>(
    target: &'static mut Box<T>,
    tx: Sender<Result<()>>,
) -> anyhow::Result<()>
where
    T: Fuga + Send + Default + 'static,
{
    let res = target.update();
    tx.send(res).await?;

    let res = target.down();
    tx.send(res).await?;

    Ok(())
}

// Subの実装がわからなかったので、勘でジェネリクスに置き換えています。
#[async_trait]
impl<T> Interface for Hoge<T>
where
    T: Fuga + Send + Default + 'static,
{
    fn create(&mut self) {
        for i in 0..10 {
            let data = T::default();

            self.store.insert(i.to_string(), Box::new(data));
        }
    }

    // エラーが複数種類出て面倒なので、anyhow::Resultを使っています。
    async fn target_method(&'static mut self) -> anyhow::Result<()> {
        //error at target_method(&mut self)
        let (tx, mut rx) = mpsc::channel(32);

        // エラーハンドリング用の配列
        let mut handles = vec![];
        self.store.iter_mut().for_each(|(_, target)| {
            let clone_tx1 = tx.clone();

            let task1 = tokio::spawn(async move {
                // 別々にtaskを作ると、target: &mut Box<T>が同時に2つ必要になるので、ボローチェッカーを通過できないです。
                // この場合は、素直に逐次処理を書くのがシンプルな解決方法だと思います。
                let res = target.update();
                match clone_tx1.send(res).await {
                    Err(err) => anyhow::bail!(err),
                    _ => {}
                }
                // 形が変わりすぎると差がわかりにくいので、適応していませんが、cargo clippyすると、↑は
                // if let Err(err) = clone_tx1.send(res).await { anyhow::bail!(err) }
                // と書くことをおすすめされます。

                let res = target.down();
                match clone_tx1.send(res).await {
                    Err(err) => anyhow::bail!(err),
                    _ => {}
                }

                Ok(())

                // エラーハンドリングが面倒なので、外部の関数に切り出しても良いです。
                // 以下の関数は33行目あたりで実装しています。
                // target_process_async(target, clone_tx1).await
            });

            handles.push(task1);
        });
        while let Some(v) = rx.recv().await {
            println!("hgoe:{:?}", &v);
        }

        // 終わっているはずのタスクを明示的にawaitして結果(Errが帰ってないか)を確認
        // while let Some...で全部終わっているはずなので、時間はかからないはず
        // 一つ目の?でJoinError(タスク内でPanicしていないか)、
        // 二つ目の?でSendError(受信側のチャンネルが閉じていないか)を伝播させています。
        for handle in handles {
            handle.await??;
        }

        Ok(())
    }
}
