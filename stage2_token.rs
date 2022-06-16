
//TOKEN


//Define the enumerated type called TokenType with the following required elements
//due to multiple traits to manually compare their complex behaviors, use Derive here
//https://doc.rust-lang.org/rust-by-example/trait/derive.html

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

pub enum TokenType{
	
	IntConstant, 
	FloatConstant, 
	Keyword, 
	Operator, 
	Identifier, 
	Invalid, 
}

//Define a struct called Token with the following attributes 
//use PartialEq trait to compare which are partial equivalence relations 
//use Clone trait to duplicate objects 
//https://doc.rust-lang.org/std/clone/trait.Clone.html
#[derive(Clone, PartialEq)]

pub struct Token{
	
	//the token type of the token 
	pub token_type: TokenType,
	
	//the text of the token as String 
	pub text: String, 
	
	
	//the line number of the token; the first line is numbered 0
	pub line_num: i32, 
	
	//the character position of the first character in the token text
	//starting at 0 for each new line, which is thereby token's column number 
	pub char_pos: i32,
	
}

//Token struct implementation 

impl Token{
	
	//the initializer 
	
	pub fn new(token_type: TokenType, text: &str, line_num: i32, char_pos: i32) ->Token{
		
		Token{
			token_type, 
			//use to_string() method to convert the given token types into string 
			text: text.to_string(), 
			line_num, 
			char_pos, 
		}
	}
}


//debug for tokens 


impl std::fmt::Debug for Token{
	
	
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
		write!(
			f, 
			"\n\ttoken line {}, column number {} = {}\n\tthe token type is {}\n",
			self.line_num, 
			self.char_pos, 
			self.text, 
			match self.token_type{
				//match all different token types
				TokenType::IntConstant => "IntConstant",
				TokenType::FloatConstant => "FloatConstant",
				TokenType::Keyword => "Keyword",
				TokenType::Operator => "Operator",
				TokenType::Identifier => "Identifier",
				TokenType::Invalid => "Invalid",
				
			}
		)
	}
}



























