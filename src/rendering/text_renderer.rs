use crate::core::{ChartError, ChartSettings};
/// Text rendering utilities with active canvas context support
use magick_rust::{DrawingWand, MagickWand};

/// Font metrics structure
#[derive(Debug, Clone)]
pub struct TextMetrics {
    pub width: f64,
    pub height: f64,
    pub ascent: f64,
    pub descent: f64,
}

/// Text renderer that uses active canvas context for accurate metrics
pub struct TextRenderer {
    settings: ChartSettings,
}

impl TextRenderer {
    /// Create a new text renderer with the given settings
    pub fn new(settings: &ChartSettings) -> Self {
        Self {
            settings: settings.clone(),
        }
    }

    /// Get font metrics using the active canvas context to prevent resolution drift
    pub fn get_font_metrics(
        &self,
        active_canvas: &MagickWand,
        text: &str,
        font_size: f64,
    ) -> Result<TextMetrics, ChartError> {
        let mut draw = DrawingWand::new();

        // Use the active canvas's resolution and other properties
        let resolution = active_canvas.get_resolution()?;
        draw.set_resolution(resolution);
        draw.set_font(&self.settings.font_family);
        draw.set_font_size(font_size);

        let metrics = draw.get_font_metrics(active_canvas, text, false)?;
        Ok(TextMetrics {
            width: metrics.text_width,
            height: metrics.text_height,
            ascent: metrics.ascent,
            descent: metrics.descent,
        })
    }

    /// Get metrics for a person's name
    pub fn get_name_metrics(
        &self,
        active_canvas: &MagickWand,
        name: &str,
    ) -> Result<Option<TextMetrics>, ChartError> {
        if name.is_empty() {
            return Ok(None);
        }
        let metrics = self.get_font_metrics(active_canvas, name, self.settings.name_font_size)?;
        Ok(Some(metrics))
    }

    /// Render text with optional outside stroke effect
    pub fn render_text_with_stroke(
        &self,
        wand: &mut MagickWand,
        text: &str,
        x: f64,
        y: f64,
        font_size: f64,
        rotation: f64,
    ) -> Result<(), ChartError> {
        // Draw stroke first if enabled
        if self.settings.use_outside_stroke {
            let mut stroke_draw = DrawingWand::new();
            stroke_draw.set_font(&self.settings.font_family);
            stroke_draw.set_font_size(font_size);
            stroke_draw.set_fill_color(&self.settings.stroke_color);
            stroke_draw.set_stroke_color(&self.settings.stroke_color);
            stroke_draw.set_stroke_width(self.settings.stroke_width);

            if rotation != 0.0 {
                stroke_draw.rotate(rotation);
            }
            stroke_draw.translate(x, y);
            stroke_draw.annotation(0.0, 0.0, text);
            wand.draw(&stroke_draw);
        }

        // Draw main text
        let mut main_draw = DrawingWand::new();
        main_draw.set_font(&self.settings.font_family);
        main_draw.set_font_size(font_size);
        main_draw.set_fill_color(&self.settings.font_color);

        if rotation != 0.0 {
            main_draw.rotate(rotation);
        }
        main_draw.translate(x, y);
        main_draw.annotation(0.0, 0.0, text);
        wand.draw(&main_draw);

        Ok(())
    }
}
