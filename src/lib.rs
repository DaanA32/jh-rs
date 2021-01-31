use std::convert::TryFrom;
mod hash;
use hash::*;
mod e8;

///

/// hash a message,
/// 
/// two inputs: message digest size in bits (hashbitlen); message (data)
/// 
/// one output:   message digest (hashval)
///
pub fn hash(bitlen: u16, data: &[u8], result: &mut [u8]) -> Result<(), JhError> {
    let bitlen = HashBitLen::try_from(bitlen)?;
    let mut state = HashState::new(bitlen);
    state.update(data);
    state.finalize(result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use rand::Fill;
    use jhffi;
    use super::hash;
    #[test]
    fn small() {
        let keccak_state: [u8; 200] = [0; 200];
        let mut result_jhffi = [0; 32];
        let mut result = [0; 32];
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
        hash(256, &keccak_state, &mut result).unwrap();
        assert_eq!(result_jhffi, result);
    }

    #[test]
    fn large() {
        let keccak_state: [u8; 1600] = [1; 1600];
        let mut result_jhffi = [0; 32];
        let mut result = [0; 32];
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
        hash(256, &keccak_state, &mut result).unwrap();
        assert_eq!(result_jhffi, result);
    }

    #[test]
    fn large_6400() {
        let keccak_state: [u8; 6400] = [1; 6400];
        let mut result_jhffi = [0; 32];
        let mut result = [0; 32];
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
        hash(256, &keccak_state, &mut result).unwrap();
        assert_eq!(result_jhffi, result);
    }

    #[test]
    fn rand_100() {
        let mut rng = rand::thread_rng();
        let mut keccak_state: [u8; 200] = [0; 200];
        let mut result_jhffi = [0; 32];
        let mut result = [0; 32];
        for _ in 0..100 {
            keccak_state.try_fill(&mut rng).unwrap();
            jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
            hash(256, &keccak_state, &mut result).unwrap();
            assert_eq!(result_jhffi, result);
        }
    }

    #[test]
    fn rand_100_large() {
        let mut rng = rand::thread_rng();
        let mut keccak_state: [u8; 1600] = [0; 1600];
        let mut result_jhffi = [0; 32];
        let mut result = [0; 32];
        for _ in 0..100 {
            keccak_state.try_fill(&mut rng).unwrap();
            jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
            hash(256, &keccak_state, &mut result).unwrap();
            assert_eq!(result_jhffi, result);
        }
    }
}
