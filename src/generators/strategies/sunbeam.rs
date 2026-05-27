use crate::core::constants::*;
use crate::core::{AncestorData, ChartError, ChartSettings, GenerationOverlay, PersonData};
use crate::generators::specs::SunbeamSpecs;
use crate::generators::strategies::strategy_trait::GenerationStrategyTrait;
use crate::rendering::{TextAbbreviator, TextRenderer};
/// Sunbeam Strategy: Generations 6-7 with dense concentric layouts
/// Implements strict text constraints and 105px concentric row spacing
/// Uses iterative generation of positions to avoid verbose repetition
use magick_rust::{CompositeOperator, DrawingWand, FilterType, MagickWand, PixelWand};

/// Sunbeam strategy for Generations 6-7
/// Handles dense layouts with aggressive text abbreviation
pub struct SunbeamStrategy {
    generation: u8,
    specs: SunbeamSpecs,
    text_renderer: TextRenderer,
    abbreviator: TextAbbreviator,
}

impl SunbeamStrategy {
    /// Create new sunbeam strategy for specific generation (6-7)
    pub fn new(generation: u8, settings: &ChartSettings) -> Result<Self, ChartError> {
        if generation >= 6 && generation <= 7 {
            let max_name_length = SunbeamSpecs::new().get_max_name_length(generation);

            Ok(Self {
                generation,
                specs: SunbeamSpecs::new(),
                text_renderer: TextRenderer::new(settings),
                abbreviator: TextAbbreviator::with_max_length(max_name_length),
            })
        } else {
            Err(ChartError::InvalidSettings(format!(
                "SunbeamStrategy only handles generations 6-7, got {}",
                generation
            )))
        }
    }

    /// Draw individual at sunbeam position with text abbreviation
    fn draw_individual_at_position(
        &self,
        wand: &mut MagickWand,
        individual: &PersonData,
        position: &crate::generators::specs::sunbeam_specs::SunbeamPositionSpec,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        let mut draw = DrawingWand::new();

        // Step 1: Translate to symmetrical center point (975.0, 975.0)
        draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);

        // Step 2: Apply rotation for sunbeam positioning
        draw.rotate(position.rotation);

        // Step 3: Apply text abbreviation to fit constraints
        let display_name = self
            .abbreviator
            .abbreviate(&individual.full_name, position.max_name_length);

        // Step 4: Set font properties
        draw.set_font(&settings.font_family);
        draw.set_font_size(position.font_sizes.name);
        draw.set_fill_color(&settings.font_color);

        // Step 5: Get font metrics using active canvas context
        let name_metrics =
            self.text_renderer
                .get_font_metrics(wand, &display_name, position.font_sizes.name)?;

        // Step 6: Print abbreviated text at calculated position
        draw.annotation(
            position.name_position.base_x,
            position.name_position.base_y,
            &display_name,
        );

        // Apply outside stroke if enabled
        if settings.use_outside_stroke {
            self.draw_stroke_effect(wand, &mut draw, &display_name, position, settings)?;
        }

        wand.draw(&draw);
        Ok(())
    }

    /// Draw stroke effect for sunbeam positions
    fn draw_stroke_effect(
        &self,
        wand: &mut MagickWand,
        draw: &mut DrawingWand,
        display_name: &str,
        position: &crate::generators::specs::sunbeam_specs::SunbeamPositionSpec,
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
            display_name,
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
            3 | 4 | 5 => {
                Box::new(super::radial::RadialStrategy::new(generation, settings).unwrap())
            }
            6 | 7 => Box::new(Self::new(generation, settings).unwrap()),
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

impl GenerationStrategyTrait for SunbeamStrategy {
    fn generate(
        &self,
        wand: &mut MagickWand,
        primary: &PersonData,
        ancestors: &AncestorData,
        settings: &ChartSettings,
    ) -> Result<(), ChartError> {
        // Validate ancestors for this generation (more flexible for high generations)
        if ancestors.count() < 8 {
            return Err(ChartError::InvalidSettings(format!(
                "Gen{} requires reasonable ancestor data (at least 8 ancestors)",
                self.generation
            )));
        }

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
            if let Some(individual) = ancestors.get_individual(&position.id) {
                // Apply text abbreviation and draw
                self.draw_individual_at_position(wand, individual, position, settings)?;
            }
            // Missing individuals are silently skipped (common in large trees)
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
        // Sunbeam generations are more flexible - just need reasonable data
        if ancestors.count() >= 8 {
            Ok(())
        } else {
            Err(ChartError::InvalidSettings(format!(
                "Gen{} requires at least 8 ancestors",
                self.generation
            )))
        }
    }

    fn overlay_settings(&self) -> GenerationOverlay {
        self.specs.get_overlay_settings(self.generation)
    }

    fn name(&self) -> &'static str {
        match self.generation {
            6 => "Gen6SunbeamStrategy",
            7 => "Gen7SunbeamStrategy",
            _ => "SunbeamStrategy",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ChartSettings;

    #[test]
    fn test_sunbeam_strategy_creation() {
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

        // Test Gen6 creation
        let gen6_strategy = SunbeamStrategy::new(6, &settings).unwrap();
        assert_eq!(gen6_strategy.generation(), 6);
        assert_eq!(gen6_strategy.name(), "Gen6SunbeamStrategy");
        assert_eq!(gen6_strategy.abbreviator.get_max_length(), 15);

        // Test Gen7 creation
        let gen7_strategy = SunbeamStrategy::new(7, &settings).unwrap();
        assert_eq!(gen7_strategy.generation(), 7);
        assert_eq!(gen7_strategy.name(), "Gen7SunbeamStrategy");
        assert_eq!(gen7_strategy.abbreviator.get_max_length(), 12);

        // Test invalid generation
        let result = SunbeamStrategy::new(5, &settings);
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

        let gen6_strategy = SunbeamStrategy::new(6, &settings).unwrap();
        let gen7_strategy = SunbeamStrategy::new(7, &settings).unwrap();

        // Test overlay scales
        assert_eq!(gen6_strategy.overlay_settings().scale, 0.80);
        assert_eq!(gen7_strategy.overlay_settings().scale, 0.85);
    }

    #[test]
    fn test_text_abbreviation() {
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

        let gen6_strategy = SunbeamStrategy::new(6, &settings).unwrap();

        // Test abbreviation with long name
        let long_name = "Alexander Hamilton Junior from New York County";
        let abbreviated = gen6_strategy.abbreviator.abbreviate(long_name, 15);

        assert!(abbreviated.len() <= 15);
        assert!(
            abbreviated.contains("Jr.")
                || abbreviated.contains("NY")
                || abbreviated.contains("Co.")
        );
    }
}
