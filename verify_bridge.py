#!/usr/bin/env python3
"""
Compact verification script for zero-copy JSON pass-through bridge
Tests the pure FFI interface with Django-style data streaming
"""

import json
import sys


def test_import():
    """Test basic import functionality"""
    try:
        import iyou_chart_kernel

        print("✅ Import successful")
        return True
    except ImportError as e:
        print(f"❌ Import failed: {e}")
        return False


def test_json_pass_through():
    """Test zero-copy JSON pass-through with mock Django data"""
    try:
        import iyou_chart_kernel

        # Mock Django data - exactly as it would come from the ORM
        primary_data = {
            "id": "I1",
            "full_name": "John Doe",
            "given_name": "John",
            "surname": "Doe",
            "birth_date": "1980-01-01",
            "birth_place": "New York",
            "death_date": None,
            "death_place": None,
        }
        primary_json = json.dumps(primary_data)

        # Mock Django settings - exactly as configured in the admin
        settings_data = {
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
        settings_json = json.dumps(settings_data)

        # Test Gen1 chart (no ancestors)
        ancestors_json = "{}"
        print("🧪 Testing Gen1 chart generation...")
        png_bytes = iyou_chart_kernel.render_chart_from_json(
            1, primary_json, ancestors_json, settings_json
        )

        # Verify PNG header
        assert png_bytes[:8] == b"\x89PNG\r\n\x1a\n", "Invalid PNG header"
        print(f"✅ Gen1 chart generated: {len(png_bytes)} bytes with valid PNG header")

        # Test Gen2 chart with parents
        ancestors_data = {
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
        ancestors_json = json.dumps(ancestors_data)

        print("🧪 Testing Gen2 chart generation...")
        png_bytes = iyou_chart_kernel.render_chart_from_json(
            2, primary_json, ancestors_json, settings_json
        )

        # Verify PNG header
        assert png_bytes[:8] == b"\x89PNG\r\n\x1a\n", "Invalid PNG header"
        print(f"✅ Gen2 chart generated: {len(png_bytes)} bytes with valid PNG header")

        return True
    except Exception as e:
        print(f"❌ JSON pass-through failed: {e}")
        import traceback

        traceback.print_exc()
        return False


def test_error_handling():
    """Test error handling with invalid JSON payloads"""
    try:
        import iyou_chart_kernel

        # Test invalid primary JSON
        try:
            iyou_chart_kernel.render_chart_from_json(1, "invalid json", "{}", "{}")
            assert False, "Should have raised ValueError for invalid JSON"
        except ValueError as e:
            print(f"✅ Caught expected JSON error: {e}")

        # Test invalid generation
        valid_primary = json.dumps(
            {
                "id": "I1",
                "full_name": "John Doe",
                "given_name": "John",
                "surname": "Doe",
                "birth_date": None,
                "birth_place": None,
                "death_date": None,
                "death_place": None,
            }
        )

        valid_settings = json.dumps(
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

        try:
            iyou_chart_kernel.render_chart_from_json(
                99, valid_primary, "{}", valid_settings
            )
            assert False, "Should have raised RuntimeError for invalid generation"
        except RuntimeError as e:
            print(f"✅ Caught expected generation error: {e}")

        return True
    except Exception as e:
        print(f"❌ Error handling test failed: {e}")
        return False


def main():
    """Run verification tests"""
    print("🔍 Zero-Copy JSON Pass-Through Bridge Verification")
    print("=" * 60)

    tests = [
        ("Import Test", test_import),
        ("JSON Pass-Through", test_json_pass_through),
        ("Error Handling", test_error_handling),
    ]

    results = []
    for name, test_func in tests:
        print(f"\n🧪 {name}")
        print("-" * 40)
        try:
            result = test_func()
            results.append(result)
        except Exception as e:
            print(f"❌ Test crashed: {e}")
            import traceback

            traceback.print_exc()
            results.append(False)

    print("\n" + "=" * 60)
    print("📊 Test Results")
    print("=" * 60)

    passed = sum(results)
    total = len(results)

    for i, (name, _) in enumerate(tests):
        status = "✅ PASS" if results[i] else "❌ FAIL"
        print(f"{status} {name}")

    print(f"\n📈 Summary: {passed}/{total} tests passed")

    if passed == total:
        print("🎉 All tests passed! Zero-copy JSON bridge is working correctly.")
        print("🚀 Ready for Django integration.")
        return 0
    else:
        print("💥 Some tests failed. Check the output above.")
        return 1


if __name__ == "__main__":
    sys.exit(main())
