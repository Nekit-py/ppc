use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::io::Write;

fn git_initialization(path: &str) -> Result<(), Box<dyn Error>> {
    //Инициализация гита
    println!("Инициализирую git репозитория...");
    let _git_init = Command::new("git").current_dir(path).arg("init").status()?;
    Ok(())
}

fn create_venv(proj_path: &str, proj_name: &str) -> Result<(), Box<dyn Error>> {
    //Создание виртуального окружения и пустого репозитория git
    //TODO: проверку на существования python в системе
    let proj_venv: &str = &[proj_path, "/venv"].join("");
    let _cvenv = Command::new("python3")
        .args(&["-m", "venv", proj_venv])
        .status()?;
    fs::create_dir([&proj_path, "/", "git"].join("")).expect("Не удалось создать папку git...");
    let mut gitignore = File::create(&[proj_path, "/git/.gitignore"].join(""))?;
    gitignore.write_all(b"__pycache__")?;
    let mut py_file = File::create(&[proj_path, "/git/", proj_name, ".py"].join(""))?;
    py_file.write_all(b"def main():\n\tprint('Hello, world!')\n\nif __name__ == '__main__':\n\tmain()")?;
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
    //project_name.chars().filter(..).collect()
    project_name.retain(|c| !r#"\ / : * ? " < > | "#.contains(c));
    if project_name.len() == 0 {
        panic!("Имя проекта должно содержать хотябы 1 разрешенный символ!");
    }
    project_name.trim().to_string()
}

fn main() {
    let (typing_path, path_check) = start_proj();
    match path_check {
        true => {
            let proj_name = check_project_name();
            let proj_path = [typing_path.clone(), "/".to_string(), proj_name.clone()].join("");
            println!("-->Создаю проект в: {}", typing_path);
            fs::create_dir([&typing_path, "/", &proj_name].join("")).expect("Не удалось создать папку проекта...");
            println!("-->Создаю виртуальное окружение...");
            create_venv(&proj_path, &proj_name).expect("При создании виртуального окружения произошла ошибка...");
            git_initialization(&[typing_path, "/".to_string() ,proj_name, "/git".to_string()].join(""))
                        .expect("Ошибка при инициализации git репозитория...");
            println!("-->Проект успешно создан!");
        }
        false => println!("Нвозможно создать проект, указан некорректный путь."),
    }
}
