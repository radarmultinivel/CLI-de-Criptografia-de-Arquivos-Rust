// Desenvolvido por L. A. Leandro São José dos Campos- SP - 23/05/2026

mod args;
mod crypto;

use std::path::Path;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use args::{Cli, Mode};

fn main() {
    let cli = Cli::parse();

    let key = crypto::cipher::derive_key(&cli.key);

    let input_path = Path::new(&cli.input);
    if !input_path.exists() {
        eprintln!("Erro: arquivo de entrada não encontrado: '{}'", cli.input);
        std::process::exit(1);
    }

    let output_path = match &cli.output {
        Some(path) => Path::new(path).to_path_buf(),
        None => {
            let extension = match cli.mode {
                Mode::Encrypt => ".enc",
                Mode::Decrypt => ".dec",
            };
            let mut filename = input_path.to_path_buf().into_os_string();
            filename.push(extension);
            Path::new(&filename).to_path_buf()
        }
    };

    if output_path.exists() {
        eprintln!(
            "Erro: arquivo de saída já existe: '{}'",
            output_path.display()
        );
        std::process::exit(1);
    }

    let file_size = match std::fs::metadata(&cli.input) {
        Ok(meta) => meta.len(),
        Err(e) => {
            eprintln!(
                "Erro: não foi possível acessar o arquivo '{}': {}",
                cli.input, e
            );
            std::process::exit(1);
        }
    };

    let pb = ProgressBar::new(file_size);
    match ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    {
        Ok(style) => {
            pb.set_style(style.progress_chars("##-"));
        }
        Err(_) => {
            pb.set_style(ProgressStyle::default_bar());
        }
    }

    let result = match cli.mode {
        Mode::Encrypt => {
            pb.set_message("Criptografando");
            crypto::io::encrypt_file(&key, &input_path, &output_path, Some(&pb))
        }
        Mode::Decrypt => {
            pb.set_message("Descriptografando");
            crypto::io::decrypt_file(&key, &input_path, &output_path, Some(&pb))
        }
    };

    match result {
        Ok(()) => {
            pb.finish_and_clear();
            println!("Operação concluída com sucesso!");
            println!("Arquivo de saída: {}", output_path.display());
        }
        Err(e) => {
            pb.finish_and_clear();
            eprintln!("Erro durante a operação: {}", e);
            let _ = std::fs::remove_file(&output_path);
            std::process::exit(1);
        }
    }
}
