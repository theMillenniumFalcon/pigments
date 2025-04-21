mod cli;
mod core;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use core::{Color, ColorExtractor};
use image::io::Reader as ImageReader;
use log::info;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    info!("Loading image from {:?}", args.input);
    let img = ImageReader::open(&args.input)?.decode()?;
    
    let extractor = ColorExtractor::new(img);
    let colors = extractor.extract_colors(args.num_colors)?;

    let output = match args.format.as_str() {
        "json" => serde_json::to_string_pretty(&colors)?,
        "text" => format_colors_text(&colors),
        _ => return Err(anyhow::anyhow!("Unsupported output format")),
    };

    if let Some(ref output_path) = args.output {
        let mut file = File::create(output_path)?;
        file.write_all(output.as_bytes())?;
        info!("Results written to {:?}", output_path);
    } else {
        println!("{}", output);
    }

    Ok(())
}

fn format_colors_text(colors: &[Color]) -> String {
    colors
        .iter()
        .map(|color| {
            format!(
                "Color: {} (RGB: {}, {}, {}) - {:.1}%",
                color.to_hex(),
                color.r,
                color.g,
                color.b,
                color.percentage
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_colors_text() {
        let colors = vec![
            Color::new(255, 0, 0, 50.0),
            Color::new(0, 0, 255, 50.0),
        ];

        let formatted = format_colors_text(&colors);
        assert!(formatted.contains("#FF0000"));
        assert!(formatted.contains("#0000FF"));
        assert!(formatted.contains("50.0%"));
    }
}
