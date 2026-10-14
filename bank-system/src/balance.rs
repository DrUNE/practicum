// Псевдоним для баланса: i64 поддерживает отрицательные значения (овердрафт)
// и достаточно велик для любых реальных денежных сумм.
// f64 не используется — из-за ошибок округления при арифметике с плавающей точкой.
use std::fmt::Display;
use std::ops::{AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Balance(pub i64);

#[derive(Debug, Clone, PartialEq)]
pub enum Currency {
    Rub,
    Usd,
    Eur,
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SubAssign for Balance {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl Sub for Balance {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Balance(self.0 - other.0)
    }
}

impl AddAssign for Balance {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl PartialOrd for Balance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}