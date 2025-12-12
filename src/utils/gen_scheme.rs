use core::f64;
use material_colors::blend::cam16_ucs;
use material_colors::color::Argb;
use material_colors::dynamic_color::DynamicScheme;
use material_colors::hct::Hct;
use material_colors::scheme::variant::{
    SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
    SchemeNeutral, SchemeRainbow, SchemeTonalSpot, SchemeVibrant,
};
use std::collections::HashMap;

use super::math::{difference_degrees, rotation_direction, sanitize_degrees_double};
use super::palettes::{
    COLOUR_NAMES, get_dark_catppuccin, get_dark_gruvbox, get_kcolors, get_light_catppuccin,
    get_light_gruvbox,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SchemeVariant {
    Content,
    Expressive,
    Fidelity,
    FruitSalad,
    Monochrome,
    Neutral,
    Rainbow,
    TonalSpot,
    Vibrant,
}

impl SchemeVariant {
    pub fn from_str(s: &str) -> Self {
        match s {
            "content" => Self::Content,
            "expressive" => Self::Expressive,
            "fidelity" => Self::Fidelity,
            "fruitsalad" => Self::FruitSalad,
            "monochrome" => Self::Monochrome,
            "neutral" => Self::Neutral,
            "rainbow" => Self::Rainbow,
            "tonalspot" => Self::TonalSpot,
            _ => Self::Vibrant,
        }
    }
}

fn lighten(color: Hct, amount: f64) -> Hct {
    let diff = (100.0 - color.get_tone()) * amount;
    Hct::from(
        color.get_hue(),
        color.get_chroma() + diff / 5.0,
        color.get_tone() + diff,
    )
}

fn darken(color: Hct, amount: f64) -> Hct {
    let diff = color.get_tone() * amount;
    Hct::from(
        color.get_hue(),
        color.get_chroma() + diff / 5.0,
        color.get_tone() - diff,
    )
}

fn grayscale(color: Hct, light: bool) -> Hct {
    let mut color = if light {
        darken(color, 0.35)
    } else {
        lighten(color, 0.65)
    };
    color.set_chroma(0.0);
    color
}

fn mix(a: Hct, b: Hct, w: f64) -> Hct {
    let blended_argb = cam16_ucs(a.into(), b.into(), w);
    Hct::new(blended_argb)
}

fn harmonize(from_hct: Hct, to_hct: Hct, tone_boost: f64) -> Hct {
    let diff_degrees = difference_degrees(from_hct.get_hue(), to_hct.get_hue());
    let rotation = (diff_degrees * 0.8).min(100.0);
    let output_hue = sanitize_degrees_double(
        from_hct.get_hue() + rotation * rotation_direction(from_hct.get_hue(), to_hct.get_hue()),
    );
    Hct::from(
        output_hue,
        from_hct.get_chroma(),
        from_hct.get_tone() * (1.0 + tone_boost),
    )
}

pub fn gen_scheme(scheme_name: &str, primary: Hct, is_dark: bool) -> HashMap<String, String> {
    let variant = SchemeVariant::from_str(scheme_name);
    let light = !is_dark;
    let contrast_level = Some(0.0);

    let dynamic_scheme: DynamicScheme = match variant {
        SchemeVariant::Content => SchemeContent::new(primary, is_dark, contrast_level).scheme,
        SchemeVariant::Expressive => SchemeExpressive::new(primary, is_dark, contrast_level).scheme,
        SchemeVariant::Fidelity => SchemeFidelity::new(primary, is_dark, contrast_level).scheme,
        SchemeVariant::FruitSalad => SchemeFruitSalad::new(primary, is_dark, contrast_level).scheme,
        SchemeVariant::Monochrome => SchemeMonochrome::new(primary, is_dark, contrast_level).scheme,
        SchemeVariant::Neutral => SchemeNeutral::new(primary, is_dark, contrast_level).scheme,
        SchemeVariant::Rainbow => SchemeRainbow::new(primary, is_dark, contrast_level).scheme,
        SchemeVariant::TonalSpot => SchemeTonalSpot::new(primary, is_dark, contrast_level).scheme,
        SchemeVariant::Vibrant => SchemeVibrant::new(primary, is_dark, contrast_level).scheme,
    };

    let mut colors_hct: HashMap<String, Hct> = HashMap::new();

    colors_hct.insert("primary".to_string(), Hct::new(dynamic_scheme.primary()));
    colors_hct.insert(
        "onPrimary".to_string(),
        Hct::new(dynamic_scheme.on_primary()),
    );
    colors_hct.insert(
        "primaryContainer".to_string(),
        Hct::new(dynamic_scheme.primary_container()),
    );
    colors_hct.insert(
        "onPrimaryContainer".to_string(),
        Hct::new(dynamic_scheme.on_primary_container()),
    );

    colors_hct.insert(
        "secondary".to_string(),
        Hct::new(dynamic_scheme.secondary()),
    );
    colors_hct.insert(
        "onSecondary".to_string(),
        Hct::new(dynamic_scheme.on_secondary()),
    );
    colors_hct.insert(
        "secondaryContainer".to_string(),
        Hct::new(dynamic_scheme.secondary_container()),
    );
    colors_hct.insert(
        "onSecondaryContainer".to_string(),
        Hct::new(dynamic_scheme.on_secondary_container()),
    );

    colors_hct.insert("tertiary".to_string(), Hct::new(dynamic_scheme.tertiary()));
    colors_hct.insert(
        "onTertiary".to_string(),
        Hct::new(dynamic_scheme.on_tertiary()),
    );
    colors_hct.insert(
        "tertiaryContainer".to_string(),
        Hct::new(dynamic_scheme.tertiary_container()),
    );
    colors_hct.insert(
        "onTertiaryContainer".to_string(),
        Hct::new(dynamic_scheme.on_tertiary_container()),
    );

    colors_hct.insert("error".to_string(), Hct::new(dynamic_scheme.error()));
    colors_hct.insert("onError".to_string(), Hct::new(dynamic_scheme.on_error()));
    colors_hct.insert(
        "errorContainer".to_string(),
        Hct::new(dynamic_scheme.error_container()),
    );
    colors_hct.insert(
        "onErrorContainer".to_string(),
        Hct::new(dynamic_scheme.on_error_container()),
    );

    colors_hct.insert(
        "background".to_string(),
        Hct::new(dynamic_scheme.background()),
    );
    colors_hct.insert(
        "onBackground".to_string(),
        Hct::new(dynamic_scheme.on_background()),
    );
    colors_hct.insert("surface".to_string(), Hct::new(dynamic_scheme.surface()));
    colors_hct.insert(
        "onSurface".to_string(),
        Hct::new(dynamic_scheme.on_surface()),
    );

    colors_hct.insert(
        "surfaceVariant".to_string(),
        Hct::new(dynamic_scheme.surface_variant()),
    );
    colors_hct.insert(
        "onSurfaceVariant".to_string(),
        Hct::new(dynamic_scheme.on_surface_variant()),
    );
    colors_hct.insert("outline".to_string(), Hct::new(dynamic_scheme.outline()));
    colors_hct.insert(
        "outlineVariant".to_string(),
        Hct::new(dynamic_scheme.outline_variant()),
    );

    colors_hct.insert("shadow".to_string(), Hct::new(dynamic_scheme.shadow()));
    colors_hct.insert("scrim".to_string(), Hct::new(dynamic_scheme.scrim()));
    colors_hct.insert(
        "inverseSurface".to_string(),
        Hct::new(dynamic_scheme.inverse_surface()),
    );
    colors_hct.insert(
        "inverseOnSurface".to_string(),
        Hct::new(dynamic_scheme.inverse_on_surface()),
    );
    colors_hct.insert(
        "inversePrimary".to_string(),
        Hct::new(dynamic_scheme.inverse_primary()),
    );

    colors_hct.insert("primary_paletteKeyColor".to_string(), primary);

    let primary_palette_key = colors_hct["primary_paletteKeyColor"];
    let primary_color = colors_hct["primary"];

    let on_primary_fixed_variant = Hct::new(dynamic_scheme.on_primary_fixed_variant());

    let gruvbox_palette = if light {
        get_light_gruvbox()
    } else {
        get_dark_gruvbox()
    };
    for (i, hct) in gruvbox_palette.into_iter().enumerate() {
        let color = if variant == SchemeVariant::Monochrome {
            grayscale(hct, light)
        } else {
            let boost = if i < 8 { 0.35 } else { 0.2 };
            let sign = if light { -1.0 } else { 1.0 };
            harmonize(hct, primary_palette_key, boost * sign)
        };
        colors_hct.insert(format!("term{}", i), color);
    }

    let catppuccin_palette = if light {
        get_light_catppuccin()
    } else {
        get_dark_catppuccin()
    };
    for (i, hct) in catppuccin_palette.into_iter().enumerate() {
        let color = if variant == SchemeVariant::Monochrome {
            grayscale(hct, light)
        } else {
            let boost = if light { -0.2 } else { 0.05 };
            harmonize(hct, primary_palette_key, boost)
        };
        colors_hct.insert(COLOUR_NAMES[i].to_string(), color);
    }

    for kcolor in get_kcolors() {
        let mut base = harmonize(kcolor.hct, primary_color, 0.1);
        let mut selection = harmonize(kcolor.hct, on_primary_fixed_variant, 0.1);

        if variant == SchemeVariant::Monochrome {
            base = grayscale(base, light);
            selection = grayscale(selection, light);
        }

        colors_hct.insert(kcolor.name.to_string(), base);
        colors_hct.insert(format!("{}Selection", kcolor.name), selection);
    }

    if variant == SchemeVariant::Neutral {
        for (_, hct) in colors_hct.iter_mut() {
            hct.set_chroma(hct.get_chroma() - 15.0);
        }
    }

    let surface = colors_hct["surface"];
    let outline = colors_hct["outline"];
    let on_background = colors_hct["onBackground"];
    let on_surface_variant = colors_hct["onSurfaceVariant"];

    colors_hct.insert("text".to_string(), on_background);
    colors_hct.insert("subtext1".to_string(), on_surface_variant);
    colors_hct.insert("subtext0".to_string(), outline);

    colors_hct.insert("overlay2".to_string(), mix(surface, outline, 0.86));
    colors_hct.insert("overlay1".to_string(), mix(surface, outline, 0.71));
    colors_hct.insert("overlay0".to_string(), mix(surface, outline, 0.57));
    colors_hct.insert("surface2".to_string(), mix(surface, outline, 0.43));
    colors_hct.insert("surface1".to_string(), mix(surface, outline, 0.29));
    colors_hct.insert("surface0".to_string(), mix(surface, outline, 0.14));

    colors_hct.insert("base".to_string(), surface);
    colors_hct.insert("mantle".to_string(), darken(surface, 0.03));
    colors_hct.insert("crust".to_string(), darken(surface, 0.05));

    let mut colors_hex: HashMap<String, String> = colors_hct
        .iter()
        .map(|(k, v)| {
            let color_argb = Argb::from(*v);

            let argb_u32 = ((color_argb.alpha as u32) << 24)
                | ((color_argb.red as u32) << 16)
                | ((color_argb.green as u32) << 8)
                | (color_argb.blue as u32);

            let hex = format!("{:08X}", argb_u32);
            (k.clone(), hex[2..].to_string())
        })
        .collect();

    if light {
        colors_hex.insert("success".to_string(), "4F6354".to_string());
        colors_hex.insert("onSuccess".to_string(), "FFFFFF".to_string());
        colors_hex.insert("successContainer".to_string(), "D1E8D5".to_string());
        colors_hex.insert("onSuccessContainer".to_string(), "0C1F13".to_string());
    } else {
        colors_hex.insert("success".to_string(), "B5CCBA".to_string());
        colors_hex.insert("onSuccess".to_string(), "213528".to_string());
        colors_hex.insert("successContainer".to_string(), "374B3E".to_string());
        colors_hex.insert("onSuccessContainer".to_string(), "D1E9D6".to_string());
    }

    colors_hex
}
