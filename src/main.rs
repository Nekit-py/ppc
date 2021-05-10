use std::any::type_name;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::{fs, io};
use std::io::Write;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn git_initialization(path: &str) -> Result<(), Box<dyn Error>> {
    //Инициализация гита
    change_dir(&path);
    println!("Инициализирую git репозиторий...");
    let _git_init = Command::new("git").arg("init").status()?;
    Ok(())
}

fn change_dir(path: &str) {
    //Переход по указанному пути
    let _chdir = Command::new("cd")
        .arg(path)
        .status()
        .expect("an unknown error occurred");
}

fn create_dir(proj_name: &str) -> std::io::Result<()> {
    //Создание пустой папки
    fs::create_dir(proj_name)?;
    Ok(())
}

fn create_venv(proj_name: &str) -> Result<(), Box<dyn Error>> {
    //Создание виртуального окружения и пустого репозитория git
    let proj_venv: &str = &[proj_name, "/venv"].join("");
    let _cvenv = Command::new("python3")
        .args(&["-m", "venv", proj_venv])
        .status()?;
    create_dir(&[proj_name, "/git"].join(""));
    let mut gitignore = File::create(&[proj_name, "/git/.gitignore"].join(""))?;
    gitignore.write_all(b"__pycache__")?;
    Ok(())
}

fn start_proj() -> (String, bool) {
    //Проверка пути переданного пользователем
    println!("Укажите путь для папки Вашего проекта:");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");
    let exist: bool = Path::new(&path.trim()).exists();
    (path.trim().to_string(), exist)
}

fn check_project_name() -> String {
    //Проверки имени папки на корректность
    println!("Укажите имя проекта:");
    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");
    //string.chars().filter(..).collect()
    project_name.retain(|c| !r#"\ / : * ? " < > | "#.contains(c));
    if project_name.len() == 0 {
        panic!("Имя проекта должно содержать хотябы 1 разрешенный символ");
    }
    project_name.trim().to_string()
}

fn main() {
    let (typing_path, path_check) = start_proj();
    match path_check {
        true => {
            let proj_name = check_project_name();
            println!("-->Создаю проект в: {}", &typing_path);
            change_dir(&typing_path);
            create_dir(&proj_name);
            println!("-->Создаю виртуальное окружение...");
            create_venv(&proj_name);
            git_initialization(&[typing_path, "/".to_string() ,proj_name, "/git".to_string()].join(""));
            println!("-->Проект успешно создан!");
        }
        false => println!("Нвозможно создать проект, указан некорректный путь."),
    }
}
