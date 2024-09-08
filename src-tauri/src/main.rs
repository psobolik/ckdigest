// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use self::hash_function::HashFunction;
use serde::Serialize;
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::regex::Regex;

mod error;
mod hash_function;

struct ProcessDigestFileParts {
    algorithm: String,
    file_name: String,
    digest: String,
}

#[derive(Serialize)]
struct DigestFileParts {
    algorithm: HashFunction,
    file: PathBuf,
    digest: String,
}

#[tauri::command]
async fn pick_file() -> Option<PathBuf> {
    FileDialogBuilder::new().pick_file()
}

#[tauri::command]
async fn pick_digest_file() -> Option<PathBuf> {
    FileDialogBuilder::new()
        .add_filter(
            "Digest Files",
            &[
                "md5", "MD5", "sha1", "SHA1", "sha224", "SHA224", "sha256", "SHA256", "sha384",
                "SHA384", "sha512", "SHA512",
            ],
        )
        .add_filter("Files with any extension", &["*"])
        .set_title("Open digest file")
        .pick_file()
}

#[tauri::command]
async fn parse_digest_file(digest_file: PathBuf) -> Result<DigestFileParts, error::Error> {
    fn full_file_path(path: &PathBuf, file_name: String) -> Result<PathBuf, error::Error> {
        let mut file = path.clone();
        file.set_file_name(file_name.as_str());
        Ok(fs::canonicalize(file)?)
    }
    let line = read_line(&digest_file)?;
    let parts = parse_line(&line)?;
    let file = full_file_path(&digest_file, parts.file_name)?;
    let algorithm = hash_function(parts.algorithm)?;
    Ok(DigestFileParts {
        algorithm,
        file,
        digest: parts.digest,
    })
}

#[tauri::command]
async fn save_digest_file(
    digested_file: PathBuf,
    hash_function: HashFunction,
    digest: &str,
) -> Result<Option<PathBuf>, error::Error> {
    let algorithm = format!("{}", hash_function);
    let directory = digested_file.parent().unwrap_or(Path::new("."));
    let mut file_name = PathBuf::from(
        digested_file
            .file_stem()
            .unwrap_or(OsString::from("digest_file").as_ref()),
    );
    file_name.set_extension(&algorithm);
    if let Some(digest_file) = FileDialogBuilder::new()
        .set_directory(directory)
        .set_file_name(file_name.to_string_lossy().as_ref())
        .save_file()
    {
        let text = format!(
            "{} ({}) = {}",
            algorithm,
            digested_file.file_name().unwrap().to_string_lossy(),
            digest
        );
        let mut file = fs::File::create(&digest_file)?;
        file.write_all(text.as_bytes())?;
        Ok(Some(digest_file))
    } else {
        Ok(None) // User didn't pick a file
    }
}

#[tauri::command]
async fn calculate_digest(
    path_buf: PathBuf,
    hash_function: HashFunction,
) -> Result<String, error::Error> {
    HashFunction::compute_digest(path_buf, hash_function)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            pick_file,
            pick_digest_file,
            parse_digest_file,
            calculate_digest,
            save_digest_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn hash_function(algorithm: String) -> Result<HashFunction, error::Error> {
    match HashFunction::try_from(algorithm) {
        Ok(hash_function) => Ok(hash_function),
        Err(_) => Err(error::Error::other("Invalid algorithm in digest file")),
    }
}

fn read_line<P>(filename: P) -> Result<String, error::Error>
where
    P: AsRef<Path>,
{
    let string = fs::read_to_string(filename)?;
    let lines: Vec<String> = string.lines().map(String::from).collect();
    match lines.len() {
        0 => Ok("".to_string()),
        _ => Ok(lines[0].clone()),
    }
}

fn parse_line(line: &str) -> Result<ProcessDigestFileParts, error::Error> {
    const TAGGED_DIGEST_PATTERN: &str =
        r"^(?<algorithm>.+)\s\((?<filename>.+)\)\s=\s(?<digest>[0-9a-fA-f]+)$";
    let regex = Regex::new(TAGGED_DIGEST_PATTERN).expect("Invalid regular expression");

    if let Some(captures) = regex.captures(line) {
        if let (Some(algorithm), Some(file_name), Some(digest)) = (
            captures.name("algorithm"),
            captures.name("filename"),
            captures.name("digest"),
        ) {
            Ok(ProcessDigestFileParts {
                algorithm: algorithm.as_str().to_string(),
                file_name: file_name.as_str().to_string(),
                digest: digest.as_str().to_string(),
            })
        } else {
            Err(error::Error::other("Missing data in digest file"))
        }
    } else {
        Err(error::Error::other("Invalid data in digest file"))
    }
}
