//
use anyhow::Result;
use strum::IntoEnumIterator;
use std::{collections::HashMap, fmt::Debug};
use crate::update::Instance;

use super::special::SpecialTask;

use super::enum_store::{OrderIndicator};

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
    order_group: HashMap<OrderIndicator, O>,
    specific: Box<S>,
}

impl<L, O, S> OverwriteManager<L, O, S>
where
    L: Send + List + 'static + Debug + Default,
    O: Send + Order + 'static + Debug + Default,
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