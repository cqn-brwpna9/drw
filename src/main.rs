use text_io::read as readin;
use turtle::Turtle;
mod ast;
mod stack;

fn read() -> Result<ast::AST, String> {
    print!(">");
    let input = readin!("{}\n");
    return ast::AST::new(input);
}

fn eval(
    syntax_tree: ast::AST,
    data_stack: &mut stack::Stack<f64>,
    dip_stack: &mut stack::Stack<f64>,
) -> String {
    let mut turtle = Turtle::new();
    turtle.set_speed(20);
    evallist(
        syntax_tree.node.children.unwrap(),
        data_stack,
        dip_stack,
        &mut turtle,
    );
    return data_stack.to_string();
}

fn evallist(
    syntax_tree: Vec<ast::ASTnode>,
    data_stack: &mut stack::Stack<f64>,
    dip_stack: &mut stack::Stack<f64>,
    turtle: &mut Turtle,
) {
    let mut _throwaway: f64 = 0.0;
    for node in syntax_tree {
        match node.nodetype {
            ast::ASTnodeType::Number => data_stack.push(node.number.unwrap()),
            ast::ASTnodeType::Command => match node.command.unwrap() {
                ast::Commands::ForwardCommand => turtle.forward(data_stack.pop().unwrap()),
                ast::Commands::TurnCommand => turtle.right(data_stack.pop().unwrap()),
                ast::Commands::DuplicateCommand => data_stack.dup(),
                ast::Commands::SwapCommand => data_stack.swap(),
                ast::Commands::PopCommand => _throwaway = data_stack.pop().unwrap(),
                ast::Commands::AddCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a + b);
                }
                ast::Commands::SubtractCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a - b);
                }
                ast::Commands::MultiplyCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a * b);
                }
                ast::Commands::DivideCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a / b);
                }
                ast::Commands::ModuloCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a % b);
                }
            },
            ast::ASTnodeType::ControlStructure => match node.structure.unwrap() {
                ast::ControlStructures::RepeatLoop => {
                    let n = data_stack.pop().unwrap() as i64;
                    for _ in 0..n {
                        evallist(
                            node.children.clone().unwrap(),
                            data_stack,
                            dip_stack,
                            turtle,
                        );
                    }
                }
                ast::ControlStructures::WhileLoop
                | ast::ControlStructures::IfBlock
                | ast::ControlStructures::DipBlock => todo!(),
            },
            ast::ASTnodeType::Container => todo!(),
        }
    }
}

fn print(string: String) {
    println!("{string}");
}

fn main() {
    let mut data_stack: stack::Stack<f64> = stack::Stack::new();
    let mut dip_stack: stack::Stack<f64> = stack::Stack::new();
    let ast_to_pass = read();
    if ast_to_pass.is_ok() {
        print(eval(ast_to_pass.unwrap(), &mut data_stack, &mut dip_stack));
    } else {
        println!("{}", ast_to_pass.unwrap_err());
    }
}
