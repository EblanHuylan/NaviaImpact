use std::time::{SystemTime, UNIX_EPOCH};

use rand_mt::Mt64;

pub fn cur_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn cur_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
}

pub fn generate_xorpad(seed: u64) -> Box<[u8]> {
    let mut mt = Mt64::new(seed);
    let new_seed = mt.next_u64();
    mt.reseed(new_seed);
    mt.next_u64();

    let mut buf = Vec::with_capacity(4096);
    for _ in 0..512 {
        buf.extend(mt.next_u64().to_be_bytes());
    }

    buf.into()
}

pub fn xor(bytes: &mut [u8], key: &[u8]) {
    for i in 0..bytes.len() {
        bytes[i] ^= key[i % key.len()];
    }
}

pub fn hash_str(value: &str) -> u32 {
    let mut hash = 0;
    value
        .chars()
        .for_each(|c| hash = (((c as u64) + 131 * hash as u64) & 0xFFFFFFFF) as u32);

    hash
}
