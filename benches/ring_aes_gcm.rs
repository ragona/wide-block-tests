extern crate ring;
extern crate criterion;

mod utils;

use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM, NONCE_LEN};
use criterion::{Criterion, black_box, criterion_group, criterion_main};


fn encrypt_file() {
    utils::ensure_buffer_loaded();
    let buffer = utils::get_buffer();

    let aad = Aad::from(b"additional associated data");
    let key_bytes = [0u8; 32];
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes).expect("Failed to create key");
    let key = LessSafeKey::new(unbound_key);
    let nonce = Nonce::assume_unique_for_key([0u8; NONCE_LEN]);

    key.seal_in_place_append_tag(nonce, aad, buffer).expect("Encryption failed");
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("encrypt 1GB with ring", |b| {
        b.iter(|| encrypt_file())
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
