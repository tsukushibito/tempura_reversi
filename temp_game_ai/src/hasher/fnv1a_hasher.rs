use std::hash::Hasher;

#[derive(Debug)]
pub struct Fnv1aHasher(u64);

impl Fnv1aHasher {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x00000100000001b3;
}

impl Default for Fnv1aHasher {
    fn default() -> Self {
        Fnv1aHasher(Self::OFFSET_BASIS)
    }
}

impl Hasher for Fnv1aHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= byte as u64;
            self.0 = self.0.wrapping_mul(Self::FNV_PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

pub type Fnv1aHashMap<K, V> =
    std::collections::HashMap<K, V, std::hash::BuildHasherDefault<Fnv1aHasher>>;
