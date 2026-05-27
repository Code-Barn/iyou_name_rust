use crate::core::constants::*;
use crate::core::{AncestorData, ChartError, ChartSettings, GenerationOverlay, PersonData};
use crate::generators::strategies::strategy_trait::GenerationStrategyTrait;
use crate::rendering::text_renderer::TextRenderer;
/// Generation 2 Strategy: Explicit dual-person spatial layout
/// Hardcoded angular rules for Father (0°) and Mother (180°)
/// Implements the exact layout from prototype_image_2generator.py
use magick_rust::{CompositeOperator, DrawingWand, FilterTypes, MagickWand, PixelWand};

/// Generation 2 specific specifications
struct Gen2Specs {
    parent_name_font_size: f64,
    parent_date_font_size: f64,
    parent_place_font_size: f64,
    parent_first_name_base_y: f64,
    parent_middle_name_base_x: f64,
    parent_middle_name_base_y: f64,
    parent_middle_name_rotation: f64,
    parent_last_name_base_x: f64,
    parent_last_name_base_y: f64,
    overlay_scale: f64,
    composite_x: i32,
    composite_y: i32,
}

impl Gen2Specs {
    fn new() -> Self {
        Self {
            // Exact values from prototype_image_2generator.py Generation2Constants
            parent_name_font_size: 44.0,
            parent_date_font_size: 28.0,
            parent_place_font_size: 24.0,
            parent_first_name_base_y: 1759.0,
            parent_middle_name_base_x: 1625.0,
            parent_middle_name_base_y: 1625.0,
            parent_middle_name_rotation: -45.0,
            parent_last_name_base_x: 1759.0,
            parent_last_name_base_y: 975.0,
            overlay_scale: 0.50,
            composite_x: 300,
            composite_y: 570,
        }
    }

    /// Get outward offset for parent positioning
    pub fn get_parent_offset(&self, rotation: f64) -> (f64, f64) {
        match rotation {
            0.0 => (0.0, self.parent_first_name_base_y - IMAGE_CENTER_Y),
            180.0 => (0.0, IMAGE_CENTER_Y - self.parent_first_name_base_y),
            _ => (0.0, 0.0),
        }
    }
}

/// Generation 2 strategy implementation
pub struct Gen2Strategy {
    specs: Gen2Specs,
    text_renderer: TextRenderer,
}

impl Gen2Strategy {
    pub fn new(settings: &ChartSettings) -> Self {
        Self {
            specs: Gen2Specs::new(),
            text_renderer: TextRenderer::new(settings),
        }
    }

    /// Draw parent with Gen2-specific angular adjustments
    fn draw_parent(
        &self,
        wand: &mut MagickWand,
        individual: &PersonData,
        rotation: f64,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        let mut draw = DrawingWand::new();

        // Apply Gen2-specific transformations: translate to center, then rotate
        draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);
        draw.rotate(rotation);

        // Use Gen2-specific font sizes
        draw.set_font(&settings.font_family);
        draw.set_font_size(self.specs.parent_name_font_size);
        draw.set_fill_color(&settings.font_color);

        // Gen2-specific outward offsets from Python prototype
        let (name_x, name_y) = self.specs.get_parent_offset(rotation);

        // Draw name with Gen2-specific positioning
        draw.annotation(name_x, name_y, &individual.full_name);

        // Apply outside stroke if enabled
        if settings.use_outside_stroke {
            self.draw_stroke_effect(&mut draw, individual, rotation, settings)?;
        }

        wand.draw(&draw);
        Ok(())
    }

    /// Draw stroke effect for Gen2
    fn draw_stroke_effect(
        &self,
        draw: &mut DrawingWand,
        individual: &PersonData,
        rotation: f64,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        let mut stroke_draw = DrawingWand::new();
        stroke_draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);
        stroke_draw.rotate(rotation);

        stroke_draw.set_font(&settings.font_family);
        stroke_draw.set_font_size(self.specs.parent_name_font_size);
        stroke_draw.set_fill_color(&settings.stroke_color);
        stroke_draw.set_stroke_color(&settings.stroke_color);
        stroke_draw.set_stroke_width(settings.stroke_width);

        let (name_x, name_y) = self.specs.get_parent_offset(rotation);
        stroke_draw.annotation(name_x, name_y, &individual.full_name);

        wand.draw(&stroke_draw);
        Ok(())
    }

    /// Composite Gen1 overlay in center
    fn composite_overlay(
        &self,
        wand: &mut MagickWand,
        overlay_wand: &MagickWand,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        let mut overlay_copy = overlay_wand.clone();

        // Scale the overlay
        let scaled_width = (1950 as f64 * self.specs.overlay_scale) as usize;
        let scaled_height = (1950 as f64 * self.specs.overlay_scale) as usize;
        overlay_copy.resize(scaled_width, scaled_height, FilterTypes::LanczosFilter)?;

        // Position in center
        let pos_x = (1950 - scaled_width) / 2;
        let pos_y = (1950 - scaled_height) / 2;

        wand.composite(
            &overlay_copy,
            CompositeOperator::OverCompositeOp,
            pos_x as i32,
            pos_y as i32,
        )?;

        Ok(())
    }
}

impl GenerationStrategyTrait for Gen2Strategy {
    fn generate(
        &self,
        wand: &mut MagickWand,
        primary: &PersonData,
        ancestors: &AncestorData,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        // Validate ancestors for Gen2
        ancestors.validate_for_generation(2)?;

        // Set canvas size
        wand.set_size(CANVAS_WIDTH, CANVAS_HEIGHT)?;
        wand.new_image(
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
            &PixelWand::new().set_color("white"),
        )?;

        // Generate Gen1 overlay first
        let gen1_strategy = super::gen1::Gen1Strategy::new(settings);
        let mut overlay_wand = MagickWand::new();
        overlay_wand.set_size(1950, 1950)?;
        overlay_wand.new_image(1950, 1950, &PixelWand::new().set_color("transparent"))?;

        gen1_strategy.generate(&mut overlay_wand, primary, &AncestorData::empty(), settings)?;

        // Draw father at 0° with Gen2-specific angular adjustments
        if let Some(father) = ancestors.get_father() {
            self.draw_parent(wand, father, 0.0, settings)?;
        }

        // Draw mother at 180° with Gen2-specific angular adjustments
        if let Some(mother) = ancestors.get_mother() {
            self.draw_parent(wand, mother, 180.0, settings)?;
        }

        // Composite Gen1 overlay in center
        self.composite_overlay(wand, &overlay_wand, settings)?;

        Ok(())
    }

    fn generation(&self) -> u8 {
        2
    }

    fn validate_ancestors(&self, ancestors: &AncestorData) -> Result<(), ChartError> {
        ancestors.validate_for_generation(2)
    }

    fn overlay_settings(&self) -> GenerationOverlay {
        GenerationOverlay {
            scale: self.specs.overlay_scale,
            position_x: self.specs.composite_x,
            position_y: self.specs.composite_y,
        }
    }

    fn name(&self) -> &'static str {
        "Gen2Strategy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ChartSettings;

    #[test]
    fn test_gen2_strategy_creation() {
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

        let strategy = Gen2Strategy::new(&settings);
        assert_eq!(strategy.generation(), 2);
        assert_eq!(strategy.name(), "Gen2Strategy");
    }

    #[test]
    fn test_gen2_parent_offsets() {
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

        let strategy = Gen2Strategy::new(&settings);
        let specs = Gen2Specs::new();

        // Test 0° rotation (father)
        let (x, y) = specs.get_parent_offset(0.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 1759.0 - 975.0);

        // Test 180° rotation (mother)
        let (x, y) = specs.get_parent_offset(180.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 975.0 - 1759.0);
    }
}
