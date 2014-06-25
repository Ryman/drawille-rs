#![crate_type = "lib"]
#![crate_id = "drawille"]

//! drawille—a terminal graphics library for Rust, based on the Python library
//! [drawille](https://github.com/asciimoo/drawille).

use std::collections::HashMap;
use std::cmp;

static PIXEL_MAP: [[int, ..2], ..4] = [[0x01, 0x08],
                                       [0x02, 0x10],
                                       [0x04, 0x20],
                                       [0x40, 0x80]];

#[deriving(Clone, Show, PartialEq, Eq)]
pub struct Canvas {
    chars: HashMap<(uint, uint), int>,
    width:  uint,
    height: uint,
}

impl Canvas {
    pub fn new(width: uint, height: uint) -> Canvas {
        Canvas {
            chars: HashMap::new(),
            width: width / 2,
            height: height / 4,
        }
    }

    pub fn clear(&mut self) {
        self.chars.clear();
    }

    pub fn set(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        *self.chars.find_or_insert((row, col), 0) |= PIXEL_MAP[y % 4][x % 2];
    }

    pub fn unset(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        *self.chars.find_or_insert((row, col), 0) &= !PIXEL_MAP[y % 4][x % 2];
    }

    pub fn toggle(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        *self.chars.find_or_insert((row, col), 0) ^= PIXEL_MAP[y % 4][x % 2];
    }

    pub fn get(&self, x: uint, y: uint) -> bool {
        let dot_index = PIXEL_MAP[y % 4][x % 2];
        let (col, row) = (x / 2, y / 4);
        let char = self.chars.find(&(row, col));
        
        match char {
            None => false,
            Some(c) => c & dot_index != 0,
        }
    }

    pub fn rows(&self) -> Vec<String> {
        let maxrow = cmp::max(self.width, self.chars.keys().map(|&(x, _)| x).max().unwrap_or(0));
        let maxcol = cmp::max(self.height, self.chars.keys().map(|&(_, y)| y).max().unwrap_or(0));

        let mut result = vec![];
        for y in range(0, maxcol + 1) {
            let mut row = String::new();
            for x in range(0, maxrow + 1) {
                let char = *self.chars.find(&(x, y)).unwrap_or(&0);
                row.push_char(if char == 0 {
                    ' '
                } else {
                    std::char::from_u32((0x2800 + char) as u32).unwrap()
                })
            }
            result.push(row);
        }
        result
    }

    pub fn frame(&self) -> String {
        self.rows().move_iter().collect::<Vec<String>>().connect("\n")
    }

    pub fn line_vec(&self, x1: uint, y1: uint, x2: uint, y2: uint) -> Vec<(uint, uint)> {
        let xdiff = cmp::max(x1, x2) - cmp::min(x1, x2);
        let ydiff = cmp::max(y1, y2) - cmp::min(y1, y2);
        let xdir = if x1 <= x2 { 1 } else { -1 };
        let ydir = if y1 <= y2 { 1 } else { -1 };

        let r = cmp::max(xdiff, ydiff);

        let mut result = vec![];
        for i in range(0, r + 1) {
            let mut x = x1;
            let mut y = y1;

            if ydiff != 0 {
                y += (i * ydiff) / r * ydir;
            }
            if xdiff != 0 {
                x += (i * xdiff) / r * xdir;
            }

            result.push((x, y));
        }
        result
    }

    pub fn line(&mut self, x1: uint, y1: uint, x2: uint, y2: uint) {
        for &(x, y) in self.line_vec(x1, y1, x2, y2).iter() {
            self.set(x, y);
        }
    }
}

pub struct Turtle {
    pub x: f32,
    pub y: f32,
    pub brush: bool,
    pub rotation: f32,
    cvs: Canvas,
}

impl Turtle {
    pub fn new(x: f32, y: f32) -> Turtle {
        Turtle {
            cvs: Canvas::new(0, 0),
            x: x,
            y: y,
            brush: true,
            rotation: 0.0,
        }
    }

    pub fn width(mut self, width: uint) -> Turtle {
        self.cvs.width = width;
        self
    }

    pub fn height(mut self, height: uint) -> Turtle {
        self.cvs.height = height;
        self
    }

    pub fn up(&mut self) {
        self.brush = false;
    }

    pub fn down(&mut self) {
        self.brush = true;
    }

    pub fn toggle(&mut self) {
        self.brush = !self.brush;
    }

    pub fn forward(&mut self, dist: f32) {
        let x = self.x + self.rotation.to_radians().cos()*dist;
        let y = self.y + self.rotation.to_radians().sin()*dist;
        self.move(x, y);
    }

    pub fn back(&mut self, dist: f32) {
        self.forward(-dist);
    }

    pub fn move(&mut self, x: f32, y: f32) {
        if self.brush {
            self.cvs.line(cmp::max(0, self.x.round() as int) as uint,
                          cmp::max(0, self.y.round() as int) as uint,
                          cmp::max(0, x.round() as int) as uint,
                          cmp::max(0, y.round() as int) as uint);
        }

        self.x = x;
        self.y = y;
    }

    pub fn right(&mut self, angle: f32) {
        self.rotation += angle;
    }

    pub fn left(&mut self, angle: f32) {
        self.rotation -= angle;
    }

    pub fn frame(&self) -> String {
        self.cvs.frame()
    }
}