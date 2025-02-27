use rand::RngCore;
use rand::SeedableRng;
use rand::rngs::SmallRng;

const LETTER_BYTES: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const LETTER_IDX_BITS: u32 = 6;
const LETTER_IDX_MASK: u64 = (1 << LETTER_IDX_BITS) - 1;
const LETTER_IDX_MAX: usize = 63 / LETTER_IDX_BITS as usize;

pub fn rand_string(n: usize) -> String {
    let mut rng = SmallRng::from_os_rng();

    let mut buffer = vec![0u8; n];
    let mut cache: u64 = 0;
    let mut remain = 0;

    for i in (0..n).rev() {
        if remain == 0 {
            cache = rng.next_u64();
            remain = LETTER_IDX_MAX;
        }

        let idx = (cache & LETTER_IDX_MASK) as usize;
        if idx < LETTER_BYTES.len() {
            buffer[i] = LETTER_BYTES[idx];
            remain -= 1;
        }

        cache >>= LETTER_IDX_BITS;
    }

    unsafe { String::from_utf8_unchecked(buffer) }
}
