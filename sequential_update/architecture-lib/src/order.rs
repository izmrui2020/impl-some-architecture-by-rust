//
use anyhow::Result;
use std::collections::HashMap;
use super::overwrite::Order;

#[derive(Default, Debug, Clone)]
pub struct OrderA {
    store: HashMap<String, String>,
}

#[async_trait::async_trait]
impl Order for OrderA {
    async fn sort(&mut self) -> Result<()>{
        self.store.insert(
            "hogehoge".to_string(),
            "fugafuga".to_string(),
        );

        Ok(())
    }

}

#[derive(Default, Debug, Clone)]
pub struct OrderB {
    store: HashMap<String, String>,
}

#[async_trait::async_trait]
impl Order for OrderB {
    async fn sort(&mut self) -> Result<()>{
        self.store.insert(
            "hogehoge".to_string(),
            "fugafuga".to_string(),
        );

        Ok(())
    }

}

#[derive(Default, Debug, Clone)]
pub struct OrderC {
    store: HashMap<String, String>,
}

#[async_trait::async_trait]
impl Order for OrderC {
    async fn sort(&mut self) -> Result<()>{
        self.store.insert(
            "hogehoge".to_string(),
            "fugafuga".to_string(),
        );

        Ok(())
    }

}
