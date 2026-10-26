#[derive(Debug)]
enum Command {
    Init,
    WaitSeconds(usize),
    WriteDataOffsetSize(u64, u32),
    ReadData { offset: u64, size: u32 },
}

fn main() {
    let commands = [
        Command::Init,
        Command::WaitSeconds(5),
        Command::WriteDataOffsetSize(0, 1024),
        Command::ReadData { offset: 512, size: 64 },
    ];

    for command in commands {
        println!("{command:?}");
    }
}
