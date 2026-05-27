use crate::core::constants::*;
use crate::core::{ChartError, ChartSettings, GenerationOverlay, PersonData};
use crate::generators::gen1::Gen1Generator;
/// Generation 2 chart generator - Parent layout with 180° recursive composition
use magick_rust::{CompositeOperator, DrawingWand, FilterType, MagickWand, PixelWand};

/// Generator for 2-generation charts (primary + parents)
pub struct Gen2Generator {
    settings: ChartSettings,
    overlay_settings: GenerationOverlay,
}

impl Gen2Generator {
    /// Create a new Gen2 generator
    pub fn new(settings: ChartSettings, overlay_settings: GenerationOverlay) -> Self {
        Self {
            settings,
            overlay_settings,
        }
    }

    /// Generate a 2-generation chart
    pub fn generate(
        &self,
        primary: &PersonData,
        father: &PersonData,
        mother: &PersonData,
    ) -> Result<Vec<u8>, ChartError> {
        // Generate Gen1 overlay
        let gen1_generator = Gen1Generator::new(self.settings.clone());
        let gen1_image = gen1_generator.generate(primary)?;

        // Create main canvas
        let mut wand = MagickWand::new();
        wand.set_size(CANVAS_WIDTH, CANVAS_HEIGHT)?;
        wand.new_image(
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
            &PixelWand::new().set_color("white"),
        )?;

        // Draw parents at 0° and 180° with proper outward offsets
        self.draw_parent(&mut wand, father, 0.0)?;
        self.draw_parent(&mut wand, mother, 180.0)?;

        // Composite Gen1 overlay in center
        self.composite_overlay(&mut wand, &gen1_image)?;

        wand.get_image_blob("PNG").map_err(ChartError::from)
    }

    fn draw_parent(
        &self,
        wand: &mut MagickWand,
        person: &PersonData,
        rotation: f64,
    ) -> Result<(), ChartError> {
        let mut draw = DrawingWand::new();

        // Apply transformations: translate to center, then rotate
        draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);
        draw.rotate(rotation);

        // Set up text rendering
        draw.set_font(&self.settings.font_family);
        draw.set_font_size(GEN2_PARENT_NAME_FONT_SIZE);
        draw.set_fill_color(&self.settings.font_color);

        // Calculate outward offsets based on rotation
        let (name_x, name_y) = match rotation {
            0.0 => (0.0, PARENT_FIRST_NAME_BASE_Y - IMAGE_CENTER_Y), // Right side
            180.0 => (0.0, IMAGE_CENTER_Y - PARENT_FIRST_NAME_BASE_Y), // Left side (inverted)
            _ => (0.0, 0.0), // Other rotations would use different constants
        };

        // Render name with outward offset from center
        draw.annotation(name_x, name_y, &person.full_name);

        // Apply outside stroke if enabled
        if self.settings.use_outside_stroke {
            self.render_stroke_effect(wand, &person.full_name, name_x, name_y, rotation)?;
        }

        wand.draw(&draw);
        Ok(())
    }

    fn render_stroke_effect(
        &self,
        wand: &mut MagickWand,
        text: &str,
        x: f64,
        y: f64,
        rotation: f64,
    ) -> Result<(), ChartError> {
        let mut stroke_draw = DrawingWand::new();
        stroke_draw.translate(IMAGE_CENTER_X, IMAGE_CENTER_Y);
        stroke_draw.rotate(rotation);

        stroke_draw.set_font(&self.settings.font_family);
        stroke_draw.set_font_size(GEN2_PARENT_NAME_FONT_SIZE);
        stroke_draw.set_fill_color(&self.settings.stroke_color);
        stroke_draw.set_stroke_color(&self.settings.stroke_color);
        stroke_draw.set_stroke_width(self.settings.stroke_width);

        stroke_draw.annotation(x, y, text);
        wand.draw(&stroke_draw);
        Ok(())
    }

    fn composite_overlay(
        &self,
        wand: &mut MagickWand,
        overlay_data: &[u8],
    ) -> Result<(), ChartError> {
        let mut overlay_wand = MagickWand::new();
        overlay_wand.read_image_blob(overlay_data)?;

        // Scale the overlay
        let scaled_width = (CANVAS_WIDTH as f64 * self.overlay_settings.scale) as usize;
        let scaled_height = (CANVAS_HEIGHT as f64 * self.overlay_settings.scale) as usize;
        overlay_wand.resize(scaled_width, scaled_height, FilterType::LanczosFilter)?;

        // Position in center
        let pos_x = (CANVAS_WIDTH as i32 - scaled_width as i32) / 2;
        let pos_y = (CANVAS_HEIGHT as i32 - scaled_height as i32) / 2;

        wand.composite(
            &overlay_wand,
            CompositeOperator::OverCompositeOp,
            pos_x,
            pos_y,
        )?;

        Ok(())
    }
}
