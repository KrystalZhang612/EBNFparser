extern crate custom_error;
use custom_error::custom_error;

use crate::scanner_stage2::Scanner;
use crate::token_stage2::{Token, TokenType};

custom_error! {SyntaxError{line_num:i32, char_pos:i32, ebnf: String} ="Error at Line {line_num} Character {char_pos}. The syntax should be: {ebnf}."}

macro_rules!syntax_error{
    ($token:expr, $ebnf:expr) => {
        Err(SyntaxError {
            line_num: $token.line_num,
            char_pos: $token.char_pos,
            ebnf: $ebnf.to_string(),
        })
    };
}

macro_rules! ebnf_run {
    ($self:ident, fn $closure:expr) => {
        ebnf_run!({}, {}, true, $self, fn $closure)
    };
    // used by other macros with args
    ($ok_block:block, $err_block:block, $is_return_err:expr, $self:ident, fn $closure:expr) => {
        let prev_index = $self.token_index;
        match $closure() {
            Ok(_) => {
                $ok_block;
            }
            Err(e) => {
                // syntax check failed, reset index
                $self.token_index = prev_index;
                $err_block;
                if $is_return_err {
                    return Err(e);
                }
            }
        }
    };
}

macro_rules! ebnf_or {
    ($ebnf:expr, $self:ident, $(fn $closure:expr),+) => {
        loop { // loop hack to exit block scope
            $(
                ebnf_run!({
                    break;
                }, {}, false,$self, fn $closure);
            )+
            if $self.token_index >= $self.all_tokens.len() {
                return syntax_error!($self.all_tokens.last().unwrap(), $ebnf)
            } else {
                return syntax_error!($self.all_tokens[$self.token_index], $ebnf);
            };
        }
    };
}

macro_rules! ebnf_optional {
    ($self:ident, $(fn $closure:expr),+) => {
        loop { // loop hack to exit block scope
            let prev_index = $self.token_index;
            $(
                ebnf_run!({}, { // err block
                    if true { // bypass #[warn(unreachable_code)]
                        // syntax check failed, reset index
                        $self.token_index = prev_index;
                        break;
                    }
                }, false, $self, fn $closure);
            )+
            break;
        }
    };
}

macro_rules! ebnf_repetition { // zero or more
    ($self:ident, $(fn $closure:expr),+) => {
        loop {
            let prev_index = $self.token_index;
            $(
                ebnf_run!({}, { // err block
                    if true {  // bypass #[warn(unreachable_code)]
                        // syntax check failed, reset index
                        $self.token_index = prev_index;
                        break;
                    }
                }, false, $self, fn $closure);
            )+
        }
    };
}

/**
 * Parses through and validates a .x file, analyzing syntax and
 * ensuring semantics. Terminates on the first error if an error exists.
 */
pub struct Parser {
    // uses scanner for tokenization
    pub all_tokens: Vec<Token>,
    // current token index
    token_index: usize,
}

// ebnf_repeat!
// ebnf_optional
// ebnf_or

impl Parser {
    // init
    pub fn new(filename: &str) -> Parser {
        Parser {
            all_tokens: Scanner::new(filename).run().all_tokens.clone(),
            token_index: 0,
        }
    }

    fn terminal_token(
        &mut self,
        token_type: TokenType,
        text: &str,
        ebnf: &str,
    ) -> Result<(), SyntaxError> {
        // no more tokens
        if self.token_index >= self.all_tokens.len() {
            return syntax_error!(self.all_tokens.last().unwrap(), ebnf);
        }

        let token = self.all_tokens[self.token_index].clone();
        if token_type == token.token_type {
            match token_type {
                TokenType::Invalid => {
                    return syntax_error!(token, ebnf);
                }
                TokenType::Operator | TokenType::Keyword => {
                    if token.text == text.to_string() {
                        self.token_index += 1;
                        // println!("OKAY BUT LIKE {}", self.token_index);
                        return Ok(());
                    }
                }
                _ => {
                    self.token_index += 1;
                    return Ok(());
                }
            }
        }
        return syntax_error!(token, ebnf);
    }

    fn syntax_program(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Program := { Declaration } MainDeclaration { FunctionDefinition }";

        ebnf_repetition!(self, fn || self.syntax_declaration());
        ebnf_run!(self, fn || self.syntax_main_declaration());
        ebnf_repetition!(self, fn || self.syntax_function_definition());

        return Ok(());
    }

    fn syntax_declaration(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)";

        ebnf_run!(self, fn || self.syntax_declaration_type());
        ebnf_or!(
            ebnf,
            self,
            fn || self.syntax_variable_declaration(),
            fn || self.syntax_function_declaration()
        );

        return Ok(());
    }

    fn syntax_main_declaration(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "MainDeclaration := void main ( ) Block";

        ebnf_run!(self, fn || self.terminal_token(TokenType::Keyword, "void", ebnf));
        ebnf_run!(self, fn || self.terminal_token(TokenType::Keyword, "main", ebnf));
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, "(", ebnf));
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ")", ebnf));
        ebnf_run!(self, fn || self.syntax_block());

        return Ok(());
    }

    fn syntax_function_definition(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "FunctionDefinition := DeclarationType ParameterBlock Block";

        ebnf_run!(self, fn || self.syntax_declaration_type());
        ebnf_run!(self, fn || self.syntax_parameter_block());
        ebnf_run!(self, fn || self.syntax_block());

        return Ok(());
    }

    fn syntax_declaration_type(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "DeclarationType := DataType Identifier";

        ebnf_run!(self, fn || self.syntax_data_type());
        ebnf_run!(self, fn || self.terminal_token(TokenType::Identifier, "", ebnf));

        return Ok(());
    }

    fn syntax_variable_declaration(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "VariableDeclaration := [= Constant] ;";

        ebnf_optional!(
            self,
            fn || self.terminal_token(TokenType::Operator, "=", "ebnf"),
            fn || self.syntax_constant()
        );
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ";", ebnf));

        return Ok(());
    }

    fn syntax_function_declaration(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "FunctionDeclaration := ParameterBlock ;";

        ebnf_run!(self, fn || self.syntax_parameter_block());
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ";", ebnf));

        return Ok(());
    }

    fn syntax_block(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Block := { {Declaration} {Statement} {FunctionDefinition} }";

        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, "{", ebnf));
        ebnf_repetition!(self, fn || self.syntax_declaration());
        ebnf_repetition!(self, fn || self.syntax_statement());
        ebnf_repetition!(self, fn || self.syntax_function_definition());
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, "}", ebnf));

        return Ok(());
    }

    fn syntax_parameter_block(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "ParameterBlock := ( [Parameter {, Parameter}] )";

        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, "(", ebnf));

        // [ Parameter {, Parameter } ]
        ebnf_optional!(
            self,
            // Parameter
            fn || self.syntax_parameter(),
            // {, Parameter }
            fn || {
                ebnf_repetition!(
                    self,
                    // ,
                    fn || self.terminal_token(TokenType::Operator, ",", "ebnf"),
                    // Parameter
                    fn || self.syntax_parameter()
                );
                return Ok(());
            }
        );
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ")", ebnf));

        return Ok(());
    }

    fn syntax_data_type(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "DataType := IntegerType | FloatType";

        ebnf_or!(
            ebnf,
            self,
            fn || self.syntax_integer_type(),
            fn || self.syntax_float_type()
        );

        return Ok(());
    }

    fn syntax_constant(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Constant := IntConstant | FloatConstant";

        ebnf_or!(ebnf, self,
            fn || self.terminal_token(TokenType::IntConstant, "", ebnf),
            fn || self.terminal_token(TokenType::FloatConstant, "", ebnf)
        );

        return Ok(());
    }

    fn syntax_statement(&mut self) -> Result<(), SyntaxError> {
        let ebnf =
            "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)";

        ebnf_or!(
            ebnf,
            self,
            fn || self.syntax_assignment(),
            fn || self.syntax_while_loop(),
            fn || self.syntax_if_statement(),
            fn || self.syntax_return_statement(),
            fn || {
                ebnf_run!(self, fn || self.syntax_expression());
                ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ";", ebnf));
                return Ok(());
            }
        );

        return Ok(());
    }

    fn syntax_parameter(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Parameter := DataType Identifier";

        ebnf_run!(self, fn || self.syntax_data_type());
        ebnf_run!(self, fn || self.terminal_token(TokenType::Identifier, "", ebnf));

        return Ok(());
    }

    fn syntax_integer_type(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "IntegerType := [unsigned] ( char | short | int | long )";

        ebnf_optional!(
            self,
            fn || self.terminal_token(TokenType::Keyword, "unsigned", ebnf)
        );
        ebnf_or!(
            ebnf,
            self,
            fn || self.terminal_token(TokenType::Keyword, "char", ebnf),
            fn || self.terminal_token(TokenType::Keyword, "short", ebnf),
            fn || self.terminal_token(TokenType::Keyword, "int", ebnf),
            fn || self.terminal_token(TokenType::Keyword, "long", ebnf)
        );

        return Ok(());
    }

    fn syntax_float_type(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "FloatType := float | double";

        ebnf_or!(
            ebnf,
            self,
            fn || self.terminal_token(TokenType::Keyword, "float", ebnf),
            fn || self.terminal_token(TokenType::Keyword, "double", ebnf)
        );

        return Ok(());
    }

    fn syntax_assignment(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Assignment := Identifier = {Identifier =} Expression ;";

        ebnf_run!(self, fn || self.terminal_token(TokenType::Identifier, "", ebnf));
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, "=", ebnf));
        ebnf_repetition!(
            self,
            fn || self.terminal_token(TokenType::Identifier, "", ebnf),
            fn || self.terminal_token(TokenType::Operator, "=", ebnf)
        );
        ebnf_run!(self, fn || self.syntax_expression());
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ";", ebnf));

        return Ok(());
    }

    fn syntax_while_loop(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "WhileLoop := while ( Expression ) Block";

        ebnf_run!(self, fn || self.terminal_token(TokenType::Keyword, "while", ebnf));
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, "(", ebnf));
        ebnf_run!(self, fn || self.syntax_expression());
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ")", ebnf));
        ebnf_run!(self, fn || self.syntax_block());

        return Ok(());
    }

    fn syntax_if_statement(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "IfStatement := if ( Expression ) Block";

        ebnf_run!(self, fn || self.terminal_token(TokenType::Keyword, "if", ebnf));
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, "(", ebnf));
        ebnf_run!(self, fn || self.syntax_expression());
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ")", ebnf));
        ebnf_run!(self, fn || self.syntax_block());

        return Ok(());
    }

    fn syntax_return_statement(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "ReturnStatement := return Expression ;";

        ebnf_run!(self, fn || self.terminal_token(TokenType::Keyword, "return", ebnf));
        ebnf_run!(self, fn || self.syntax_expression());
        ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ";", ebnf));

        return Ok(());
    }

    fn syntax_expression(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Expression := SimpleExpression [ RelationOperator SimpleExpression ]";

        ebnf_run!(self, fn || self.syntax_simple_expression());
        ebnf_optional!(
            self,
            fn || self.syntax_relation_operator(),
            fn || self.syntax_simple_expression()
        );

        return Ok(());
    }

    fn syntax_simple_expression(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "SimpleExpression := Term { AddOperator Term }";

        ebnf_run!(self, fn || self.syntax_term());
        ebnf_repetition!(
            self,
            fn || self.syntax_add_operator(),
            fn || self.syntax_term()
        );

        return Ok(());
    }

    fn syntax_term(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Term := Factor { MultOperator Factor }";

        ebnf_run!(self, fn || self.syntax_factor());
        ebnf_repetition!(
            self,
            fn || self.syntax_mult_operator(),
            fn || self.syntax_factor()
        );

        return Ok(());
    }

    fn syntax_factor(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "Factor := ( ( Expression ) ) | Constant | ( Identifier [ ( [ Expression {, Expression} ] ) ] )";

        ebnf_or!(
            ebnf,
            self,
            // ( Expression )
            fn || {
                ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, "(", ebnf));
                ebnf_run!(self, fn || self.syntax_expression());
                ebnf_run!(self, fn || self.terminal_token(TokenType::Operator, ")", ebnf));
                return Ok(());
            },
            // Constant
            fn || self.syntax_constant(),
            // Identifier [ ( [ Expression {, Expression} ] ) ]
            fn || {
                // Identifier
                ebnf_run!(self, fn || self.terminal_token(TokenType::Identifier, "", ebnf));
                // [ ( [ Expression {, Expression} ] ) ]
                ebnf_optional!(
                    self,
                    // (
                    fn || self.terminal_token(TokenType::Operator, "(", ebnf),
                    // [ Expression {, Expression} ]
                    fn || {
                        ebnf_optional!(
                            self,
                            // Expression
                            fn || self.syntax_expression(),
                            // {, Expression}
                            fn || {
                                ebnf_repetition!(
                                    self,
                                    // ,
                                    fn || self.terminal_token(TokenType::Operator, ",", ebnf),
                                    // Expression
                                    fn || self.syntax_expression()
                                );
                                return Ok(());
                            }
                        );
                        return Ok(());
                    },
                    // )
                    fn || self.terminal_token(TokenType::Operator, ")", ebnf)
                );
                return Ok(());
            }
        );

        return Ok(());
    }

    fn syntax_relation_operator(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )";

        ebnf_or!(
            ebnf,
            self,
            fn || self.terminal_token(TokenType::Operator, "==", ebnf),
            fn || self.terminal_token(TokenType::Operator, "<", ebnf),
            fn || self.terminal_token(TokenType::Operator, ">", ebnf),
            fn || self.terminal_token(TokenType::Operator, "<=", ebnf),
            fn || self.terminal_token(TokenType::Operator, ">=", ebnf),
            fn || self.terminal_token(TokenType::Operator, "!=", ebnf)
        );

        return Ok(());
    }

    fn syntax_add_operator(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "AddOperator := + | -";

        ebnf_or!(
            ebnf,
            self,
            fn || self.terminal_token(TokenType::Operator, "+", ebnf),
            fn || self.terminal_token(TokenType::Operator, "-", ebnf)
        );

        return Ok(());
    }

    fn syntax_mult_operator(&mut self) -> Result<(), SyntaxError> {
        let ebnf = "MultOperator := * | /";

        ebnf_or!(
            ebnf,
            self,
            fn || self.terminal_token(TokenType::Operator, "*", ebnf),
            fn || self.terminal_token(TokenType::Operator, "/", ebnf)
        );

        return Ok(());
    }

    // runs the parser
    pub fn run(&mut self) -> String {
        match self.syntax_program() {
            Ok(_) => "Input program is syntactically correct.".to_string(),
            Err(e) => e.to_string(),
        }
    }
}

pub fn test_run() {
    let mut parser = Parser::new("example7.x");
    println!("{}", parser.run());
}

pub fn test() {
    assert_eq!(
        Parser::new("example1.x").run(),
        "Input program is syntactically correct.".to_string()
    );
    assert_eq!(
        Parser::new("example2.x").run(),
        "Input program is syntactically correct.".to_string()
    );
    assert_eq!( // check error msg
        Parser::new("example3.x").run(), 
        "Error at Line 1 Character 0. The syntax should be: MainDeclaration := void main ( ) Block.".to_string()
    );
    assert_eq!( // check error msg
        Parser::new("example4.x").run(), 
        "Error at Line 1 Character 0. The syntax should be: MainDeclaration := void main ( ) Block.".to_string()
    );
    assert_eq!(
        Parser::new("example5.x").run(), 
        "Error at Line 2 Character 0. The syntax should be: MainDeclaration := void main ( ) Block.".to_string()
    );
    assert_eq!(
        Parser::new("example6.x").run(), 
        "Error at Line 1 Character 4. The syntax should be: Block := { {Declaration} {Statement} {FunctionDefinition} }.".to_string()
    );
    assert_eq!( // check error msg
        Parser::new("example7.x").run(), 
        "Error at Line 1 Character 0. The syntax should be: MainDeclaration := void main ( ) Block.".to_string()
    );
    assert_eq!( // check error msg
        Parser::new("example8.x").run(), 
        "Error at Line 1 Character 0. The syntax should be: MainDeclaration := void main ( ) Block.".to_string()
    );

    println!("All tests passed");
}

