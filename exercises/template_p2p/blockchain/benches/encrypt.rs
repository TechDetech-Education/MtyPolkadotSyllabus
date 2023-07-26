use blockchain::encryption;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const LOREM_IPSUM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
    In id turpis hendrerit magna congue molestie tincidunt ac tellus. \
    Vestibulum erat dolor, euismod tincidunt nisi in, porta tincidunt enim. \
    Aenean non ullamcorper lacus, eu accumsan justo. \
    Pellentesque ultrices odio vel pharetra porta. \
    Aliquam fringilla bibendum quam, at sagittis dolor condimentum at. \
    Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. \
    Donec quis scelerisque orci, vitae vestibulum eros. \
    Phasellus pretium sodales hendrerit. \
    Pellentesque enim turpis, iaculis ut dignissim ut, consequat ac ex. \
    Nam mollis varius arcu a faucibus. \
    Nunc ac mi vel ligula luctus placerat nec sed velit. \
    Nunc ut cursus velit. Etiam turpis velit, rhoncus et tincidunt id, bibendum id ante.";

// for version
fn encrypt(shared_key: u32, message: &str) -> String {
    let mut ciphertext = String::new();
    for c in message.chars() {
        ciphertext.push(char::from_u32(c as u32 + shared_key).unwrap_or(c));
    }
    ciphertext
}

fn criterion_benchmark(c: &mut Criterion) {
    let shared_key: u32 = 255;
    let exchange = encryption::Exchange {
        private_key: 10,
        public_key: 5,
        shared_key,
    };

    c.bench_function("for_loop", |b| {
        b.iter(|| encrypt(shared_key, black_box(LOREM_IPSUM)))
    });

    c.bench_function("iter_loop", |b| {
        b.iter(|| exchange.encrypt(black_box(LOREM_IPSUM)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
