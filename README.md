# EBNFparser
A parser project following the given EBNF grammars in Rust. 

Stage 1: 
CStream works properly
method to run to test Stage 1:
go to the src. 
open main.rs file and stage1.rs, testing.txt and example1.x file (make sure they are all in the same path)
start running the main file by removing mod stage1 and stage1::test() comments
it runs properly and should output "Stage 1 successfully approached! All CStream tests passed!"
which indicates that the Stage 1 was approached successfully. 

Stage 2: 
Both Tokens and Scanner runs 
method to run to test Stage 2:
go to the src
open main.rs file and tokens_stage2.rs and scanner_stage2.rs files, along with all given example files from 1-8
(make sure they are all in the same path or directory)
start running the main file by removing mod stage2_token, mod scanner_stage2 and scanner_stage2::test() comments 
it runs properly and should output "Stage 2 successfully approached! All characters in example1.x file tests passed!"
which indicates that the Stage 2 was approached successfully. 

Stage 3 and Stage 4:
custom_error crate was included into the dependecies within the Cargo.toml. 
parser worked properly, I used macro for specificing the EBNF grammar. 

To run our program, run these in terminal: 
cd projectfoldername
cargo update 
cargo build 
cargo run 
or cargo run exampleN.x 
it should convert .x file into their corresponding .xhtml file. 
