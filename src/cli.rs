use crate::{merkle_error::MerkleError, merkle_tree::MerkleTree, proof_element::ProofElement, side::Side};
// use anyhow::{Context, Error, Result};
use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

/// CLI tool for tree-related operations
#[derive(Parser, Debug)]
#[command(name = "tree")]
#[command(about = "Tree command-line tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Merkle tree instance to hold the state
    #[arg(skip)]
    mktree: MerkleTree,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows the tree structure
    Show,
    /// Adds an element to the tree
    Add { 
        element: String, 

        #[arg(short = 'H', long = "hashed")]
        hashed: bool, 
    },
    /// Verifies a proof for a given hash
    Verify { hash: String, proof_file: PathBuf },
    /// Generates a proof for a given hash.
    Proof { hash: String },
    /// Builds a tree with the provided elements.
    Build { 
        elements: Vec<String>,

        #[arg(short = 'H', long = "hashed")]
        hashed: bool,
    },
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            command: Commands::Show, // Placeholder, will be overridden by parsing
            mktree: MerkleTree::new_empty(),
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
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

            // Execute the command and handle any errors
            if let Err(e) = self.execute_command(command) {
                println!("Error: {}", e);
            }
        }

        Ok(())
    }

    fn execute_command(&mut self, command: Commands) -> Result<(), MerkleError> {
        match command {
            Commands::Show => {
                println!("{}", self.mktree);
            }
            Commands::Add { element, hashed } => {
                let custom_message = if !hashed { "hashed and " } else {""};
                self.mktree.add(element.clone(),hashed)?;
                println!("Element '{}' {}added to the tree", element, custom_message);
            }
            Commands::Verify { hash, proof_file } => {
                let proof = parse_proof(proof_file)?;

                let result = self.mktree.verify(hash, proof)?;
                if result {
                    println!("Verification successful. Correct proof for the given element.");
                } else {
                    println!("Verification failed. Incorrect proof or element.");
                }
            }
            Commands::Proof { hash } => {
                let proof = self.mktree.gen_proof(hash.clone())?;
                println!("Generated proof:");
                for element in proof {
                    println!("  {} - {}", element.hash, element.side);
                };
            }
            Commands::Build { elements, hashed } => {
                let custom_message = if !hashed { "hashes of " } else {""};
                self.mktree = MerkleTree::build(elements.clone(), hashed)?;
                println!("Tree built with {}elements {:?}", custom_message, &elements);
            }
        }
        Ok(())
    }
}

/// Parses the proof file into a list of 'ProofElement's
fn parse_proof(proof_file: PathBuf) -> Result<Vec<ProofElement>, MerkleError> {
    let file = File::open(proof_file).map_err(|e| MerkleError::ParsingError(e.to_string()))?;
    let reader = BufReader::new(file);

    let mut proof = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| MerkleError::ParsingError(e.to_string()))?;
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 2 || (parts[1] != "left" && parts[1] != "right") {
            return Err(MerkleError::ParsingError(format!("Incorrect format in line: {}",line)));
        }
        let hash = parts[0].to_string();
        let side = if parts[1] == "left" {Side::Left} else {Side::Right};
        let proof_elem = ProofElement { hash, side };
        proof.push(proof_elem);
    }

    Ok(proof)
}
