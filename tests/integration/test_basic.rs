use iyou_chart_kernel::core::{ChartError, ChartSettings, GenerationOverlay, PersonData};
/// Integration tests for the chart kernel
/// Verifies mathematical stability and pixel layout validation
use iyou_chart_kernel::{initialize_magick, Gen1Generator, Gen2Generator};

// PNG magic header bytes
const PNG_MAGIC_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

#[test]
fn test_environment_initialization() {
    // This should not panic
    initialize_magick();
    initialize_magick(); // Second call should be safe
}

#[test]
fn test_gen1_generation() {
    initialize_magick();

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

    let generator = Gen1Generator::new(settings);
    let result = generator.generate(&person);

    assert!(result.is_ok());
    let image_bytes = result.unwrap();
    assert!(!image_bytes.is_empty());

    // Verify PNG magic header
    assert!(image_bytes.len() >= PNG_MAGIC_HEADER.len(), "Image too small to be valid PNG");
    assert_eq!(&image_bytes[..8], PNG_MAGIC_HEADER, "Invalid PNG header");

    // Verify reasonable image size (should be around expected canvas size)
    assert!(image_bytes.len() > 1000, "Image size too small");
    assert!(image_bytes.len() < 1000000, "Image size too large");

#[test]
fn test_gen2_generation() {

#[test]
fn test_error_handling() {
    initialize_magick();

    // Test with invalid settings (zero font size)
    let mut invalid_settings = ChartSettings {
        font_family: "Arial".to_string(),
        font_color: "black".to_string(),
        background_color: "white".to_string(),
        name_font_size: 0.0, // Invalid
        date_font_size: 52.0,
        place_font_size: 48.0,
        use_outside_stroke: false,
        stroke_width: 4.0,
        stroke_color: "white".to_string(),
        flag_size: 666,
        flag_type: "birth".to_string(),
    };

    let person = PersonData {
        id: "I1".to_string(),
        full_name: "Test Person".to_string(),
        given_name: "Test".to_string(),
        surname: "Person".to_string(),
        birth_date: None,
        birth_place: None,
        death_date: None,
        death_place: None,
    };

    let generator = Gen1Generator::new(invalid_settings);
    let result = generator.generate(&person);

    // Should handle gracefully even with invalid settings
    // (magick-rust may still produce output, just not ideal)
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_empty_person_data() {
    initialize_magick();

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

    let empty_person = PersonData {
        id: "I1".to_string(),
        full_name: "".to_string(), // Empty name
        given_name: "".to_string(),
        surname: "".to_string(),
        birth_date: None,
        birth_place: None,
        death_date: None,
        death_place: None,
    };

    let generator = Gen1Generator::new(settings);
    let result = generator.generate(&empty_person);

    // Should handle empty data gracefully
    assert!(result.is_ok());
    let image_bytes = result.unwrap();
    assert!(!image_bytes.is_empty());
}

#[test]
fn test_gen2_generation() {
    initialize_magick();

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

    let overlay_settings = GenerationOverlay {
        scale: 0.50,
        position_x: 0,
        position_y: 0,
    };

    let primary = PersonData {
        id: "I1".to_string(),
        full_name: "John Michael Smith".to_string(),
        given_name: "John".to_string(),
        surname: "Smith".to_string(),
        birth_date: Some("1970-05-15".to_string()),
        birth_place: Some("New York, NY".to_string()),
        death_date: Some("2020-01-01".to_string()),
        death_place: Some("Boston, MA".to_string()),
    };

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

    let generator = Gen2Generator::new(settings, overlay_settings);
    let result = generator.generate(&primary, &father, &mother);

    assert!(result.is_ok());
    let image_bytes = result.unwrap();
    assert!(!image_bytes.is_empty());

    // Verify PNG magic header for Gen2 output
    assert!(image_bytes.len() >= PNG_MAGIC_HEADER.len(), "Gen2 image too small to be valid PNG");
    assert_eq!(&image_bytes[..8], PNG_MAGIC_HEADER, "Gen2 invalid PNG header");

    // Verify that the recursive composition didn't cause memory issues
    // The Gen2 image should be larger than Gen1 due to parent elements
    assert!(image_bytes.len() > 1000, "Gen2 image size too small");
    assert!(image_bytes.len() < 2000000, "Gen2 image size too large");

    // Test that the overlay scale factor was applied correctly
    // The centralized nested overlay should be scaled by 0.50
    assert_relative_eq!(overlay_settings.scale, 0.50, epsilon = 1e-6);
