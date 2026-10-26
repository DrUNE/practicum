use bank_system::{Balance, Currency, Name, Storage};
use std::io::{self, BufRead, Write};

fn main() {
    // Загружаем состояние из файла (или создаём с нуля, если файла нет)
    let mut storage = Storage::load_data("balance.csv");

    println!("=== Bank CLI Utils ===");
    println!("Команды:");
    println!("  add <name> <balance>      - добавить пользователя");
    println!("  remove <name>             - удалить пользователя");
    println!("  deposit <name> <amount>   - пополнить баланс");
    println!("  withdraw <name> <amount>  - снять со счёта");
    println!("  balance <name>            - показать баланс");
    println!("  list                      - показать всех пользователей");
    println!("  exit                      - выйти");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        // flush нужен, чтобы приглашение "> " появилось до того, как пользователь начнёт вводить
        print!("> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        // read_line возвращает 0 байт при EOF (например, Ctrl+D) — выходим из цикла
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "add" => {
                if args.len() != 3 {
                    println!("Пример: add John 100");
                    continue;
                }
                let name: Name = args[1].to_string();
                let balance: i64 = match args[2].parse() {
                    Ok(b) => b,
                    Err(_) => {
                        println!("Сумма должна быть целым числом");
                        continue;
                    }
                };
                if storage.add_user(name.clone(), Currency::Rub).is_some() {
                    let _ = storage.deposit(&name, Balance(balance));
                    println!("Пользователь '{}' добавлен с балансом {}", name, balance);
                    storage.save("balance.csv");
                } else {
                    println!("Пользователь '{}' уже существует", name);
                }
            }
            "remove" => {
                if args.len() != 2 {
                    println!("Пример: remove John");
                    continue;
                }
                let name = args[1].to_string();
                match storage.remove_user(&name) {
                    Some(_) => {
                        println!("Пользователь '{}' удалён", name);
                        storage.save("balance.csv");
                    }
                    None => println!("Пользователь '{}' не найден", name),
                }
            }
            "deposit" => {
                if args.len() != 3 {
                    println!("Пример: deposit John 100");
                    continue;
                }
                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть целым числом");
                        continue;
                    }
                };
                match storage.deposit(&name, Balance(amount)) {
                    Ok(bal) => {
                        println!("Баланс '{}' увеличен на {}. Итого: {}", name, amount, bal);
                        storage.save("balance.csv");
                    }
                    Err(e) => println!("Ошибка: {}", e),
                }
            }
            "withdraw" => {
                if args.len() != 3 {
                    println!("Пример: withdraw John 100");
                    continue;
                }
                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть целым числом");
                        continue;
                    }
                };
                match storage.withdraw(&name, Balance(amount)) {
                    Ok(bal) => {
                        println!("С баланса '{}' снято {}. Итого: {}", name, amount, bal);
                        storage.save("balance.csv");
                    }
                    Err(e) => println!("Ошибка: {}", e),
                }
            }
            "balance" => {
                if args.len() != 2 {
                    println!("Пример: balance John");
                    continue;
                }
                let name = args[1].to_string();
                match storage.get_balance(&name) {
                    Some(b) => println!("Баланс '{}': {}", name, b),
                    None => println!("Пользователь '{}' не найден", name),
                }
            }
            "list" => {
                for (user, balance) in storage.get_all() {
                    println!("{}: {}", user, balance);
                }
            }
            "exit" => break,
            _ => println!("Неизвестная команда: '{}'", args[0]),
        }
    }

    println!("Выход. Все изменения сохранены.");
}
