use burn::{
    config::Config,
    module::{Module, Param},
    nn::{Embedding, EmbeddingConfig},
    prelude::Backend,
    tensor::{Int, Tensor},
};

#[derive(Config, Debug)]
pub struct SparseLinearConfig {
    pub d_input: usize,
    pub d_output: usize,
    #[config(default = true)]
    pub bias: bool,
}

#[derive(Module, Debug)]
pub struct SparseLinear<B: Backend> {
    pub embedding: Embedding<B>,
    pub bias: Option<Param<Tensor<B, 1>>>,
}

impl SparseLinearConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> SparseLinear<B> {
        let embedding = EmbeddingConfig::new(self.d_input, self.d_output).init(device);

        let bias = if self.bias {
            Some(Param::from_tensor(Tensor::zeros([self.d_output], device)))
        } else {
            None
        };

        SparseLinear { embedding, bias }
    }

    pub fn init_with_weights<B: Backend>(
        &self,
        device: &B::Device,
        embedding_weight: Tensor<B, 2>,
        bias_weight: Option<Tensor<B, 1>>,
    ) -> SparseLinear<B> {
        let mut embedding = EmbeddingConfig::new(self.d_input, self.d_output).init(device);
        embedding.weight = Param::from_tensor(embedding_weight);

        let bias = if self.bias {
            Some(Param::from_tensor(
                bias_weight.unwrap_or_else(|| Tensor::zeros([self.d_output], device)),
            ))
        } else {
            None
        };

        SparseLinear { embedding, bias }
    }
}

impl<B: Backend> SparseLinear<B> {
    pub fn forward(&self, indices: Tensor<B, 2, Int>, values: Tensor<B, 2>) -> Tensor<B, 2> {
        // 1. Lookup embeddings (rows of W^T or columns of W)
        // indices shape: [B, N]
        // embedded shape: [B, N, D_out]
        let embedded = self.embedding.forward(indices);

        // 2. Prepare values for scaling (element-wise multiplication)
        // values shape: [B, N]
        // We need to multiply element-wise with `embedded`, so reshape `values`
        // to allow broadcasting: [B, N] -> [B, N, 1]
        let [batch_size, num_sparse_features] = values.dims();
        // Ensure values are float for multiplication if they aren't already
        let values_float = values.clone(); // Use .float() if input type might not be float
        let reshaped_values = values_float.reshape([batch_size, num_sparse_features, 1]);
        // Alternative using unsqueeze:
        // let weights = values_float.unsqueeze::<3>(); // Check Burn API docs for exact behavior

        // 3. Scale embeddings by feature values
        // embedded: [B, N, D_out], weights: [B, N, 1]
        // scaled_embedded shape: [B, N, D_out]
        let scaled_embedded = embedded.mul(reshaped_values);

        // 4. Sum scaled embeddings along the sparse feature dimension (dimension 1)
        // summed shape: [B, D_out]
        let summed = scaled_embedded.sum_dim(1);
        let summed = summed.squeeze::<2>(1); // Remove the dimension of size 1

        // 5. Add bias if enabled
        if let Some(ref bias_param) = self.bias {
            // Add bias vector (broadcasts along batch dimension)
            // summed: [B, D_out], bias: [D_out]
            let bias = bias_param.val().unsqueeze();
            summed.add(bias)
        } else {
            summed
        }
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module (SparseLinear, Config, etc.)
    const AFFECTED_PRECISION: usize = 3; // Decimal places for float comparisons

    use burn::backend::NdArray;

    /// Helper function to create a SparseLinear module with known weights and bias for testing.
    fn create_module_with_known_weights(
        d_input: usize,
        d_output: usize,
        bias_enabled: bool,
        device: &<NdArray as burn::prelude::Backend>::Device,
    ) -> SparseLinear<NdArray> {
        let config = SparseLinearConfig {
            d_input,
            d_output,
            bias: bias_enabled,
        };

        // Define known weights and bias values
        // Ensure d_input >= 5 and d_output >= 3 for these values
        let embedding_weight_values = [
            [0.0, 0.0, 0.0], // Index 0 (often used for padding)
            [1.0, 2.0, 3.0], // Index 1
            [4.0, 5.0, 6.0], // Index 2
            [7.0, 8.0, 9.0], // Index 3
            [0.1, 0.2, 0.3], // Index 4
        ];
        let bias_values = [0.5, -0.5, 0.0]; // Bias for d_output=3

        // Create tensors on the specified device
        let embedding_weight_tensor = Tensor::<_, 2>::from_floats(embedding_weight_values, device);
        let bias_tensor = Tensor::<_, 1>::from_floats(bias_values, device);

        // Initialize the SparseLinear module with known weights and bias
        config.init_with_weights(device, embedding_weight_tensor, Some(bias_tensor))
    }

    #[test]
    fn test_sparse_linear_forward_with_bias() {
        let device = burn::backend::ndarray::NdArrayDevice::Cpu;
        let d_input = 5;
        let d_output = 3;
        let module: SparseLinear<burn::backend::NdArray> =
            create_module_with_known_weights(d_input, d_output, true, &device);

        // Input Data (Batch Size = 2, Num Sparse Features = 2)
        // Use i32 for indices, compatible with TestBackend's Int default
        let indices = Tensor::from_ints([[1, 3], [2, 0]], &device);
        let values = Tensor::from_floats([[2.0, 1.0], [0.5, 10.0]], &device);

        // Expected Output Calculation (derived manually in previous thoughts)
        // Batch 1: (E[1]*2.0 + E[3]*1.0) + Bias = ([2.0, 4.0, 6.0] + [7.0, 8.0, 9.0]) + [0.5, -0.5, 0.0] = [9.5, 11.5, 15.0]
        // Batch 2: (E[2]*0.5 + E[0]*10.0) + Bias = ([2.0, 2.5, 3.0] + [0.0, 0.0, 0.0]) + [0.5, -0.5, 0.0] = [2.5, 2.0, 3.0]
        let expected_output =
            Tensor::<NdArray, 2>::from_floats([[9.5, 11.5, 15.0], [2.5, 2.0, 3.0]], &device);

        // Perform forward pass
        let output = module.forward(indices, values);

        // Compare results using assert_approx_eq for floating point numbers
        output
            .into_data()
            .assert_approx_eq(&expected_output.into_data(), AFFECTED_PRECISION);
    }

    #[test]
    fn test_sparse_linear_forward_without_bias() {
        let device = burn::backend::ndarray::NdArrayDevice::Cpu;
        let d_input = 5;
        let d_output = 3;
        // Create module with bias disabled
        let module: SparseLinear<NdArray> =
            create_module_with_known_weights(d_input, d_output, false, &device); // bias_enabled = false

        // Input Data (same as previous test)
        let indices = Tensor::from_ints([[1, 3], [2, 0]], &device);
        let values = Tensor::from_floats([[2.0, 1.0], [0.5, 10.0]], &device);

        // Expected Output Calculation (without bias)
        // Batch 1: (E[1]*2.0 + E[3]*1.0) = [9.0, 12.0, 15.0]
        // Batch 2: (E[2]*0.5 + E[0]*10.0) = [2.0, 2.5, 3.0]
        let expected_output =
            Tensor::<NdArray, 2>::from_floats([[9.0, 12.0, 15.0], [2.0, 2.5, 3.0]], &device);

        // Perform forward pass
        let output = module.forward(indices, values);

        // Compare results
        output
            .into_data()
            .assert_approx_eq(&expected_output.into_data(), AFFECTED_PRECISION);
    }

    #[test]
    fn test_sparse_linear_forward_zero_value() {
        let device = burn::backend::ndarray::NdArrayDevice::Cpu;
        let d_input = 5;
        let d_output = 3;
        // Create module with bias enabled
        let module: SparseLinear<NdArray> =
            create_module_with_known_weights(d_input, d_output, true, &device);

        // Input Data: Feature at index 3 has value 0.0 in the first batch item
        let indices = Tensor::from_ints([[1, 3], [2, 0]], &device);
        let values = Tensor::from_floats([[2.0, 0.0], [0.5, 10.0]], &device); // Value for index 3 is 0.0

        // Expected Output Calculation
        // Batch 1: (E[1]*2.0 + E[3]*0.0) + Bias = ([2.0, 4.0, 6.0] + [0.0, 0.0, 0.0]) + [0.5, -0.5, 0.0] = [2.5, 3.5, 6.0]
        // Batch 2: (E[2]*0.5 + E[0]*10.0) + Bias = ([2.0, 2.5, 3.0] + [0.0, 0.0, 0.0]) + [0.5, -0.5, 0.0] = [2.5, 2.0, 3.0]
        let expected_output =
            Tensor::<NdArray, 2>::from_floats([[2.5, 3.5, 6.0], [2.5, 2.0, 3.0]], &device);

        // Perform forward pass
        let output = module.forward(indices, values);

        // Compare results
        output
            .into_data()
            .assert_approx_eq(&expected_output.into_data(), AFFECTED_PRECISION);
    }

    #[test]
    fn test_sparse_linear_forward_padding_index() {
        let device = burn::backend::ndarray::NdArrayDevice::Cpu;
        let d_input = 5;
        let d_output = 3;
        // Create module with bias enabled
        let module: SparseLinear<NdArray> =
            create_module_with_known_weights(d_input, d_output, true, &device);

        // Input Data: Second batch item uses padding index 0 with a non-zero value.
        // Embedding vector for index 0 is [0, 0, 0], so it should contribute zero regardless of value.
        let indices = Tensor::from_ints([[1, 3], [2, 0]], &device);
        let values = Tensor::from_floats([[2.0, 1.0], [0.5, 10.0]], &device); // Same as first test

        // Expected Output Calculation (should be same as first test case)
        // Batch 1: [9.5, 11.5, 15.0]
        // Batch 2: (E[2]*0.5 + E[0]*10.0) + Bias = ([2.0, 2.5, 3.0] + [0.0, 0.0, 0.0]) + [0.5, -0.5, 0.0] = [2.5, 2.0, 3.0]
        let expected_output =
            Tensor::<NdArray, 2>::from_floats([[9.5, 11.5, 15.0], [2.5, 2.0, 3.0]], &device);

        // Perform forward pass
        let output = module.forward(indices, values);

        // Compare results
        output
            .into_data()
            .assert_approx_eq(&expected_output.into_data(), AFFECTED_PRECISION);
    }
}
