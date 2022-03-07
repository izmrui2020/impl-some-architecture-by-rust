//
use anyhow::Result;

mod utils;
mod draw;

use self::draw::draw::Termondayo;


fn main() -> Result<()> {

    let mut termon = Termondayo::new();

    termon.terminal.clear()?;
    termon.terminal.hide_cursor()?;

    loop {
        termon.draw();
    }
    
    termon.terminal.clear()?;
    termon.terminal.show_cursor()?;

    Ok(())
}
