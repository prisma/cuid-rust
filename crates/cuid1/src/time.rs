use crate::text::to_base_string;

pub fn timestamp() -> String {
    to_base_string(cuid_util::millis_since_unix_epoch())
}

#[cfg(test)]
mod time_tests {
    use super::super::BASE;
    use super::*;

    // NOTE: this will start failing in ~2059, at which point this will need to
    // be updated to 9
    #[test]
    fn test_timestamp_len() {
        assert_eq!(timestamp().len(), 8);
    }

    #[test]
    fn test_timestamp() {
        assert!(
            (cuid_util::millis_since_unix_epoch()
                - u128::from_str_radix(&timestamp(), BASE as u32).unwrap())
                < 5
        )
    }
}
