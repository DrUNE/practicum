use crate::{Account, Balance, Currency, Name};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Storage — основное хранилище банковских аккаунтов.
/// Внутри — HashMap, где ключ: имя пользователя, значение: Account.
pub struct Storage {
    accounts: HashMap<Name, Account>,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    /// Создаёт новый пустой банк.
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }

    pub fn new_from(accounts: HashMap<Name, Account>) -> Self {
        Storage { accounts }
    }

    /// Добавляет пользователя. Возвращает Some(0) при успехе,
    /// None если пользователь уже существует.
    pub fn add_user(&mut self, name: Name, currency: Currency) -> Option<Balance> {
        if self.accounts.contains_key(&name) {
            None
        } else {
            self.accounts
                .insert(name.clone(), Account::new(name, Balance(0), currency));
            Some(Balance(0))
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
        if amount < Balance(0) {
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

    const CREDIT_LIMIT: Balance = Balance(-1000);

    /// Снимает средства с баланса пользователя.
    /// Возвращает Err если пользователь не найден, сумма отрицательна
    /// или средств недостаточно.
    pub fn withdraw(&mut self, name: &Name, amount: Balance) -> Result<Balance, String> {
        if amount < Balance(0) {
            return Err("Сумма снятия не может быть отрицательной".into());
        }
        match self.accounts.get_mut(name) {
            Some(acc) => {
                if acc.balance - Self::CREDIT_LIMIT < amount {
                    Err(format!(
                        "Превышен кредитный лимит: баланс {}, запрошено {}",
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

    /// Возвращает всех пользователей и их балансы.
    pub fn get_all(&self) -> Vec<(Name, Balance)> {
        self.accounts.iter().map(|(n, acc)| (n.clone(), acc.balance)).collect()
    }

    /// Загружает данные из файла.
    /// Если файл существует, загружает его содержимое, иначе возвращает storage с пользователями по умолчанию.
    /// # Arguments
    ///
    /// * `file` - путь к файлу.
    ///
    /// # Returns
    ///
    /// Возвращает структуру `Storage` с загруженными данными.
    pub fn load_data(file: &str) -> Storage {
        let mut accounts = HashMap::new();

        if Path::new(file).exists() {
            let f = fs::File::open(file).unwrap();
            // BufReader читает данные блоками, а не байт за байтом —
            // это делает построчное чтение через .lines() быстрым и эффективным
            let reader = BufReader::new(f);

            for line in reader.lines().map_while(Result::ok) {
                let parts: Vec<&str> = line.trim().split(',').collect();
                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    let balance: i64 = parts[1].parse().unwrap_or(0);
                    accounts.insert(
                        name.clone(),
                        Account::new(name.clone(), Balance(balance), Currency::Rub),
                    );
                }
            }
        } else {
            // Файла нет — создаём пользователей с нуля
            let mut storage = Storage::new_from(accounts);
            for u in ["John", "Alice", "Bob", "Vasya"] {
                storage.add_user(u.to_string(), Currency::Rub);
            }
            return storage;
        }

        Storage::new_from(accounts)
    }

    /// Сохраняет текущее состояние в CSV-файл формата "Name,Balance".
    /// Используем fs::write — данные собираются в строку целиком и записываются одним вызовом.
    pub fn save(&self, file: &str) {
        let mut data = String::new();
        for (name, balance) in self.get_all() {
            data.push_str(&format!("{},{}\n", name, balance));
        }
        fs::write(file, data).expect("Не удалось записать файл");
    }
}

// ==================== ТЕСТЫ ====================
#[cfg(test)] // этот модуль компилируй, только когда запускают тесты». В обычную сборку он не попадёт.
mod tests {
    use std::io::Cursor;

    use super::*; // подключаем всё из родительского модуля

    #[test]
    fn test_new_storage_is_empty() {
        let storage = Storage::new();
        assert_eq!(storage.accounts.len(), 0);
    }

    #[test]
    fn test_add_user_success() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string(), Currency::Usd), Some(Balance(0)));
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
        storage.deposit(&"Bob".to_string(), Balance(100)).unwrap();
        assert_eq!(storage.remove_user(&"Bob".to_string()), Some(Balance(100)));
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
        let result = storage.deposit(&"Charlie".to_string(), Balance(200));
        assert_eq!(result, Ok(Balance(200)));
        assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(Balance(200)));
    }

    #[test]
    fn test_deposit_negative_amount_fails() {
        let mut storage = Storage::new();
        storage.add_user("Charlie".to_string(), Currency::Eur);
        assert!(storage.deposit(&"Charlie".to_string(), Balance(-50)).is_err());
    }

    #[test]
    fn test_deposit_nonexistent_user_fails() {
        let mut storage = Storage::new();
        assert!(storage.deposit(&"Dana".to_string(), Balance(100)).is_err());
    }

    #[test]
    fn test_withdraw_success() {
        let mut storage = Storage::new();
        storage.add_user("Eve".to_string(), Currency::Rub);
        storage.deposit(&"Eve".to_string(), Balance(500)).unwrap();
        let result = storage.withdraw(&"Eve".to_string(), Balance(200));
        assert_eq!(result, Ok(Balance(300)));
        assert_eq!(storage.get_balance(&"Eve".to_string()), Some(Balance(300)));
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut storage = Storage::new();
        storage.add_user("Frank".to_string(), Currency::Usd);
        storage.deposit(&"Frank".to_string(), Balance(50)).unwrap();
        // Пытаемся снять больше, чем есть + лимит
        assert!(storage.withdraw(&"Frank".to_string(), Balance(1100)).is_err());
        // Баланс не изменился
        assert_eq!(storage.get_balance(&"Frank".to_string()), Some(Balance(50)));
    }

    #[test]
    fn test_withdraw_negative_amount_fails() {
        let mut storage = Storage::new();
        storage.add_user("Grace".to_string(), Currency::Eur);
        assert!(storage.withdraw(&"Grace".to_string(), Balance(-10)).is_err());
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
        storage.deposit(&"Hank".to_string(), Balance(1000)).unwrap();
        storage.deposit(&"Hank".to_string(), Balance(500)).unwrap();
        assert_eq!(storage.get_balance(&"Hank".to_string()), Some(Balance(1500)));
    }

    #[test]
    fn test_different_currencies() {
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string(), Currency::Usd);
        storage.add_user("Bob".to_string(), Currency::Rub);

        // проверяем, что оба пользователя добавлены успешно
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(Balance(0)));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(Balance(0)));
    }

    #[test]
    fn test_credit_limit_fails() {
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string(), Currency::Usd);

        // Снимаем 1500 при балансе 0 — превышаем лимит -1000
        assert!(storage.withdraw(&"Alice".to_string(), Balance(1500)).is_err());

        // Баланс не изменился после неудачного снятия
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(Balance(0)));
    }

    #[test]
    fn test_credit_limit_success() {
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string(), Currency::Usd);

        // Снимаем 500 при балансе 0 — лимит не превышен
        assert!(storage.withdraw(&"Alice".to_string(), Balance(500)).is_ok());

        // Баланс уменьшился на 500 после успешного снятия
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(Balance(-500)));
    }

    #[test]
    fn test_load_data_from_cursor() {
        // Загружаем данные из курсора
        let data = b"Alice,1000\nBob,2000\n";
        let mut cursor = Cursor::new(&data[..]);
        let mut storage = Storage::new();
        let reader = BufReader::new(&mut cursor);
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(',').collect();
            assert_eq!(parts.len(), 2);
            let name = parts[0].to_string();
            let balance = Balance(parts[1].parse().unwrap());
            storage.add_user(name.clone(), Currency::Usd);
            storage.deposit(&name, balance).unwrap();
        }
        // Проверяем, что данные загружены корректно
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(Balance(1000)));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(Balance(2000)));
    }
}
