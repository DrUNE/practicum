use bank_system::{Balance, Name, Storage};
use std::env;

fn main() {
    // load_data читает CSV построчно через BufReader — демонстрация BufRead
    // Если файл balance.csv не существует — создаются пользователи по умолчанию
    let mut storage = Storage::load_data("balance.csv");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Использование:");
        eprintln!("  cargo run --example cli -- deposit <name> <amount>");
        eprintln!("  cargo run --example cli -- withdraw <name> <amount>");
        eprintln!("  cargo run --example cli -- balance <name>");
        return;
    }

    match args[1].as_str() {
        "deposit" => {
            if args.len() != 4 {
                eprintln!("Пример: cargo run --example cli -- deposit Alice 1000");
                return;
            }
            let name: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Сумма должна быть целым числом");
            match storage.deposit(&name, Balance(amount)) {
                Ok(bal) => {
                    println!("Пополнено '{}' на {}. Баланс: {}", name, amount, bal);
                    // Сохраняем новое состояние в CSV после каждого изменения
                    storage.save("balance.csv");
                }
                Err(e) => eprintln!("Ошибка: {}", e),
            }
        }
        "withdraw" => {
            if args.len() != 4 {
                eprintln!("Пример: cargo run --example cli -- withdraw Alice 500");
                return;
            }
            let name: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Сумма должна быть целым числом");
            match storage.withdraw(&name, Balance(amount)) {
                Ok(bal) => {
                    println!("Снято с '{}' {}. Баланс: {}", name, amount, bal);
                    storage.save("balance.csv");
                }
                Err(e) => eprintln!("Ошибка: {}", e),
            }
        }
        "balance" => {
            if args.len() != 3 {
                eprintln!("Пример: cargo run --example cli -- balance Alice");
                return;
            }
            let name: Name = args[2].clone();
            match storage.get_balance(&name) {
                Some(b) => println!("Баланс '{}': {}", name, b),
                None => eprintln!("Пользователь '{}' не найден", name),
            }
        }
        _ => eprintln!(
            "Неизвестная команда: '{}'. Доступны: deposit, withdraw, balance",
            args[1]
        ),
    }
}
