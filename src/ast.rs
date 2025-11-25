use crate::stack;
use std::collections::HashMap;
#[derive(Clone, PartialEq, Debug)]
pub struct AST {
    pub node: ASTnode,
    code: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ASTnode {
    pub nodetype: ASTnodeType,
    pub command: Option<Commands>,
    pub structure: Option<ControlStructures>,
    pub number: Option<f64>,
    pub children: Option<Vec<ASTnode>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Commands {
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
    DegreeCommand,
    RadianCommand,
    ColorCommand,
    PenDownCommand,
    PenUpCommand,
    SizeCommand,
    DebugCommand,
    PowerCommand,
    LogCommand,
    EulerNumCommand,
    SquareRootCommand,
    SineCommand,
    CeilingCommand,
    FloorCommand,
    RoundCommand,
    LessThanCommand,
    GreaterThanCommand,
    EqualCommand,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ControlStructures {
    RepeatLoop,
    WhileLoop,
    DipBlock,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ASTnodeType {
    Container,
    Command,
    ControlStructure,
    Number,
}

const ALLOWED_CHARS: [char; 45] = [
    '^', '~', '.', ':', 'p', '+', '-', '*', '/', '%', ' ', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '[', ']', '{', '}', '(', ')', 'o', 'r', 'c', 'd', 'u', 's', '?', 'P', 'l', 'e', 'q',
    'S', 'C', 'f', 'R', '>', '<', '=',
];
const CONVERSION_MAP: [(char, Commands); 28] = [
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
    ('o', Commands::DegreeCommand),
    ('r', Commands::RadianCommand),
    ('c', Commands::ColorCommand),
    ('d', Commands::PenDownCommand),
    ('u', Commands::PenUpCommand),
    ('s', Commands::SizeCommand),
    ('?', Commands::DebugCommand),
    ('P', Commands::PowerCommand),
    ('l', Commands::LogCommand),
    ('e', Commands::EulerNumCommand),
    ('q', Commands::SquareRootCommand),
    ('S', Commands::SineCommand),
    ('C', Commands::CeilingCommand),
    ('f', Commands::FloorCommand),
    ('R', Commands::RoundCommand),
    ('>', Commands::LessThanCommand),
    ('<', Commands::GreaterThanCommand),
    ('=', Commands::EqualCommand),
]; //just use HashMap::from when actually needed
const BRACK_CONV_MAP: [(char, ControlStructures); 3] = [
    ('[', ControlStructures::RepeatLoop),
    ('{', ControlStructures::WhileLoop),
    ('(', ControlStructures::DipBlock),
]; //ditto
const ALLOWED_COMMANDS: [char; 28] = [
    '^', '~', '.', ':', 'p', '+', '-', '*', '/', '%', 'o', 'r', 'c', 'd', 'u', 's', '?', 'P', 'l',
    'e', 'q', 'S', 'C', 'f', 'R', '>', '<', '=',
];
const ALLOWED_BRACKETS: [char; 6] = ['[', ']', '{', '}', '(', ')'];
const NUMBER_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn verify(code_in: String) -> Result<Vec<char>, String> {
    let mut out = Vec::new();
    let mut appended = false;
    let mut bracket_check_stack: stack::Stack<char> = stack::Stack::new();
    let mut peek_bracket: char;
    for i in code_in.chars() {
        for j in ALLOWED_CHARS {
            if i == j {
                out.push(i);
                appended = true;
                if i == '(' || i == '{' || i == '[' {
                    bracket_check_stack.push(i);
                }
                peek_bracket = match bracket_check_stack.peek() {
                    Some(n) => *n,
                    None => ' ',
                };
                if (i == ')' && peek_bracket == '(')
                    || (i == '}' && peek_bracket == '{')
                    || (i == ']' && peek_bracket == '[')
                {
                    let _throwaway = bracket_check_stack.pop();
                    break;
                }
                if i == ')' || i == ']' || i == '}' {
                    return Err("Mismached brackets".to_string());
                }
                break;
            }
        }
        if !appended {
            return Err(format!("{i} is not a valid command").to_string());
        }
        appended = false;
    }
    if !bracket_check_stack.is_empty() {
        return Err("Mismached brackets".to_string());
    }
    return Ok(out);
}
impl AST {
    pub fn new(code_in: String) -> Result<Self, String> {
        let code_verified: Result<Vec<char>, String> = verify(code_in.clone());
        if !code_verified.is_ok() {
            return Err(code_verified.unwrap_err()); //propagate the error to the repl
        }
        let code_unwrapped = code_verified.unwrap();
        let mut head_node = ASTnode {
            nodetype: ASTnodeType::Container,
            command: None,
            structure: None,
            number: None,
            children: Some(Vec::new()),
        };
        head_node.populate_children(code_unwrapped);
        let new_ast = AST {
            code: code_in,
            node: head_node,
        };
        return Ok(new_ast);
    }
}
impl ASTnode {
    pub fn populate_children(&mut self, code_in: Vec<char>) {
        let conversion_map: HashMap<char, Commands> = HashMap::from(CONVERSION_MAP);
        let brack_conv_map: HashMap<char, ControlStructures> = HashMap::from(BRACK_CONV_MAP);
        let mut idx = 0;
        let mut token: char;
        'token_loop: while idx < code_in.len() {
            token = code_in[idx];
            for com in ALLOWED_COMMANDS {
                if token == com {
                    //dealing with a command
                    self.children.as_mut().unwrap().push(ASTnode {
                        nodetype: ASTnodeType::Command,
                        command: Some(conversion_map.get(&token).unwrap().clone()),
                        structure: None,
                        number: None,
                        children: None,
                    });
                    idx += 1;
                    continue 'token_loop;
                }
            }
            for brack in ALLOWED_BRACKETS {
                if token == brack {
                    //dealing with a bracket
                    idx += 1;
                    let mut code_to_push: Vec<char> = Vec::new();
                    let mut bracket_depth: i64 = 1;
                    let mut current_pos: char;
                    while idx < code_in.len() {
                        current_pos = code_in[idx];
                        if bracket_depth == 0 {
                            break;
                        }
                        bracket_depth +=
                            if current_pos == ')' || current_pos == ']' || current_pos == '}' {
                                -1
                            } else if current_pos == '(' || current_pos == '{' || current_pos == '['
                            {
                                1
                            } else {
                                0
                            };
                        code_to_push.push(current_pos);
                        idx += 1;
                    }
                    let _throwaway = code_to_push.pop();
                    let mut new_ast_node: ASTnode = ASTnode {
                        nodetype: ASTnodeType::ControlStructure,
                        command: None,
                        structure: Some(brack_conv_map.get(&token).unwrap().clone()),
                        number: None,
                        children: Some(Vec::new()),
                    };
                    new_ast_node.populate_children(code_to_push);
                    self.children.as_mut().unwrap().push(new_ast_node);
                    continue 'token_loop;
                }
            }
            for num in NUMBER_CHARS {
                if token == num {
                    //dealing with a number
                    let mut topush: f64 = token.clone().to_string().parse::<f64>().unwrap();
                    idx += 1;
                    'numloop: while idx < code_in.len() {
                        //while loop for bounds checking(blocks should be able to end with numbers)
                        token = code_in[idx];
                        for num2 in NUMBER_CHARS {
                            if token == num2 {
                                topush *= 10.0;
                                topush += token.clone().to_string().parse::<f64>().unwrap();
                                idx += 1;
                                continue 'numloop;
                            }
                        }
                        break;
                    }
                    self.children.as_mut().unwrap().push(ASTnode {
                        nodetype: ASTnodeType::Number,
                        command: None,
                        structure: None,
                        number: Some(topush),
                        children: None,
                    });
                    continue 'token_loop;
                }
            }
            if token == ' ' {
                idx += 1;
            }
        }
    }
}

#[test]
fn verify_test() {
    let should_work_tokens: Result<Vec<char>, String> = verify("4[5^90~]".to_string());
    println!("testing 4[5^90~]");
    assert_eq!(
        vec!['4', '[', '5', '^', '9', '0', '~', ']'],
        should_work_tokens.unwrap()
    );
    assert_eq!(verify("4[|5^90~]".to_string()).is_ok(), false);
    let should_not_work: Result<Vec<char>, String> = verify("[]]".to_string());
    println!("testing []]");
    assert_eq!(should_not_work.is_ok(), false);
    println!("testing [}}");
    assert_eq!(verify("[}".to_string()).is_ok(), false);
    println!("testing ({{)}}");
    assert_eq!(verify("({)}".to_string()).is_ok(), false);
    println!("testing []");
    assert_eq!(verify("[]".to_string()).is_ok(), true);
    println!("testing {{}}{{()[]}}");
    assert_eq!(verify("{}{()[]}".to_string()).is_ok(), true);
    println!("testing [{{()}}]"); //the extra {} are for format! 
    assert_eq!(verify("[{()}]".to_string()).is_ok(), true);
}
#[test]
fn astnew_test() {
    let should_work: Result<AST, String> = AST::new("2 2+[5^90~]".to_string());
    assert_eq!(should_work.is_ok(), true);
    assert_eq!(
        should_work.unwrap().node.children.clone().unwrap()[2],
        ASTnode {
            nodetype: ASTnodeType::Command,
            command: Some(Commands::AddCommand),
            structure: None,
            number: None,
            children: None,
        }
    );
    let should_work: Result<AST, String> = AST::new("4[5^90~]".to_string());
    assert_eq!(
        should_work.unwrap().node.children.clone().unwrap()[0],
        ASTnode {
            nodetype: ASTnodeType::Number,
            command: None,
            structure: None,
            number: Some(4.0),
            children: None,
        }
    );
    let should_work: Result<AST, String> = AST::new("91".to_string());
    assert_eq!(
        should_work.unwrap().node.children.clone().unwrap()[0],
        ASTnode {
            nodetype: ASTnodeType::Number,
            command: None,
            structure: None,
            number: Some(91.0),
            children: None,
        }
    );
    let should_work: Result<AST, String> = AST::new("2[180~3[10^90~]]".to_string());
    assert_eq!(
        should_work.unwrap().node.children.clone().unwrap()[1]
            .children
            .clone()
            .unwrap()[3] //I dont know WHY cargo fmt insits this is the best way to format this and at this point, i'm too scared to ask
            .children
            .clone()
            .unwrap()[1],
        ASTnode {
            nodetype: ASTnodeType::Command,
            command: Some(Commands::ForwardCommand),
            structure: None,
            number: None,
            children: None,
        }
    );
}
