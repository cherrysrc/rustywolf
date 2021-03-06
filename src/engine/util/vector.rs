use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
#[repr(C)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        return Vector2 { x, y };
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    pub fn dist_sq(&self, other: &Vector2<T>) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        dx * dx + dy * dy
    }

    pub fn mag_sq(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T: Add> Add for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        return Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub> Sub for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        return Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        return Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        return Vector2 {
            x: -self.x,
            y: -self.y,
        };
    }
}
