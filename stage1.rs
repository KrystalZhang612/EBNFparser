//Stage 1: Read in the File
//Here to define a CStream struct that is able to read the input file character by character

//use the fs reference to an open file on the filesystem 
use std::fs; 

//CStream struct 

pub struct CStream {
	
	//the file name 
	pub xfilename: String, 
	
	//the current number of lines
	pub line_num: i32,
	
	
	//the current character's position as char_pos
	pub char_pos: i32,
	
	//read the whole file at once
	//and store it in a vector of strings
	//so we need to file the pointer here 
	input_file: Vec<String>, 

	
	//the contents of the current line 
	current_line_contents: String, 
	
	
	
}

//CStream struct implementation 

impl CStream {
	
	//the initializer to take the name of the input x file as the only one arguement
	
	pub fn new(xfilename: &str) -> CStream {
		
		//read the initial contents inside the x file once
		//character by character and store them into string 
		//use fs::read_to_string function to read the entire contents of a file into a string
		//https://doc.rust-lang.org/std/fs/fn.read_to_string.html
		//use expect here for error handling when reading the input x file
		
		let initial_file_contents = fs::read_to_string(xfilename).expect("Error occurred when reading the input file!"); 
		
		//use the function to_string() to convert any data types into a string here
		//https://doc.rust-lang.org/std/string/trait.ToString.html?search=to_string
		//use lines().map()..collect here to read the x file contents by lines 
		let input_file: Vec<String> = initial_file_contents.lines().map(|mystring| mystring.to_string()).collect(); 
		
		CStream {
			
			xfilename: xfilename.to_string(), 
			
			//initialize the line number of the current character, starting with line 0
			line_num: 0, 
			
			//initialize the position of the current character on this line 
			//starting with position -1 since haven’t started reading the file yet
			char_pos: -1, 
			
			//clone vectors and strings of the read file 
			input_file: input_file.clone(), 
			
			//clone the contents of the current line starting from beginning of the converted file 
			current_line_contents:input_file[0].clone(),
			
						
		}
		
	}
	
	//Return true if there are still characters available, 
	//return false if we have reached the end of the buffer file 
	
	pub fn more_available(&self) -> bool {
		
		/*if the current line number as pointer sized unsigned integer less than the 
		maximum size of the buffer file, there are still more characters to be read, 
		return true. otherwise, EOF, return false 
		*/
		//https://doc.rust-lang.org/std/primitive.usize.html
		
		(self.line_num as usize) < (self.input_file.len()-1)
		
	}
	

	//Moving to the next character and returns it
	
	pub fn get_next_char(&mut self)-> char{
		
		self.char_pos += 1; 
		
		while self.char_pos as usize >= self.current_line_contents.len(){
			
			self.line_num += 1; 
			self.current_line_contents = self.input_file[self.line_num as usize].clone(); 
			self.char_pos = 0; 
			
		}
		
		return self.current_line_contents.chars().nth(self.char_pos as usize).unwrap();
	
	}
	
	
	//return the current character based on the character position on line_num 
	
	pub fn get_cur_char(&self)-> char {
		
		//get the &str of the current char_pos by using chars().nth(char index).unwrap() method
		//https://www.codegrepper.com/code-examples/whatever/rust+get+nth+char+in+string
		return self.current_line_contents.chars().nth(self.char_pos as usize).unwrap(); 
		
		
	}
	
	
	//return the next characters
	//the next character is the kth characters at char pos 1  
	pub fn peek_next_char(&self)->char {
		
		return self.peek_ahead_char(1); 
	}
	
	

	
	//Return the kth character ahead in the stream 
	
	pub fn peek_ahead_char(&self, k: usize)-> char {
		
		//clone the current variables and store them 
		//copy the strings and vectors using clone() method as above 
		
		
		let mut line_num = self.line_num; 
		
		let mut char_pos = self.char_pos; 
		
		let mut current_line_contents = self.input_file[line_num as usize].clone(); 
		
		let mut kth_character = k as i32; 
		
		
		//traverse the kth characters through the vector of strings 
		
		while (kth_character + char_pos) >= (current_line_contents.len() as i32) {
			
			kth_character -= current_line_contents.len() as i32 - char_pos; 
			
			char_pos = 0;
			
			//increment line number 
			line_num += 1; 
			
			
			//save the current line contents as a copy 
			
			current_line_contents = self.input_file[line_num as usize].clone(); 
			
		}
		
		//looping through the rest kth characters 
		 char_pos += kth_character; 
		
		//obtain a &str at the kth character 
		
		return current_line_contents.chars().nth(char_pos as usize).unwrap(); 
		
	}
	
	
	
	
	
	//similarly, locate the kth characters ahead in the input file 
	
	pub fn locate_ahead_char(&mut self, k: usize){
		
		let mut kth_character = k; 
		
		//traverse the kth characters through the vector of strings 
		
		while  ((kth_character as i32)+self.char_pos) >= (self.current_line_contents.len() as i32) {
			
			kth_character -= self.current_line_contents.len() - (self.char_pos as usize); 
			
			self.char_pos = 0; 
			
			//increment line number 
			self.line_num = self.line_num + 1;
			
			
			
			//save the current line contents as a copy 
			self.current_line_contents = self.input_file[self.line_num as usize].clone();
			
		}
		//retrieving through the rest kth characters 
		//located ahead the kth characters 
		self.char_pos += kth_character as i32; 

	}
	
	
	
}





//Stage 1 testing codes 

pub fn CStreamTest(){
	
	let mut t = CStream::new("TestingTextFile.txt"); 
	
	//debugging using assert_eq!
	//char pos = 0 always at the current character index 
	
	assert_eq!(t.peek_next_char(),'b'); 
	assert_eq!(t.peek_ahead_char(6),'l'); 
	assert_eq!(t.get_next_char(),'b'); 
	assert_eq!(t.peek_ahead_char(10),' ');
	assert_eq!(t.get_next_char(),'i');
	assert_eq!(t.get_next_char(),'g');
	assert_eq!(t.get_cur_char(),'g');
	assert_eq!(t.peek_ahead_char(12),'g'); 
	//see if return false when reached the EOF 
	assert_eq!(t.more_available(),true); 
	//move to starting at the char pos at kth character as char pos 0 
	t.locate_ahead_char(10); 
	//see if successfully skip lines of whitespaces 
	assert_eq!(t.get_cur_char(),'a'); 
	assert_eq!(t.get_next_char(),'n'); 
	assert_eq!(t.get_cur_char(),'n');
	assert_eq!(t.get_next_char(),'g');
	assert_eq!(t.get_cur_char(),'g'); 
	//regard the cur char pos as 0, move two next pos
	assert_eq!(t.get_next_char(),'o'); 
	assert_eq!(t.get_next_char(),'g');
	assert_eq!(t.get_next_char(),'r'); 
	assert_eq!(t.get_next_char(),'a'); 
	assert_eq!(t.get_next_char(),'p'); 
	assert_eq!(t.get_next_char(),'e'); 
	assert_eq!(t.get_next_char(),'f'); 
	assert_eq!(t.get_next_char(),'r'); 
	assert_eq!(t.get_next_char(),'u'); 
	assert_eq!(t.get_next_char(),'i'); 
	assert_eq!(t.get_next_char(),'t'); 
	assert_eq!(t.get_next_char(),'c'); 
	//should reached the EOF 
	assert_eq!(t.more_available(),false); 
	
	//if all CStraem functions tested correctly. output
	//otherwise, the main thread will panic.
	
	println!("Stage 1 successfully approached! All CStream tests passed!");
	

	
}









