//! Chart generators for different generations

pub mod specs;
pub mod strategies;
pub mod unified_generator;

pub use specs::{RadialSpecs, SunbeamSpecs};
pub use strategies::{
    Gen1Strategy, Gen2Strategy, GenerationStrategyTrait, RadialStrategy, SunbeamStrategy,
};
pub use unified_generator::UnifiedChartGenerator;
