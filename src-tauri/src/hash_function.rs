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

    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
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

            Self::SHA3_224 => Self::compute_sha3_224(path_buf),
            Self::SHA3_256 => Self::compute_sha3_256(path_buf),
            Self::SHA3_384 => Self::compute_sha3_384(path_buf),
            Self::SHA3_512 => Self::compute_sha3_512(path_buf),
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
        match value.to_ascii_uppercase().as_str() {
            "MD5" => Ok(HashFunction::MD5), // openssl and md5sum
            "SHA1" => Ok(HashFunction::SHA1), // openssl and shasum

            "SHA2-224" => Ok(HashFunction::SHA224), // openssl
            "SHA224" => Ok(HashFunction::SHA224), // shasum

            "SHA2-256" => Ok(HashFunction::SHA256), // openssl
            "SHA256" => Ok(HashFunction::SHA256), // shasum

            "SHA2-384" => Ok(HashFunction::SHA384), // openssl
            "SHA384" => Ok(HashFunction::SHA384), // shasum

            "SHA2-512" => Ok(HashFunction::SHA512), // openssl
            "SHA512" => Ok(HashFunction::SHA512), // shasum

            "SHA2-512/224" => Ok(HashFunction::SHA512_224), // openssl
            "SHA512/224" => Ok(HashFunction::SHA512_224), // shasum
            "SHA512_224" => Ok(HashFunction::SHA512_224), // frontend

            "SHA2-512/256" => Ok(HashFunction::SHA512_256), // openssl
            "SHA512/256" => Ok(HashFunction::SHA512_256), // shasum
            "SHA512_256" => Ok(HashFunction::SHA512_256), // frontend

            "SHA3-224" => Ok(HashFunction::SHA3_224), // openssl
            "SHA3_224" => Ok(HashFunction::SHA3_224), // frontend

            "SHA3-256" => Ok(HashFunction::SHA3_256), // openssl
            "SHA3_256" => Ok(HashFunction::SHA3_256), // frontend

            "SHA3-384" => Ok(HashFunction::SHA3_384), // openssl
            "SHA3_384" => Ok(HashFunction::SHA3_384), // frontend

            "SHA3-512" => Ok(HashFunction::SHA3_512), // openssl
            "SHA3_512" => Ok(HashFunction::SHA3_512), // frontend



            _ => Err(error::Error::new("HashFunction", format!("Invalid hash function: '{}'", value).as_str())),
        }
    }
}

impl Display for HashFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HashFunction::MD5 => String::from("MD5"),
            HashFunction::SHA1 => String::from("SHA1"),
            HashFunction::SHA224 => String::from("SHA2-224"),
            HashFunction::SHA256 => String::from("SHA2-256"),
            HashFunction::SHA384 => String::from("SHA2-384"),
            HashFunction::SHA512 => String::from("SHA2-512"),
            HashFunction::SHA512_224 => String::from("SHA2-512/224"),
            HashFunction::SHA512_256 => String::from("SHA2-512/256"),
            HashFunction::SHA3_224 => String::from("SHA3-224"),
            HashFunction::SHA3_256 => String::from("SHA3-256"),
            HashFunction::SHA3_384 => String::from("SHA3-384"),
            HashFunction::SHA3_512 => String::from("SHA3-512"),
        };
        write!(f, "{}", str)
    }
}