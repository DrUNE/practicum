use std::collections::HashMap;

// Псевдоним для удобства: имя пользователя — это строка
pub type Name = String;

// Псевдоним для баланса: i64 поддерживает отрицательные значения (овердрафт)
// и достаточно велик для любых реальных денежных сумм.
// f64 не используется — из-за ошибок округления при арифметике с плавающей точкой.
pub type Balance = i64;

// Валюта
#[derive(Debug, Clone, PartialEq)]
pub enum Currency {
    Rub,
    Usd,
    Eur,
}

/// Структура Account хранит данные одного пользователя.
/// Расширяема: в будущем можно добавить email, дату создания и т.д.
#[derive(Debug, Clone)]
pub struct Account {
    pub name: Name,
    pub balance: Balance,
    pub currency: Currency,
}

impl Account {
    fn new(name: Name, currency: Currency) -> Self {
        Account { name, balance: 0, currency }
    }
}

/// Storage — основное хранилище банковских аккаунтов.
/// Внутри — HashMap, где ключ: имя пользователя, значение: Account.
pub struct Storage {
    accounts: HashMap<Name, Account>,
}

impl Storage {
    /// Создаёт новый пустой банк.
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }

    /// Добавляет пользователя. Возвращает Some(0) при успехе,
    /// None если пользователь уже существует.
    pub fn add_user(&mut self, name: Name, currency: Currency) -> Option<Balance> {
        if self.accounts.contains_key(&name) {
            None
        } else {
            self.accounts.insert(name.clone(), Account::new(name, currency));
            Some(0)
        }
    }

    /// Удаляет пользователя. Возвращает Some(balance) при успехе,
    /// None если пользователь не найден.
    pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name).map(|acc| acc.balance)
    }

    /// Возвращает баланс пользователя или None если не найден.
    pub fn get_balance(&self, name: &Name) -> Option<Balance> {
        self.accounts.get(name).map(|acc| acc.balance)
    }

    /// Пополняет баланс пользователя на указанную сумму.
    /// Возвращает Err если пользователь не найден или сумма отрицательна.
    pub fn deposit(&mut self, name: &Name, amount: Balance) -> Result<Balance, String> {
        if amount < 0 {
            return Err("Сумма пополнения не может быть отрицательной".into());
        }
        match self.accounts.get_mut(name) {
            Some(acc) => {
                acc.balance += amount;
                Ok(acc.balance)
            }
            None => Err(format!("Пользователь '{}' не найден", name)),
        }
    }

    /// Снимает средства с баланса пользователя.
    /// Возвращает Err если пользователь не найден, сумма отрицательна
    /// или средств недостаточно.
    pub fn withdraw(&mut self, name: &Name, amount: Balance) -> Result<Balance, String> {
        if amount < 0 {
            return Err("Сумма снятия не может быть отрицательной".into());
        }
        match self.accounts.get_mut(name) {
            Some(acc) => {
                if acc.balance < amount {
                    Err(format!(
                        "Недостаточно средств: баланс {}, запрошено {}",
                        acc.balance, amount
                    ))
                } else {
                    acc.balance -= amount;
                    Ok(acc.balance)
                }
            }
            None => Err(format!("Пользователь '{}' не найден", name)),
        }
    }
}

// ==================== ТЕСТЫ ====================
#[cfg(test)]      // этот модуль компилируй, только когда запускают тесты». В обычную сборку он не попадёт.
mod tests {
    use super::*; // подключаем всё из родительского модуля

    #[test]
    fn test_new_storage_is_empty() {
        let storage = Storage::new();
        assert_eq!(storage.accounts.len(), 0);
    }

    #[test]
    fn test_add_user_success() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string(), Currency::Usd), Some(0));
    }

    #[test]
    fn test_add_user_duplicate() {
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string(), Currency::Usd);
        // Второй раз — пользователь уже существует
        assert_eq!(storage.add_user("Alice".to_string(), Currency::Usd), None);
    }

    #[test]
    fn test_remove_user_returns_balance() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string(), Currency::Rub);
        storage.deposit(&"Bob".to_string(), 100).unwrap();
        assert_eq!(storage.remove_user(&"Bob".to_string()), Some(100));
    }

    #[test]
    fn test_remove_nonexistent_user() {
        let mut storage = Storage::new();
        assert_eq!(storage.remove_user(&"Ghost".to_string()), None);
    }

    #[test]
    fn test_deposit_increases_balance() {
        let mut storage = Storage::new();
        storage.add_user("Charlie".to_string(), Currency::Eur);
        let result = storage.deposit(&"Charlie".to_string(), 200);
        assert_eq!(result, Ok(200));
        assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(200));
    }

    #[test]
    fn test_deposit_negative_amount_fails() {
        let mut storage = Storage::new();
        storage.add_user("Charlie".to_string(), Currency::Eur);
        assert!(storage.deposit(&"Charlie".to_string(), -50).is_err());
    }

    #[test]
    fn test_deposit_nonexistent_user_fails() {
        let mut storage = Storage::new();
        assert!(storage.deposit(&"Dana".to_string(), 100).is_err());
    }

    #[test]
    fn test_withdraw_success() {
        let mut storage = Storage::new();
        storage.add_user("Eve".to_string(), Currency::Rub);
        storage.deposit(&"Eve".to_string(), 500).unwrap();
        let result = storage.withdraw(&"Eve".to_string(), 200);
        assert_eq!(result, Ok(300));
        assert_eq!(storage.get_balance(&"Eve".to_string()), Some(300));
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut storage = Storage::new();
        storage.add_user("Frank".to_string(), Currency::Usd);
        storage.deposit(&"Frank".to_string(), 50).unwrap();
        // Пытаемся снять больше, чем есть
        assert!(storage.withdraw(&"Frank".to_string(), 100).is_err());
        // Баланс не изменился
        assert_eq!(storage.get_balance(&"Frank".to_string()), Some(50));
    }

    #[test]
    fn test_withdraw_negative_amount_fails() {
        let mut storage = Storage::new();
        storage.add_user("Grace".to_string(), Currency::Eur);
        assert!(storage.withdraw(&"Grace".to_string(), -10).is_err());
    }

    #[test]
    fn test_get_balance_nonexistent() {
        let storage = Storage::new();
        assert_eq!(storage.get_balance(&"Nobody".to_string()), None);
    }

    #[test]
    fn test_multiple_deposits() {
        let mut storage = Storage::new();
        storage.add_user("Hank".to_string(), Currency::Rub);
        storage.deposit(&"Hank".to_string(), 1000).unwrap();
        storage.deposit(&"Hank".to_string(), 500).unwrap();
        assert_eq!(storage.get_balance(&"Hank".to_string()), Some(1500));
    }

    #[test]
    fn test_different_currencies() {
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string(), Currency::Usd);
        storage.add_user("Bob".to_string(), Currency::Rub);

        // проверяем, что оба пользователя добавлены успешно
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(0));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(0));
    }
}
