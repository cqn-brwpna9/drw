use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub struct DrwBox {
    pub r: Item,
    pub g: Item,
    pub b: Item,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Item {
    Nil,
    Number(f64),
    Box(Box<DrwBox>),
}

impl DrwBox {
    pub fn new(newr: Item, newg: Item, newb: Item) -> DrwBox {
        DrwBox {
            r: newr,
            g: newg,
            b: newb,
        }
    }
    pub fn from_nums(newr: f64, newg: f64, newb: f64) -> DrwBox {
        DrwBox {
            r: Item::from_num(newr),
            g: Item::from_num(newg),
            b: Item::from_num(newb),
        }
    }
}

impl fmt::Display for DrwBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "〚{} {} {}〛",
            self.clone().r,
            self.clone().g,
            self.clone().b,
        )
    }
}

impl Item {
    pub fn from_num(n: f64) -> Item {
        Item::Number(n)
    }
    pub fn from_box(bx: DrwBox) -> Item {
        Item::Box(Box::new(bx))
    }
    pub fn zero() -> Item {
        Item::Number(0.0)
    }
    pub fn get_number(self) -> f64 {
        match self {
            Item::Number(n) => n,
            Item::Box(bx) => bx.r.get_number(),
            Item::Nil => {
                panic!("Tried to call get_number on a nil value. This is a bug in the interpreter")
            }
        }
    }
    pub fn get_number_or(self, or: f64) -> f64 {
        match self {
            Item::Number(n) => n,
            Item::Box(bx) => bx.r.get_number(),
            Item::Nil => or,
        }
    }
    pub fn get_box(self) -> DrwBox {
        match self {
            Item::Box(bx) => *bx,
            Item::Number(n) => DrwBox::from_nums(n, 0.0, 0.0),
            Item::Nil => {
                panic!("Tried to call get_box on a nil value. This is a bug in the interpreter")
            }
        }
    }
    pub fn is_truthy(&self) -> bool {
        match self {
            Item::Number(n) => *n != 0.0,
            Item::Nil => false,
            Item::Box(bx) => bx.r.is_truthy() || bx.g.is_truthy() || bx.b.is_truthy(),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Number(n) => write!(f, "{}", n),
            Item::Nil => write!(f, ","),
            Item::Box(bx) => write!(f, "{}", *bx),
        }
    }
}
