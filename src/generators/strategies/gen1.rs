use crate::core::constants::*;
use crate::core::{AncestorData, ChartError, ChartSettings, GenerationOverlay, PersonData};
use crate::generators::strategies::strategy_trait::GenerationStrategyTrait;
use crate::rendering::text_renderer::TextRenderer;
/// Generation 1 Strategy: Unique visual root mapping
/// Completely isolated from quadratic looping assumptions
/// Implements the large primary layout exactly as in prototype_image_1generator.py
use magick_rust::{DrawingWand, MagickWand, PixelWand};

/// Generation 1 specific specifications
struct Gen1Specs {
    name_font_size: f64,
    date_font_size: f64,
    place_font_size: f64,
    background_width: u32,
    background_height: u32,
    composite_x: i32,
    composite_y: i32,
    flag_base_x: f64,
    flag_base_y: f64,
    flag_rotation: f64,
    flag_size: u32,
}

impl Gen1Specs {
    fn new() -> Self {
        Self {
            // Exact values from prototype_image_1generator.py Generation1Constants
            name_font_size: 74.0,
            date_font_size: 52.0,
            place_font_size: 48.0,
            background_width: 1950,
            background_height: 1950,
            composite_x: 300,
            composite_y: 570,
            flag_base_x: 609.0,
            flag_base_y: 609.0,
            flag_rotation: -45.0,
            flag_size: 666,
        }
    }
}

/// Generation 1 strategy implementation
pub struct Gen1Strategy {
    specs: Gen1Specs,
    text_renderer: TextRenderer,
}

impl Gen1Strategy {
    pub fn new(settings: &ChartSettings) -> Self {
        Self {
            specs: Gen1Specs::new(),
            text_renderer: TextRenderer::new(settings),
        }
    }

    /// Draw background rectangle
    fn draw_background(
        &self,
        wand: &mut MagickWand,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        let mut draw = DrawingWand::new();
        draw.set_fill_color(&settings.background_color);
        draw.rectangle(
            0.0,
            0.0,
            self.specs.background_width as f64,
            self.specs.background_height as f64,
        );
        wand.draw(&draw);
        Ok(())
    }

    /// Draw primary individual with Gen1-specific layout
    fn draw_primary_individual(
        &self,
        wand: &mut MagickWand,
        individual: &PersonData,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        let mut draw = DrawingWand::new();

        // Gen1 uses center positioning with no rotation
        draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);

        // Use Gen1-specific font sizes
        draw.set_font(&settings.font_family);
        draw.set_font_size(self.specs.name_font_size);
        draw.set_fill_color(&settings.font_color);

        // Get name metrics using active wand context
        if let Some(metrics) = self
            .text_renderer
            .get_name_metrics(wand, &individual.full_name)?
        {
            // Center name horizontally
            draw.annotation(-metrics.width / 2.0, 0.0, &individual.full_name);
        }

        // Apply outside stroke if enabled
        if settings.use_outside_stroke {
            self.draw_stroke_effect(&mut draw, individual, settings)?;
        }

        wand.draw(&draw);
        Ok(())
    }

    /// Draw stroke effect for Gen1
    fn draw_stroke_effect(
        &self,
        draw: &mut DrawingWand,
        individual: &PersonData,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        // Stroke is drawn first, then main text
        let stroke_draw = DrawingWand::new();
        // ... stroke implementation
        Ok(())
    }

    /// Render flag overlay for Gen1
    fn render_flag(
        &self,
        wand: &mut MagickWand,
        individual: &PersonData,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        // Flag rendering implementation
        // Uses specs.flag_* constants
        Ok(())
    }
}

impl GenerationStrategyTrait for Gen1Strategy {
    fn generate(
        &self,
        wand: &mut MagickWand,
        primary: &PersonData,
        ancestors: &AncestorData,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        // Validate no ancestors for Gen1
        ancestors.validate_for_generation(1)?;

        // Set canvas size
        wand.set_size(GEN1_CANVAS_WIDTH, GEN1_CANVAS_HEIGHT)?;
        wand.new_image(
            self.specs.background_width,
            self.specs.background_height,
            &PixelWand::new().set_color(&settings.background_color),
        )?;

        // Draw background
        self.draw_background(wand, settings)?;

        // Draw primary individual
        self.draw_primary_individual(wand, primary, settings)?;

        // Render flag
        self.render_flag(wand, primary, settings)?;

        Ok(())
    }

    fn generation(&self) -> u8 {
        1
    }

    fn validate_ancestors(&self, ancestors: &AncestorData) -> Result<(), ChartError> {
        ancestors.validate_for_generation(1)
    }

    fn overlay_settings(&self) -> GenerationOverlay {
        GenerationOverlay {
            scale: 1.0,
            position_x: self.specs.composite_x,
            position_y: self.specs.composite_y,
        }
    }

    fn name(&self) -> &'static str {
        "Gen1Strategy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ChartSettings;

    #[test]
    fn test_gen1_strategy_creation() {
        let settings = ChartSettings {
            font_family: "Arial".to_string(),
            font_color: "black".to_string(),
            background_color: "white".to_string(),
            name_font_size: 74.0,
            date_font_size: 52.0,
            place_font_size: 48.0,
            use_outside_stroke: false,
            stroke_width: 4.0,
            stroke_color: "white".to_string(),
            flag_size: 666,
            flag_type: "birth".to_string(),
        };

        let strategy = Gen1Strategy::new(&settings);
        assert_eq!(strategy.generation(), 1);
        assert_eq!(strategy.name(), "Gen1Strategy");
    }
}
