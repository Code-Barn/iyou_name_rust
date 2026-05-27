/// Integration tests for the unified generator
/// Tests polymorphic dispatch and strategy routing

use iyou_chart_kernel::{initialize_magick, UnifiedChartGenerator, AncestorData};
use iyou_chart_kernel::core::{PersonData, ChartSettings};

const PNG_MAGIC_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

#[test]
fn test_unified_generator_gen1() {
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

    let generator = UnifiedChartGenerator::new(settings);

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

    let ancestors = AncestorData::new();

    // Generate Gen1 chart
    let result = generator.generate(1, &primary, &ancestors);
    assert!(result.is_ok());

    let image_bytes = result.unwrap();
    assert!(!image_bytes.is_empty());

    // Verify PNG header
    assert!(image_bytes.len() >= PNG_MAGIC_HEADER.len());
    assert_eq!(&image_bytes[..8], PNG_MAGIC_HEADER, "Invalid PNG header for Gen1");
}

#[test]
fn test_unified_generator_gen2() {
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

    let generator = UnifiedChartGenerator::new(settings);

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

    let mut ancestors = AncestorData::new();

    // Add father
    ancestors.add_individual("1", PersonData {
        id: "I2".to_string(),
        full_name: "Michael Johnson Smith".to_string(),
        given_name: "Michael".to_string(),
        surname: "Smith".to_string(),
        birth_date: Some("1945-03-22".to_string()),
        birth_place: Some("Chicago, IL".to_string()),
        death_date: None,
        death_place: None,
    });

    // Add mother
    ancestors.add_individual("2", PersonData {
        id: "I3".to_string(),
        full_name: "Sarah Elizabeth Wilson".to_string(),
        given_name: "Sarah".to_string(),
        surname: "Wilson".to_string(),
        birth_date: Some("1948-11-10".to_string()),
        birth_place: Some("Boston, MA".to_string()),
        death_date: None,
        death_place: None,
    });

    // Generate Gen2 chart
    let result = generator.generate(2, &primary, &ancestors);
    assert!(result.is_ok());

    let image_bytes = result.unwrap();
    assert!(!image_bytes.is_empty());

    // Verify PNG header
    assert!(image_bytes.len() >= PNG_MAGIC_HEADER.len());
    assert_eq!(&image_bytes[..8], PNG_MAGIC_HEADER, "Invalid PNG header for Gen2");

    // Gen2 should be larger than Gen1 due to parent elements
    assert!(image_bytes.len() > 1000);
}

#[test]
fn test_strategy_dispatch() {
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

    let generator = UnifiedChartGenerator::new(settings);

    // Test strategy access
    assert!(generator.is_supported(1));
    assert!(generator.is_supported(2));
    assert!(!generator.is_supported(3));

    let gen1_strategy = generator.get_strategy(1).unwrap();
    assert_eq!(gen1_strategy.name(), "Gen1Strategy");
    assert_eq!(gen1_strategy.generation(), 1);

    let gen2_strategy = generator.get_strategy(2).unwrap();
    assert_eq!(gen2_strategy.name(), "Gen2Strategy");
    assert_eq!(gen2_strategy.generation(), 2);
}

#[test]
fn test_unsupported_generation() {
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

    let generator = UnifiedChartGenerator::new(settings);

    let primary = PersonData {
        id: "I1".to_string(),
        full_name: "Test Person".to_string(),
        given_name: "Test".to_string(),
        surname: "Person".to_string(),
        birth_date: None,
        birth_place: None,
        death_date: None,
        death_place: None,
    };

    let ancestors = AncestorData::new();

    // Test unsupported generation
    let result = generator.generate(3, &primary, &ancestors);
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error.to_string().contains("Generation 3 not supported"));
    }
}
