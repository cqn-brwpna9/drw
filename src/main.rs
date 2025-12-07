use std::env;
use std::fs;
mod ast;
mod stack;
mod turtle;

const THE_NUMBER_OF_RADIANS_IN_A_CIRCLE: f64 = 6.283185307179586;
const EULERS_NUMBER: f64 = 2.7182818284590452;
const THE_NUMBER_OF_DEGREES_IN_A_CIRCLE: f64 = 360.0;

fn read() -> Result<ast::AST, String> {
    let input: String = env::args().collect::<Vec<String>>()[1].clone();
    let mut program = fs::read_to_string(input).expect("Should have been able to read the file");
    let _ = program.pop();
    return ast::AST::new(program);
}

fn eval(
    syntax_tree: ast::AST,
    data_stack: &mut stack::Stack<f64>,
    dip_stack: &mut stack::Stack<f64>,
    drawing_turtle: &mut turtle::Turtle,
) -> String {
    evallist(
        syntax_tree.node.children.unwrap(),
        data_stack,
        dip_stack,
        drawing_turtle,
    );
    return data_stack.to_string();
}

#[allow(unreachable_patterns)]
fn evallist(
    syntax_tree: Vec<ast::ASTnode>,
    data_stack: &mut stack::Stack<f64>,
    dip_stack: &mut stack::Stack<f64>,
    drawing_turtle: &mut turtle::Turtle,
) {
    let mut _throwaway: f64 = 0.0;
    for node in syntax_tree {
        match node.nodetype {
            ast::ASTnodeType::Number => data_stack.push(node.number.unwrap()),
            ast::ASTnodeType::Command => match node.command.unwrap() {
                ast::Commands::ForwardCommand => {
                    drawing_turtle.forward(data_stack.pop().unwrap() as f32);
                    drawing_turtle.push();
                }
                ast::Commands::TurnCommand => drawing_turtle.turn(data_stack.pop().unwrap() as f32),
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
                ast::Commands::DegreeCommand => {
                    if drawing_turtle.using_degrees() {
                        data_stack.push(THE_NUMBER_OF_DEGREES_IN_A_CIRCLE);
                    } else {
                        drawing_turtle.set_turn_mode(turtle::TurnModes::DEGREE);
                    }
                }
                ast::Commands::RadianCommand => {
                    if drawing_turtle.using_radians() {
                        data_stack.push(THE_NUMBER_OF_RADIANS_IN_A_CIRCLE);
                    } else {
                        drawing_turtle.set_turn_mode(turtle::TurnModes::RADIAN);
                    }
                }
                ast::Commands::ColorCommand => {
                    let r = data_stack.pop().unwrap();
                    let g = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    drawing_turtle.set_color(r as u8, g as u8, b as u8);
                }
                ast::Commands::PenDownCommand => drawing_turtle.pen_down(),
                ast::Commands::PenUpCommand => drawing_turtle.pen_up(),
                ast::Commands::SizeCommand => {
                    drawing_turtle.set_pen_size(data_stack.pop().unwrap() as f32)
                }
                ast::Commands::DebugCommand => println!("{}", data_stack.to_string()),
                ast::Commands::PowerCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a.powf(b));
                }
                ast::Commands::LogCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    data_stack.push(a.log(b));
                }
                ast::Commands::EulerNumCommand => data_stack.push(EULERS_NUMBER),
                ast::Commands::SquareRootCommand => {
                    //I don't love doing this this way, but what rustc wants, rustc gets
                    let a = data_stack.pop().unwrap();
                    data_stack.push(a.sqrt());
                }
                ast::Commands::SineCommand => {
                    let a = data_stack.pop().unwrap();
                    data_stack.push(a.sin());
                }
                ast::Commands::CeilingCommand => {
                    let a = data_stack.pop().unwrap();
                    data_stack.push(a.ceil());
                }
                ast::Commands::FloorCommand => {
                    let a = data_stack.pop().unwrap();
                    data_stack.push(a.floor());
                }
                ast::Commands::RoundCommand => {
                    let a = data_stack.pop().unwrap();
                    data_stack.push(a.round());
                }
                ast::Commands::LessThanCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    if a < b {
                        data_stack.push(1.);
                    } else {
                        data_stack.push(0.);
                    }
                }
                ast::Commands::GreaterThanCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    if a > b {
                        data_stack.push(1.);
                    } else {
                        data_stack.push(0.);
                    }
                }
                ast::Commands::EqualCommand => {
                    let a = data_stack.pop().unwrap();
                    let b = data_stack.pop().unwrap();
                    if a == b {
                        data_stack.push(1.);
                    } else {
                        data_stack.push(0.);
                    }
                }
                ast::Commands::DipCommand => data_stack.dip(dip_stack),
                ast::Commands::UndipCommand => dip_stack.dip(data_stack),
                _ => unreachable!(),
            },
            ast::ASTnodeType::ControlStructure => match node.structure.unwrap() {
                ast::ControlStructures::RepeatLoop => {
                    let n = data_stack.pop().unwrap() as i64;
                    for _ in 0..n {
                        evallist(
                            node.children.clone().unwrap(),
                            data_stack,
                            dip_stack,
                            drawing_turtle,
                        );
                    }
                }
                ast::ControlStructures::WhileLoop => {
                    while data_stack.pop().unwrap() as f64 != 0.0 {
                        evallist(
                            node.children.clone().unwrap(),
                            data_stack,
                            dip_stack,
                            drawing_turtle,
                        );
                    }
                }
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
    let mut drawing_turtle = turtle::Turtle::new();
    let ast_to_pass = read();
    if ast_to_pass.is_ok() {
        print(eval(
            ast_to_pass.unwrap(),
            &mut data_stack,
            &mut dip_stack,
            &mut drawing_turtle,
        ));
    } else {
        println!("{}", ast_to_pass.unwrap_err());
    }
    if drawing_turtle.should_render() {
        drawing_turtle.render();
    }
}
