use std::fs;

//here we are parsing the .x files into xhtml file
//following the given EBNF grammar. 

use crate::stage3::Parser;

use crate::stage2_token::{Token, TokenType};

//outputting the xhmtl file 

pub struct XHTML {
    
    parser: Parser,
    
    //result
    
    res: String,
}

//struct implemntation

//read into stream 

impl XHTML {
    pub fn new(xfilename: &str) -> XHTML {
        
        XHTML {
            parser: Parser::new(xfilename),
            
            res: "".to_string()
        }
    }
    
    
    //online resources for titling reference

    fn for_result(&mut self) -> String {
        let mut result = 
r#"
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en">
<head>
<title>X Formatted file</title>
</head>
<body bgcolor="navy" text="orange" link="orange" vlink="orange">
<font face="Courier New">
"#.to_string();

        let mut line_num = 0;
        
        let mut char_pos = 0;
        
        for token in self.parser.all_tokens.iter() {
            
            
            {
                // new line
                while line_num < token.line_num {
                    
                    res += "<br />\n";
                    
                    line_num += 1;
                    
                    char_pos = 0;
                }
                
                // new character after skipping whitespace
                
                while char_pos < token.char_pos {
                    
                    if char_pos + 4 <= token.char_pos {
                        
                        res += "&nbsp;&nbsp;&nbsp;&nbsp;";
                        
                        char_pos += 4;
                        
                    } else {
                        res+= " ";
                        char_pos += 1;
                    }
                }
            }
            //match different token types 
            

            match token.token_type {
                
                TokenType::Keyword | TokenType::Operator => {
                    
                    res += &format!("<font color=\"white\"><b>{}</b></font>", token.text);
                },
                TokenType::IntConstant | TokenType::FloatConstant => {
                    
                    res += &format!("<font color=\"aqua\"><b>{}</b></font>", token.text);
                },
                
                TokenType::Identifier => {
                    
                    res += &format!("<font color=\"yellow\">{}</font>", token.text);
                },
                
                _ => {
                    res += &format!("<font color=\"red\">{}</font>", token.text);
                }
            }
            line_num = token.line_num;
            char_pos = token.char_pos + token.text.len() as i32;
            
            
        }

        res +=
r#"
</font>
</body>
</html>
"#;
        //clone the result to the saved buffer file 
            
        self.res = result.clone();
        
        return res;
    }

    pub fn run(&mut self) {
        
        self.parser.run();
        
        let res = self.for_result();
        
        fs::write("example.xhtml", result).expect("error when writing the file!");
    }
}

//worked

pub fn test_run() {
    
    let mut xhtml = XHTML::new("example1.x");
    
    xhtml.run();
}

pub fn test() {}
