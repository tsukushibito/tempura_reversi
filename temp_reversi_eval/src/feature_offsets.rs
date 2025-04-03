use super::patterns::PATTERNS;

pub const fn make_feature_offsets() -> [u16; PATTERNS.len()] {
    let mut offsets = [0; PATTERNS.len()];
    let mut offset = 0;
    let mut i = 0;
    while i < PATTERNS.len() {
        offsets[i + 0] = offset;
        offsets[i + 1] = offset;
        offsets[i + 2] = offset;
        offsets[i + 3] = offset;
        offset = offset + PATTERNS[i].len() as u16;
        i += 4;
    }
    offsets
}

pub const FEATURE_OFFSETS: [u16; PATTERNS.len()] = make_feature_offsets();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_offsets() {
        let offsets = make_feature_offsets();
        assert_eq!(offsets[0], 0);
        assert_eq!(offsets[1], 0);
        assert_eq!(offsets[2], 0);
        assert_eq!(offsets[3], 0);
        assert_eq!(offsets[4], PATTERNS[4].len() as u16);
        assert_eq!(offsets[5], PATTERNS[4].len() as u16);
        assert_eq!(offsets[6], PATTERNS[4].len() as u16);
        assert_eq!(offsets[7], PATTERNS[4].len() as u16);
    }
}
