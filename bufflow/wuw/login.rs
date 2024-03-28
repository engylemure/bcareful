use std::io;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "secret";
    // let mut user = String::new();
    let mut user: [u8; 5] = [0; 5];
    print!("username:");
    io::stdout().flush()?;
    io::stdin().read(&mut user)?;
    let mut password: [u8; 100] = [0; 100];
    print!("password:");
    io::stdout().flush()?;
    io::stdin().read(&mut password)?;
    if expected
        == std::str::from_utf8(&password)?
            .trim_end_matches(|c| c == '\0')
            .trim()
    {
        println!("wuw: authenticated");
    } else {
        let user = std::str::from_utf8(&user)?
            .trim_end_matches(|c| c == '\0')
            .trim();
        println!("wuw: wrong password for the '{user}' expected a different secret");
    }
    Ok(())
}
