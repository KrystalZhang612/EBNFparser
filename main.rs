use std::env;
mod cstream_stage1;
mod parser_stage3; 
mod token_stage2; 
mod scanner_stage2; 
mod xhtml_out; 

fn main() {

	env::set_var("RUST_BACKTRACE", "1");
	//Testing Stage 1: 
	cstream_stage1::CStreamTest(); 
	//should print "Stage 1 successfully approached! All CStream tests passed!"
	//Stage 1 approached. 
	
	//Testing Stage 2:
	scanner_stage2::ScannerTest(); 
	//should print "Stage 2 successfully approached! All characters in example1.x file tests passed!"
	//Stage 2 approached 
	
	//Testing Stage 3:
	parser_stage3::test(); 
	parser_stage3::test_run(); 
	
	//outputting xhtml file
	xhtml_out::test_run();
	//should be converting .x file into .xhtml file.	
	
}
