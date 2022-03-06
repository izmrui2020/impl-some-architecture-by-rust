//
use std::sync::Arc;

use eyre::Result;
use std::cell::RefCell;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
