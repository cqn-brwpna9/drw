use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum ItemType{
    Number,
    Box,
}

#[derive(PartialEq, Clone, Debug)]
pub struct DrwBox{
    pub r:f64,
    pub g:f64,
    pub b:f64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Item{
    pub itemtype: ItemType,
    pub number: Option<f64>,
    pub boxed: Option<DrwBox>,
}

impl DrwBox{
    pub fn new(newr:f64,newg:f64,newb:f64)->DrwBox{
        DrwBox{r:newr, g:newg, b:newb}
    }
}

impl fmt::Display for DrwBox{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "〚{},{},{}〛", self.clone().r, self.clone().g, self.clone().b)
    }
}

impl Item{
    pub fn from_num(item:f64)->Item{
        Item{itemtype:ItemType::Number, number:Some(item), boxed:None}
    }
    pub fn from_box(item:DrwBox)->Item{
        Item{itemtype:ItemType::Box, number:None, boxed:Some(item)}
    }
    pub fn get_number(self)->f64{
        if self.itemtype==ItemType::Number{
            return self.number.unwrap();
        }else{
            panic!("Boxes cannot be operated on");
        }
    }
    pub fn get_box(self)->DrwBox{
        if self.itemtype==ItemType::Box{
            return self.boxed.unwrap();
        }else{
            panic!("Exepected a box");
        }
    }
}

impl fmt::Display for Item{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.itemtype==ItemType::Number{
            return write!(f, "{}", self.clone().get_number().to_string());
        }else{
            return write!(f, "{}", self.clone().get_box().to_string());
        }
    }
}
