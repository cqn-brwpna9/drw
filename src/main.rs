use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use text_io::read as readin;
use turtle::Turtle;
mod ast;
mod stack;

const THE_NUMBER_OF_RADIANS_IN_A_CIRCLE: f64=6.283185307179586;
const EULERS_NUMBER: f64=2.7182818284590452;
const THE_NUMBER_OF_DEGREES_IN_A_CIRCLE: f64=360.0;

fn read() -> Result<ast::AST, String> {
    print!(">");
    let mut input = Vec::new();
    let linein: String = readin!("{}\n");
    let _ = BufReader::new(File::open(linein.as_str()).unwrap()).read_until(b'\0', &mut input);
    let _ = input.pop(); //take off the ending null charicter
    return ast::AST::new(
        String::from_utf8(input).unwrap_or("Hey, theres somthing wrong with that file".to_string()),
    );
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

#[allow(unreachable_patterns)]
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
                ast::Commands::DegreeCommand=>{
                  if turtle.is_using_degrees(){
                      data_stack.push(THE_NUMBER_OF_DEGREES_IN_A_CIRCLE);
                  }else{
                      turtle.use_degrees();
                  }  
                }
                ast::Commands::RadianCommand=>{
                  if turtle.is_using_radians(){
                      data_stack.push(THE_NUMBER_OF_RADIANS_IN_A_CIRCLE);
                  }else{
                      turtle.use_radians();
                  }  
                }
                ast::Commands::ColorCommand=>{
                    let r=data_stack.pop().unwrap();
                    let g=data_stack.pop().unwrap();
                    let b=data_stack.pop().unwrap();
                    turtle.set_pen_color([r,g,b]);
                }
                ast::Commands::PenDownCommand=>turtle.pen_down(),
                ast::Commands::PenUpCommand=>turtle.pen_up(),
                ast::Commands::SizeCommand=>turtle.set_pen_size(data_stack.pop().unwrap()),
                ast::Commands::DebugCommand=>println!("{}",data_stack.to_string()),
                ast::Commands::PowerCommand=>{
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a.powf(b));
                }
                ast::Commands::LogCommand=>{
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a.log(b));
                }
                ast::Commands::EulerNumCommand=>data_stack.push(EULERS_NUMBER),
                ast::Commands::SquareRootCommand=>{//I don't love doing this this way, but what rustc wants, rustc gets
                    let a=data_stack.pop().unwrap();
                    data_stack.push(a.sqrt());
                }
                ast::Commands::SineCommand=>{
                    let a=data_stack.pop().unwrap();
                    data_stack.push(a.sin());
                }
                ast::Commands::CeilingCommand=>{
                    let a=data_stack.pop().unwrap();
                    data_stack.push(a.ceil());
                }
                ast::Commands::FloorCommand=>{
                    let a=data_stack.pop().unwrap();
                    data_stack.push(a.floor());
                }
                ast::Commands::RoundCommand=>{
                    let a=data_stack.pop().unwrap();
                    data_stack.push(a.round());
                }
                ast::Commands::LessThanCommand=>{
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    if a<b{
                        data_stack.push(1.);
                    }else{
                        data_stack.push(0.);
                   }
                }
                ast::Commands::GreaterThanCommand=>{
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    if a>b{
                        data_stack.push(1.);
                    }else{
                        data_stack.push(0.);
                   }
                }
                ast::Commands::EqualCommand=>{
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    if a==b{
                        data_stack.push(1.);
                    }else{
                        data_stack.push(0.);
                   }
                }
                _=>panic!("You somehow made an nonsense ASTnode. Good job! If you are getting this error and you have not messed with the code, I really don't know what's going on. If you have, you made the issue"),//Everything should be matched
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
                ast::ControlStructures::WhileLoop | ast::ControlStructures::DipBlock => todo!(),
            },
            ast::ASTnodeType::Container => todo!(),
        }
    }
}

fn print(string: String) {
    println!("{string}");
}

fn main() {
    turtle::start();
    let mut data_stack: stack::Stack<f64> = stack::Stack::new();
    let mut dip_stack: stack::Stack<f64> = stack::Stack::new();
    let ast_to_pass = read();
    if ast_to_pass.is_ok() {
        print(eval(ast_to_pass.unwrap(), &mut data_stack, &mut dip_stack));
    } else {
        println!("{}", ast_to_pass.unwrap_err());
    }
}
