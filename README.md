# Rusty Merkle Tree

An implementation of a [Merkle Tree](https://www.bitpanda.com/academy/en/lessons/everything-you-need-to-know-about-merkle-trees/) in Rust.

## Setup

1. **Clone** the repository.
2. **Navigate** to the repository's folder.
3. **Run** the application with:

    ```bash
    cargo run
    ```

4. **Enjoy!**

## Commands

- `tree build <elements> [--hashed]`  
  Builds the tree with the given elements. Use the `--hashed` flag if the elements are already hashed.

- `tree add <element> [--hashed]`  
  Adds an element to the tree. Use the `--hashed` flag if the element is already hashed.

- `tree show`  
  Displays the tree structure on screen.

- `tree verify <hash> <proof_file>`  
  Verifies if a proof is correct for a given hash in the tree. Replace `<hash>` with the actual hash value.

- `tree proof <hash>`  
  Generates a proof for a given hash. Replace `<hash>` with the actual hash value.

- `tree --help`  
  Prints help information.

- `q`  
  Exits the program.

### Examples

- `tree build a b c d`  
  Builds a tree hashing elements 'a', 'b', 'c', and 'd'.

- `tree build hash1 hash2 --hashed`  
  Builds a tree with the provided hashes (without rehashing them). Replace `hash1` and `hash2` with actual hash values.

- `tree add a`  
  Adds the hash of element 'a' to the tree.

- `tree add hash --hashed`  
  Adds the provided hash directly to the tree. Replace `hash` with the actual hash value.

- `tree verify hash ./examples/proof_a.csv`  
  Verifies the proof for a given hash using the provided proof file. Replace `hash` with the actual hash value.

### Proof File

The proof file should be a `.csv` with the format `hash;side`.  
See the example in the **examples** folder.
