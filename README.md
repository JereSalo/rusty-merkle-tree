# Rusty Merkle Tree
Implementation of a [Merkle Tree](https://www.bitpanda.com/academy/en/lessons/everything-you-need-to-know-about-merkle-trees/) in Rust.

## Setup
*Clone* the repository  
*Enter* to the repository's folder  
*Run* ```cargo run```  
**Enjoy!**

## Commands
```tree build <elements> [--hashed]``` – Builds the tree with the given elements. Use the ```--hashed``` flag if the elements are already hashed.  
```tree add <element> [--hashed]``` – Adds an element to the tree. Use the ```--hashed``` flag if the element is already hashed.  
```tree show``` – Displays the tree structure on screen.  
```tree verify <hash> <proof_file>``` – Verifies a if a proof is correct for a given hash in the tree.  
```tree proof <hash>``` – Generates a proof for a given hash.  
```tree --help``` – Prints help information.  
```q``` – Exits the program.

### Proof File
```.csv``` file with format *hash;side*  
See example in folder **examples**
