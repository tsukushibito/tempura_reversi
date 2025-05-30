use plotters::prelude::*;
use std::fs;

pub fn generate_loss_plot(artifact_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut train_losses = Vec::new();
    let mut valid_losses = Vec::new();
    let mut epochs = Vec::new();

    let mut epoch = 1;
    loop {
        let train_loss_file = format!("{}/train/epoch-{}/Loss.log", artifact_dir, epoch);
        let valid_loss_file = format!("{}/valid/epoch-{}/Loss.log", artifact_dir, epoch);

        if !std::path::Path::new(&train_loss_file).exists() {
            break;
        }

        if let Ok(avg_loss) = read_loss_from_file(&train_loss_file) {
            train_losses.push(avg_loss);
            epochs.push(epoch as f32);
        } else {
            break;
        }

        if let Ok(avg_loss) = read_loss_from_file(&valid_loss_file) {
            valid_losses.push(avg_loss);
        } else {
            println!("⚠️  Validation data not found for epoch {}", epoch);
        }

        epoch += 1;
    }

    if epochs.is_empty() {
        println!("⚠️  No loss data found in {}", artifact_dir);
        println!(
            "    Expected format: {}/train/epoch-N/Loss.log",
            artifact_dir
        );
        println!(
            "                  or: {}/valid/epoch-N/Loss.log",
            artifact_dir
        );
        return Ok(());
    }

    let plot_path = format!("{}/loss_plot.png", artifact_dir);
    let root = BitMapBackend::new(&plot_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_epoch = epochs.len() as f32;

    let mut all_losses = train_losses.clone();
    all_losses.extend(&valid_losses);

    if all_losses.is_empty() {
        return Err("No valid loss data found".into());
    }

    let max_loss = all_losses.iter().fold(0.0f32, |acc, &x| acc.max(x));
    let min_loss = all_losses.iter().fold(f32::MAX, |acc, &x| acc.min(x));

    let mut chart = ChartBuilder::on(&root)
        .caption("Training and Validation Loss", ("sans-serif", 40))
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

    if !valid_losses.is_empty() {
        let valid_epochs: Vec<f32> = (1..=valid_losses.len()).map(|i| i as f32).collect();

        chart
            .draw_series(LineSeries::new(
                valid_epochs
                    .iter()
                    .zip(valid_losses.iter())
                    .map(|(&x, &y)| (x, y)),
                &RED,
            ))?
            .label("Validation Loss")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));
    }

    chart.configure_series_labels().draw()?;
    root.present()?;

    println!("📈 Loss plot saved to: {}", plot_path);
    println!("📊 Processed {} epochs", epochs.len());

    if !train_losses.is_empty() {
        let initial_train_loss = train_losses[0];
        let final_train_loss = train_losses[train_losses.len() - 1];
        let train_improvement =
            ((initial_train_loss - final_train_loss) / initial_train_loss * 100.0).abs();

        println!("📉 Training - Initial loss: {:.4}", initial_train_loss);
        println!("📉 Training - Final loss: {:.4}", final_train_loss);
        println!("📉 Training - Improvement: {:.2}%", train_improvement);
    }

    if !valid_losses.is_empty() {
        let initial_valid_loss = valid_losses[0];
        let final_valid_loss = valid_losses[valid_losses.len() - 1];
        let valid_improvement =
            ((initial_valid_loss - final_valid_loss) / initial_valid_loss * 100.0).abs();

        println!("📉 Validation - Initial loss: {:.4}", initial_valid_loss);
        println!("📉 Validation - Final loss: {:.4}", final_valid_loss);
        println!("📉 Validation - Improvement: {:.2}%", valid_improvement);

        let train_final = train_losses[train_losses.len() - 1];
        let valid_final = final_valid_loss;
        let gap = ((valid_final - train_final) / train_final * 100.0).abs();

        if gap > 10.0 {
            println!("⚠️  Potential overfitting detected (gap: {:.2}%)", gap);
        } else {
            println!("✅ Good generalization (gap: {:.2}%)", gap);
        }
    } else {
        println!("ℹ️  No validation data found");
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
