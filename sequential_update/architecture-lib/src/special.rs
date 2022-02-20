//
use anyhow::Result;
use std::collections::HashMap;
use super::overwrite::Special;

#[derive(Debug, Clone, Default)]
pub struct SpecialTask {
    store: HashMap<String, String>,
}

impl SpecialTask {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Special for SpecialTask {
    async fn hoge(&mut self) -> Result<()> {
        self.store.insert(
            "hogehoge".to_string(),
            "fugafuga".to_string(),
        );

        Ok(())
    }
}