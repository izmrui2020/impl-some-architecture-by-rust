//
use anyhow::Result;
use async_trait::async_trait;
use super::overwrite::List;

#[derive(Debug, Default, Clone)]
pub struct ListA {
    store: Vec<String>,
}

impl ListA {
    fn new() -> Self {
        Self {
            store: Vec::new(),
        }
    }
}

#[async_trait]
impl List for ListA {
    async fn check(&mut self) -> Result<()> {
        self.store.push("hogehog".to_string());

        self.store.push("fugafuga".to_string());
        self.store.push("fugafuga".to_string());
        self.store.push("hogehog".to_string());

        Ok(())
    }

}

#[derive(Debug, Default, Clone)]
pub struct ListB {
    store: Vec<String>,
}

impl ListB {
    fn new() -> Self {
        Self {
            store: Vec::new(),
        }
    }
}

#[async_trait]
impl List for ListB {
    async fn check(&mut self) -> Result<()> {
        self.store.push("hogehog".to_string());

        self.store.push("fugafuga".to_string());
        self.store.push("fugafuga".to_string());
        self.store.push("hogehog".to_string());

        Ok(())
    }

}