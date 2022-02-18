//
use anyhow::Result;


#[async_trait::async_trait]
pub trait Instance {
    async fn init(&mut self) -> Result<()>;
    async fn update(&mut self) -> Result<()>;
    async fn specific_function(&mut self) -> Result<()>;
}

pub struct Manager<T> {
    instance: T,
}

impl<T> Manager<T>
where
    T: Send + Instance + 'static
{
    pub fn new(instance: T) -> Self {
        Self {
            instance
        }
    }

    pub async fn create_manager(&mut self) -> Result<()> {
        self.instance.init().await?;
        Ok(())
    }

    pub async fn update_manager(&mut self) -> Result<()> {
        self.instance.update().await?;
        Ok(())
    }

    pub async fn specific_manager(&mut self) -> Result<()> {
        self.instance.specific_function().await?;
        Ok(())
    }
}