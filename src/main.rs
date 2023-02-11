use std::env;
use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{stdin, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct UnixProject {
    base_path: String,
    project_name: String,
    project_path: Option<String>,
}

impl UnixProject {
    fn check_base_path(&mut self) {
        //Проверка пути директории в которой будет создаваться проект
        if Path::new(self.base_path.trim()).exists() == true {
            self.base_path = self.base_path.trim().to_string();
        } else {
            let cwd = env::current_dir();
            self.base_path = cwd.unwrap().into_os_string().into_string().unwrap();
        }
    }

    fn check_project_name(&mut self) {
        //Очищает имя проекта для корректного создания папки
        self.project_name
            .retain(|c| !r#"\ / : * ? " < > | "#.contains(c));
        if self.project_name.len() == 0 {
            println!("Некорректно введено имя проекта. Создается по умолчанию -> new_project");
            self.project_name = "new_project".to_string();
        }
        self.project_name = self.project_name.trim().to_string();
    }

    fn create_folder(&mut self) {
        //Создание папки проекта
        let proj_folder = [
            self.base_path.clone(),
            "/".to_string(),
            self.project_name.clone(),
        ]
        .join("");
        self.project_path = Some(proj_folder.clone());
        create_dir(proj_folder).unwrap_or_else(|e| panic!("Не удалось создать папку - {:?}", e));
    }

    fn create_main(&self) -> Result<(), Box<dyn Error>> {
        let mut py_file =
            File::create(&[self.project_path.clone().unwrap(), "/main.py".to_string()].join(""))?;
        py_file.write_all(
            b"def main():\n\tprint('Hello, world!')\n\nif __name__ == '__main__':\n\tmain()",
        )?;
        Ok(())
    }

    fn create_venv(&self) -> Result<(), Box<dyn Error>> {
        println!("Создание виртуального окружения...");
        let _cvenv = Command::new("python3")
            .args(&[
                "-m",
                "venv",
                &[self.project_path.clone().unwrap().as_str(), "/env"].join(""),
            ])
            .status()?;
        Ok(())
    }

    fn git_init(&self) -> Result<(), Box<dyn Error>> {
        println!("Создание файла .gitignore");
        let mut gitignore = File::create(
            &[
                self.project_path.clone().unwrap(),
                "/.gitignore".to_string(),
            ]
            .join(""),
        )?;
        gitignore.write_all(b"__pycache__\nenv")?;

        println!("Инициализация git репозитория.");
        let _git_init = Command::new("git")
            .current_dir([self.project_path.clone().unwrap()].join(""))
            .arg("init")
            .status()?;
        Ok(())
    }

    pub fn create_project(&mut self) {
        self.check_base_path();
        self.check_project_name();
        self.create_folder();
        self.create_main()
            .unwrap_or_else(|e| panic!("Не удалось создать точку входа - {:?}", e));
        self.create_venv()
            .unwrap_or_else(|e| panic!("Не удалось создать виртуальное окружение - {:?}", e));
        self.git_init()
            .unwrap_or_else(|e| panic!("Не инициализировать репозиторий - {:?}", e));
        println!(
            "Проект {:?}. Путь: {:?}",
            self.project_name,
            self.project_path.clone().unwrap()
        );
    }
}
fn main() {
    println!("{}", env::consts::OS);

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

    new_project.create_project();
    // print!("Project -> {:#?}", new_project);
}

