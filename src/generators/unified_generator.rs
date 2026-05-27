use crate::core::{AncestorData, ChartError, ChartSettings, PersonData};
use crate::generators::strategies::{
    Gen1Strategy, Gen2Strategy, GenerationStrategyTrait, RadialStrategy, SunbeamStrategy,
};
use magick_rust::MagickWand;
/// Unified API interface that routes to specialized strategies
/// Maintains a single interface for all generations (1-7)
use std::collections::HashMap;

/// Unified chart generator that dispatches to specialized strategies
pub struct UnifiedChartGenerator {
    settings: ChartSettings,
    strategies: HashMap<u8, Box<dyn GenerationStrategyTrait>>,
}

impl UnifiedChartGenerator {
    /// Create a new unified generator with all supported strategies
    pub fn new(settings: ChartSettings) -> Self {
        let mut strategies = HashMap::new();

        // Register all strategies
        strategies.insert(1, Box::new(Gen1Strategy::new(&settings)));
        strategies.insert(2, Box::new(Gen2Strategy::new(&settings)));
        strategies.insert(3, Box::new(RadialStrategy::new(3, &settings).unwrap()));
        strategies.insert(4, Box::new(RadialStrategy::new(4, &settings).unwrap()));
        strategies.insert(5, Box::new(RadialStrategy::new(5, &settings).unwrap()));
        strategies.insert(6, Box::new(SunbeamStrategy::new(6, &settings).unwrap()));
        strategies.insert(7, Box::new(SunbeamStrategy::new(7, &settings).unwrap()));

        Self {
            settings,
            strategies,
        }
    }

    /// Generate chart for specified generation using polymorphic dispatch
    pub fn generate(
        &self,
        generation: u8,
        primary: &PersonData,
        ancestors: &AncestorData,
    ) -> Result<Vec<u8>, ChartError> {
        // Get strategy for requested generation
        let strategy = self.strategies.get(&generation).ok_or_else(|| {
            ChartError::InvalidSettings(format!("Generation {} not supported", generation))
        })?;

        // Create main canvas
        let mut wand = MagickWand::new();
        wand.set_size(1950, 1950)?;
        wand.new_image(
            1950,
            1950,
            &magick_rust::PixelWand::new().set_color("white"),
        )?;

        // Dispatch to specialized strategy
        strategy.generate(&mut wand, primary, ancestors, &self.settings)?;

        // Return PNG bytes
        wand.get_image_blob("PNG").map_err(ChartError::from)
    }

    /// Get strategy for testing/debugging
    pub fn get_strategy(&self, generation: u8) -> Option<&Box<dyn GenerationStrategyTrait>> {
        self.strategies.get(&generation)
    }

    /// Check if a generation is supported
    pub fn is_supported(&self, generation: u8) -> bool {
        self.strategies.contains_key(&generation)
    }

    /// Get list of supported generations
    pub fn supported_generations(&self) -> Vec<u8> {
        self.strategies.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ChartSettings, PersonData};

    #[test]
    fn test_unified_generator_creation() {
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

        let generator = UnifiedChartGenerator::new(settings);
        assert_eq!(generator.supported_generations(), vec![1, 2]);
        assert!(generator.is_supported(1));
        assert!(generator.is_supported(2));
        assert!(!generator.is_supported(3));
    }

    #[test]
    fn test_strategy_access() {
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

        let generator = UnifiedChartGenerator::new(settings);

        // Test strategy access
        let gen1_strategy = generator.get_strategy(1).unwrap();
        assert_eq!(gen1_strategy.name(), "Gen1Strategy");
        assert_eq!(gen1_strategy.generation(), 1);

        let gen2_strategy = generator.get_strategy(2).unwrap();
        assert_eq!(gen2_strategy.name(), "Gen2Strategy");
        assert_eq!(gen2_strategy.generation(), 2);
    }
}
