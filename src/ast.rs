use std::collections::HashMap;
use crate::stack;

pub struct AST{
    nodes: Vec<ASTnode>,
    code: String,
}
struct ASTnode{
     nodetype: ASTnodeType,
     command: Option<Commands>,
     structure: Option<ControlStructures>,
     children: Option<Vec<ASTnode>>,
}
pub enum Commands{
    ForwardCommand,
    TurnCommand,
    DuplicateCommand,
    SwapCommand,
    PopCommand,
    AddCommand,
    SubtractCommand,
    MultiplyCommand,
    DivideCommand,
    ModuloCommand,
}
pub enum ControlStructures{
    RepeatLoop
}
enum ASTnodeType{
    Empty,
    Command,
    ControlStructure,
}
const allowed_chars: [char; 23]=['^','~','.',':','p','+','-','*','/','%',' ','0','1','2','3','4','5','6','7','8','9','[',']'];
const conversion_map: [(char, Commands); 10]=[
    ('^', Commands::ForwardCommand),
    ('~', Commands::TurnCommand),
    ('.', Commands::DuplicateCommand),
    (':', Commands::SwapCommand),
    ('p', Commands::PopCommand),
    ('+', Commands::AddCommand),
    ('-', Commands::SubtractCommand),
    ('*', Commands::MultiplyCommand),
    ('/', Commands::DivideCommand),
    ('%', Commands::ModuloCommand),
];
fn verify(code_in: String)-> Result<Vec<char>,String>{
    let mut out=Vec::new();
    let mut appended=false;
    let mut bracket_check_stack: stack::Stack<char>=stack::Stack::new();
    let mut peek_bracket: char;
    for i in code_in.chars(){
        for j in allowed_chars{
            if i==j{
                out.push(i);
                println!("pushed {i}");
                appended=true;
                if i == '(' || i == '{' || i == '[' || i == '<' {
                    bracket_check_stack.push(i);
                    println!("pushed bracket {i}");
                }
                peek_bracket=match bracket_check_stack.peek(){
                     Some(n)=> *n,
                     None=> ' ',
                };
                if  (i == ')'&&peek_bracket=='(') || (i == '}'&&peek_bracket=='{') || (i == ']'&&peek_bracket=='[') || (i == '>'&&peek_bracket=='<'){
                     println!("found closing maching bracket {i}");
                     let throwaway=bracket_check_stack.pop();
                }
                break;
            }
        }
        if !appended{
            println!("could not find {i}");
            return Err("{i} is not a valid command".to_string());
        }
        appended=false;
    }
    if !bracket_check_stack.is_empty(){
          return Err("Mismached brackets".to_string());
    }
    return Ok(out);
}
impl AST{
    pub fn new(code_in: String)-> Self{
        let mut new_ast = AST{code: code_in, nodes: Vec::new()};
        return new_ast;
    }
}
impl ASTnode{}

#[test]
fn verify_test(){
    let mut should_work_tokens: Result<Vec<char>,String>=verify("4[5^90~]".to_string());
    println!("{}",should_work_tokens.is_ok());
    assert_eq!(vec!['4','[','5','^','9','0','~',']'],should_work_tokens.unwrap());
    assert_eq!(verify("4[u5^90~]".to_string()).is_ok(),false);
    let mut should_not_work: Result<Vec<char>,String>=verify("[]]".to_string());
    assert_eq!(should_not_work.is_ok(),false);
    assert_eq!(verify("[}".to_string()).is_ok(),false);
    assert_eq!(verify("({)}".to_string()).is_ok(),false);
    assert_eq!(verify("[]".to_string()).is_ok(),true);
    assert_eq!(verify("{}{()[]}[[[]<>]]]".to_string()).is_ok(),true);
    assert_eq!(verify("[{(<>)}]".to_string()).is_ok(),false);
    
}

