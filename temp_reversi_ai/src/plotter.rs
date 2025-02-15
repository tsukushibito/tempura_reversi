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
    // Compute global maximum loss from all phases
    let max_loss = phase_losses_data
        .iter()
        .flat_map(|phase_map| phase_map.values())
        .cloned()
        .fold(0. / 0., f32::max);
    let epochs = phase_losses_data.len();

    // Organize phase series from the input data
    let mut phase_series: HashMap<usize, Vec<(usize, f32)>> = HashMap::new();
    for (epoch, phase_map) in phase_losses_data.iter().enumerate() {
        for (&phase, &loss) in phase_map.iter() {
            phase_series.entry(phase).or_default().push((epoch, loss));
        }
    }
    // Sort phases in ascending order
    let mut sorted_phases: Vec<_> = phase_series.keys().cloned().collect();
    sorted_phases.sort();

    let colors = vec![&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];

    // Group phases into chunks of 10 and generate one chart per group.
    for (group_idx, phase_chunk) in sorted_phases.chunks(10).enumerate() {
        // Construct an output filename for each group graph.
        let group_file = format!(
            "{}_group{}.png",
            file_path.trim_end_matches(".png"),
            group_idx
        );
        let root = BitMapBackend::new(&group_file, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let caption = format!(
            "Validation Phase Losses (Phases {}-{})",
            phase_chunk.first().unwrap(),
            phase_chunk.last().unwrap()
        );
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..epochs, 0.0..max_loss)?;

        chart.configure_mesh().draw()?;

        for (i, phase) in phase_chunk.iter().enumerate() {
            if let Some(series) = phase_series.get(phase) {
                let color = colors[i % colors.len()];
                chart
                    .draw_series(LineSeries::new(series.clone(), color))?
                    .label(format!("Phase {}", phase))
                    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
            }
        }

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;
    }

    Ok(())
}
