pub mod device;
pub mod pipeline;

#[cfg(test)]
mod tests {
    use crate::device::HeadlessContext;
    use crate::pipeline::{LagosPipeline, LttbPoint};
    use egui_plot::{Line, Plot, PlotPoints};
    use std::sync::Arc;

    #[pollster::test]
    async fn test_render_sparkline() {
        let ctx = Arc::new(HeadlessContext::new().await);
        let mut pipeline = LagosPipeline::new(ctx.clone());

        // Generate 10,000 points of a sine wave
        let input_points: Vec<LttbPoint> = (0..10000)
            .map(|i| {
                let x = i as f32;
                let y = (x * 0.01).sin();
                LttbPoint { x, y }
            })
            .collect();

        let width = 1920;
        let height = 1080;
        let output_count = 2000;

        let rgba_data = pipeline.process_and_render(
            &input_points,
            output_count,
            width,
            height,
            |ctx, decimated| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let plot_points: PlotPoints = decimated
                        .iter()
                        .map(|p| [p.x as f64, p.y as f64])
                        .collect();
                    let line = Line::new(plot_points);

                    Plot::new("sparkline")
                        .view_aspect(width as f32 / height as f32)
                        .show(ui, |plot_ui| {
                            plot_ui.line(line);
                        });
                });
            },
        ).await;

        assert_eq!(rgba_data.len(), (width * height * 4) as usize);

        // Save to PNG
        image::RgbaImage::from_raw(width, height, rgba_data)
            .expect("Failed to create image from raw bytes")
            .save("test_render.png")
            .expect("Failed to save test_render.png");

        println!("Test render saved to test_render.png");
    }
}
