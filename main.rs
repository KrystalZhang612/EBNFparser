use std::env;
mod stage1;
mod stage3; 
mod stage2_token; 
mod stage2_scanner; 
mod xhtml_out; 

fn main() {

	env::set_var("RUST_BACKTRACE", "1");
	//Testing Stage 1: 
	stage1::CStreamTest(); 
	//should print "Stage 1 successfully approached! All CStream tests passed!"
	//Stage 1 approached. 
	
	//Testing Stage 2:
	stage2_scanner::ScannerTest(); 
	//should print "Stage 2 successfully approached! All characters in example1.x file tests passed!"
	//Stage 2 approached 
	
	//Testing Stage 3:
	stage3::test(); 
	stage3::test_run(); 
	
	//outputting xhtml file
	xhtml_out::test_run();
	//should be converting .x file into .xhtml file.	
	
}
