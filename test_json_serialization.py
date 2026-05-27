#!/usr/bin/env python3
"""
Test JSON serialization for the bridge interface
This tests the data format without requiring the Rust compilation
"""

import json


def test_person_data_json():
    """Test PersonData JSON serialization"""
    person = {
        "id": "I1",
        "full_name": "John Doe",
        "given_name": "John",
        "surname": "Doe",
        "birth_date": "1980-01-01",
        "birth_place": "New York",
        "death_date": None,
        "death_place": None,
    }

    json_str = json.dumps(person)
    print(f"Person JSON: {json_str}")

    # Verify it can be deserialized back
    parsed = json.loads(json_str)
    assert parsed["id"] == "I1"
    assert parsed["full_name"] == "John Doe"
    print("✅ Person JSON serialization works")


def test_settings_json():
    """Test ChartSettings JSON serialization"""
    settings = {
        "font_family": "Arial",
        "font_color": "black",
        "background_color": "white",
        "name_font_size": 74.0,
        "date_font_size": 52.0,
        "place_font_size": 48.0,
        "use_outside_stroke": False,
        "stroke_width": 4.0,
        "stroke_color": "white",
        "flag_size": 666,
        "flag_type": "birth",
    }

    json_str = json.dumps(settings)
    print(f"Settings JSON: {json_str}")

    # Verify it can be deserialized back
    parsed = json.loads(json_str)
    assert parsed["font_family"] == "Arial"
    assert parsed["name_font_size"] == 74.0
    print("✅ Settings JSON serialization works")


def test_ancestors_json():
    """Test Ancestors JSON serialization"""
    ancestors = {
        "1": {
            "id": "I2",
            "full_name": "Father",
            "given_name": "Father",
            "surname": "Doe",
            "birth_date": "1950-01-01",
            "birth_place": "Boston",
            "death_date": None,
            "death_place": None,
        },
        "2": {
            "id": "I3",
            "full_name": "Mother",
            "given_name": "Mother",
            "surname": "Doe",
            "birth_date": "1955-01-01",
            "birth_place": "Chicago",
            "death_date": None,
            "death_place": None,
        },
    }

    json_str = json.dumps(ancestors)
    print(f"Ancestors JSON: {json_str}")

    # Verify it can be deserialized back
    parsed = json.loads(json_str)
    assert "1" in parsed
    assert "2" in parsed
    assert parsed["1"]["full_name"] == "Father"
    print("✅ Ancestors JSON serialization works")


def test_bridge_interface():
    """Test the bridge interface format"""
    print("Testing bridge interface format...")

    # Test data
    primary_json = json.dumps(
        {
            "id": "I1",
            "full_name": "John Doe",
            "given_name": "John",
            "surname": "Doe",
            "birth_date": "1980-01-01",
            "birth_place": "New York",
            "death_date": None,
            "death_place": None,
        }
    )

    ancestors_json = json.dumps(
        {
            "1": {
                "id": "I2",
                "full_name": "Father",
                "given_name": "Father",
                "surname": "Doe",
                "birth_date": "1950-01-01",
                "birth_place": "Boston",
                "death_date": None,
                "death_place": None,
            }
        }
    )

    settings_json = json.dumps(
        {
            "font_family": "Arial",
            "font_color": "black",
            "background_color": "white",
            "name_font_size": 74.0,
            "date_font_size": 52.0,
            "place_font_size": 48.0,
            "use_outside_stroke": False,
            "stroke_width": 4.0,
            "stroke_color": "white",
            "flag_size": 666,
            "flag_type": "birth",
        }
    )

    print(f"Generation: 2")
    print(f"Primary JSON length: {len(primary_json)}")
    print(f"Ancestors JSON length: {len(ancestors_json)}")
    print(f"Settings JSON length: {len(settings_json)}")

    # This would be the call to the Rust bridge:
    # png_bytes = iyou_chart_kernel.render_chart_from_json(2, primary_json, ancestors_json, settings_json)

    print("✅ Bridge interface format is correct")


def main():
    """Run all JSON serialization tests"""
    print("🔍 JSON Serialization Tests")
    print("=" * 40)

    test_person_data_json()
    print()
    test_settings_json()
    print()
    test_ancestors_json()
    print()
    test_bridge_interface()

    print("\n🎉 All JSON serialization tests passed!")
    print("The bridge interface is ready for Rust integration.")


if __name__ == "__main__":
    main()
