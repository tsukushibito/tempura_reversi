use std::ops::Index;

#[derive(Debug, Clone, Default)]
pub struct SparseFeature {
    pub indices: Vec<usize>,
    pub values: Vec<f32>,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub enum SparseFeatureError {
    LengthMismatch,
    IndexOutOfBounds,
    DuplicateIndices,
}

impl SparseFeature {
    pub fn new(
        indices: Vec<usize>,
        values: Vec<f32>,
        length: usize,
    ) -> Result<Self, SparseFeatureError> {
        if indices.len() != values.len() {
            return Err(SparseFeatureError::LengthMismatch);
        }
        if indices.iter().any(|&i| i >= length) {
            return Err(SparseFeatureError::IndexOutOfBounds);
        }
        let unique_indices: std::collections::HashSet<_> = indices.iter().collect();
        if unique_indices.len() != indices.len() {
            return Err(SparseFeatureError::DuplicateIndices);
        }

        let mut combined: Vec<(usize, f32)> = indices.into_iter().zip(values).collect();
        combined.sort_by_key(|&(i, _)| i);

        let (sorted_indices, sorted_values): (Vec<usize>, Vec<f32>) = combined.into_iter().unzip();

        Ok(SparseFeature {
            indices: sorted_indices,
            values: sorted_values,
            length,
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn assign(&mut self, index: usize, value: f32) {
        if index >= self.length {
            panic!(
                "Index out of bounds: the size is {}, but the index is {}",
                self.length, index
            );
        }

        match self.indices.binary_search(&index) {
            Ok(pos) => {
                if value == 0.0 {
                    self.indices.remove(pos);
                    self.values.remove(pos);
                } else {
                    self.values[pos] = value;
                }
            }
            Err(pos) => {
                if value != 0.0 {
                    self.indices.insert(pos, index);
                    self.values.insert(pos, value);
                }
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<f32> {
        if index >= self.length {
            return None;
        }

        match self.indices.binary_search(&index) {
            Ok(pos) => Some(self.values[pos]),
            Err(_) => Some(0.0),
        }
    }

    pub fn concat(&self, other: &SparseFeature) -> Result<SparseFeature, SparseFeatureError> {
        let new_length = self.length + other.length;

        let mut new_indices = self.indices.clone();
        let mut new_values = self.values.clone();

        new_indices.extend(other.indices.iter().map(|&index| index + self.length));
        new_values.extend(&other.values);

        SparseFeature::new(new_indices, new_values, new_length)
    }
}

impl Index<usize> for SparseFeature {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!(
                "Index out of bounds: the size is {}, but the index is {}",
                self.length, index
            );
        }

        match self.indices.binary_search(&index) {
            Ok(pos) => &self.values[pos],
            Err(_) => &0.0,
        }
    }
}

impl std::fmt::Display for SparseFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements: Vec<String> = self
            .iter()
            .map(|(index, value)| format!("{}: {:.2}", index, value))
            .collect();

        write!(
            f,
            "SparseFeature {{ length: {}, elements: {{ {} }} }}",
            self.length,
            elements.join(", ")
        )
    }
}

pub struct SparseFeatureIter<'a> {
    indices: std::slice::Iter<'a, usize>,
    values: std::slice::Iter<'a, f32>,
}

impl<'a> Iterator for SparseFeatureIter<'a> {
    type Item = (usize, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.indices.next(), self.values.next()) {
            (Some(&index), Some(&value)) => Some((index, value)),
            _ => None,
        }
    }
}

impl SparseFeature {
    pub fn iter(&self) -> SparseFeatureIter<'_> {
        SparseFeatureIter {
            indices: self.indices.iter(),
            values: self.values.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let sparse = SparseFeature::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        assert_eq!(sparse.len(), 5);
        assert_eq!(sparse.get(0), Some(1.0));
        assert_eq!(sparse.get(1), Some(0.0));
        assert_eq!(sparse.get(2), Some(2.0));
    }

    #[test]
    fn test_new_length_mismatch() {
        let result = SparseFeature::new(vec![0, 2], vec![1.0], 5);
        assert!(matches!(result, Err(SparseFeatureError::LengthMismatch)));
    }

    #[test]
    fn test_new_index_out_of_bounds() {
        let result = SparseFeature::new(vec![0, 5], vec![1.0, 2.0], 5);
        assert!(matches!(result, Err(SparseFeatureError::IndexOutOfBounds)));
    }

    #[test]
    fn test_new_duplicate_indices() {
        let result = SparseFeature::new(vec![0, 0], vec![1.0, 2.0], 5);
        assert!(matches!(result, Err(SparseFeatureError::DuplicateIndices)));
    }

    #[test]
    fn test_assign_add_element() {
        let mut sparse = SparseFeature::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        sparse.assign(3, 4.0);
        assert_eq!(sparse.get(3), Some(4.0));
        assert_eq!(sparse.get(1), Some(0.0));
    }

    #[test]
    fn test_assign_update_element() {
        let mut sparse = SparseFeature::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        sparse.assign(2, 3.0);
        assert_eq!(sparse.get(2), Some(3.0));
    }

    #[test]
    fn test_assign_remove_element() {
        let mut sparse = SparseFeature::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        sparse.assign(2, 0.0);
        assert_eq!(sparse.get(2), Some(0.0));
    }

    #[test]
    fn test_index_operator() {
        let sparse = SparseFeature::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        assert_eq!(sparse[0], 1.0);
        assert_eq!(sparse[1], 0.0);
        assert_eq!(sparse[2], 2.0);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let sparse = SparseFeature::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        let _ = sparse[5];
    }

    #[test]
    fn test_display() {
        let sparse = SparseFeature::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let output = format!("{}", sparse);
        assert_eq!(
            output,
            "SparseFeature { length: 5, elements: { 0: 1.00, 2: 2.00 } }"
        );
    }

    #[test]
    fn test_iter() {
        let sparse = SparseFeature::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        let elements: Vec<(usize, f32)> = sparse.iter().collect();
        assert_eq!(elements, vec![(0, 1.0), (2, 2.0), (4, 3.0)]);
    }

    #[test]
    fn test_concat() {
        let sparse1 = SparseFeature::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let sparse2 = SparseFeature::new(vec![1, 3], vec![3.0, 4.0], 4).unwrap();

        let concatenated = sparse1.concat(&sparse2).unwrap();

        assert_eq!(concatenated.len(), 9);
        assert_eq!(concatenated.get(0), Some(1.0));
        assert_eq!(concatenated.get(2), Some(2.0));
        assert_eq!(concatenated.get(6), Some(3.0));
        assert_eq!(concatenated.get(8), Some(4.0));
    }
}
