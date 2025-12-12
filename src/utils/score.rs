use anyhow::Result;
use image::GenericImageView;
use material_colors::color::Argb;
use material_colors::dislike::{fix_if_disliked, is_disliked};
use material_colors::hct::Hct;
use material_colors::quantize::Quantizer;
use material_colors::quantize::QuantizerCelebi;
use std::collections::HashMap;
use std::path::Path;

use super::math::sanitize_degrees_int;

const TARGET_CHROMA: f64 = 48.0;
const WEIGHT_PROPORTION: f64 = 0.7;
const WEIGHT_CHROMA_ABOVE: f64 = 0.3;
const WEIGHT_CHROMA_BELOW: f64 = 0.1;
const CUTOFF_CHROMA: f64 = 5.0;
const CUTOFF_EXCITED_PROPORTION: f64 = 0.01;

struct ScoredHct {
    hct: Hct,
    score: f64,
}

fn calculate_score(colors_to_population: &HashMap<u32, u32>, filter_enabled: bool) -> Hct {
    let mut hue_population = [0u32; 360];
    let mut population_sum = 0u32;
    let mut colors_hct: Vec<Hct> = Vec::new();

    for (&argb, &population) in colors_to_population {
        let hct = Hct::new(Argb::from_u32(argb));
        colors_hct.push(hct);
        let hue = hct.get_hue().round() as usize % 360;
        hue_population[hue] += population;
        population_sum += population;
    }

    let mut hue_excited_proportions = [0.0f64; 360];

    for hue in 0..360 {
        let proportion = hue_population[hue] as f64 / population_sum as f64;
        for i in (hue as i32 - 14)..=(hue as i32 + 16) {
            let neighbor_hue = sanitize_degrees_int(i) as usize;
            hue_excited_proportions[neighbor_hue] += proportion;
        }
    }

    let mut scored_hct: Vec<ScoredHct> = Vec::new();

    for hct in colors_hct {
        let hue = sanitize_degrees_int(hct.get_hue().round() as i32) as usize;
        let proportion = hue_excited_proportions[hue];

        if filter_enabled
            && (hct.get_chroma() < CUTOFF_CHROMA || proportion <= CUTOFF_EXCITED_PROPORTION)
        {
            continue;
        }

        let proportion_score = proportion * 100.0 * WEIGHT_PROPORTION;
        let chroma_weight = if hct.get_chroma() < TARGET_CHROMA {
            WEIGHT_CHROMA_BELOW
        } else {
            WEIGHT_CHROMA_ABOVE
        };
        let chroma_score = (hct.get_chroma() - TARGET_CHROMA) * chroma_weight;
        let score = proportion_score + chroma_score;

        scored_hct.push(ScoredHct { hct, score });
    }

    scored_hct.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let mut primary: Option<Hct> = None;

    for cutoff in (0..=20).rev() {
        let cutoff_f = cutoff as f64;
        for item in &scored_hct {
            if item.hct.get_chroma() > cutoff_f && item.hct.get_tone() > cutoff_f * 3.0 {
                primary = Some(item.hct);
                break;
            }
        }
        if primary.is_some() {
            break;
        }
    }

    match primary {
        Some(p) => {
            if is_disliked(&p) {
                fix_if_disliked(p)
            } else {
                p
            }
        }
        None => {
            if filter_enabled {
                calculate_score(colors_to_population, false)
            } else {
                Hct::new(Argb::from_u32(0xFF4285F4))
            }
        }
    }
}

pub fn score_image(image_path: &str) -> Result<Hct> {
    let img = image::open(Path::new(image_path))?;
    let (width, height) = img.dimensions();

    let mut pixels = Vec::with_capacity((width * height) as usize);
    for pixel in img.pixels() {
        let (_x, _y, rgba) = pixel;
        let argb = ((rgba[3] as u32) << 24)
            | ((rgba[0] as u32) << 16)
            | ((rgba[1] as u32) << 8)
            | (rgba[2] as u32);
        pixels.push(Argb::from_u32(argb));
    }

    let result = QuantizerCelebi::quantize(&pixels, 128);
    let colors_to_population: HashMap<u32, u32> = result
        .color_to_count
        .iter()
        .map(|(k, v)| {
            let k_u32 = ((k.alpha as u32) << 24)
                | ((k.red as u32) << 16)
                | ((k.green as u32) << 8)
                | (k.blue as u32);

            (k_u32, *v)
        })
        .collect();

    Ok(calculate_score(&colors_to_population, true))
}
