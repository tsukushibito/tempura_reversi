use core::fmt;
use std::ops::{Add, Div, Index, Mul};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SparseVector {
    indices: Vec<usize>,
    values: Vec<f32>,
    length: usize,
}

#[derive(Debug, Clone)]
pub enum SparseVectorError {
    LengthMismatch,
    IndexOutOfBounds,
    DuplicateIndices,
}

impl fmt::Display for SparseVectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SparseVectorError::LengthMismatch => write!(f, "Sparse vector length mismatch"),
            SparseVectorError::IndexOutOfBounds => write!(f, "Sparse vector index out of bounds"),
            SparseVectorError::DuplicateIndices => {
                write!(f, "Sparse vector contains duplicate indices")
            }
        }
    }
}

impl std::error::Error for SparseVectorError {}

impl SparseVector {
    pub fn new(
        indices: Vec<usize>,
        values: Vec<f32>,
        length: usize,
    ) -> Result<Self, SparseVectorError> {
        if indices.len() != values.len() {
            return Err(SparseVectorError::LengthMismatch);
        }
        if indices.iter().any(|&i| i >= length) {
            return Err(SparseVectorError::IndexOutOfBounds);
        }
        let unique_indices: std::collections::HashSet<_> = indices.iter().collect();
        if unique_indices.len() != indices.len() {
            return Err(SparseVectorError::DuplicateIndices);
        }

        let mut combined: Vec<(usize, f32)> = indices.into_iter().zip(values).collect();
        combined.sort_by_key(|&(i, _)| i);

        let (sorted_indices, sorted_values): (Vec<usize>, Vec<f32>) = combined.into_iter().unzip();

        Ok(SparseVector {
            indices: sorted_indices,
            values: sorted_values,
            length,
        })
    }

    pub fn from(elements: &[(usize, f32)], length: usize) -> Result<Self, SparseVectorError> {
        let indices: Vec<usize> = elements.iter().map(|(i, _)| *i).collect();
        let values: Vec<f32> = elements.iter().map(|(_, v)| *v).collect();
        Self::new(indices, values, length)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn indices(&self) -> &Vec<usize> {
        &self.indices
    }

    pub fn values(&self) -> &Vec<f32> {
        &self.values
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

    pub fn concat(&self, other: &SparseVector) -> Result<SparseVector, SparseVectorError> {
        let new_length = self.length + other.length;

        let mut new_indices = self.indices.clone();
        let mut new_values = self.values.clone();

        new_indices.extend(other.indices.iter().map(|&index| index + self.length));
        new_values.extend(&other.values);

        SparseVector::new(new_indices, new_values, new_length)
    }

    pub fn dot(&self, other: &[f32]) -> Result<f32, SparseVectorError> {
        if other.len() != self.length {
            return Err(SparseVectorError::LengthMismatch);
        }
        let other_values: Vec<f32> = self.indices.iter().map(|&i| other[i]).collect();
        let dot = self
            .values
            .iter()
            .zip(other_values.iter())
            .map(|(v, o)| v * o)
            .sum();

        Ok(dot)
    }
}

impl Index<usize> for SparseVector {
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

impl Add for SparseVector {
    type Output = Self;

    fn add(self, rhs: SparseVector) -> Self::Output {
        assert_eq!(self.length, rhs.length, "Vectors must have the same length");

        let mut indices = Vec::new();
        let mut values = Vec::new();

        let mut self_iter = self.indices.iter().zip(self.values.iter()).peekable();
        let mut rhs_iter = rhs.indices.iter().zip(rhs.values.iter()).peekable();

        while let (Some(&(i1, v1)), Some(&(i2, v2))) = (self_iter.peek(), rhs_iter.peek()) {
            if i1 == i2 {
                let sum = v1 + v2;
                if sum != 0.0 {
                    indices.push(*i1);
                    values.push(sum);
                }
                self_iter.next();
                rhs_iter.next();
            } else if i1 < i2 {
                indices.push(*i1);
                values.push(*v1);
                self_iter.next();
            } else {
                indices.push(*i2);
                values.push(*v2);
                rhs_iter.next();
            }
        }

        for (i, v) in self_iter {
            indices.push(*i);
            values.push(*v);
        }

        for (i, v) in rhs_iter {
            indices.push(*i);
            values.push(*v);
        }

        SparseVector::new(indices, values, self.length).unwrap()
    }
}

impl Mul<f32> for SparseVector {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        let values = self.values.iter().map(|v| v * scalar).collect();
        Self {
            indices: self.indices.clone(),
            values,
            length: self.length,
        }
    }
}

impl Div<f32> for SparseVector {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        if scalar == 0.0 {
            panic!("Division by zero is not allowed!");
        }

        let values = self.values.iter().map(|v| v / scalar).collect();

        Self {
            indices: self.indices.clone(),
            values,
            length: self.length,
        }
    }
}

impl std::fmt::Display for SparseVector {
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

pub struct SparseVectorIter<'a> {
    indices: std::slice::Iter<'a, usize>,
    values: std::slice::Iter<'a, f32>,
}

impl<'a> Iterator for SparseVectorIter<'a> {
    type Item = (usize, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.indices.next(), self.values.next()) {
            (Some(&index), Some(&value)) => Some((index, value)),
            _ => None,
        }
    }
}

impl SparseVector {
    pub fn iter(&self) -> SparseVectorIter<'_> {
        SparseVectorIter {
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
        let sparse = SparseVector::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        assert_eq!(sparse.len(), 5);
        assert_eq!(sparse.get(0), Some(1.0));
        assert_eq!(sparse.get(1), Some(0.0));
        assert_eq!(sparse.get(2), Some(2.0));
    }

    #[test]
    fn test_new_length_mismatch() {
        let result = SparseVector::new(vec![0, 2], vec![1.0], 5);
        assert!(matches!(result, Err(SparseVectorError::LengthMismatch)));
    }

    #[test]
    fn test_new_index_out_of_bounds() {
        let result = SparseVector::new(vec![0, 5], vec![1.0, 2.0], 5);
        assert!(matches!(result, Err(SparseVectorError::IndexOutOfBounds)));
    }

    #[test]
    fn test_new_duplicate_indices() {
        let result = SparseVector::new(vec![0, 0], vec![1.0, 2.0], 5);
        assert!(matches!(result, Err(SparseVectorError::DuplicateIndices)));
    }

    #[test]
    fn test_assign_add_element() {
        let mut sparse = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        sparse.assign(3, 4.0);
        assert_eq!(sparse.get(3), Some(4.0));
        assert_eq!(sparse.get(1), Some(0.0));
    }

    #[test]
    fn test_assign_update_element() {
        let mut sparse = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        sparse.assign(2, 3.0);
        assert_eq!(sparse.get(2), Some(3.0));
    }

    #[test]
    fn test_assign_remove_element() {
        let mut sparse = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        sparse.assign(2, 0.0);
        assert_eq!(sparse.get(2), Some(0.0));
    }

    #[test]
    fn test_index_operator() {
        let sparse = SparseVector::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        assert_eq!(sparse[0], 1.0);
        assert_eq!(sparse[1], 0.0);
        assert_eq!(sparse[2], 2.0);
    }

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let sparse = SparseVector::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        let _ = sparse[5];
    }

    #[test]
    fn test_display() {
        let sparse = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let output = format!("{}", sparse);
        assert_eq!(
            output,
            "SparseFeature { length: 5, elements: { 0: 1.00, 2: 2.00 } }"
        );
    }

    #[test]
    fn test_iter() {
        let sparse = SparseVector::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        let elements: Vec<(usize, f32)> = sparse.iter().collect();
        assert_eq!(elements, vec![(0, 1.0), (2, 2.0), (4, 3.0)]);
    }

    #[test]
    fn test_concat() {
        let sparse1 = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let sparse2 = SparseVector::new(vec![1, 3], vec![3.0, 4.0], 4).unwrap();

        let concatenated = sparse1.concat(&sparse2).unwrap();

        assert_eq!(concatenated.len(), 9);
        assert_eq!(concatenated.get(0), Some(1.0));
        assert_eq!(concatenated.get(2), Some(2.0));
        assert_eq!(concatenated.get(6), Some(3.0));
        assert_eq!(concatenated.get(8), Some(4.0));
    }

    #[test]
    fn test_mul_scalar() {
        let sparse = SparseVector::new(vec![0, 2, 4], vec![1.0, 2.0, 3.0], 5).unwrap();
        let result = sparse.clone() * 2.0;
        assert_eq!(result.get(0), Some(2.0));
        assert_eq!(result.get(2), Some(4.0));
        assert_eq!(result.get(4), Some(6.0));
    }

    #[test]
    fn test_div_scalar() {
        let sparse = SparseVector::new(vec![0, 2, 4], vec![2.0, 4.0, 6.0], 5).unwrap();
        let result = sparse.clone() / 2.0;
        assert_eq!(result.get(0), Some(1.0));
        assert_eq!(result.get(2), Some(2.0));
        assert_eq!(result.get(4), Some(3.0));
    }

    #[test]
    #[should_panic]
    fn test_div_by_zero() {
        let sparse = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let _ = sparse / 0.0;
    }

    #[test]
    fn test_add_sparse_vectors() {
        let sparse1 = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let sparse2 = SparseVector::new(vec![1, 2], vec![3.0, 4.0], 5).unwrap();

        let result = sparse1 + sparse2;
        assert_eq!(result.get(0), Some(1.0));
        assert_eq!(result.get(1), Some(3.0));
        assert_eq!(result.get(2), Some(6.0));
    }

    #[test]
    fn test_dot_product() {
        let sparse = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let dense = vec![1.0, 0.0, 3.0, 0.0, 0.0];

        let result = sparse.dot(&dense).unwrap();
        assert_eq!(result, 7.0); // 1*1 + 2*3 = 7
    }

    #[test]
    fn test_dot_product_length_mismatch() {
        let sparse = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let dense = vec![1.0, 0.0, 3.0]; // Length mismatch

        let result = sparse.dot(&dense);
        assert!(matches!(result, Err(SparseVectorError::LengthMismatch)));
    }

    #[test]
    fn test_empty_sparse_vector() {
        let sparse = SparseVector::default();
        assert!(sparse.is_empty());
        assert_eq!(sparse.len(), 0);
        assert_eq!(sparse.indices().len(), 0);
        assert_eq!(sparse.values().len(), 0);
    }

    #[test]
    fn test_iter_empty_sparse_vector() {
        let sparse = SparseVector::default();
        let elements: Vec<(usize, f32)> = sparse.iter().collect();
        assert!(elements.is_empty());
    }

    #[test]
    fn test_concat_with_empty_sparse_vector() {
        let sparse1 = SparseVector::new(vec![0, 2], vec![1.0, 2.0], 5).unwrap();
        let sparse2 = SparseVector::default();

        let concatenated = sparse1.concat(&sparse2).unwrap();
        assert_eq!(concatenated.len(), 5);
        assert_eq!(concatenated.get(0), Some(1.0));
        assert_eq!(concatenated.get(2), Some(2.0));
    }
}
