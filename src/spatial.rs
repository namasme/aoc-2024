use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn is_parallel(&self, other: &Self) -> bool
    where
        T: Mul<Output = T> + Copy + Eq,
    {
        self.x * other.y == self.y * other.x
    }

    pub fn dot(&self, other: &Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        self.x * other.x + self.y * other.y
    }

    pub fn is_between(&self, first: &Point2D<T>, second: &Point2D<T>) -> bool
    where
        T: PartialOrd,
    {
        ((first.x <= self.x && self.x <= second.x || second.x <= self.x && self.x <= first.x)
            && self.y == first.y
            && self.y == second.y)
            || (((first.y <= self.y && self.y <= second.y)
                || (second.y <= self.y && self.y <= first.y))
                && self.x == first.x
                && self.x == second.x)
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

impl<T> Sub for Point2D<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<T> for Point2D<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
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
