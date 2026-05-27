use crate::core::constants::*;
use crate::core::{AncestorData, ChartError, ChartSettings, GenerationOverlay, PersonData};
use crate::generators::specs::RadialSpecs;
use crate::generators::strategies::strategy_trait::GenerationStrategyTrait;
use crate::rendering::text_renderer::TextRenderer;
/// Radial Strategy: Generations 3-5 with quadrant tracking
/// Configuration-driven specs with native matrix transformations
/// Implements precise layout from prototype_image_3generator.py through prototype_image_5generator.py
use magick_rust::{CompositeOperator, DrawingWand, FilterType, MagickWand, PixelWand};

/// Radial strategy for Generations 3-5
/// Uses configuration-driven specifications with native transformations
pub struct RadialStrategy {
    generation: u8,
    specs: RadialSpecs,
    text_renderer: TextRenderer,
}

impl RadialStrategy {
    /// Create new radial strategy for specific generation (3-5)
    pub fn new(generation: u8, settings: &ChartSettings) -> Result<Self, ChartError> {
        if generation >= 3 && generation <= 5 {
            Ok(Self {
                generation,
                specs: RadialSpecs::new(),
                text_renderer: TextRenderer::new(settings),
            })
        } else {
            Err(ChartError::InvalidSettings(format!(
                "RadialStrategy only handles generations 3-5, got {}",
                generation
            )))
        }
    }

    /// Draw individual at radial position using native transformations
    fn draw_individual_at_position(
        &self,
        wand: &mut MagickWand,
        individual: &PersonData,
        position: &crate::generators::specs::radial_specs::RadialPositionSpec,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        let mut draw = DrawingWand::new();

        // Step 1: Translate to symmetrical center point (975.0, 975.0)
        draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);

        // Step 2: Apply quadrant or segment angle rotation
        draw.rotate(position.rotation);

        // Step 3: Set font properties using position-specific sizes
        draw.set_font(&settings.font_family);
        draw.set_font_size(position.font_sizes.name);
        draw.set_fill_color(&settings.font_color);

        // Step 4: Get font metrics using active canvas context
        let name_metrics = self.text_renderer.get_font_metrics(
            wand,
            &individual.full_name,
            position.font_sizes.name,
        )?;

        // Step 5: Print text layers relative to transformed coordinate plane
        // Push characters into target outer ring slots using outward base offsets
        draw.annotation(
            position.name_position.base_x,
            position.name_position.base_y,
            &individual.full_name,
        );

        // Apply outside stroke if enabled
        if settings.use_outside_stroke {
            self.draw_stroke_effect(wand, &mut draw, individual, position, settings)?;
        }

        wand.draw(&draw);
        Ok(())
    }

    /// Draw stroke effect for radial positions
    fn draw_stroke_effect(
        &self,
        wand: &mut MagickWand,
        draw: &mut DrawingWand,
        individual: &PersonData,
        position: &crate::generators::specs::radial_specs::RadialPositionSpec,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        let mut stroke_draw = DrawingWand::new();

        // Apply same transformations as main text
        stroke_draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);
        stroke_draw.rotate(position.rotation);

        // Set stroke properties
        stroke_draw.set_font(&settings.font_family);
        stroke_draw.set_font_size(position.font_sizes.name);
        stroke_draw.set_fill_color(&settings.stroke_color);
        stroke_draw.set_stroke_color(&settings.stroke_color);
        stroke_draw.set_stroke_width(settings.stroke_width);

        // Draw stroke at same position
        stroke_draw.annotation(
            position.name_position.base_x,
            position.name_position.base_y,
            &individual.full_name,
        );

        wand.draw(&stroke_draw);
        Ok(())
    }

    /// Create previous generation strategy for recursive composition
    fn create_previous_strategy(
        &self,
        generation: u8,
        settings: &ChartSettings,
    ) -> Box<dyn GenerationStrategyTrait> {
        match generation {
            1 => Box::new(super::gen1::Gen1Strategy::new(settings)),
            2 => Box::new(super::gen2::Gen2Strategy::new(settings)),
            3 | 4 | 5 => Box::new(Self::new(generation, settings).unwrap()),
            _ => panic!("Invalid previous generation"),
        }
    }

    /// Composite overlay with generation-specific settings
    fn composite_overlay(
        &self,
        wand: &mut MagickWand,
        overlay_data: &[u8],
        overlay_settings: &GenerationOverlay,
    ) -> Result<(), ChartError> {
        let mut overlay_wand = MagickWand::new();
        overlay_wand.read_image_blob(overlay_data)?;

        // Scale the overlay using generation-specific scale factor
        let scaled_width = (1950 as f64 * overlay_settings.scale) as usize;
        let scaled_height = (1950 as f64 * overlay_settings.scale) as usize;
        overlay_wand.resize(scaled_width, scaled_height, FilterType::LanczosFilter)?;

        // Position in center
        let pos_x = (1950 - scaled_width) / 2;
        let pos_y = (1950 - scaled_height) / 2;

        wand.composite(
            &overlay_wand,
            CompositeOperator::OverCompositeOp,
            pos_x as i32,
            pos_y as i32,
        )?;

        Ok(())
    }
}

impl GenerationStrategyTrait for RadialStrategy {
    fn generate(
        &self,
        wand: &mut MagickWand,
        primary: &PersonData,
        ancestors: &AncestorData,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        // Validate ancestors for this generation
        ancestors.validate_for_generation(self.generation)?;

        // Set canvas size
        wand.set_size(1950, 1950)?;
        wand.new_image(1950, 1950, &PixelWand::new().set_color("white"))?;

        // Get positions for this generation
        let positions = self.specs.get_positions(self.generation);

        // Draw primary individual at center (position 0)
        self.draw_individual_at_position(wand, primary, &positions[0], settings)?;

        // Draw all ancestor positions
        for position in &positions[1..] {
            // Skip position 0 (primary)
            if let Some(individual) = ancestors.get_individual(position.id) {
                // Gracefully handle missing data - skip if not present
                self.draw_individual_at_position(wand, individual, position, settings)?;
            }
            // Missing individuals are silently skipped without panicking
        }

        // Generate and composite previous generation overlay
        let prev_gen = self.generation - 1;
        let prev_strategy = self.create_previous_strategy(prev_gen, settings);

        let mut overlay_wand = MagickWand::new();
        overlay_wand.set_size(1950, 1950)?;
        overlay_wand.new_image(1950, 1950, &PixelWand::new().set_color("transparent"))?;

        // Generate previous generation
        prev_strategy.generate(&mut overlay_wand, primary, ancestors, settings)?;

        // Get overlay settings for this generation
        let overlay_settings = self.specs.get_overlay_settings(self.generation);

        // Composite with proper scaling and positioning
        self.composite_overlay(
            wand,
            &overlay_wand.get_image_blob("PNG")?,
            &overlay_settings,
        )?;

        Ok(())
    }

    fn generation(&self) -> u8 {
        self.generation
    }

    fn validate_ancestors(&self, ancestors: &AncestorData) -> Result<(), ChartError> {
        ancestors.validate_for_generation(self.generation)
    }

    fn overlay_settings(&self) -> GenerationOverlay {
        self.specs.get_overlay_settings(self.generation)
    }

    fn name(&self) -> &'static str {
        match self.generation {
            3 => "Gen3RadialStrategy",
            4 => "Gen4RadialStrategy",
            5 => "Gen5RadialStrategy",
            _ => "RadialStrategy",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ChartSettings;

    #[test]
    fn test_radial_strategy_creation() {
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

        // Test Gen3 creation
        let gen3_strategy = RadialStrategy::new(3, &settings).unwrap();
        assert_eq!(gen3_strategy.generation(), 3);
        assert_eq!(gen3_strategy.name(), "Gen3RadialStrategy");

        // Test Gen4 creation
        let gen4_strategy = RadialStrategy::new(4, &settings).unwrap();
        assert_eq!(gen4_strategy.generation(), 4);
        assert_eq!(gen4_strategy.name(), "Gen4RadialStrategy");

        // Test Gen5 creation
        let gen5_strategy = RadialStrategy::new(5, &settings).unwrap();
        assert_eq!(gen5_strategy.generation(), 5);
        assert_eq!(gen5_strategy.name(), "Gen5RadialStrategy");

        // Test invalid generation
        let result = RadialStrategy::new(6, &settings);
        assert!(result.is_err());
    }

    #[test]
    fn test_overlay_settings() {
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

        let gen3_strategy = RadialStrategy::new(3, &settings).unwrap();
        let gen4_strategy = RadialStrategy::new(4, &settings).unwrap();
        let gen5_strategy = RadialStrategy::new(5, &settings).unwrap();

        // Test exact overlay scales from Python prototypes
        assert_eq!(gen3_strategy.overlay_settings().scale, 0.60);
        assert_eq!(gen4_strategy.overlay_settings().scale, 0.7143);
        assert_eq!(gen5_strategy.overlay_settings().scale, 0.75);
    }
}
