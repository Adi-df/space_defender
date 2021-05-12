use std::ops::{Add, AddAssign, Sub, SubAssign};

use hecs::World;

#[derive(Clone,Debug)]
pub struct Position(f32, f32);
#[derive(Clone,Debug)]
pub struct Size(f32, f32);
#[derive(Clone,Debug)]
pub struct Velocity(f32, f32);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }
}
impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self(w, h)
    }
}
impl Velocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        Self(vx, vy)
    }
}


// Math
impl Add<Position> for Position {
    type Output = Self;

    fn add(self, pos: Position) -> Self {
        Self(self.0 + pos.0, self.1 + pos.1)
    }
}
impl Add<Size> for Position {
    type Output = Self;

    fn add(self, pos: Size) -> Self {
        Self(self.0 + pos.0, self.1 + pos.1)
    }
}
impl Add<Velocity> for Position {
    type Output = Self;

    fn add(self, pos: Velocity) -> Self {
        Self(self.0 + pos.0, self.1 + pos.1)
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl AddAssign<Size> for Position {
    fn add_assign(&mut self, rhs: Size) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Position> for Position {
    type Output = Self;

    fn sub(self, pos: Position) -> Self {
        Self(self.0 - pos.0, self.1 - pos.1)
    }
}
impl Sub<Size> for Position {
    type Output = Self;

    fn sub(self, pos: Size) -> Self {
        Self(self.0 - pos.0, self.1 - pos.1)
    }
}
impl Sub<Velocity> for Position {
    type Output = Self;

    fn sub(self, pos: Velocity) -> Self {
        Self(self.0 - pos.0, self.1 - pos.1)
    }
}

impl SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}
impl SubAssign<Size> for Position {
    fn sub_assign(&mut self, rhs: Size) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}
impl SubAssign<Velocity> for Position {
    fn sub_assign(&mut self, rhs: Velocity) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}
