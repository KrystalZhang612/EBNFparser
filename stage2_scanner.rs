
//SCANNER 
//include the CStream crate
use crate::stage1::CStream; 
use crate::stage2_token::{Token, TokenType};


//Write a struct called Scanner that will tokenize
//the file read from Stage 1 into the token types defined in TokenType.


pub struct Scanner{
	
	//the converted string of the input x file after reading
	t:CStream, 
	//name the vector all_tokens and create it in the main function 
	pub all_tokens: Vec<Token>,
	
	//match the intended keywords and operators from TokenType elements
	//store into vectors of strings 
	op_matched: Vec<String>, 
	
	key_matched: Vec<String>, 
	
}


//Scanner struct implementation 

impl Scanner {
	//initializer read the input x file 
	pub fn new(xfilename: &str) -> Scanner{
		
		Scanner{
			
			t:CStream::new(xfilename), 
			
			all_tokens: Vec::new(), 
			
			//use Vec type here to access tokentype values by index
			//since it implements the Index trait. 
			//https://web.mit.edu/rust-lang_v1.26.0/arch/amd64_ubuntu1404/share/doc/rust/html/std/vec/struct.Vec.html
			//use iterating operations to collect as successful vector of strings values
			//https://doc.rust-lang.org/rust-by-example/error/iter_result.html
			
			op_matched:vec![
				"(", ",", ")", "{", "}", "=", 
			"==", "<", ">", "<=", ">=", "!=", 
			"+", "-", "*", "/",";",]
			.into_iter().map(|mystring| mystring.to_string())
			.collect::<Vec<String>>(), 
			//similarly to the valid key elements 
			key_matched:vec! [
				"unsigned", "char", "short", "int", "long", 
				"float", "double", "while", "if", "return", 
				"void", "main",
			]. into_iter().map(|mystring| mystring.to_string())
			.collect::<Vec<String>>(), 
		}
		
	}
	
	//get_next_token() function
	//use Option type to handle the optional token type here 
	//https://doc.rust-lang.org/std/option/
	pub fn next_token(&mut self)-> Option<Token>{
		
		//determine if there are more characters to read
		//check if reach the end of file 
		if !self.t.more_available(){
			//EOF
			return None; 
		}
	
		
		//create a vector of characters 
		//determine if the character at the first index position matches the operators 
		let mut seeking_for_matches = "".to_string();
		
		//iterate through the vector of chars 
		let first_op_char: Vec<char> = self.op_matched.iter().map(|mystring| mystring.chars().nth(0).unwrap()).collect(); 
		
		//if at EOF, conver the char at current pos into string
		//use is_whitespace() method to detect if the char has the any white spaces
		//https://doc.rust-lang.org/std/primitive.char.html
		//if not yet reach the EOF
		while self.t.more_available(){
			//convert the next character read into string type
			seeking_for_matches = self.t.get_next_char().to_string();
			//if the last char does not contain white spaces, break. 
			//if contains, skip 
			if !seeking_for_matches.chars().last().unwrap().is_whitespace(){
				break; 
			}
		}
		
		//infinitely loop through the vector until encounters EOF, whitespace properties or operators 
		//https://www.tutorialspoint.com/loop-keyword-in-rust-programming
		
		loop{
			//if more chars to read 
			if !self.t.more_available(){
				//check if last token is empty 
				//https://doc.rust-lang.org/nightly/std/index.html?search=is_empty
				//if not empty, create a token from the string 
				if !seeking_for_matches.is_empty()	{
					return Some(self.classify_tokens(&seeking_for_matches)); 
				}
				//EOF 
				return None; 
				
			}
			
			//move to next char 
			let next_char = self.t.peek_next_char(); 
			
			/*stop at operator chars but not the intended operators, non-operator and
			whitespaces the intended operators*/
			if !next_char.is_whitespace() && !first_op_char.contains(&next_char) 
			&& !(first_op_char.contains(&seeking_for_matches.chars().nth(0).unwrap())
				&& (next_char.is_alphanumeric() || next_char=='_'))
			{
					
				//with the next character, here to update the matches seeking 
				//use format! method here for error handling 
				//https://doc.rust-lang.org/std/macro.format.html
				seeking_for_matches = format!("{}{}", seeking_for_matches, self.t.get_next_char()); 
				}else{
					
					return Some(self.classify_tokens(&seeking_for_matches));
				
			}
			
		}
		
	}
	
	
	//create a token from the string to return the matched token 
	fn classify_tokens(&mut self, seeking_for_matches: &String) -> Token{
		
		//CASE 1: Invalid 
		//initializer of the token type as invalid tokentype
		let mut token_type = TokenType::Invalid; 
		
		//starting from the 1st char at pos 0 
		let first_char = seeking_for_matches.chars().nth(0).unwrap(); 
		
		//check the float constant 
		//first check if numeric 
		//https://www.educative.io/edpresso/how-to-check-if-a-character-in-rust-is-numeric
		//either puerly number or single dot 
		
		//CASE 2: intented Keywords 
		if self.key_matched.contains(&seeking_for_matches){
			
			token_type = TokenType::Keyword; 
		}
		
		//CASE 3: intended Operator 
		else if self.op_matched.contains(&seeking_for_matches){
			
			token_type = TokenType::Operator; 
			
		}
		
		//CASE 4: intented Integer Constant
		//similar to number_only case, obtain the token of integer only, which is numeric 
		else if seeking_for_matches.chars().into_iter().all(|mychar| mychar.is_numeric()){
			
			token_type = TokenType::IntConstant; 
		}
		
		//CASE 5: Float constant = both number and dot 
		else if first_char.is_numeric() || first_char == '.'{
			
			//if only a dot without the numeric number (return true contents)
			//use count() method to return the number of elements in the iterator without counting in the loop 
			//return the numeric value of object pointed by a pointer integer variable only after filtering 
			let dot_only = seeking_for_matches.chars().into_iter().filter(|mychar| *mychar == '.').count() == 1; 
		
			//if only number .(return true contents ) 
			//use filter() method to filter out only the number at this char position in the iteration
			//https://docs.rs/filters/latest/filters/
			let number_only = seeking_for_matches.chars().into_iter().filter(|mychar| *mychar != '.').all(|mychar| mychar.is_numeric()); 
	
			//if both dot and number -> matched float constant type 
			if number_only && dot_only{
				
				token_type = TokenType::FloatConstant; 
			}
			
		}
		
		
		//CASE 6: intended Identifier 
		//use built-in function is_alphabetic() to test if byte is ASCII alphabetic: i.e. A-Z, a-z
		//https://docs.rs/nom/4.0.0/nom/fn.is_alphabetic.html
		//aka names of variables and functions etc. 
		//since Rust commonly uses underscore as prefix as reserved identifiers as well
		//either prefix/1st char alphabetic or underscore 
		else if first_char.is_alphabetic() || first_char == '_' {
			
			token_type = TokenType::Identifier; 
		}
		
		//save tokens and obtain a copy of everything 
		
		let token = Token::new(
			
			token_type, 
			
			//return the memory address 
			&seeking_for_matches, 
			
			self.t.line_num, 
			//indexing up 
			self.t.char_pos + 1 - seeking_for_matches.len() as i32, 
			
			
		); 
		
		//use clone() to get copies of all tokens 
		self.all_tokens.push(token.clone()); 
		
		//Scanner returned a vector of tokens! 
		
		return token; 
		
	}
	
	
	//run to test scanner 
	pub fn run(&mut self) -> &mut Scanner {
		
		while self.next_token() != None{}
		//called get_next_token() function to return the next token 
		//as read from the .x file. the token type is returned. 
		self
	}
}
	
//run_test 	
	
pub fn run_test() {
	let mut scan = Scanner::new("example1.x");
	println!("{:?}", scan.run().all_tokens);
}



//test if scanner analyzed all different token cases properly
//also properly handle the error cases 

//here we are already given the XTHML output case for example1.x and example2.x  
//so test the example1.x here 

//tokentype, 
//row_num starting with 1 as the top most line_num, 
//char_pos starting with left most as 0 

pub fn ScannerTest(){
	
	//also check if successfully skip all whitespaces 
	let all_token_tests = vec![(
		
		"example1.x",
		vec![
			Token::new(TokenType::Keyword, "float", 1, 0),
			Token::new(TokenType::Identifier, "Foo", 1, 6),
			Token::new(TokenType::Operator, "(", 1, 9),
			Token::new(TokenType::Keyword, "int", 1, 10),
			Token::new(TokenType::Identifier, "val", 1, 14),
			Token::new(TokenType::Operator, ")", 1, 17),
			Token::new(TokenType::Operator, ";", 1, 18),
			Token::new(TokenType::Keyword, "void", 3, 0),
			Token::new(TokenType::Keyword, "main", 3, 5),
			Token::new(TokenType::Operator, "(", 3, 9),
			Token::new(TokenType::Operator, ")", 3, 10),
			Token::new(TokenType::Operator, "{", 3, 11),
			Token::new(TokenType::Keyword, "float", 4, 4),
			Token::new(TokenType::Identifier, "Value", 4, 10),
			Token::new(TokenType::Operator, ";", 4, 15),
			Token::new(TokenType::Identifier, "Value", 6, 4),
			Token::new(TokenType::Operator, "=", 6, 10),
			Token::new(TokenType::Identifier, "Foo", 6, 12),
			Token::new(TokenType::Operator, "(", 6, 15),
			Token::new(TokenType::IntConstant, "7", 6, 16),
			Token::new(TokenType::Operator, ")", 6, 17),
			Token::new(TokenType::Operator, ";", 6, 18),
			Token::new(TokenType::Operator, "}", 7, 0),
			Token::new(TokenType::Keyword, "float", 9, 0),
			Token::new(TokenType::Identifier, "Foo", 9, 6),
			Token::new(TokenType::Operator, "(", 9, 9),
			Token::new(TokenType::Keyword, "int", 9, 10),
			Token::new(TokenType::Identifier, "val", 9, 14),
			Token::new(TokenType::Operator, ")", 9, 17),
			Token::new(TokenType::Operator, "{", 9, 18),
			Token::new(TokenType::Keyword, "float", 10, 4),
			Token::new(TokenType::Identifier, "TestFloat", 10, 10),
			Token::new(TokenType::Operator, "=", 10, 20),
			Token::new(TokenType::FloatConstant, "1.4", 10, 22),
			Token::new(TokenType::Operator, ";", 10, 25),
			Token::new(TokenType::Keyword, "while", 12, 4),
			Token::new(TokenType::Operator, "(", 12, 9),
			Token::new(TokenType::Identifier, "val", 12, 10),
			Token::new(TokenType::Operator, ">", 12, 14),
			Token::new(TokenType::IntConstant, "0", 12, 16),
			Token::new(TokenType::Operator, ")", 12, 17),
			Token::new(TokenType::Operator, "{", 12, 18),
			Token::new(TokenType::Identifier, "TestFloat", 13, 8),
			Token::new(TokenType::Operator, "=", 13, 18),
			Token::new(TokenType::Identifier, "TestFloat", 13, 20),
			Token::new(TokenType::Operator, "*", 13, 30),
			Token::new(TokenType::Identifier, "TestFloat", 13, 32),
			Token::new(TokenType::Operator, ";", 13, 41),
			Token::new(TokenType::Identifier, "val", 14, 8),
			Token::new(TokenType::Operator, "=", 14, 12),
			Token::new(TokenType::Identifier, "val", 14, 14),
			Token::new(TokenType::Operator, "-", 14, 18),
			Token::new(TokenType::IntConstant, "1", 14, 20),
			Token::new(TokenType::Operator, ";", 14, 21),
			Token::new(TokenType::Operator, "}", 15, 4),
			Token::new(TokenType::Keyword, "return", 16, 4),
			Token::new(TokenType::Identifier, "TestFloat", 16, 11),
			Token::new(TokenType::Operator, ";", 16, 20),
			Token::new(TokenType::Operator, "}", 17, 0),
			
						
//all characters are tested properly in example1.x file
			
		],
	)];
	

	
	//error handling 
	let has_error = false;
	for (i, (input, expected)) in all_token_tests.iter().enumerate() {
		let actual = Scanner::new(input).run().all_tokens.clone();
		let res = actual.iter().eq(expected.iter());
		assert!(
			res,
			"\nTest {} failed. On input\n{},\nexpected: '{:?}'\n but got '{:?}'",
			i, input, expected, actual
		);
	}
	if !has_error {
		println!("Stage 2 successfully approached! All characters in example1.x file tests passed!");
	}
}

	
