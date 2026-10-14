use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::{Balance, Name};

#[derive(Debug, Clone)]
pub struct Account {
    pub name: Name,
    pub balance: Balance,
}

impl Account {
    fn new(name: Name) -> Self {
        Account { name, balance: 0 }
    }
}

pub struct Storage {
    accounts: HashMap<Name, Account>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }

    /// Добавляет пользователя. Some(0) — успех, None — уже существует.
    pub fn add_user(&mut self, name: Name) -> Option<Balance> {
        if self.accounts.contains_key(&name) {
            None
        } else {
            self.accounts.insert(name.clone(), Account::new(name));
            Some(0)
        }
    }

    /// Удаляет пользователя. Some(balance) — успех, None — не найден.
    pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name).map(|acc| acc.balance)
    }

    /// Возвращает баланс или None если пользователь не найден.
    pub fn get_balance(&self, name: &Name) -> Option<Balance> {
        self.accounts.get(name).map(|acc| acc.balance)
    }

    /// Пополняет баланс. Возвращает новый баланс или Err.
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

    /// Снимает средства. Возвращает новый баланс или Err.
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

    /// Возвращает всех пользователей и их балансы.
    pub fn get_all(&self) -> Vec<(Name, Balance)> {
        self.accounts
            .iter()
            .map(|(n, acc)| (n.clone(), acc.balance))
            .collect()
    }

    /// Загружает данные из CSV-файла формата "Name,Balance".
    /// Если файл не существует — создаёт хранилище с пользователями по умолчанию.
    pub fn load_data(file: &str) -> Storage {
        let mut storage = Storage::new();

        if Path::new(file).exists() {
            let f = fs::File::open(file).unwrap();
            // BufReader читает данные блоками, а не байт за байтом —
            // это делает построчное чтение через .lines() быстрым и эффективным
            let reader = BufReader::new(f);

            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.trim().split(',').collect();
                    if parts.len() == 2 {
                        let name = parts[0].to_string();
                        let balance: i64 = parts[1].parse().unwrap_or(0);
                        storage.add_user(name.clone());
                        let _ = storage.deposit(&name, balance);
                    }
                }
            }
        } else {
            // Файла нет — создаём пользователей с нуля
            for u in ["John", "Alice", "Bob", "Vasya"] {
                storage.add_user(u.to_string());
            }
        }

        storage
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{BufReader, BufWriter, Cursor, Write};

    #[test]
    fn test_add_user() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string()), Some(0));
        assert_eq!(storage.add_user("Alice".to_string()), None);
    }

    #[test]
    fn test_remove_user() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        storage.deposit(&"Bob".to_string(), 100).unwrap();
        assert_eq!(storage.remove_user(&"Bob".to_string()), Some(100));
        assert_eq!(storage.remove_user(&"Bob".to_string()), None);
    }

    #[test]
    fn test_nonexistent_user() {
        let mut storage = Storage::new();
        assert!(storage.deposit(&"Dana".to_string(), 100).is_err());
        assert!(storage.withdraw(&"Dana".to_string(), 50).is_err());
        assert_eq!(storage.get_balance(&"Dana".to_string()), None);
    }

    #[test]
    fn test_deposit_negative_fails() {
        let mut storage = Storage::new();
        storage.add_user("Eve".to_string());
        assert!(storage.deposit(&"Eve".to_string(), -50).is_err());
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut storage = Storage::new();
        storage.add_user("Frank".to_string());
        storage.deposit(&"Frank".to_string(), 100).unwrap();
        assert!(storage.withdraw(&"Frank".to_string(), 200).is_err());
        // Баланс не изменился
        assert_eq!(storage.get_balance(&"Frank".to_string()), Some(100));
    }

    // --- Тесты для работы с файлами ---

    #[test]
    fn test_load_data_existing_file() {
        let file_path = "test_load_1_1_2.csv";

        // Создаём CSV-файл вручную
        let mut f = File::create(file_path).unwrap();
        writeln!(f, "John,100").unwrap();
        writeln!(f, "Alice,200").unwrap();
        writeln!(f, "Bob,50").unwrap();

        let storage = Storage::load_data(file_path);

        assert_eq!(storage.get_balance(&"John".to_string()), Some(100));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(200));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(50));
        // Vasya не был в файле — None
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None);

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_save_creates_file_with_correct_data() {
        let file_path = "test_save_1_1_2.csv";

        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();
        storage.deposit(&"Alice".to_string(), 300).unwrap();

        storage.save(file_path);

        let contents = fs::read_to_string(file_path).unwrap();
        let mut lines: Vec<&str> = contents.lines().collect();
        // Сортируем — HashMap не гарантирует порядок
        lines.sort();

        assert_eq!(lines, vec!["Alice,300", "John,150"]);

        fs::remove_file(file_path).unwrap();
    }

    // --- Тесты с Cursor (вместо реальных файлов) ---

    #[test]
    fn test_load_data_from_cursor() {
        // Cursor позволяет читать из памяти так же, как из файла
        let data = b"John,100\nAlice,200\nBob,50\n";
        let mut cursor = Cursor::new(&data[..]);

        let mut storage = Storage::new();
        let reader = BufReader::new(&mut cursor);
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let balance: i64 = parts[1].parse().unwrap_or(0);
                storage.add_user(name.clone());
                storage.deposit(&name, balance).unwrap();
            }
        }

        assert_eq!(storage.get_balance(&"John".to_string()), Some(100));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(200));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(50));
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None);
    }

    #[test]
    fn test_save_writes_to_cursor_correctly() {
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();
        storage.deposit(&"Alice".to_string(), 300).unwrap();

        // Пишем в Cursor вместо файла на диске
        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);
        {
            let mut writer = BufWriter::new(&mut cursor);
            for (name, balance) in storage.get_all() {
                writeln!(writer, "{},{}", name, balance).unwrap();
            }
            writer.flush().unwrap();
        }

        // Читаем обратно из памяти
        cursor.set_position(0);
        let mut lines: Vec<String> = BufReader::new(cursor)
            .lines()
            .map(|l| l.unwrap())
            .collect();
        lines.sort();

        assert_eq!(lines, vec!["Alice,300", "John,150"]);
    }
}
