pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> where T:Clone {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn length(&self) -> usize {
        self.stack.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
    pub fn dip(&mut self, top: &mut Stack<T>) {
        top.push(self.pop().unwrap())
    }
    pub fn swap(&mut self) {
        let temp: T = self.pop().unwrap();
        let temp2: T = self.pop().unwrap();
        self.push(temp);
        self.push(temp2);
    }
    pub fn dup(&mut self){
        self.push(self.peek().unwrap().clone())
    }
}
#[test]
fn push_pop_test() {
    let mut a: Stack<u8> = Stack::new();
    let o;
    a.push(2);
    a.push(1);
    o = a.pop();
    assert_eq!(o.unwrap(), 1);
    assert_eq!(a.pop().unwrap(),2);
}
#[test]
fn is_empty_test() {
    let mut a: Stack<u8> = Stack::new();
    assert!(a.is_empty());
    a.push(1);
    assert!(!a.is_empty());
    a.pop();
    assert!(a.is_empty());
}
#[test]
fn length_test(){
    let mut a: Stack<u8> = Stack::new();
    assert_eq!(a.length() as u8, 0);
    a.push(1);
    a.push(2);
    a.push(3);
    assert_eq!(a.length() as u8, 3);
}
#[test]
fn peek_test(){
    let mut a: Stack<u8> = Stack::new();
    a.push(1);
    assert_eq!(*a.peek().unwrap(),1);
    assert_eq!(a.pop().unwrap(),1);
}
#[test]
fn dip_test(){
    let mut a: Stack<u8> = Stack::new();
    let mut b: Stack<u8> = Stack::new();
    a.push(3);
    a.push(1);
    a.dip(&mut b);
    assert_eq!(*b.peek().unwrap(),1);
    assert_eq!(*a.peek().unwrap(),3);
    a.push(2);
    b.dip(&mut a);
    assert_eq!(b.length() as u8,0);
    assert_eq!(a.pop().unwrap(),1);
    assert_eq!(a.pop().unwrap(),2);
    assert_eq!(a.pop().unwrap(),3);
}
#[test]
fn swap_test(){
    let mut a: Stack<u8> = Stack::new();
    a.push(1);
    a.push(2);
    a.swap();
    assert_eq!(a.pop().unwrap(),1);
    assert_eq!(a.pop().unwrap(),2);
}
#[test]
fn dup_test(){
    let mut a: Stack<u8> = Stack::new();
    a.push(1);
    a.dup();
    assert_eq!(a.pop().unwrap(),1);
    assert_eq!(a.pop().unwrap(),1);
}
