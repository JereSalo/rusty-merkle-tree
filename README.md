# Rusty Merkle Tree
Implementation of a [Merkle Tree](https://brilliant.org/wiki/merkle-tree/) in Rust.

## Setup
*Clone* the repository  
*Enter* to the repository's folder  
*Run* ```cargo run```  
**Enjoy!**

## Commands
```tree build <elements>``` – Builds the tree with the given elements.  
```tree add <element>``` – Adds element to tree  
```tree show``` – Shows tree  
```tree verify <hash> <proof_file>``` – Verifies a proof for a given hash  
```tree proof <hash>``` – Generates a proof for a given hash

### Proof File
```.csv``` file with format *hash;side*  
See example in folder **examples**
