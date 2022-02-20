//
use anyhow::Result;
use architecture_lib::overwrite::OverwriteManager;
use architecture_lib::update::{Instance, Manager};

#[tokio::main]
async fn main() -> Result<()> {
    
    let mut instance = OverwriteManager::new();

    let mut update = Manager::new(instance.clone());
    
    update.create_manager().await?;
    
    println!("finish program");
    
    Ok(())
}

