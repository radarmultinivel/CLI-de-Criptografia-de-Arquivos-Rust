// Desenvolvido por L. A. Leandro São José dos Campos- SP - 23/05/2026

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use indicatif::ProgressBar;

use super::cipher;

const MAGIC: &[u8; 4] = b"SCRY";
const CHUNK_SIZE: usize = 1_048_576; // 1 MiB
const NONCE_LEN: usize = 12;

pub fn encrypt_file(
    key: &[u8; 32],
    input: &Path,
    output: &Path,
    progress: Option<&ProgressBar>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output)?;
    let mut writer = BufWriter::new(output_file);

    writer.write_all(MAGIC)?;
    writer.write_all(&(CHUNK_SIZE as u32).to_le_bytes())?;

    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let mut chunk_size = 0usize;
        while chunk_size < buffer.len() {
            match reader.read(&mut buffer[chunk_size..])? {
                0 => break,
                n => chunk_size += n,
            }
        }

        if chunk_size == 0 {
            break;
        }

        let (nonce, encrypted) = cipher::encrypt_chunk(key, &buffer[..chunk_size]);

        writer.write_all(&nonce)?;
        writer.write_all(&(encrypted.len() as u32).to_le_bytes())?;
        writer.write_all(&encrypted)?;

        if let Some(pb) = progress {
            pb.inc(chunk_size as u64);
        }
    }

    writer.flush()?;
    Ok(())
}

pub fn decrypt_file(
    key: &[u8; 32],
    input: &Path,
    output: &Path,
    progress: Option<&ProgressBar>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(input)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output)?;
    let mut writer = BufWriter::new(output_file);

    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err("Formato de arquivo inválido: magic bytes incorretos".into());
    }

    let mut chunk_size_bytes = [0u8; 4];
    reader.read_exact(&mut chunk_size_bytes)?;
    let _chunk_size = u32::from_le_bytes(chunk_size_bytes);

    loop {
        let mut nonce = vec![0u8; NONCE_LEN];
        if reader.read_exact(&mut nonce).is_err() {
            break;
        }

        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes)?;
        let encrypted_len = u32::from_le_bytes(len_bytes) as usize;

        let mut encrypted = vec![0u8; encrypted_len];
        reader.read_exact(&mut encrypted)?;

        let plaintext = cipher::decrypt_chunk(key, &nonce, &encrypted)
            .map_err(|_| "Falha na descriptografia: chave incorreta ou dados corrompidos")?;

        writer.write_all(&plaintext)?;

        if let Some(pb) = progress {
            pb.inc((NONCE_LEN + 4 + encrypted_len) as u64);
        }
    }

    writer.flush()?;
    Ok(())
}
