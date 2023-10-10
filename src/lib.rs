use num_cpus;
use rayon::prelude::*;
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM, NONCE_LEN};

pub struct AesGcmEncryptor {
    master_key: [u8; 32],
}

impl AesGcmEncryptor {
    pub fn new(master_key: [u8; 32]) -> Self {
        AesGcmEncryptor { master_key }
    }

    fn encrypt_chunk(&self, chunk: &mut [u8], block_num: usize) {
        let aad = Aad::from(b"additional associated data");
        let unbound_key =
            UnboundKey::new(&AES_256_GCM, &self.master_key).expect("Failed to create key");
        let key = LessSafeKey::new(unbound_key);
        let mut nonce_bytes = [0u8; NONCE_LEN];
        nonce_bytes[0..std::mem::size_of::<usize>()].copy_from_slice(&block_num.to_be_bytes());
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        _ = key
            .seal_in_place_separate_tag(nonce, aad, chunk)
            .expect("Encryption failed");
    }

    pub fn encrypt(&self, data: &mut [u8]) {
        let num_cores = num_cpus::get();
        let chunk_size = data.len() / num_cores;
        let mut chunks: Vec<_> = data.chunks_mut(chunk_size).collect();

        chunks
            .par_iter_mut()
            .enumerate()
            .for_each(|(block_num, chunk)| self.encrypt_chunk(chunk, block_num));
    }
}
