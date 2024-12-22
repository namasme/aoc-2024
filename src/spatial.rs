use std::ops::{Add, Neg};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Add for Point2D<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<T: From<bool> + Neg<Output = T>> From<Direction> for Point2D<T> {
    fn from(direction: Direction) -> Self {
        let zero = T::from(false);
        let one = T::from(true);
        match direction {
            Direction::Up => Point2D { x: zero, y: one },
            Direction::Down => Point2D { x: zero, y: -one },
            Direction::Left => Point2D { x: -one, y: zero },
            Direction::Right => Point2D { x: one, y: zero },
        }
    }
}

impl Direction {
    pub fn as_delta<T>(&self) -> Point2D<T>
    where
        T: From<i32>,
    {
        match self {
            Direction::Up => Point2D::new(T::from(0), T::from(1)),
            Direction::Down => Point2D::new(T::from(0), T::from(-1)),
            Direction::Left => Point2D::new(T::from(-1), T::from(0)),
            Direction::Right => Point2D::new(T::from(1), T::from(0)),
        }
    }

    pub fn rotate(self, orientation: Orientation) -> Self {
        match orientation {
            Orientation::Clockwise => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Orientation::Counterclockwise => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Clockwise,
    Counterclockwise,
}
