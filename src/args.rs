// Desenvolvido por L. A. Leandro São José dos Campos- SP - 23/05/2026

use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

#[derive(Parser)]
#[command(
    name = "secure-cryptor",
    version,
    about = "Ferramenta de criptografia AES-256-GCM para arquivos locais",
    long_about = "Secure Cryptor CLI - Criptografe e descriptografe arquivos com AES-256-GCM.\n\n\
                  Utiliza buffers de I/O para processamento eficiente mesmo em arquivos grandes.\n\
                  A chave fornecida é derivada via SHA-256 para uma chave de 256 bits."
)]
pub struct Cli {
    #[arg(short, long, value_enum, help = "Modo de operação: encrypt ou decrypt")]
    pub mode: Mode,

    #[arg(short, long, help = "Caminho do arquivo de entrada")]
    pub input: String,

    #[arg(short, long, help = "Caminho do arquivo de saída (opcional, auto-definido se omitido)")]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        help = "Senha ou chave para criptografia/descriptografia",
        long_help = "Uma senha em texto puro. Será derivada via SHA-256 para uma chave AES de 256 bits.\n\
                     Recomenda-se usar senhas longas e complexas para segurança máxima."
    )]
    pub key: String,
}
