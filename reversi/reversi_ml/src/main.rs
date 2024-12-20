use burn::{backend::Wgpu, tensor::Tensor};

mod data;
mod model;

fn main() {
    println!("Hello, world!");

    let device = burn::backend::wgpu::WgpuDevice::default();
    let tensor = Tensor::<Wgpu<f32, i32>, 1>::from_floats([0.0, 0.0], &device).reshape([1, 2]);
    println!("{:?}", tensor.shape());
}
