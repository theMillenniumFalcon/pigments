use crate::core::error::{PigmentsError, Result};
use image::{DynamicImage, GenericImageView};
use linfa::Dataset;
use linfa::traits::{Fit, Predict};
use linfa_clustering::KMeans;
use ndarray::{Array2, Axis};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub percentage: f32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, percentage: f32) -> Self {
        Color {
            r,
            g,
            b,
            percentage,
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

pub struct ColorExtractor {
    image: DynamicImage,
}

impl ColorExtractor {
    pub fn new(image: DynamicImage) -> Self {
        ColorExtractor { image }
    }

    pub fn extract_colors(&self, num_colors: usize) -> Result<Vec<Color>> {
        if num_colors < 1 {
            return Err(PigmentsError::InvalidColorCount(
                "Number of colors must be at least 1".to_string(),
            ));
        }

        info!("Starting color extraction");
        let (width, height) = self.image.dimensions();
        info!("Original image size: {}x{}", width, height);
        
        // Downsample the image to make processing faster
        let max_dimension = 500; // Reduced from 1000
        let (new_width, new_height) = if width > max_dimension || height > max_dimension {
            let ratio = max_dimension as f32 / width.max(height) as f32;
            ((width as f32 * ratio) as u32, (height as f32 * ratio) as u32)
        } else {
            (width, height)
        };
        
        info!("Resizing image to {}x{}", new_width, new_height);
        let resized = self.image.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
        let rgb_image = resized.to_rgb8();
        let total_pixels = (new_width * new_height) as f32;
        
        info!("Converting image to RGB data");
        let mut pixel_data = Vec::new();
        
        for pixel in rgb_image.pixels() {
            pixel_data.extend_from_slice(&[pixel[0] as f32, pixel[1] as f32, pixel[2] as f32]);
        }

        let num_pixels = pixel_data.len() / 3;
        info!("Created dataset with {} pixels", num_pixels);

        info!("Creating observation matrix");
        let observations = Array2::from_shape_vec(
            (num_pixels, 3),
            pixel_data,
        ).map_err(|e| PigmentsError::ImageProcessError(e.to_string()))?;

        info!("Creating dataset");
        let dataset = Dataset::from(observations);

        info!("Starting k-means clustering with {} colors", num_colors);
        let kmeans = KMeans::params(num_colors)
            .max_n_iterations(20)  // Reduced iterations
            .tolerance(1e-2)      // Increased tolerance
            .fit(&dataset)
            .map_err(|e| PigmentsError::ColorExtractionError(e.to_string()))?;

        info!("Clustering complete, processing results");
        let predictions = kmeans.predict(&dataset);
        let centroids = kmeans.centroids();

        info!("Counting pixels in clusters");
        let mut cluster_counts = vec![0; num_colors];
        for &cluster in predictions.iter() {
            cluster_counts[cluster as usize] += 1;
        }

        info!("Converting centroids to colors");
        let colors: Vec<Color> = centroids
            .axis_iter(Axis(0))
            .zip(cluster_counts.iter())
            .map(|(centroid, &count)| {
                Color::new(
                    centroid[0].clamp(0.0, 255.0) as u8,
                    centroid[1].clamp(0.0, 255.0) as u8,
                    centroid[2].clamp(0.0, 255.0) as u8,
                    (count as f32 / total_pixels) * 100.0,
                )
            })
            .collect();

        info!("Color extraction complete");
        Ok(colors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    #[test]
    fn test_color_extraction() {
        // Create a simple test image with known colors
        let mut img = RgbImage::new(100, 100);
        for x in 0..50 {
            for y in 0..100 {
                img.put_pixel(x, y, Rgb([255, 0, 0])); // Red
            }
        }
        for x in 50..100 {
            for y in 0..100 {
                img.put_pixel(x, y, Rgb([0, 0, 255])); // Blue
            }
        }

        let extractor = ColorExtractor::new(DynamicImage::ImageRgb8(img));
        let colors = extractor.extract_colors(2).unwrap();

        assert_eq!(colors.len(), 2);
        // The percentages should be close to 50% each
        assert!((colors[0].percentage - 50.0).abs() < 5.0);
        assert!((colors[1].percentage - 50.0).abs() < 5.0);
    }
} 