use plotters::prelude::*;
use std::fs;

pub fn generate_loss_plot(artifact_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: epoch ディレクトリの場所を修正
    let mut train_losses = Vec::new();
    let mut epochs = Vec::new();

    let mut epoch = 1;
    loop {
        let epoch_dir = format!("{}/epoch-{}", artifact_dir, epoch);
        let train_loss_file = format!("{}/Loss.log", epoch_dir);

        if !std::path::Path::new(&train_loss_file).exists() {
            break;
        }

        if let Ok(avg_loss) = read_loss_from_file(&train_loss_file) {
            train_losses.push(avg_loss);
            epochs.push(epoch as f32);
        }

        epoch += 1;
    }

    if epochs.is_empty() {
        println!("⚠️  No loss data found in {}", artifact_dir);
        println!("    Expected format: {}/epoch-N/Loss.log", artifact_dir);
        return Ok(());
    }

    let plot_path = format!("{}/loss_plot.png", artifact_dir);
    let root = BitMapBackend::new(&plot_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_epoch = epochs.len() as f32;
    let max_loss = train_losses.iter().fold(0.0f32, |acc, &x| acc.max(x));
    let min_loss = train_losses.iter().fold(f32::MAX, |acc, &x| acc.min(x));

    let mut chart = ChartBuilder::on(&root)
        .caption("Training Loss", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(1f32..max_epoch, min_loss * 0.9..max_loss * 1.1)?;

    chart
        .configure_mesh()
        .x_desc("Epoch")
        .y_desc("Loss")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            epochs
                .iter()
                .zip(train_losses.iter())
                .map(|(&x, &y)| (x, y)),
            &BLUE,
        ))?
        .label("Training Loss")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    chart.configure_series_labels().draw()?;
    root.present()?;

    println!("📈 Loss plot saved to: {}", plot_path);
    println!("📊 Processed {} epochs", epochs.len());

    if !train_losses.is_empty() {
        let initial_loss = train_losses[0];
        let final_loss = train_losses[train_losses.len() - 1];
        let improvement = ((initial_loss - final_loss) / initial_loss * 100.0).abs();

        println!("📉 Initial loss: {:.4}", initial_loss);
        println!("📉 Final loss: {:.4}", final_loss);
        println!("📉 Improvement: {:.2}%", improvement);
    }

    Ok(())
}

fn read_loss_from_file(file_path: &str) -> Result<f32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut total_weighted_loss = 0.0f64;
    let mut total_samples = 0usize;

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            if let (Ok(loss), Ok(count)) = (parts[0].parse::<f64>(), parts[1].parse::<usize>()) {
                total_weighted_loss += loss * count as f64;
                total_samples += count;
            }
        }
    }

    if total_samples > 0 {
        Ok((total_weighted_loss / total_samples as f64) as f32)
    } else {
        Err("No valid loss data found in file".into())
    }
}
