mod os_vars;

use os_vars::*;
use std::env;
use std::io::stdin;

fn main() {
    let cur_os = env::consts::OS;

    println!("Укажите путь для проекта:");
    let mut base_path = String::new();

    stdin()
        .read_line(&mut base_path)
        .expect("Failed to read line");

    println!("Введите имя проекта:");
    let mut project_name = String::new();

    stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    let mut new_project = UnixProject {
        base_path: base_path,
        project_name: project_name,
        project_path: None,
    };

    match cur_os {
        OsVar::MACOS | OsVar::LINUX => new_project.create_project(),
        OsVar::WINDOWS => println!("Пока не реализовано)"),
        _ => println!("Какая-то другая ОС"),
    }
}

