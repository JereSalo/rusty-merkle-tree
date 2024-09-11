use merkle_tree::cli;

fn main() -> anyhow::Result<()> {
    let mut cli = cli::Cli::new();
    cli.run()
}
