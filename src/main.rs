use std::io;

use merkle_tree::cli;

fn main() -> Result<(), io::Error> {
    let mut cli = cli::Cli::new();
    cli.run()
}
