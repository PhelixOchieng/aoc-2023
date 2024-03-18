use std::io;

mod one;

fn main() -> io::Result<()> {
    one::invoke()?;

    Ok(())
}
