use std::io;
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Loop.
    loop {
        // Read.
        let mut rl = rustyline::Editor::<()>::with_config(
            rustyline::Config::builder()
                .build()
        );
        let line = rl.readline(">> ")?;
        rl.add_history_entry(line.clone());

        // Eval
        if line == "quit" {
            break;
        }

        // Print.
        println!("{}", line);
    }

    Ok(())
}
