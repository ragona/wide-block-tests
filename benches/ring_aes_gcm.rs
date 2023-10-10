extern crate criterion;
extern crate ring;

use wide_block_tests::AesGcmEncryptor;
use criterion::{Criterion, criterion_group, criterion_main};
mod utils;

fn benchmark(c: &mut Criterion) {
    utils::ensure_buffer_loaded();
    let mut buffer = utils::get_buffer();

    let master_key = [0u8; 32];  // Your chosen master key
    let encryptor = AesGcmEncryptor::new(master_key);

    c.bench_function("encrypt 1GB with ring in parallel", |b| {
        b.iter(|| {
            encryptor.encrypt(&mut buffer);
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
