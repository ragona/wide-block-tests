extern crate aes_gcm;
extern crate criterion;

mod utils;

use aes_gcm::{Aes256Gcm, KeyInit, aead::generic_array::GenericArray};
use aes_gcm::aead::Aead;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn encrypt_file() {
    utils::ensure_buffer_loaded();
    let buffer = utils::get_buffer();

    let key = GenericArray::from_slice(&[0u8; 32]);
    let cipher = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&[0u8; 12]);

    let _ciphertext = cipher.encrypt(nonce, buffer.as_ref()).expect("Encryption failed");
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("encrypt 1GB with RustCrypto", |b| {
        b.iter(|| encrypt_file())
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
