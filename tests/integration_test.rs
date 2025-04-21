use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_extract_colors_from_image() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.child("test.png");
    let output_path = temp.child("colors.json");

    // Create a test image with known colors
    let mut img = image::RgbImage::new(100, 100);
    for x in 0..50 {
        for y in 0..100 {
            img.put_pixel(x, y, image::Rgb([255, 0, 0])); // Red
        }
    }
    for x in 50..100 {
        for y in 0..100 {
            img.put_pixel(x, y, image::Rgb([0, 0, 255])); // Blue
        }
    }
    img.save(&input_path).unwrap();

    // Run the application
    let status = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-i")
        .arg(input_path.path())
        .arg("-n")
        .arg("2")
        .arg("-f")
        .arg("json")
        .arg("-o")
        .arg(output_path.path())
        .status()
        .unwrap();

    assert!(status.success());

    // Verify the output file exists and contains valid JSON
    output_path.assert(predicate::path::exists());
    let output_content = std::fs::read_to_string(output_path.path()).unwrap();
    
    // Parse the JSON output
    let colors: Vec<serde_json::Value> = serde_json::from_str(&output_content).unwrap();
    assert_eq!(colors.len(), 2);
    
    // Check that we have both red and blue colors
    let has_red = colors.iter().any(|c| c["r"].as_u64().unwrap() == 255 && c["g"].as_u64().unwrap() == 0 && c["b"].as_u64().unwrap() == 0);
    let has_blue = colors.iter().any(|c| c["r"].as_u64().unwrap() == 0 && c["g"].as_u64().unwrap() == 0 && c["b"].as_u64().unwrap() == 255);
    assert!(has_red);
    assert!(has_blue);
    
    // Check percentages are close to 50%
    for color in colors {
        let percentage = color["percentage"].as_f64().unwrap();
        assert!((percentage - 50.0).abs() < 5.0);
    }
} 