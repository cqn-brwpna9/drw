use std::collections::HashMap;
use std::env;
use std::fs;
mod ast;
mod item;
mod stack;
mod turtle;

const THE_NUMBER_OF_RADIANS_IN_A_CIRCLE: f64 = 6.283185307179586;
const EULERS_NUMBER: f64 = 2.7182818284590452;
const THE_NUMBER_OF_DEGREES_IN_A_CIRCLE: f64 = 360.0;

fn read() -> (Result<ast::AST, String>, HashMap<char, ast::AST>) {
    let input: String = env::args().collect::<Vec<String>>()[1].clone();
    let mut program = fs::read_to_string(input).expect("Should have been able to read the file");
    let _ = program.pop();
    let mut functions: HashMap<char, String> = HashMap::new();
    let mut non_function: Vec<String> = Vec::new();
    for line in program.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        let mut line_chars_no_comments: Vec<char> = Vec::new();
        for i in line_chars {
            if i == '#' {
                break; //we've found ourselves a comment
            }
            line_chars_no_comments.push(i)
        }
        if line_chars_no_comments.len() == 1 || line_chars_no_comments.len() == 0 {
            //is (short) line of code
            non_function.push(line_chars_no_comments.into_iter().collect());
        } else {
            if line_chars_no_comments[1] == '_' {
                //Is function
                functions.insert(
                    line_chars_no_comments[0],
                    line_chars_no_comments[2..].to_vec().into_iter().collect(),
                );
            } else {
                //is line of code
                non_function.push(line_chars_no_comments.into_iter().collect());
            }
        }
    }
    let main_code = non_function.join(" ");
    let mut function_names: Vec<char> = Vec::new();
    for i in functions.keys() {
        function_names.push(*i);
    }
    let mut function_asts: HashMap<char, ast::AST> = HashMap::new();
    for (name, body) in &functions {
        function_asts.insert(
            *name,
            ast::AST::new(body.to_string(), function_names.clone()).unwrap(),
        );
    }
    let main_ast = ast::AST::new(main_code, function_names);
    return (main_ast, function_asts);
}
fn eval(
    syntax_tree: ast::AST,
    functions: HashMap<char, ast::AST>,
    data_stack: &mut stack::Stack<item::Item>,
    dip_stack: &mut stack::Stack<item::Item>,
    drawing_turtle: &mut turtle::Turtle,
) -> String {
    evallist(
        syntax_tree.node.children.unwrap(),
        functions,
        data_stack,
        dip_stack,
        drawing_turtle,
    );
    return data_stack.to_string();
}

#[allow(unreachable_patterns)]
fn evallist(
    syntax_tree: Vec<ast::ASTnode>,
    functions: HashMap<char, ast::AST>,
    data_stack: &mut stack::Stack<item::Item>,
    dip_stack: &mut stack::Stack<item::Item>,
    drawing_turtle: &mut turtle::Turtle,
) {
    let mut _throwaway: item::Item;
    for node in syntax_tree {
        match node.nodetype {
            ast::ASTnodeType::Number => data_stack.push(item::Item::from_num(node.number.unwrap())),
            ast::ASTnodeType::Command => match node.command.unwrap() {
                ast::Commands::ForwardCommand => {
                    drawing_turtle.forward(
                        data_stack
                            .pop()
                            .unwrap_or(item::Item::from_num(0.0))
                            .get_number() as f32,
                    );
                    drawing_turtle.push();
                }
                ast::Commands::TurnCommand => drawing_turtle.turn(
                    data_stack
                        .pop()
                        .unwrap_or(item::Item::from_num(0.0))
                        .get_number() as f32,
                ),
                ast::Commands::DuplicateCommand => data_stack.dup(),
                ast::Commands::SwapCommand => data_stack.swap(),
                ast::Commands::PopCommand => _throwaway = data_stack.pop().unwrap(),
                ast::Commands::AddCommand => dyadic_op(&|a, b| a + b, 0.0, data_stack),
                ast::Commands::SubtractCommand => dyadic_op(&|a, b| a - b, 0.0, data_stack),
                ast::Commands::MultiplyCommand => dyadic_op(&|a, b| a * b, 1.0, data_stack),
                ast::Commands::DivideCommand => dyadic_op(&|a, b| a / b, 1.0, data_stack),
                ast::Commands::ModuloCommand => dyadic_op(&|a, b| a % b, 1.0, data_stack),
                ast::Commands::DegreeCommand => {
                    if drawing_turtle.using_degrees() {
                        data_stack.push(item::Item::from_num(THE_NUMBER_OF_DEGREES_IN_A_CIRCLE));
                    } else {
                        drawing_turtle.set_turn_mode(turtle::TurnModes::DEGREE);
                    }
                }
                ast::Commands::RadianCommand => {
                    if drawing_turtle.using_radians() {
                        data_stack.push(item::Item::from_num(THE_NUMBER_OF_RADIANS_IN_A_CIRCLE));
                    } else {
                        drawing_turtle.set_turn_mode(turtle::TurnModes::RADIAN);
                    }
                }
                ast::Commands::ColorCommand => {
                    if data_stack.peek().unwrap().itemtype == item::ItemType::Number {
                        let r = data_stack
                            .pop()
                            .unwrap_or(item::Item::from_num(255.0))
                            .get_number();
                        let g = data_stack
                            .pop()
                            .unwrap_or(item::Item::from_num(255.0))
                            .get_number();
                        let b = data_stack
                            .pop()
                            .unwrap_or(item::Item::from_num(255.0))
                            .get_number();
                        drawing_turtle.set_color(r as u8, g as u8, b as u8);
                    } else {
                        let the_box = data_stack
                            .pop()
                            .unwrap_or(item::Item::from_box(item::DrwBox::from_nums(
                                255.0, 255.0, 255.0,
                            )))
                            .get_box();
                        drawing_turtle.set_color(
                            the_box.r.get_number() as u8,
                            the_box.g.get_number() as u8,
                            the_box.b.get_number() as u8,
                        );
                    }
                }
                ast::Commands::PenDownCommand => drawing_turtle.pen_down(),
                ast::Commands::PenUpCommand => drawing_turtle.pen_up(),
                ast::Commands::SizeCommand => drawing_turtle.set_pen_size(
                    data_stack
                        .pop()
                        .unwrap_or(item::Item::from_num(1.0))
                        .get_number() as f32,
                ),
                ast::Commands::DebugCommand => println!("{}", data_stack.to_string()),
                ast::Commands::PowerCommand => dyadic_op(&|a, b| a.powf(b), 1.0, data_stack),
                ast::Commands::LogCommand => dyadic_op(&|a, b| a.log(b), EULERS_NUMBER, data_stack),
                ast::Commands::EulerNumCommand => {
                    data_stack.push(item::Item::from_num(EULERS_NUMBER))
                }
                ast::Commands::SquareRootCommand => monadic_op(&|a| a.sqrt(), 1.0, data_stack),
                ast::Commands::SineCommand => monadic_op(&|a| a.sin(), 0.0, data_stack),
                ast::Commands::CeilingCommand => monadic_op(&|a| a.ceil(), 0.0, data_stack),
                ast::Commands::FloorCommand => monadic_op(&|a| a.floor(), 0.0, data_stack),
                ast::Commands::RoundCommand => monadic_op(&|a| a.round(), 1.0, data_stack),
                ast::Commands::LessThanCommand => comp_op(&|a, b| a < b, data_stack),
                ast::Commands::GreaterThanCommand => comp_op(&|a, b| a > b, data_stack),
                ast::Commands::EqualCommand => comp_op(&|a, b| a == b, data_stack),
                ast::Commands::DipCommand => data_stack.dip(dip_stack),
                ast::Commands::UndipCommand => dip_stack.dip(data_stack),
                ast::Commands::BoxCommand => {
                    let r = data_stack.pop().unwrap_or(item::Item::from_num(0.0));
                    let g = data_stack.pop().unwrap_or(item::Item::from_num(0.0));
                    let b = data_stack.pop().unwrap_or(item::Item::from_num(0.0));
                    data_stack.push(item::Item::from_box(item::DrwBox::new(r, g, b)));
                }
                ast::Commands::UnboxCommand => {
                    let the_box = data_stack
                        .pop()
                        .unwrap_or(item::Item::from_box(item::DrwBox::from_nums(0.0, 0.0, 0.0)))
                        .get_box();
                    data_stack.push(the_box.b);
                    data_stack.push(the_box.g);
                    data_stack.push(the_box.r);
                }
                _ => unreachable!(), //should never happen. make this "a bug was found in the interpreter" error
            },
            ast::ASTnodeType::ControlStructure => match node.structure.unwrap() {
                ast::ControlStructures::RepeatLoop => {
                    let n = data_stack.pop().unwrap_or(item::Item::from_num(0.0));
                    if n.itemtype == item::ItemType::Box {
                        panic!("Cannot iterate over boxes!\n Got: {n}")
                    }
                    let num: f64 = n.get_number();
                    if num != num.floor() {
                        panic!("Cannot iterate a non-whole number of times!\n Got: {num}")
                    }
                    if num < 0.0 {
                        panic!(
                            "Cannot iterate a negative number of times! (this isnt Uiua)\n Got: {num}"
                        )
                    }
                    for _ in 0..num as u64 {
                        evallist(
                            node.children.clone().unwrap(),
                            functions.clone(),
                            data_stack,
                            dip_stack,
                            drawing_turtle,
                        );
                    }
                }
                ast::ControlStructures::WhileLoop => {
                    while data_stack.pop().unwrap().is_truthy() {
                        evallist(
                            node.children.clone().unwrap(),
                            functions.clone(),
                            data_stack,
                            dip_stack,
                            drawing_turtle,
                        );
                    }
                }
            },
            ast::ASTnodeType::Function => evallist(
                functions
                    .get(&node.function.unwrap())
                    .unwrap()
                    .node
                    .children
                    .clone()
                    .unwrap(),
                functions.clone(),
                data_stack,
                dip_stack,
                drawing_turtle,
            ),
            ast::ASTnodeType::Container => unreachable!(), //should never happen. make this "a bug was found in the interpreter" error
        }
    }
}

fn print(string: String) {
    println!("{string}");
}

fn main() {
    let mut data_stack: stack::Stack<item::Item> = stack::Stack::new();
    let mut dip_stack: stack::Stack<item::Item> = stack::Stack::new();
    let mut drawing_turtle = turtle::Turtle::new();

    let asts_to_pass = read();
    if asts_to_pass.0.is_ok() {
        print(eval(
            asts_to_pass.0.unwrap(),
            asts_to_pass.1,
            &mut data_stack,
            &mut dip_stack,
            &mut drawing_turtle,
        ));
    } else {
        println!("{}", asts_to_pass.0.unwrap_err());
    }
    if drawing_turtle.should_render() {
        drawing_turtle.render();
    }
}
fn dyadic_op(f: &dyn Fn(f64, f64) -> f64, default: f64, data_stack: &mut stack::Stack<item::Item>) {
    let a = data_stack.pop().unwrap_or(item::Item::from_num(default));
    let b = data_stack.pop().unwrap_or(item::Item::from_num(default));
    data_stack.push(_apply_dyadic_op(f, a, b));
}
//there is probably some optimization that could be done here because you can quicken somthing once you know one arg is numeric
fn _apply_dyadic_op(f: &dyn Fn(f64, f64) -> f64, a: item::Item, b: item::Item) -> item::Item {
    if a.itemtype == item::ItemType::Number {
        if b.itemtype == item::ItemType::Number {
            //a and b are both numbers, so just normally apply
            item::Item::from_num(f(a.get_number(), b.get_number()))
        } else {
            //a is a number, b is a box so apply to all elements of b using this function
            item::Item::from_box(item::DrwBox::new(
                _apply_dyadic_op(f, a.clone(), b.clone().get_box().r),
                _apply_dyadic_op(f, a.clone(), b.clone().get_box().g),
                _apply_dyadic_op(f, a.clone(), b.clone().get_box().b),
            ))
        }
    } else {
        if b.itemtype == item::ItemType::Number {
            //a is a box and b is a number, so do the same thing as (num, box)
            item::Item::from_box(item::DrwBox::new(
                _apply_dyadic_op(f, a.clone().get_box().r, b.clone()),
                _apply_dyadic_op(f, a.clone().get_box().g, b.clone()),
                _apply_dyadic_op(f, a.clone().get_box().b, b.clone()),
            ))
        } else {
            //both are boxes so ~~its scary~~ it applys each element to the one in the other arg
            item::Item::from_box(item::DrwBox::new(
                _apply_dyadic_op(f, a.clone().get_box().r, b.clone().get_box().r),
                _apply_dyadic_op(f, a.clone().get_box().g, b.clone().get_box().g),
                _apply_dyadic_op(f, a.clone().get_box().b, b.clone().get_box().b),
            ))
        }
    }
}
fn monadic_op(f: &dyn Fn(f64) -> f64, default: f64, data_stack: &mut stack::Stack<item::Item>) {
    let a = data_stack.pop().unwrap_or(item::Item::from_num(default));
    data_stack.push(_apply_monadic_op(f, a));
}
fn _apply_monadic_op(f: &dyn Fn(f64) -> f64, a: item::Item) -> item::Item {
    if a.itemtype == item::ItemType::Number {
        item::Item::from_num(f(a.get_number()))
    } else {
        item::Item::from_box(item::DrwBox::new(
            _apply_monadic_op(f, a.clone().get_box().r),
            _apply_monadic_op(f, a.clone().get_box().g),
            _apply_monadic_op(f, a.clone().get_box().b),
        ))
    }
}
fn comp_op(f: &dyn Fn(f64, f64) -> bool, data_stack: &mut stack::Stack<item::Item>) {
    let a = data_stack.pop();
    let b = data_stack.pop();
    //comparison operators return false by default, which is why they are special cased
    data_stack.push(match a {
        Some(a_item) => match b {
            Some(b_item) => {
                //turn f into a regular function so it can be passed to _apply_dyadic_op
                let f: &dyn Fn(f64, f64) -> f64 = &|a, b| if f(a, b) { 1.0 } else { 0.0 };
                _apply_dyadic_op(f, a_item, b_item)
            }
            None => item::Item::from_num(0.0),
        },
        None => item::Item::from_num(0.0),
    });
}
