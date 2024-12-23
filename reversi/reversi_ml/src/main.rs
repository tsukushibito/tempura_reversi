use burn::{backend::Wgpu, data::dataloader::batcher::Batcher, prelude::Backend, tensor::Tensor};
use data::{ReversiBatcher, ReversiItem};

mod data;
mod model;
mod training;

fn main() {
    println!("Hello, world!");

    let device = burn::backend::wgpu::WgpuDevice::default();
    let batcher = ReversiBatcher::<Wgpu>::new(device);
    let items = [
        ReversiItem {
            feature: vec![0.0, 1.0, 2.0, 3.0],
            value: 0.0,
        },
        ReversiItem {
            feature: vec![4.0, 5.0, 6.0, 7.0],
            value: 1.0,
        },
        ReversiItem {
            feature: vec![8.0, 9.0, 0.0, 1.0],
            value: 2.0,
        },
    ];
    let batch = batcher.batch(items.to_vec());
    println!("{:?}", batch);
}
