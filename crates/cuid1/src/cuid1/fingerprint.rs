use num::bigint;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_512};
#[cfg(not(target_arch = "wasm32"))]
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{error::CuidError, BASE};

const BIG_LENGTH: u8 = 4;

// =============================================================================
// THREAD LOCALS
// =============================================================================
// Each thread generating CUIDs gets its own:
// - 64-bit counter, randomly initialized to some value between 0 and 2056, inclusive
// - fingerprint, a hash with added entropy, derived from a random number between
//   2063 and 4125, inclusive, the process ID, and the thread ID

thread_local! {
    /// Fingerprint! The original implementation is a hash of:
    /// - stringified keys of the global object
    /// - added entropy
    ///
    /// For us, we'll use
    /// - A few random numbers
    /// - the process ID
    /// - the thread ID (which also ensures our CUIDs will be different per thread)
    ///
    /// This is pretty non-language, non-system dependent, so it allows us to
    /// compile to wasm and so on.
    static FINGERPRINT: String = hash(
        [
            thread_rng().gen::<u128>().to_be_bytes(),
            thread_rng().gen::<u128>().to_be_bytes(),
            #[cfg(not(target_arch="wasm32"))]
            u128::from(std::process::id()).to_be_bytes(),
            #[cfg(not(target_arch="wasm32"))]
            u128::from(get_thread_id()).to_be_bytes(),
        ],
        BIG_LENGTH.into(),
    );
}

#[cfg(not(target_arch = "wasm32"))]
/// Retrieves the current thread's ID.
fn get_thread_id() -> u64 {
    // ThreadId doesn't implement debug or display, but it does implement Hash,
    // so we can get the hash value to use in our fingerprint.
    let mut hasher = DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

/// Retrieves the thread-local fingerprint.
fn get_fingerprint() -> String {
    FINGERPRINT.with(|x| x.clone())
}

// Hashing
// =======

/// Hash a value, including an additional salt of randomly generated data.
//
// Updated 2023-08-08 to match the updated JS implementation, which is:
//
// ```js
// const hash = (input = "") => {
//   // Drop the first character because it will bias the histogram
//   // to the left.
//   return bufToBigInt(sha3(input)).toString(36).slice(1);
// };
// ```
//
// We don't drop the first character, because it doesn't actually affect the
// histogram (the comment in the reference implementation is incorrect).
fn hash<S: AsRef<[u8]>, T: IntoIterator<Item = S>>(input: T, length: u16) -> String {
    let mut hasher = Sha3_512::new();

    for block in input {
        hasher.update(block.as_ref());
    }

    // 512 bits (64 bytes) of data ([u8; 64])
    let hash = hasher.finalize();

    // We'll convert the bytes directly to a big, unsigned int and then use
    // its builtin radix conversion.
    //
    // We don't use bigint for the rest of our base conversions, because it's
    // significantly slower, but we use it here since we need to deal with the
    // 512-bit integer from the hash function.
    let mut res = bigint::BigUint::from_bytes_be(&hash).to_str_radix(BASE.into());

    // Note that truncate panics if the length does not fall on a char boundary,
    // but we don't need to worry about that since all the chars will be ASCII.
    res.truncate(length.into());

    res
}

pub fn fingerprint() -> Result<String, CuidError> {
    let fingerprint = get_fingerprint();
    Ok(fingerprint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_length() {
        assert_eq!(fingerprint().unwrap().len(), BIG_LENGTH as usize)
    }
}

#[cfg(nightly)]
#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_fingerprint(b: &mut Bencher) {
        b.iter(|| {
            fingerprint().unwrap();
        })
    }
}
