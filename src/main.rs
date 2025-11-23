mod application;
mod domain;
mod infrastructure;
mod interface;

use domain::services::ConsoleError;
use interface::command_factory::CommandFactory;
use interface::credentials_loader::CredentialsLoader;
use interface::executor::CommandExecutor;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let credentials = CredentialsLoader::try_load().unwrap_or_else(|e| {
        eprintln!("Ошибка получения credentials: {}", e.consol_log());
        std::process::exit(1);
    });

    let command = CommandFactory::from_args(&args).unwrap_or_else(|e| {
        eprintln!("Ошибка распознавания команды {e}");
        std::process::exit(1);
    });

    let executor = CommandExecutor::new(credentials);

    executor.run_command(command).unwrap_or_else(|e| {
        eprintln!("Ошибка: {}", e.consol_log());
        std::process::exit(1);
    });
    std::process::exit(0);
}
