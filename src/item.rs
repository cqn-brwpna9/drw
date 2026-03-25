use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum ItemType {
    Number,
    Box,
}

#[derive(PartialEq, Clone, Debug)]
pub struct DrwBox {
    pub r: Item,
    pub g: Item,
    pub b: Item,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Item {
    pub itemtype: ItemType,
    pub number: Option<f64>,
    pub boxed: Option<Box<DrwBox>>,
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
            self.clone().r.to_string(),
            self.clone().g.to_string(),
            self.clone().b.to_string(),
        )
    }
}

impl Item {
    pub fn from_num(item: f64) -> Item {
        Item {
            itemtype: ItemType::Number,
            number: Some(item),
            boxed: None,
        }
    }
    pub fn from_box(item: DrwBox) -> Item {
        Item {
            itemtype: ItemType::Box,
            number: None,
            boxed: Some(Box::new(item)),
        }
    }
    pub fn get_number(self) -> f64 {
        if self.itemtype == ItemType::Number {
            self.number.unwrap()
        } else {
            self.boxed.unwrap().r.get_number()
        }
    }
    pub fn get_box(self) -> DrwBox {
        if self.itemtype == ItemType::Box {
            *(self.boxed.unwrap())
        } else {
            DrwBox::from_nums(self.number.unwrap(), 0.0, 0.0)
        }
    }
    pub fn is_truthy(self) -> bool {
        if self.itemtype == ItemType::Number {
            self.number.unwrap() != 0.0
        } else {
            let item = *(self.boxed.unwrap());
            if item != DrwBox::from_nums(0.0, 0.0, 0.0) {
                item.r.is_truthy() || item.g.is_truthy() || item.b.is_truthy()
            } else {
                false
            }
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.itemtype == ItemType::Number {
            write!(f, "{}", self.clone().get_number().to_string())
        } else {
            write!(f, "{}", self.clone().get_box().to_string())
        }
    }
}
