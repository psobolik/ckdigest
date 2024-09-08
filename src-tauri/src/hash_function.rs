use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use crate::error;

#[derive(Clone, Serialize, Deserialize)]
pub enum HashFunction {
    MD5,
    SHA1,

    SHA224,
    SHA256,
    SHA384,
    SHA512,
    SHA512_224,
    SHA512_256,

    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

impl HashFunction {
    pub fn compute_digest(path_buf: PathBuf, hash_function: Self) -> Result<String, error::Error> {
        match hash_function {
            Self::MD5 => Self::compute_md5(path_buf),
            Self::SHA1 => Self::compute_sha1(path_buf),

            Self::SHA224 => Self::compute_sha224(path_buf),
            Self::SHA256 => Self::compute_sha256(path_buf),
            Self::SHA384 => Self::compute_sha384(path_buf),
            Self::SHA512 => Self::compute_sha512(path_buf),
            Self::SHA512_224 => Self::compute_sha512_224(path_buf),
            Self::SHA512_256 => Self::compute_sha512_256(path_buf),

            Self::Sha3_224 => Self::compute_sha3_224(path_buf),
            Self::Sha3_256 => Self::compute_sha3_256(path_buf),
            Self::Sha3_384 => Self::compute_sha3_384(path_buf),
            Self::Sha3_512 => Self::compute_sha3_512(path_buf),
        }
    }
    // Obsolete algorithms
    fn compute_md5(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        let digest = md5::compute(&bytes);
        Ok(format!("{:x}", digest))
    }

    fn compute_sha1(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha1::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    // SHA-2 algorithms
    fn compute_sha224(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha224::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    fn compute_sha256(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha256::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    fn compute_sha384(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha384::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    fn compute_sha512(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha512::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    fn compute_sha512_224(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha512_224::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    fn compute_sha512_256(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha512_256::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }
    // SHA-3 algorithms
    fn compute_sha3_224(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha3_224::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    fn compute_sha3_256(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha3_256::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    fn compute_sha3_384(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha3_384::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }

    fn compute_sha3_512(path_buf: PathBuf) -> Result<String, error::Error> {
        let mut file = File::open(path_buf)?;
        let mut buffer = [0; 0x1000];
        let mut hasher = Sha3_512::new();
        let digest = {
            while let Ok(n) = file.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }
}

impl TryFrom<String> for HashFunction {
    type Error = error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "MD5" => Ok(HashFunction::MD5),
            "SHA1" => Ok(HashFunction::SHA1),

            "SHA224" => Ok(HashFunction::SHA224),
            "SHA256" => Ok(HashFunction::SHA256),
            "SHA384" => Ok(HashFunction::SHA384),
            "SHA512" => Ok(HashFunction::SHA512),
            "SHA512_224" => Ok(HashFunction::SHA512_224),
            "SHA512_256" => Ok(HashFunction::SHA512_256),

            "Sha3_224" => Ok(HashFunction::Sha3_224),
            "Sha3_256" => Ok(HashFunction::Sha3_256),
            "Sha3_384" => Ok(HashFunction::Sha3_384),
            "Sha3_512" => Ok(HashFunction::Sha3_512),

            _ => Err(error::Error::new("HashFunction", format!("Invalid hash function: '{}'", value).as_str())),
        }
    }
}

impl Display for HashFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HashFunction::MD5 => String::from("MD5"),
            HashFunction::SHA1 => String::from("SHA1"),
            HashFunction::SHA224 => String::from("SHA224"),
            HashFunction::SHA256 => String::from("SHA256"),
            HashFunction::SHA384 => String::from("SHA384"),
            HashFunction::SHA512 => String::from("SHA512"),
            HashFunction::SHA512_224 => String::from("SHA512_224"),
            HashFunction::SHA512_256 => String::from("SHA512_256"),
            HashFunction::Sha3_224 => String::from("Sha3_224"),
            HashFunction::Sha3_256 => String::from("Sha3_256"),
            HashFunction::Sha3_384 => String::from("Sha3_384"),
            HashFunction::Sha3_512 => String::from("Sha3_512"),
        };
        write!(f, "{}", str)
    }
}