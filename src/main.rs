use text_io::read as readin;
mod ast;
mod stack;

fn read() -> String {
    print!(">");
    let input = readin!("{}\n");
    return input;
}

fn eval(
    ast: String,
    data_stack: &mut stack::Stack<f64>,
    dip_stack: &mut stack::Stack<f64>,
) -> String {
    println!("in eval, got {ast}");
    return ast;
}

fn print(string: String) {
    println!("{string}");
}

fn main() {
    let mut data_stack: stack::Stack<f64> = stack::Stack::new();
    let mut dip_stack: stack::Stack<f64> = stack::Stack::new();
    loop {
        print(eval(read(), &mut data_stack, &mut dip_stack));
    }
}
