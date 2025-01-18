use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> [Self; 4]
    where
        T: From<bool> + Sub<Output = T> + Add<Output = T> + Copy,
    {
        let one = T::from(true);
        [
            Point2D::new(self.x, self.y + one),
            Point2D::new(self.x + one, self.y),
            Point2D::new(self.x, self.y - one),
            Point2D::new(self.x - one, self.y),
        ]
    }

    pub fn advance(&self, direction: Direction) -> Self
    where
        T: From<bool> + Sub<Output = T> + Add<Output = T> + Copy,
    {
        let one = T::from(true);
        match direction {
            Direction::Up => Point2D::new(self.x, self.y + one),
            Direction::Right => Point2D::new(self.x + one, self.y),
            Direction::Down => Point2D::new(self.x, self.y - one),
            Direction::Left => Point2D::new(self.x - one, self.y),
        }
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
        T: Add<Output = T>
            + Copy
            + Copy
            + Eq
            + From<bool>
            + Mul<Output = T>
            + PartialOrd
            + Sub<Output = T>,
    {
        let to_self = *self - *first;
        let to_second = *second - *first;

        to_self.is_parallel(&to_second) // vectors live in the same line
                && to_self.dot(&to_second) >= T::from(false) // point in the same direction
                && to_self.dot(&to_self) <= to_second.dot(&to_second) // first is closer to self than to second
    }
}

pub trait Point2DCast<T: TryInto<U>, U> {
    fn cast(self) -> Result<Point2D<U>, <T as TryInto<U>>::Error>;
}

impl<T, U> Point2DCast<T, U> for Point2D<T>
where
    T: TryInto<U>,
{
    fn cast(self) -> Result<Point2D<U>, <T as TryInto<U>>::Error> {
        Ok(Point2D {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
        })
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
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
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
