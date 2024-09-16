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
```tree verify <hash> <proof_file>``` – Verifies if a proof is correct for a given hash in the tree.  
```tree proof <hash>``` – Generates a proof for a given hash.  
```tree --help``` – Prints help information.  
```q``` – Exits the program.

### Examples
```tree build a b c d``` – Builds a tree hashing elements 'a', 'b', 'c' and 'd'.  
```tree build ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb 3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d --hashed``` – Builds a tree with the provided hashes (without rehashing them).  
```tree add a``` – Adds hash of element 'a' to the tree  
```tree add ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb --hashed``` – Adds directly to the tree the provided hash.  
```tree verify ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb ./examples/proof_a.csv```  
```tree proof ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb```  
<br>
### Proof File
```.csv``` file with format *hash;side*  
See example in folder **examples**
