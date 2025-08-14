use text_io::read as readin;

fn read() -> String{
    print!(">");
    let input=readin!("{}\n");
    return input;
}

fn eval(ast: String)->String{
	println!("in eval, got {ast}");
	return ast;
}

fn print(string: String){
	println!("{string}");
}


fn main() {
    loop{
       print(eval(read()));
    }
}

