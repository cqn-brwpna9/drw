use crate::item;
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
    pub function: Option<char>, //named refrence for lazy evaluation and, therefore, recursion
    pub children: Option<Vec<ASTnode>>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
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
    DipCommand,
    UndipCommand,
    BoxCommand,
    UnboxCommand,
    IsBoxCommand,
    RotCommand,
    UnrotCommand,
    NilCommand,
    IsNilCommand,
    MatchCommand,
    ExecCommand,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
pub enum ControlStructures {
    RepeatLoop,
    WhileLoop,
    Quote,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ASTnodeType {
    Container,
    Command,
    ControlStructure,
    Number,
    Function,
}

const ALLOWED_CHARS: [char; 56] = [
    '^', '~', '.', ':', '!', '+', '-', '*', '/', '%', ' ', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '[', ']', '{', '}', '(', ')', 'o', 'r', 'c', 'd', 'u', 's', '?', 'P', 'l', 'e', 'q',
    'S', 'C', 'f', 'R', '>', '<', '=', 'B', 'U', 'I', '@', '&', ',', ';', '|', '`', '"', '\'',
];

const CONVERSION_MAP: [(char, Commands); 39] = [
    ('^', Commands::ForwardCommand),
    ('~', Commands::TurnCommand),
    ('.', Commands::DuplicateCommand),
    (':', Commands::SwapCommand),
    ('!', Commands::PopCommand),
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
    ('(', Commands::DipCommand),
    (')', Commands::UndipCommand),
    ('B', Commands::BoxCommand),
    ('U', Commands::UnboxCommand),
    ('I', Commands::IsBoxCommand),
    ('@', Commands::RotCommand),
    ('&', Commands::UnrotCommand),
    (',', Commands::NilCommand),
    (';', Commands::IsNilCommand),
    ('|', Commands::MatchCommand),
    ('`', Commands::ExecCommand),
]; //just use HashMap::from when actually needed

const BRACK_CONV_MAP: [(char, ControlStructures); 3] = [
    ('[', ControlStructures::RepeatLoop),
    ('{', ControlStructures::WhileLoop),
    ('"', ControlStructures::Quote),
]; //ditto

const ALLOWED_COMMANDS: [char; 39] = [
    '^', '~', '.', ':', '!', '+', '-', '*', '/', '%', 'o', 'r', 'c', 'd', 'u', 's', '?', 'P', 'l',
    'e', 'q', 'S', 'C', 'f', 'R', '>', '<', '=', '(', ')', 'B', 'U', 'I', '@', '&', ',', ';', '|',
    '`',
];

const ALLOWED_BRACKETS: [char; 6] = ['[', ']', '{', '}', '"', '\''];

const NUMBER_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn verify(code_in: String, functions: Vec<char>) -> Result<Vec<char>, String> {
    let mut out = Vec::new();
    let mut appended = false;
    let mut bracket_check_stack: stack::Stack<char> = stack::Stack::new();
    let mut peek_bracket: char;
    for i in code_in.chars() {
        for j in ALLOWED_CHARS {
            if i == j {
                out.push(i);
                appended = true;
                if i == '{' || i == '[' || i == '"' {
                    bracket_check_stack.push(i);
                }
                peek_bracket = match bracket_check_stack.peek() {
                    Some(n) => *n,
                    None => ' ',
                };
                if (i == '}' && peek_bracket == '{')
                    || (i == ']' && peek_bracket == '[')
                    || (i == '\'' && peek_bracket == '"')
                {
                    let _throwaway = bracket_check_stack.pop();
                    break;
                }
                if i == ']' || i == '}' || i == '\'' {
                    return Err("Mismached brackets".to_string());
                }
                break;
            }
        }
        for j in &functions {
            if i == *j {
                out.push(i);
                appended = true;
                break;
            }
        }
        if !appended {
            return Err(format!("{i} is not a valid command or named function").to_string());
        }
        appended = false;
    }
    if !bracket_check_stack.is_empty() {
        return Err("Mismached brackets".to_string());
    }
    Ok(out)
}
impl AST {
    pub fn new(code_in: String, functions: Vec<char>) -> Result<Self, String> {
        let code_verified: Result<Vec<char>, String> = verify(code_in.clone(), functions.clone());
        if !code_verified.is_ok() {
            return Err(code_verified.unwrap_err()); //propagate the error to the repl
        }
        let code_unwrapped = code_verified.unwrap();
        let mut head_node = ASTnode {
            nodetype: ASTnodeType::Container,
            command: None,
            structure: None,
            number: None,
            function: None,
            children: Some(Vec::new()),
        };
        head_node.populate_children(code_unwrapped, functions);
        let new_ast = AST {
            code: code_in,
            node: head_node,
        };
        Ok(new_ast)
    }
}
impl ASTnode {
    pub fn populate_children(&mut self, code_in: Vec<char>, functions: Vec<char>) {
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
                        function: None,
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
                            if current_pos == ']' || current_pos == '}' || current_pos == '\'' {
                                -1
                            } else if current_pos == '{' || current_pos == '[' || current_pos == '"'
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
                        function: None,
                        children: Some(Vec::new()),
                    };
                    new_ast_node.populate_children(code_to_push, functions.clone());
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
                        function: None,
                        children: None,
                    });
                    continue 'token_loop;
                }
            }
            for fun in &functions {
                if token == *fun {
                    //dealing with a function
                    self.children.as_mut().unwrap().push(ASTnode {
                        nodetype: ASTnodeType::Function,
                        command: None,
                        structure: None,
                        number: None,
                        function: Some(token),
                        children: None,
                    });
                    idx += 1;
                    continue 'token_loop;
                }
            }
            if token == ' ' {
                idx += 1;
            }
        }
    }
    pub fn to_item(quote: Vec<ASTnode>) -> item::Item {
        #[rustfmt::skip] // rustfmt likes to split up (Commands, char)
        let bac_conv_map: HashMap<Commands, char> = HashMap::from(
            TryInto::<[_; 39]>::try_into(
		CONVERSION_MAP
		    .iter()
		    .map(|x| (x.1, x.0))
		    .collect::<Vec<(Commands, char)>>(),
	    )
            .unwrap(),
        );
        let bac_ctl_map: HashMap<ControlStructures, char> = HashMap::from(
            TryInto::<[_; 3]>::try_into(
                BRACK_CONV_MAP
                    .iter()
                    .map(|x| (x.1, x.0))
                    .collect::<Vec<(ControlStructures, char)>>(),
            )
            .unwrap(),
        );
        quote
            .iter()
            .map(|node| match node.nodetype {
                ASTnodeType::Container => panic!("Waiter, theres a container in my quote!"),
                ASTnodeType::Number => item::DrwBox::new(
                    item::Item::zero(),
                    item::Item::from_num(node.number.unwrap()),
                    item::Item::nil()
                ),
                ASTnodeType::Command => {
                    let x = bac_conv_map.get(&node.command.unwrap()).unwrap();
                    let mut o = -1.0;
                    for (idx, i) in ALLOWED_COMMANDS.iter().enumerate() {
                        if i == x {
                            o = idx as f64;
                            break;
                        }
                    }
                    if o == -1.0 {
                        panic!("Bad command in quote construction");
                    }
                    item::DrwBox::new(
                        item::Item::from_num(1.0),
                        item::Item::from_num(o),
                        item::Item::nil(),
                    )
                }
                ASTnodeType::ControlStructure => {
                    let x = bac_ctl_map.get(&node.structure.unwrap()).unwrap();
                    let mut o = -1.0;
                    for (idx, i) in ALLOWED_BRACKETS.iter().enumerate() {
                        if i == x {
                            o = idx as f64;
                            break;
                        }
                    }
                    if o == -1.0 {
                        panic!("Bad ctl structure in quote construction");
                    }
                    item::DrwBox::new(
                        item::Item::from_num(2.0),
                        item::Item::from_num(o),
                        ASTnode::to_item(node.children.clone().unwrap()),
                    )
                }
                ASTnodeType::Function => item::DrwBox::new(
                    item::Item::from_num(3.0),
                    item::Item::from_num((node.function.unwrap() as u32) as f64),
                    item::Item::nil()
                ),
            })
            .rfold(item::Item::nil(), |acc, node| {
                item::Item::from_box(item::DrwBox::new(
                    item::Item::from_box(node),
                    acc,
                    item::Item::nil(),
                ))
            })
    }
    pub fn from_item(mut quote: item::Item, functions: &HashMap<char, AST>) -> Vec<ASTnode> {
        let mut tokens = Vec::new();
        let conv_map = HashMap::from(CONVERSION_MAP);
        let brack_map = HashMap::from(BRACK_CONV_MAP);
        while quote != item::Item::nil() {
            if quote.itemtype != item::ItemType::Box {
                panic!("the quote {quote} is not a box");
            }
            let bx = *(quote.boxed.unwrap());
            quote = bx.g;
            let tok = bx.r;
            if tok.itemtype != item::ItemType::Box {
                panic!("the token {tok} is not a box");
            }
            let tok = *(tok.boxed.unwrap());
            let id = tok.r;
            if id.itemtype != item::ItemType::Number {
                panic!("{id} is not a number, and as such not a valid id for a quote token");
            }
            let n = tok.g;
            if n.itemtype != item::ItemType::Number {
                panic!("{n} is not a number, and as such not a valid id for a quote function identifier");
            }
            let n = n.number.unwrap();
            let m = tok.b;
            match id.number.unwrap() {
                0.0 => tokens.push(ASTnode {
                    nodetype: ASTnodeType::Number,
                    command: None,
                    structure: None,
                    number: Some(n),
                    function: None,
                    children: None,
                }),
                1.0 => {
                    let command = conv_map
                        .get(&ALLOWED_COMMANDS[n as usize])
                        .expect(&format!("{n} is not an allowed command id for quotes"));
                    tokens.push(ASTnode {
                        nodetype: ASTnodeType::Command,
                        command: Some(*command),
                        structure: None,
                        number: None,
                        function: None,
                        children: None,
                    });
                }
                2.0 => {
                    if m.itemtype == item::ItemType::Number {
                        panic!("numbers (like {m}) are not valid children of control structures");
                    }
                    let struc = brack_map
                        .get(&ALLOWED_BRACKETS[n as usize])
                        .expect(&format!("{n} is not an allowed structure id for quotes"));
                    tokens.push(ASTnode {
                        nodetype: ASTnodeType::ControlStructure,
                        command: None,
                        structure: Some(*struc),
                        number: None,
                        function: None,
                        children: Some(ASTnode::from_item(m, &functions)),
                    });
                }
                3.0 => {
                    let u = char::from_u32(n as u32)
                        .expect("could not make the charicter id {n} into a char.");
                    if !functions.contains_key(&u) {
                        panic!(
                            "{u} (code {n}) is not a defined function. found in decoding a quote"
                        );
                    }
                    tokens.push(ASTnode {
                        nodetype: ASTnodeType::Function,
                        command: None,
                        structure: None,
                        number: None,
                        function: Some(u),
                        children: None,
                    });
                }
                x => panic!("{x} is not a recognised token type id for quote decoding"),
            }
        }
        tokens
    }
}

#[test]
fn verify_test() {
    let should_work_tokens: Result<Vec<char>, String> = verify("4[5^90~]".to_string(), Vec::new());
    println!("testing 4[5^90~]");
    assert_eq!(
        vec!['4', '[', '5', '^', '9', '0', '~', ']'],
        should_work_tokens.unwrap()
    );
    assert_eq!(verify("4[|5^90~]".to_string(), Vec::new()).is_ok(), false);
    let should_not_work: Result<Vec<char>, String> = verify("[]]".to_string(), Vec::new());
    println!("testing []]");
    assert_eq!(should_not_work.is_ok(), false);
    println!("testing [}}");
    assert_eq!(verify("[}".to_string(), Vec::new()).is_ok(), false);
    println!("testing [{{]}}");
    assert_eq!(verify("[{]}".to_string(), Vec::new()).is_ok(), false);
    println!("testing []");
    assert_eq!(verify("[]".to_string(), Vec::new()).is_ok(), true);
    println!("testing {{}}{{{{}}[]}}");
    assert_eq!(verify("{}{{}[]}".to_string(), Vec::new()).is_ok(), true);
    println!("testing [{{[]}}]"); //the extra {} are for format!
    assert_eq!(verify("[{[]}]".to_string(), Vec::new()).is_ok(), true);
}
#[test]
fn astnew_test() {
    let should_work: Result<AST, String> = AST::new("2 2+[5^90~]".to_string(), Vec::new());
    assert_eq!(should_work.is_ok(), true);
    assert_eq!(
        should_work.unwrap().node.children.clone().unwrap()[2],
        ASTnode {
            nodetype: ASTnodeType::Command,
            command: Some(Commands::AddCommand),
            structure: None,
            number: None,
            function: None,
            children: None,
        }
    );
    let should_work: Result<AST, String> = AST::new("4[5^90~]".to_string(), Vec::new());
    assert_eq!(
        should_work.unwrap().node.children.clone().unwrap()[0],
        ASTnode {
            nodetype: ASTnodeType::Number,
            command: None,
            structure: None,
            number: Some(4.0),
            function: None,
            children: None,
        }
    );
    let should_work: Result<AST, String> = AST::new("91".to_string(), Vec::new());
    assert_eq!(
        should_work.unwrap().node.children.clone().unwrap()[0],
        ASTnode {
            nodetype: ASTnodeType::Number,
            command: None,
            structure: None,
            number: Some(91.0),
            function: None,
            children: None,
        }
    );
    let should_work: Result<AST, String> = AST::new("2[180~3[10^90~]]".to_string(), Vec::new());
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
            function: None,
        }
    );
    let quote_test: Result<AST, String> = AST::new("\"+-\"('`'`".to_string(), Vec::new());
    println!("{:#?}", quote_test);
    assert!(false);
}
