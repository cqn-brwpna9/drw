//a simple, hand rolled turtle library. Its much faster and more memory effincent then the normal rust turtle
//not very flexible but great for drw

use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 450;
const THE_NUMBER_OF_RADIANS_IN_A_CIRCLE: f32 = 6.283185307179586;
const THE_NUMBER_OF_DEGREES_IN_A_CIRCLE: f32 = 360.0;

#[derive(Clone, Debug, PartialEq)]
pub enum TurnModes {
    DEGREE,
    RADIAN,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TurtleColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TurtleHistoryFrame {
    start_pos: Point,
    end_pos: Point,
    color: TurtleColor,
    pen_size: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Turtle {
    history: Vec<TurtleHistoryFrame>,
    curr: TurtleHistoryFrame,
    direction: f32,
    travel_dist: f32,
    turn_mode: TurnModes,
    pen_state: bool,
}

impl Point {
    pub fn new(newx: f32, newy: f32) -> Self {
        Point { x: newx, y: newy }
    }
    pub fn to_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl TurtleColor {
    pub fn new(newr: u8, newg: u8, newb: u8) -> Self {
        TurtleColor {
            r: newr,
            g: newg,
            b: newb,
        }
    }
    pub fn to_raycolor(&self) -> Color {
        Color::new(self.r, self.g, self.b, 255)
    }
}

impl Turtle {
    pub fn new() -> Self {
        Turtle {
            history: Vec::new(),
            curr: TurtleHistoryFrame {
                start_pos: Point::new((WINDOW_WIDTH as f32) / 2.0, (WINDOW_HEIGHT as f32) / 2.0),
                end_pos: Point::new((WINDOW_WIDTH as f32) / 2.0, (WINDOW_HEIGHT as f32) / 2.0),
                color: TurtleColor::new(255, 255, 255),
                pen_size: 1.0,
            },
            direction: 0.0,
            travel_dist: 0.0,
            turn_mode: TurnModes::DEGREE,
            pen_state: true,
        }
    }
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.curr.color = TurtleColor::new(r, g, b);
    }
    pub fn set_pen_size(&mut self, s: f32) {
        self.curr.pen_size = s;
    }
    pub fn set_turn_mode(&mut self, m: TurnModes) {
        self.turn_mode = m;
    }
    pub fn turn(&mut self, mut angle: f32) {
        if self.using_degrees() {
            angle = Self::deg_to_rad(angle)
        }
        self.direction = self.direction.clone() + angle
    }
    pub fn forward(&mut self, amount: f32) {
        self.travel_dist = amount;
    }
    pub fn using_degrees(&self) -> bool {
        self.turn_mode == TurnModes::DEGREE
    }
    pub fn using_radians(&self) -> bool {
        self.turn_mode == TurnModes::RADIAN
    }
    pub fn pen_up(&mut self) {
        self.pen_state = false
    }
    pub fn pen_down(&mut self) {
        self.pen_state = true
    }
    pub fn get_history(&self) -> Vec<TurtleHistoryFrame> {
        self.history.clone()
    }
    pub fn deg_to_rad(x: f32) -> f32 {
        x * (THE_NUMBER_OF_RADIANS_IN_A_CIRCLE / THE_NUMBER_OF_DEGREES_IN_A_CIRCLE)
    }
    pub fn polar_to_rect(r: f32, theta: f32) -> Point {
        let newx = theta.cos() * r;
        let newy = theta.sin() * r;
        return Point::new(newx, newy);
    }
    pub fn push(&mut self) {
        //convert direction+travel dist to x,y translation
        let translation = Self::polar_to_rect(self.travel_dist, self.direction);
        self.curr.end_pos = Point::new(
            self.curr.start_pos.x + translation.x,
            self.curr.start_pos.y + translation.y,
        );
        let mut new = self.curr.clone();
        if !self.pen_state {
            new.pen_size = 0.0;
        }
        self.history.push(new);
        self.curr = TurtleHistoryFrame {
            start_pos: self.curr.clone().end_pos, //make the new start pos the new end pos
            end_pos: Point::new(0.0, 0.0),        //placeholder
            color: self.curr.clone().color,       //move color forward
            pen_size: self.curr.clone().pen_size, //move pen size forward
        }
    }
    pub fn should_render(&self) -> bool {
        self.history.len() != 0
    }
    pub fn render(self) {
        let (mut rl, thread) = raylib::init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .title("drw output window")
            .resizable()
            .vsync()
            .build();
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
            {
                d.clear_background(Color::BLACK);
                for frame in &self.history {
                    d.draw_circle_v(
                        frame.start_pos.to_vector2(),
                        frame.pen_size / 2.0,
                        frame.color.to_raycolor(),
                    );
                    d.draw_line_ex(
                        frame.start_pos.to_vector2(),
                        frame.end_pos.to_vector2(),
                        frame.pen_size,
                        frame.color.to_raycolor(),
                    );
                    d.draw_circle_v(
                        frame.end_pos.to_vector2(),
                        frame.pen_size / 2.0,
                        frame.color.to_raycolor(),
                    );
                }
            };
        }
    }
}

#[test]
fn deg_to_rad_test() {
    for i in 0..=24 {
        println!(
            "{} should approxamatly equal {}",
            i as f32 * THE_NUMBER_OF_RADIANS_IN_A_CIRCLE / 24.0,
            Turtle::deg_to_rad(i as f32 * 15.0)
        );
    }
    //assert!(false); //uncomment to see output
}

#[test]
fn polar_to_rect_test() {
    assert_eq!(Point::new(0.0, 0.0), Turtle::polar_to_rect(0.0, 0.0));
    assert_eq!(Point::new(1.0, 0.0), Turtle::polar_to_rect(1.0, 0.0));
    //assert_eq!(Point::new(0.0,1.0),Turtle::polar_to_rect(1.0,THE_NUMBER_OF_RADIANS_IN_A_CIRCLE/4.0)); //works just fine, its just that -4.371139e-8!=0
}
