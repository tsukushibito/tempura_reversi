use plotters::prelude::*;
use std::collections::HashMap;

pub fn plot_overall_loss(
    losses: &[f32],
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_loss = losses.iter().cloned().fold(0. / 0., f32::max);
    let mut chart = ChartBuilder::on(&root)
        .caption("Overall Validation Loss", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..losses.len(), 0.0..max_loss)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        losses.iter().enumerate().map(|(i, &loss)| (i, loss)),
        &RED,
    ))?;

    Ok(())
}

pub fn plot_phase_losses(
    phase_losses_data: &[HashMap<usize, f32>],
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_loss = phase_losses_data
        .iter()
        .flat_map(|phase_map| phase_map.values())
        .cloned()
        .fold(0. / 0., f32::max);

    let epochs = phase_losses_data.len();
    let mut chart = ChartBuilder::on(&root)
        .caption("Validation Phase Losses", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..epochs, 0.0..max_loss)?;

    chart.configure_mesh().draw()?;

    // 各 phase ごとに系列を整理して描画
    let mut phase_series: HashMap<usize, Vec<(usize, f32)>> = HashMap::new();
    for (epoch, phase_map) in phase_losses_data.iter().enumerate() {
        for (&phase, &loss) in phase_map.iter() {
            phase_series.entry(phase).or_default().push((epoch, loss));
        }
    }

    let colors = vec![&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];
    for (idx, (phase, series)) in phase_series.into_iter().enumerate() {
        let color = colors[idx % colors.len()];
        chart
            .draw_series(LineSeries::new(series, color))?
            .label(format!("Phase {}", phase))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
