use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};

pub struct BlurConfig {
    pub sigma: f32,
    pub radius: u32,
}

impl BlurConfig {
    pub fn new(sigma: f32) -> Self {
        // Calculate radius from sigma (typically 3*sigma is sufficient)
        let radius = (sigma * 3.0).ceil() as u32;
        Self { sigma, radius }
    }

    pub fn light() -> Self {
        Self::new(0.8)
    }

    pub fn moderate() -> Self {
        Self::new(1.2)
    }

    pub fn heavy() -> Self {
        Self::new(2.0)
    }
}

impl Default for BlurConfig {
    fn default() -> Self {
        Self::moderate()
    }
}

#[allow(dead_code)]
pub fn apply_gaussian_blur(img: &DynamicImage, config: &BlurConfig) -> DynamicImage {
    // Use the built-in blur for now, but we can implement custom Gaussian if needed
    img.blur(config.sigma)
}

#[allow(dead_code)]
pub fn apply_custom_gaussian_blur(img: &DynamicImage, config: &BlurConfig) -> DynamicImage {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    // Generate Gaussian kernel
    let kernel = generate_gaussian_kernel(config.sigma, config.radius);
    let kernel_size = (2 * config.radius + 1) as i32;

    let mut output_img: RgbImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0.0f32;
            let mut g_sum = 0.0f32;
            let mut b_sum = 0.0f32;
            let mut weight_sum = 0.0f32;

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let px = x as i32 + kx - config.radius as i32;
                    let py = y as i32 + ky - config.radius as i32;

                    if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                        let pixel = rgb_img.get_pixel(px as u32, py as u32);
                        let weight = kernel[(ky * kernel_size + kx) as usize];

                        r_sum += f32::from(pixel[0]) * weight;
                        g_sum += f32::from(pixel[1]) * weight;
                        b_sum += f32::from(pixel[2]) * weight;
                        weight_sum += weight;
                    }
                }
            }

            if weight_sum > 0.0 {
                let r = (r_sum / weight_sum).round().clamp(0.0, 255.0) as u8;
                let g = (g_sum / weight_sum).round().clamp(0.0, 255.0) as u8;
                let b = (b_sum / weight_sum).round().clamp(0.0, 255.0) as u8;

                output_img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
    }

    DynamicImage::ImageRgb8(output_img)
}

#[allow(dead_code)]
fn generate_gaussian_kernel(sigma: f32, radius: u32) -> Vec<f32> {
    let size = (2 * radius + 1) as usize;
    let mut kernel = Vec::with_capacity(size * size);
    let sigma_sq = sigma * sigma;
    let two_sigma_sq = 2.0 * sigma_sq;
    let norm_factor = 1.0 / (std::f32::consts::PI * two_sigma_sq);

    let center = radius as i32;
    let mut sum = 0.0f32;

    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - center;
            let dy = y as i32 - center;
            let distance_sq = (dx * dx + dy * dy) as f32;

            let weight = norm_factor * (-distance_sq / two_sigma_sq).exp();
            kernel.push(weight);
            sum += weight;
        }
    }

    // Normalize kernel so weights sum to 1
    for weight in &mut kernel {
        *weight /= sum;
    }

    kernel
}

pub struct NoiseReduction {
    threshold: f32,
    strength: f32,
}

impl NoiseReduction {
    pub fn new(threshold: f32, strength: f32) -> Self {
        Self {
            threshold,
            strength,
        }
    }

    pub fn apply(&self, img: &DynamicImage) -> DynamicImage {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let mut output_img: RgbImage = ImageBuffer::new(width, height);

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center_pixel = rgb_img.get_pixel(x, y);
                let mut r_sum = f32::from(center_pixel[0]);
                let mut g_sum = f32::from(center_pixel[1]);
                let mut b_sum = f32::from(center_pixel[2]);
                let mut count = 1;

                // Check surrounding pixels
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let px = (x as i32 + dx) as u32;
                        let py = (y as i32 + dy) as u32;
                        let neighbor = rgb_img.get_pixel(px, py);

                        let diff = color_difference(center_pixel, neighbor);
                        if diff < self.threshold {
                            r_sum += f32::from(neighbor[0]);
                            g_sum += f32::from(neighbor[1]);
                            b_sum += f32::from(neighbor[2]);
                            count += 1;
                        }
                    }
                }

                if count > 1 {
                    let avg_r = r_sum / count as f32;
                    let avg_g = g_sum / count as f32;
                    let avg_b = b_sum / count as f32;

                    let final_r = lerp(f32::from(center_pixel[0]), avg_r, self.strength);
                    let final_g = lerp(f32::from(center_pixel[1]), avg_g, self.strength);
                    let final_b = lerp(f32::from(center_pixel[2]), avg_b, self.strength);

                    output_img.put_pixel(
                        x,
                        y,
                        Rgb([
                            final_r.round() as u8,
                            final_g.round() as u8,
                            final_b.round() as u8,
                        ]),
                    );
                } else {
                    output_img.put_pixel(x, y, *center_pixel);
                }
            }
        }

        // Copy border pixels
        for y in 0..height {
            if y == 0 || y == height - 1 {
                for x in 0..width {
                    output_img.put_pixel(x, y, *rgb_img.get_pixel(x, y));
                }
            } else {
                output_img.put_pixel(0, y, *rgb_img.get_pixel(0, y));
                output_img.put_pixel(width - 1, y, *rgb_img.get_pixel(width - 1, y));
            }
        }

        DynamicImage::ImageRgb8(output_img)
    }
}

fn color_difference(c1: &Rgb<u8>, c2: &Rgb<u8>) -> f32 {
    let dr = f32::from(c1[0]) - f32::from(c2[0]);
    let dg = f32::from(c1[1]) - f32::from(c2[1]);
    let db = f32::from(c1[2]) - f32::from(c2[2]);
    (dr * dr + dg * dg + db * db).sqrt()
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    fn create_test_image() -> DynamicImage {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(10, 10, |x, y| {
            if (x + y) % 2 == 0 {
                Rgb([255, 255, 255])
            } else {
                Rgb([0, 0, 0])
            }
        });
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_blur_config() {
        let light = BlurConfig::light();
        assert!(light.sigma < 1.0);

        let heavy = BlurConfig::heavy();
        assert!(heavy.sigma > 1.5);

        let moderate = BlurConfig::moderate();
        assert!(moderate.sigma > light.sigma);
        assert!(moderate.sigma < heavy.sigma);
    }

    #[test]
    fn test_gaussian_kernel_generation() {
        let kernel = generate_gaussian_kernel(1.0, 1);
        assert_eq!(kernel.len(), 9); // 3x3 kernel

        // Sum should be approximately 1.0
        let sum: f32 = kernel.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_apply_gaussian_blur() {
        let img = create_test_image();
        let config = BlurConfig::light();

        let blurred = apply_gaussian_blur(&img, &config);
        assert_eq!(blurred.width(), 10);
        assert_eq!(blurred.height(), 10);
    }

    #[test]
    fn test_custom_gaussian_blur() {
        let img = create_test_image();
        let config = BlurConfig::new(0.5);

        let blurred = apply_custom_gaussian_blur(&img, &config);
        assert_eq!(blurred.width(), 10);
        assert_eq!(blurred.height(), 10);
    }

    #[test]
    fn test_noise_reduction() {
        let img = create_test_image();
        let noise_filter = NoiseReduction::new(50.0, 0.3);

        let filtered = noise_filter.apply(&img);
        assert_eq!(filtered.width(), 10);
        assert_eq!(filtered.height(), 10);
    }

    #[test]
    fn test_color_difference() {
        let white = Rgb([255, 255, 255]);
        let black = Rgb([0, 0, 0]);
        let gray = Rgb([128, 128, 128]);

        assert_eq!(color_difference(&white, &white), 0.0);
        assert!(color_difference(&white, &black) > color_difference(&white, &gray));
    }
}
