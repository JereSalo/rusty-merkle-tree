use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use crate::{merkle_tree::MerkleTree, proof_element::ProofElement};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};


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
    Add { element: String },
    /// Verifies a proof for a given hash
    Verify { hash: String, proof_file: PathBuf },
    /// Generates a proof for a given hash.
    Proof { hash: String },
    /// Builds a tree with the provided elements.
    Build { elements: Vec<String> },
}

pub fn run() -> Result<()> {
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

        // Parse command
        let command = match Cli::try_parse_from(parts.iter()) {
            Ok(cli) => cli.command,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        if let Err(e) = execute_command(command, &mut mktree) {
            println!("Error: {}", e);
        }
    }

    Ok(())
}

/// Execute the appropriate action for each command
fn execute_command(command: Commands, mktree: &mut MerkleTree) -> Result<()> {
    match command {
        Commands::Show => {
            println!("{}", mktree);
        }
        Commands::Add { element } => {
            println!("Element '{}' hashed and added to the tree", element);
            mktree.add_element(element)?;
        }
        Commands::Verify { hash, proof_file } => {
            let proof = parse_proof(proof_file)?;

            let result = mktree.verify(hash, proof)?;
            if result {
                println!("Verification successful. Correct proof for the given element.");
            } else {
                println!("Verification failed. Incorrect proof or element.");
            }
        }
        Commands::Proof { hash } => {
            let proof = mktree.gen_proof(hash.clone())?;
            println!("Generated proof:");
            for element in proof {
                let hash = element.hash;
                let position = if element.left { "left" } else { "right" };
                println!("  {} - {}", hash, position);
            }
        }
        Commands::Build { elements } => {
            println!("Tree built with elements {:?}", &elements);
            *mktree = MerkleTree::build(elements)?;
        }
    }
    Ok(())
}

/// Parses the proof file into a list of `ProofElement`s
fn parse_proof(proof_file: PathBuf) -> Result<Vec<ProofElement>, Error> {
    let file = File::open(proof_file)?;
    let reader = BufReader::new(file);

    let mut proof = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 2 || (parts[1] != "left" && parts[1] != "right") {
            return Err(anyhow::anyhow!(
                "ERROR: Invalid proof element format - {}",
                line
            ));
        }
        let hash = parts[0].to_string();
        let left = parts[1] == "left";
        let proof_elem = ProofElement { hash, left };
        proof.push(proof_elem);
    }

    Ok(proof)
}
