use crate::balance::{Balance, Currency};
/// Структура Account хранит данные одного пользователя.
/// Расширяема: в будущем можно добавить email, дату создания и т.д.
#[derive(Debug, Clone)]
pub struct Account {
    pub name: Name,
    pub balance: Balance,
    pub currency: Currency,
}

impl Account {
    pub fn new(name: Name, balance: Balance, currency: Currency) -> Self {
        Account {
            name,
            balance,
            currency,
        }
    }
}

// Псевдоним для удобства: имя пользователя — это строка
pub type Name = String;
