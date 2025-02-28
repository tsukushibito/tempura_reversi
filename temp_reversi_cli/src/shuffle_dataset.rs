use temp_reversi_ai::learning::GameDataset;

pub fn shuffle_dataset(src_base_path: &str, dst_base_path: &str) -> std::io::Result<()> {
    println!(
        "Shuffling dataset... (src: {}, dst: {})",
        src_base_path, dst_base_path
    );
    let mut datasets = GameDataset::load_auto(src_base_path)?;
    datasets.shuffle();
    datasets.save_auto(dst_base_path)
}
