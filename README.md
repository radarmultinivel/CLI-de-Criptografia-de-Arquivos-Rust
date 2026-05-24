Desenvolvido por L. A. Leandro São José dos Campos- SP - 23/05/2026

# Secure Cryptor CLI

Criptografia AES-256-GCM para arquivos locais.

---

## Objetivo

Utilitario de linha de comando para criptografar e descriptografar arquivos utilizando o algoritmo AES-256-GCM
(Galois/Counter Mode). Projetado para processamento eficiente de arquivos de qualquer tamanho com consumo de
memoria constante, aproveitando o sistema de Ownership e Borrowing do Rust.

---

## Requisitos

- Sistema operacional: Windows, Linux ou macOS
- Rust edicao 2021+ (para compilacao)
- Sem dependencias externas em tempo de execucao

---

## Especificacoes

| Especificacao | Detalhe |
|---------------|---------|
| Algoritmo | AES-256-GCM (autenticacao e confidencialidade) |
| Tamanho da chave | 256 bits (32 bytes) |
| Derivacao de chave | SHA-256 a partir de senha fornecida pelo usuario |
| Nonce | 12 bytes aleatorios por bloco (OsRng) |
| Tamanho do bloco | 1 MiB (1.048.576 bytes) |
| Tag GCM | 16 bytes (incluida no ciphertext) |
| Formato do arquivo | Magic "SCRY" + chunk_size + [nonce + len + data] |

---

## Arquitetura

```
src/
 main.rs          # Ponto de entrada: parsing de argumentos e dispatcher
 args.rs          # Estruturas de CLI com clap
 crypto/
  mod.rs          # Reexportacao dos modulos cipher e io
  cipher.rs       # Primitivas criptograficas (encrypt/decrypt por bloco)
  io.rs           # Leitura e escrita de arquivos com buffers

Fluxo de execucao:

 [Usuario] -> CLI (clap) -> main.rs
                              |
                    Deriva chave (SHA-256)
                              |
                    Abre arquivo de entrada (BufReader)
                              |
                    [Encrypt]                    [Decrypt]
                        |                            |
                    Para cada bloco de 1 MiB    Le magic bytes "SCRY"
                              |                    |
                    Gera nonce aleatorio       Le chunk_size
                              |                    |
                    AES-256-GCM encrypt        Para cada bloco:
                              |                    |
                    Escreve nonce + len +      Le nonce (12 B)
                    ciphertext no output       Le encrypted_len (u32 LE)
                              |                    |
                    Atualiza barra de          Le ciphertext
                    progresso (indicatif)          |
                                               AES-256-GCM decrypt
                                                    |
                                              Escreve plaintext no output
                                                    |
                                              Atualiza barra de progresso
```

## Stacks, Tecnologias e Dependencias

| Componente | Biblioteca | Versao |
|------------|------------|--------|
| Interface CLI | clap (derive) | 4 |
| Criptografia AES-256-GCM | aes-gcm | 0.10 |
| Derivacao de chave (SHA-256) | sha2 | 0.10 |
| Numeros aleatorios seguros | rand | 0.8 |
| Codificacao hexadecimal | hex | 0.4 |
| Barra de progresso | indicatif | 0.17 |

Linguagem: Rust (edition 2021), compilador rustc + gerenciador de pacotes cargo.

---

## Instalacao

### Pre-requisitos

- Rust e Cargo instalados (https://rustup.rs/)

### Compilacao

```bash
git clone <url-do-repositorio>
cd <diretorio>
cargo build --release
```

O binario estara em `./target/release/secure-cryptor-cli.exe` (Windows) ou
`./target/release/secure-cryptor-cli` (Linux/macOS).

---

## Manual do Usuario

### Criptografar um arquivo

```bash
secure-cryptor-cli --mode encrypt --input documento.txt --key "minha-senha"
```

Gera `documento.txt.enc` no mesmo diretorio.

### Descriptografar um arquivo

```bash
secure-cryptor-cli --mode decrypt --input documento.txt.enc --key "minha-senha"
```

Gera `documento.txt.enc.dec` no mesmo diretorio.

### Especificar arquivo de saida

```bash
secure-cryptor-cli --mode encrypt --input foto.jpg --output backup.enc --key "senha-forte"
```

### Opcoes disponiveis

| Opcao | Descricao |
|-------|-----------|
| `-m, --mode <MODE>` | Modo de operacao: `encrypt` ou `decrypt` |
| `-i, --input <FILE>` | Caminho do arquivo de entrada |
| `-o, --output <FILE>` | Caminho do arquivo de saida (opcional) |
| `-k, --key <KEY>` | Senha para derivacao da chave AES-256 |

### Exemplos em lote

```bash
# Windows (PowerShell)
Get-ChildItem *.txt | ForEach-Object {
  secure-cryptor-cli --mode encrypt --input $_.Name --key "chave-mestra"
}
```

```bash
# Linux/macOS
for f in *.txt; do
  secure-cryptor-cli --mode encrypt --input "$f" --key "chave-mestra"
done
```

---

## Testes

```bash
cargo test
```

### Cenarios cobertos

- `test_round_trip_small`: criptografa e descriptografa mensagem curta
- `test_large_chunk`: round-trip com 1 MB de dados
- `test_empty_chunk`: bloco vazio processado corretamente
- `test_wrong_key_fails`: chave incorreta retorna erro (sem panic)
- `test_multiple_chunks_independence`: cada bloco e independente e recuperavel

---

## Formato do Arquivo Criptografado

```
Offset  Campo             Tamanho
------  -----             -------
0       Magic "SCRY"      4 bytes
4       Chunk Size (LE)   4 bytes
8       [Bloco 0]
          Nonce           12 bytes
          Encrypted Len   4 bytes (u32 LE)
          Encrypted Data  variavel (inclui tag GCM 16 B)
        [Bloco 1...]
```
