use std::{collections::HashMap, sync::LazyLock};

use temp_reversi_eval::{
    feature::{canonicalize_pattern_feature, Feature},
    patterns::PATTERNS,
};

/// A structure for efficiently packing and mapping feature indices for the reversi evaluation function.
///
/// `FeaturePacker` handles the conversion between different feature representations by maintaining
/// mappings between raw feature values and their packed indices, allowing for efficient lookup
/// during evaluation.
///
/// # Fields
///
/// * `index_map` - Vector of hashmaps that map feature values (u16) to their corresponding
///   packed indices (u16) for each feature type.
/// * `index_offsets` - Vector of offsets (u32) for each feature type, used to calculate the
///   absolute position in the packed feature vector.
pub struct FeaturePacker {
    pub index_map: Vec<HashMap<u16, u16>>,
    pub index_offsets: Vec<u32>,
    pub packed_feature_size: usize,
}

impl FeaturePacker {
    pub fn new() -> Self {
        // Each pattern has 90-degree rotational symmetry, so four patterns can be combined into one.
        let base_pattern_count = PATTERNS.len() / 4;
        let mut index_map = vec![HashMap::new(); base_pattern_count];
        let mut index_offset = 0;
        let mut index_offsets = vec![0; base_pattern_count];

        for (base_pattern_index, pattern) in PATTERNS.iter().step_by(4).enumerate() {
            index_offsets[base_pattern_index] = index_offset;

            let mut packed_index = 0;
            let feature_size = 3u16.pow(pattern.len() as u32);

            for feature in 0..feature_size {
                let pattern_index = base_pattern_index * 4;
                let canonical_feature = canonicalize_pattern_feature(pattern_index, feature);
                if let Some(&existing_index) = index_map[base_pattern_index].get(&canonical_feature)
                {
                    // If the feature is already in the map, assign the existing index
                    index_map[base_pattern_index].insert(feature, existing_index);
                } else {
                    // If the feature is not in the map, assign a new index
                    index_map[base_pattern_index].insert(feature, packed_index);
                    packed_index += 1;
                }
            }

            index_offset += packed_index as u32;
        }

        let packed_feature_size = index_map.iter().map(|m| m.len()).sum::<usize>();

        Self {
            index_map,
            index_offsets,
            packed_feature_size,
        }
    }

    pub fn pack(&self, feature: &Feature) -> Feature {
        let mut packed_feature = feature.clone();
        for (i, &index) in feature.indices.iter().enumerate() {
            let base_pattern_index = i / 4;
            if let Some(&packed_index) = self.index_map[base_pattern_index].get(&index) {
                packed_feature.indices[i] = packed_index;
            } else {
                panic!("Feature index not found in index map.");
            }
        }
        packed_feature
    }

    pub fn packed_index(&self, pattern_index: usize, feature_index: u16) -> Option<u16> {
        let base_pattern_index = pattern_index / 4;
        self.index_map[base_pattern_index]
            .get(&feature_index)
            .copied()
    }

    pub fn packed_feature_to_vector(&self, packed_feature: &Feature) -> Vec<u8> {
        let mut packed_vector = vec![0; self.packed_feature_size];
        for (i, &index) in packed_feature.indices.iter().enumerate() {
            let absolute_index = self.index_offsets[i / 4] as usize + index as usize;
            packed_vector[absolute_index] += 1;
        }

        packed_vector
    }
}

pub static FEATURE_PACKER: LazyLock<FeaturePacker> = LazyLock::new(FeaturePacker::new);

#[cfg(test)]
mod tests {
    use burn::backend::autodiff::checkpoint::base;
    use temp_reversi_core::Bitboard;
    use temp_reversi_eval::feature::extract_feature;

    use super::*;

    /// Tests the `FeaturePacker` struct.
    ///
    /// It creates a new `FeaturePacker`, packs a feature, and checks if the packed
    /// feature has the expected size. It also verifies the packed indices against
    /// expected values.
    ///
    /// The test uses a bitboard with all squares empty and checks the packed feature
    /// indices against known values. The packed feature size is determined from pattern
    /// definitions, symmetric pattern definitions, and Burnside's lemma.
    #[test]
    fn test_feature_packer() {
        let black = 0;
        let white = 0;
        let bitboard = Bitboard::new(black, white);

        let feature = extract_feature(&bitboard);

        let feature_packer = FeaturePacker::new();
        let packed_feature = feature_packer.pack(&feature);

        assert_eq!(packed_feature.indices.len(), feature.indices.len());

        // 3^4 + (3^8 - 3^4) / 2 = 3321
        assert_eq!(packed_feature.indices[0], 3320);
        assert_eq!(packed_feature.indices[1], 3320);
        assert_eq!(packed_feature.indices[2], 3320);
        assert_eq!(packed_feature.indices[3], 3320);
        assert_eq!(packed_feature.indices[4], 3320);
        assert_eq!(packed_feature.indices[5], 3320);
        assert_eq!(packed_feature.indices[6], 3320);
        assert_eq!(packed_feature.indices[7], 3320);
        assert_eq!(packed_feature.indices[8], 3320);
        assert_eq!(packed_feature.indices[9], 3320);
        assert_eq!(packed_feature.indices[10], 3320);
        assert_eq!(packed_feature.indices[11], 3320);

        // 3^9 + (3^10 - 3^9) / 2 = 39366
        assert_eq!(packed_feature.indices[12], 39365);
        assert_eq!(packed_feature.indices[13], 39365);
        assert_eq!(packed_feature.indices[14], 39365);
        assert_eq!(packed_feature.indices[15], 39365);

        // 3^4 + (3^7 - 3^4) / 2 = 1134
        assert_eq!(packed_feature.indices[16], 1133);
        assert_eq!(packed_feature.indices[17], 1133);
        assert_eq!(packed_feature.indices[18], 1133);
        assert_eq!(packed_feature.indices[19], 1133);

        // 3^3 + (3^6 - 3^3) / 2 = 378
        assert_eq!(packed_feature.indices[20], 377);
        assert_eq!(packed_feature.indices[21], 377);
        assert_eq!(packed_feature.indices[22], 377);
        assert_eq!(packed_feature.indices[23], 377);

        // 3^3 + (3^5 - 3^3) / 2 = 135
        assert_eq!(packed_feature.indices[24], 134);
        assert_eq!(packed_feature.indices[25], 134);
        assert_eq!(packed_feature.indices[26], 134);
        assert_eq!(packed_feature.indices[27], 134);

        // 3^5 + (3^10 - 3^5) / 2 = 29646
        assert_eq!(packed_feature.indices[28], 29645);
        assert_eq!(packed_feature.indices[29], 29645);
        assert_eq!(packed_feature.indices[30], 29645);
        assert_eq!(packed_feature.indices[31], 29645);
        assert_eq!(packed_feature.indices[32], 29645);
        assert_eq!(packed_feature.indices[33], 29645);
        assert_eq!(packed_feature.indices[34], 29645);
        assert_eq!(packed_feature.indices[35], 29645);
        assert_eq!(packed_feature.indices[36], 29645);
        assert_eq!(packed_feature.indices[37], 29645);
        assert_eq!(packed_feature.indices[38], 29645);
        assert_eq!(packed_feature.indices[39], 29645);
        assert_eq!(packed_feature.indices[40], 29645);
        assert_eq!(packed_feature.indices[41], 29645);
        assert_eq!(packed_feature.indices[42], 29645);
        assert_eq!(packed_feature.indices[43], 29645);

        // 3^6 + (3^9 - 3^6) / 2 = 10206
        assert_eq!(packed_feature.indices[44], 10205);
        assert_eq!(packed_feature.indices[45], 10205);
        assert_eq!(packed_feature.indices[46], 10205);
        assert_eq!(packed_feature.indices[47], 10205);

        // 3^6 + (3^10 - 3^6) / 2 = 29889
        assert_eq!(packed_feature.indices[48], 29888);
        assert_eq!(packed_feature.indices[49], 29888);
        assert_eq!(packed_feature.indices[50], 29888);
        assert_eq!(packed_feature.indices[51], 29888);
        assert_eq!(packed_feature.indices[52], 29888);
        assert_eq!(packed_feature.indices[53], 29888);
        assert_eq!(packed_feature.indices[54], 29888);
        assert_eq!(packed_feature.indices[55], 29888);

        // 3^7 + (3^10 - 3^7) / 2 = 30618
        assert_eq!(packed_feature.indices[56], 30617);
        assert_eq!(packed_feature.indices[57], 30617);
        assert_eq!(packed_feature.indices[58], 30617);
        assert_eq!(packed_feature.indices[59], 30617);
        assert_eq!(packed_feature.indices[60], 30617);
        assert_eq!(packed_feature.indices[61], 30617);
        assert_eq!(packed_feature.indices[62], 30617);
        assert_eq!(packed_feature.indices[63], 30617);
    }

    #[test]
    fn test_pack_to_vector() {
        let black = 0;
        let white = 0;
        let bitboard = Bitboard::new(black, white);

        let feature = extract_feature(&bitboard);

        let feature_packer = FeaturePacker::new();
        let packed_feature = feature_packer.pack(&feature);
        let packed_vector = feature_packer.packed_feature_to_vector(&packed_feature);

        assert_eq!(packed_vector.len(), feature_packer.packed_feature_size);

        for (pattern_index, &index) in feature.indices.iter().enumerate() {
            let base_pattern_index = pattern_index / 4;
            let absolute_index =
                feature_packer.index_offsets[base_pattern_index] as usize + index as usize;
            let index_offset = feature_packer.index_offsets[base_pattern_index] as usize;
            let expected = if index as i64 == index_offset as i64 - 1 {
                4
            } else {
                0
            };
            assert_eq!(packed_vector[absolute_index], expected);
        }
    }
}
