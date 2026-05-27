use iyou_chart_kernel::core::{ChartSettings, GenerationOverlay, PersonData};
/// Simple example demonstrating how to use the iyou_chart_kernel
use iyou_chart_kernel::{initialize_magick, Gen1Generator, Gen2Generator};
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ImageMagick environment
    initialize_magick();

    // Create chart settings
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

    // Create sample person data
    let person = PersonData {
        id: "I1".to_string(),
        full_name: "John Michael Smith".to_string(),
        given_name: "John".to_string(),
        surname: "Smith".to_string(),
        birth_date: Some("1970-05-15".to_string()),
        birth_place: Some("New York, NY".to_string()),
        death_date: Some("2020-01-01".to_string()),
        death_place: Some("Boston, MA".to_string()),
    };

    // Generate Gen1 chart
    println("Generating Gen1 chart...");
    let gen1_generator = Gen1Generator::new(settings.clone());
    let gen1_image = gen1_generator.generate(&person)?;

    // Save Gen1 image
    let mut gen1_file = File::create("gen1_output.png")?;
    gen1_file.write_all(&gen1_image)?;
    println("Gen1 chart saved to gen1_output.png");

    // Create parent data for Gen2
    let father = PersonData {
        id: "I2".to_string(),
        full_name: "Michael Johnson Smith".to_string(),
        given_name: "Michael".to_string(),
        surname: "Smith".to_string(),
        birth_date: Some("1945-03-22".to_string()),
        birth_place: Some("Chicago, IL".to_string()),
        death_date: None,
        death_place: None,
    };

    let mother = PersonData {
        id: "I3".to_string(),
        full_name: "Sarah Elizabeth Wilson".to_string(),
        given_name: "Sarah".to_string(),
        surname: "Wilson".to_string(),
        birth_date: Some("1948-11-10".to_string()),
        birth_place: Some("Boston, MA".to_string()),
        death_date: None,
        death_place: None,
    };

    // Create overlay settings for Gen2
    let overlay_settings = GenerationOverlay {
        scale: 0.50,
        position_x: 0,
        position_y: 0,
    };

    // Generate Gen2 chart
    println("Generating Gen2 chart...");
    let gen2_generator = Gen2Generator::new(settings, overlay_settings);
    let gen2_image = gen2_generator.generate(&person, &father, &mother)?;

    // Save Gen2 image
    let mut gen2_file = File::create("gen2_output.png")?;
    gen2_file.write_all(&gen2_image)?;
    println("Gen2 chart saved to gen2_output.png");

    println("Example completed successfully!");
    Ok(())
}
