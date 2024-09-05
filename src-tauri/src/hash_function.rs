use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

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
    pub fn compute_digest(path_buf: PathBuf, hash_function: Self) -> Result<String, ()> {
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
    // Obsolete
    fn compute_md5(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut bytes = vec![];
            if f.read_to_end(&mut bytes).is_ok() {
                let digest = md5::compute(bytes);
                Ok(format!("{:x}", digest))
            } else {
                Err(()) // Couldn't read file
            }
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha1(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha1::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    // SHA-2
    fn compute_sha224(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha224::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha256(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha256::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha384(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha384::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha512(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha512::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha512_224(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha512_224::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha512_256(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha512_256::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }
    // SHA-3
    fn compute_sha3_224(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha3_224::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha3_256(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha3_256::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha3_384(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha3_384::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }

    fn compute_sha3_512(path_buf: PathBuf) -> Result<String, ()> {
        if let Ok(mut f) = File::open(path_buf) {
            let mut data = [0; 0x1000];
            let mut hasher = Sha3_512::new();
            let digest = {
                while let Ok(n) = f.read(&mut data) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&data[..n]);
                }
                hasher.finalize()
            };
            Ok(format!("{:x}", digest))
        } else {
            Err(()) // Couldn't open file
        }
    }
}

impl TryFrom<String> for HashFunction {
    type Error = ();

    fn try_from(value: String) -> Result<Self, ()> {
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

            _ => Err(()),
        }
    }
}
