use crate::core::constants::*;
use crate::core::{ChartError, ChartSettings, PersonData};
use crate::rendering::text_renderer::TextRenderer;
/// Generation 1 chart generator - Single person layout
use magick_rust::{DrawingWand, MagickWand, PixelWand};

/// Generator for 1-generation charts (single person)
pub struct Gen1Generator {
    settings: ChartSettings,
}

impl Gen1Generator {
    /// Create a new Gen1 generator
    pub fn new(settings: ChartSettings) -> Self {
        Self { settings }
    }

    /// Generate a 1-generation chart
    pub fn generate(&self, person: &PersonData) -> Result<Vec<u8>, ChartError> {
        let mut wand = MagickWand::new();
        wand.set_size(GEN1_CANVAS_WIDTH, GEN1_CANVAS_HEIGHT)?;
        wand.new_image(
            GEN1_BACKGROUND_WIDTH,
            GEN1_BACKGROUND_HEIGHT,
            &PixelWand::new().set_color(&self.settings.background_color),
        )?;

        // Draw background
        self.draw_background(&mut wand)?;

        // Draw text using active context for metrics
        self.draw_text(&mut wand, person)?;

        wand.get_image_blob("PNG").map_err(ChartError::from)
    }

    fn draw_background(&self, wand: &mut MagickWand) -> Result<(), ChartError> {
        let mut draw = DrawingWand::new();
        draw.set_fill_color(&self.settings.background_color);
        draw.rectangle(
            0.0,
            0.0,
            GEN1_BACKGROUND_WIDTH as f64,
            GEN1_BACKGROUND_HEIGHT as f64,
        );
        wand.draw(&draw);
        Ok(())
    }

    fn draw_text(&self, wand: &mut MagickWand, person: &PersonData) -> Result<(), ChartError> {
        let text_renderer = TextRenderer::new(&self.settings);

        // Get metrics using the active wand context
        if let Some(metrics) = text_renderer.get_name_metrics(wand, &person.full_name)? {
            let mut draw = DrawingWand::new();
            draw.set_font(&self.settings.font_family);
            draw.set_font_size(self.settings.name_font_size);
            draw.set_fill_color(&self.settings.font_color);

            // Center text at IMAGE_CENTER with proper outward offset
            draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);
            draw.annotation(-metrics.width / 2.0, 0.0, &person.full_name);

            // Apply outside stroke if enabled
            if self.settings.use_outside_stroke {
                self.render_stroke_effect(wand, &person.full_name, -metrics.width / 2.0, 0.0)?;
            }

            wand.draw(&draw);
        }

        Ok(())
    }

    fn render_stroke_effect(
        &self,
        wand: &mut MagickWand,
        text: &str,
        x: f64,
        y: f64,
    ) -> Result<(), ChartError> {
        let mut stroke_draw = DrawingWand::new();
        stroke_draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);
        stroke_draw.set_font(&self.settings.font_family);
        stroke_draw.set_font_size(self.settings.name_font_size);
        stroke_draw.set_fill_color(&self.settings.stroke_color);
        stroke_draw.set_stroke_color(&self.settings.stroke_color);
        stroke_draw.set_stroke_width(self.settings.stroke_width);
        stroke_draw.annotation(x, y, text);
        wand.draw(&stroke_draw);
        Ok(())
    }
}
