use clap::Parser;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(about = "Python project creator")]
pub struct Cli {
    #[arg(short = 'p', long = "path")]
    path: Option<PathBuf>,
    #[arg(short = 'g', long = "git", default_value_t = false)]
    git: bool,
    #[arg(short = 'n', long = "name")]
    name: String,
    // #[arg(short = 'r')]
    // requirements: Option<PathBuf>,
}

impl Cli {
    //Проверка пути директории в которой будет создаваться проект
    fn check_and_create_folder(&mut self) {
        match &self.path {
            Some(path) => {
                if path.exists() {
                    self.path = Some(path.clone())
                } else {
                    panic!("{:?} - не существует", path)
                }
            }
            None => self.path = Some(env::current_dir().unwrap()),
        }
    }

    //Очищает имя проекта для корректного создания папки
    fn check_project_name(&mut self) {
        self.name.retain(|c| !r#"\ / : * ? " < > | "#.contains(c));
        if self.name.is_empty() {
            println!("Некорректно введено имя проекта. Создается по умолчанию -> new_project");
            self.name = "new_project".to_string();
        }
        self.name = self.name.trim().to_string();
    }

    fn create_main(&mut self) -> Result<(), Box<dyn Error>> {
        let mut main = self.path.clone().unwrap().into_os_string();
        main.push("main.py");
        let mut py_file = File::create(main)?;
        py_file.write_all(b"def main():\n\tpass\n\nif __name__ == '__main__':\n\tmain()")?;
        Ok(())
    }

    fn create_venv(&self) -> Result<(), Box<dyn Error>> {
        println!("Создание виртуального окружения...");
        self.path.clone().unwrap().push("env");
        let _cvenv = Command::new("python3")
            .args([
                "-m",
                "venv",
                self.path.clone().unwrap().to_str().unwrap(), 
            ])
            .status()?;
        Ok(())
    }

    fn git_init(&self) -> Result<(), Box<dyn Error>> {
        println!("Создание файла .gitignore");
        let mut gitignore_path = self.path.clone().unwrap();
        gitignore_path.push(".gitignore");
        let mut gitignore = File::create(gitignore_path)?;
        gitignore.write_all(b"__pycache__\nenv\n**.env\n**.log")?;

        println!("Инициализация git репозитория.");
        let _git_init = Command::new("git")
            .current_dir(self.path.clone().unwrap())
            .arg("init")
            .status()?;
        Ok(())
    }

    pub fn create(&mut self) -> Result<(), Box<dyn Error>> {
        self.check_and_create_folder();
        self.check_project_name();
        self.create_main()?;
        self.create_venv()?;
        if self.git {
            self.git_init()?;
        }
        Ok(())
    }
}
