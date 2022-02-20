//
use anyhow::Result;
use strum::IntoEnumIterator;
use tokio_stream::StreamExt;
use std::{collections::HashMap, fmt::Debug};
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

#[derive(Default, Clone)]
pub struct OverwriteManager<L, O, S>
{
    list_group: HashMap<String, L>,
    order_group: HashMap<OrderKind, O>,
    specific: Box<S>,
}

impl<L, O, S> OverwriteManager<L, O, S>
where
    L: Send + List + 'static + Debug + Default,
    O: Send + Order + 'static + Debug + Default + Clone,
    S: Send + Special + 'static + Debug + Default,
{
    fn new() -> Self {
        Self {
            list_group: HashMap::new(),
            order_group: HashMap::new(),
            specific: Box::default(),
        }
        
    }

}

#[async_trait::async_trait]
impl<L, O, S>  Instance for OverwriteManager<L, O, S>
where
    L: Send + List + 'static + Debug + Default,
    O: Send + Order + 'static + Debug + Default,
    S: Send + Special + 'static + Debug + Default,
{
    async fn init(&mut self) -> Result<()> {

        let mut stream = tokio_stream::iter(OrderKind::iter());

        tokio::pin!(stream);

        while let Some(v) = stream.next().await {
            self.order_group.entry(v)
                .or_insert_with(|| {
                    OrderIndicator::v.default()
                })
                .sort();

        }

        for (key, val) in self.order_group.iter_mut() {
            val.sort();
        }

        Ok(())
    }


    async fn update(&mut self) -> Result<()> {
        Ok(())
    }


    async fn specific_function(&mut self) -> Result<()> {
        Ok(())
    }

}