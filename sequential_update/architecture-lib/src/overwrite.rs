//
use anyhow::Result;
use strum::IntoEnumIterator;
use tokio_stream::StreamExt;
use std::{collections::HashMap, fmt::Debug};
use crate::enum_store::ListIndicator;
use crate::update::Instance;
use super::enum_store::{OrderIndicator, OrderKind};

use super::special::SpecialTask;

#[async_trait::async_trait]
pub trait List {
    async fn check(&mut self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait Order {
    async fn sort(&mut self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait Special {
    async fn hoge(&mut self) -> Result<()>;
}

pub trait ForOrderKind<T> {
    fn create_instance(&self) -> OrderIndicator;
}

#[derive(Default, Clone)]
pub struct OverwriteManager {
    list_group: HashMap<String, ListIndicator>,
    order_group: HashMap<OrderKind, OrderIndicator>,
    specific: SpecialTask,
}

impl OverwriteManager {
    pub fn new() -> Self {
        Self {
            list_group: HashMap::new(),
            order_group: HashMap::new(),
            specific: SpecialTask::default(),
        }
        
    }

}

#[async_trait::async_trait]
impl Instance for OverwriteManager {
    async fn init(&mut self) -> Result<()> {

        let stream = tokio_stream::iter(OrderKind::iter());

        tokio::pin!(stream);


        while let Some(v) = stream.next().await {
            println!("this is {:?}", &v);

            self.order_group.entry(v.clone())
                .or_insert_with(|| {
                    v.create_instance()
                })
                .sort().await?;

        }

        for i in self.order_group.iter() {
            println!("instance: {:?}", i);
        }

        // for (key, val) in self.order_group.iter_mut() {
        //     val.sort();
        // }

        Ok(())
    }


    async fn update(&mut self) -> Result<()> {

        

        Ok(())
    }


    async fn specific_function(&mut self) -> Result<()> {
        Ok(())
    }

}