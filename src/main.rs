use anyhow::Result;
use clap::{Parser, Subcommand};
use merkle_tree::merkle_tree::MerkleTree;
use std::io::{self, Write};

/// CLI tool for tree-related operations
#[derive(Parser, Debug)]
#[command(name = "tree")]
#[command(about = "Tree command-line tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows the tree structure
    Show,

    /// Adds an element to the tree
    Add {
        /// The element to add
        element: String,
    },

    /// Verifies a proof for a given hash
    Verify {
        /// The hash to verify
        hash: String,
    },

    /// Generates a proof for a given hash.
    Proof {
        /// The hash to generate proof for
        hash: String,
    },

    /// Builds a tree with the provided elements.
    Build {
        /// Elements to build the tree with
        elements: Vec<String>,
    },
}

fn main() -> Result<()> {
    let mut mktree: MerkleTree = MerkleTree::new_empty();
    loop {
        // Display prompt
        print!("tree> ");
        io::stdout().flush()?; // Flush prompt to the terminal

        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Trim the input and split it into parts
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        // Exit condition
        if input == "exit" {
            break;
        }

        // Skip empty input
        if parts.is_empty() {
            continue;
        }

        // Simulate command-line arguments parsing by passing `parts` to `clap`
        let command = match Cli::try_parse_from(parts.iter()) {
            Ok(cli) => cli.command,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        // Match on the parsed subcommand
        match command {
            Commands::Show => {
                println!("{}", mktree);
            }
            Commands::Add { element } => {
                println!("Element '{}' hashed and added to the tree", element);
                mktree.add_element(element)?;
            }
            Commands::Verify { hash } => {
                println!("Verifying hash: {}", hash);
                verify_hash(&hash);
            }
            Commands::Proof { hash } => {
                println!("Generated proof:");
                let proof = mktree.gen_proof(hash)?;
                for element in proof{
                    let hash = element.hash;
                    let position = if element.left { "left" } else { "right" };
                    println!("  {} - {}", hash, position);
                }
            }
            Commands::Build { elements } => {
                println!("Tree built with elements {:?}", &elements);
                mktree = MerkleTree::build(elements)?;
            }
        }
    }

    Ok(())
}

// Mock functions for the subcommands
fn show_tree() {
    println!("Tree structure is shown here.");
}

fn add_element(element: &str) {
    println!("Element {} has been added to the tree.", element);
}

fn verify_hash(hash: &str) {
    println!("Hash {} has been verified.", hash);
}

fn generate_proof(hash: &str) {
    println!("Proof generated for hash {}.", hash);
}
