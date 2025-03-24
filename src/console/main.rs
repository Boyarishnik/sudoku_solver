use std::io;
use sudoku_rules::Field;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let mut f = Field::new();

    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        match buf
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()[..]
        {
            [100, row, col] => f.cancel_insertion((row, col)),

            [row, col, val] => {
                if let Ok(_) = f.try_push((row, col), val as u8) {
                    println!("Все круто\n{f}");
                } else {
                    println!("Не получилось:(");
                }
            }

            [0] => {
                f.solve().expect("Не получилось достроить решение");
                println!("{f}");
                break;
            }
            _ => panic!("asdasdasd"),
        }
    }
}
