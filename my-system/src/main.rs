use bank_system::*;

use std::env;
use std::process;

fn print_usage() {
    eprintln!("Использование:");
    eprintln!("  cargo run -- add <name>");
    eprintln!("  cargo run -- deposit <name> <amount>");
    eprintln!("  cargo run -- withdraw <name> <amount>");
    eprintln!("  cargo run -- balance <name>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    // Примечание: в реальном приложении хранилище нужно сохранять на диск.
    // Здесь мы создаём его заново при каждом запуске и заранее добавляем
    // пользователей — для демонстрации работы библиотеки.
    let mut storage = Storage::new();

    let users = [
        ("Alice", Currency::Rub),
        ("Bob", Currency::Rub),
        ("John", Currency::Rub),
    ];
    // Предзаполняем пользователей для демонстрации
    for (name, currency) in users {
        storage.add_user(Name::from(name), currency);
    }

    match args[1].as_str() {
        "add" => {
            if args.len() != 3 {
                eprintln!("Ошибка: укажите имя пользователя");
                eprintln!("Пример: cargo run -- add Alice");
                process::exit(1);
            }
            let name: Name = args[2].clone();
            match storage.add_user(name.clone(), Currency::Rub) {
                Some(_) => println!("Пользователь '{}' добавлен", name),
                None => println!("Пользователь '{}' уже существует", name),
            }
        }

        "deposit" => {
            if args.len() != 4 {
                eprintln!("Ошибка: укажите имя и сумму");
                eprintln!("Пример: cargo run -- deposit Alice 1000");
                process::exit(1);
            }
            let name: Name = args[2].clone();
            let amount = match args[3].parse() {
                Ok(n) => Balance(n),
                Err(_) => {
                    eprintln!("Ошибка: сумма должна быть целым числом");
                    process::exit(1);
                }
            };
            match storage.deposit(&name, amount) {
                Ok(new_balance) => {
                    println!(
                        "Пополнено: '{}' +{}. Новый баланс: {}",
                        name, amount, new_balance
                    );
                }
                Err(e) => {
                    eprintln!("Ошибка: {}", e);
                    process::exit(1);
                }
            }
        }

        "withdraw" => {
            if args.len() != 4 {
                eprintln!("Ошибка: укажите имя и сумму");
                eprintln!("Пример: cargo run -- withdraw Alice 500");
                process::exit(1);
            }
            let name: Name = args[2].clone();
            let amount = match args[3].parse() {
                Ok(n) => Balance(n),
                Err(_) => {
                    eprintln!("Ошибка: сумма должна быть целым числом");
                    process::exit(1);
                }
            };
            match storage.withdraw(&name, amount) {
                Ok(new_balance) => println!(
                    "Снято: '{}' -{}. Новый баланс: {}",
                    name, amount, new_balance
                ),
                Err(e) => {
                    eprintln!("Ошибка: {}", e);
                    process::exit(1);
                }
            }
        }

        "balance" => {
            if args.len() != 3 {
                eprintln!("Ошибка: укажите имя пользователя");
                eprintln!("Пример: cargo run -- balance Alice");
                process::exit(1);
            }
            let name: Name = args[2].clone();
            match storage.get_balance(&name) {
                Some(b) => println!("Баланс '{}': {}", name, b),
                None => {
                    eprintln!("Пользователь '{}' не найден", name);
                    process::exit(1);
                }
            }
        }

        _ => {
            eprintln!("Неизвестная команда: '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}
