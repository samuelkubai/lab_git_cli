use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

pub enum Tree {
    BlobEntry {
        name: String,
        hash: String,
    },
    TreeEntry {
        name: String,
        hash: String,
        children: Vec<Tree>,
    },
}

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>,
}

impl Blob {
    pub fn from_path(path: &PathBuf) -> io::Result<Blob> {
        // TODO: Research what is a path buf
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?; // TODO: Refresh on how to pass variables to functions

        let mut sha = Sha1::new();
        sha.input(&bytes);

        Ok(Blob {
            hash: sha.result_str(),
            data: bytes,
        })
    }
}
