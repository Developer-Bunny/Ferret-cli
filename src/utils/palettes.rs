use material_colors::color::Argb;
use material_colors::hct::Hct;

pub fn hex_to_hct(hex: &str) -> Hct {
    let color_u32 = u32::from_str_radix(hex, 16).unwrap_or(0);
    let full_alpha = 0xFF000000 | color_u32;
    let argb = Argb::from_u32(full_alpha);
    Hct::new(argb)
}

pub fn get_light_gruvbox() -> Vec<Hct> {
    vec![
        hex_to_hct("FDF9F3"),
        hex_to_hct("FF6188"),
        hex_to_hct("A9DC76"),
        hex_to_hct("FC9867"),
        hex_to_hct("FFD866"),
        hex_to_hct("F47FD4"),
        hex_to_hct("78DCE8"),
        hex_to_hct("333034"),
        hex_to_hct("121212"),
        hex_to_hct("FF6188"),
        hex_to_hct("A9DC76"),
        hex_to_hct("FC9867"),
        hex_to_hct("FFD866"),
        hex_to_hct("F47FD4"),
        hex_to_hct("78DCE8"),
        hex_to_hct("333034"),
    ]
}

pub fn get_dark_gruvbox() -> Vec<Hct> {
    vec![
        hex_to_hct("282828"),
        hex_to_hct("CC241D"),
        hex_to_hct("98971A"),
        hex_to_hct("D79921"),
        hex_to_hct("458588"),
        hex_to_hct("B16286"),
        hex_to_hct("689D6A"),
        hex_to_hct("A89984"),
        hex_to_hct("928374"),
        hex_to_hct("FB4934"),
        hex_to_hct("B8BB26"),
        hex_to_hct("FABD2F"),
        hex_to_hct("83A598"),
        hex_to_hct("D3869B"),
        hex_to_hct("8EC07C"),
        hex_to_hct("EBDBB2"),
    ]
}

pub fn get_light_catppuccin() -> Vec<Hct> {
    vec![
        hex_to_hct("dc8a78"),
        hex_to_hct("dd7878"),
        hex_to_hct("ea76cb"),
        hex_to_hct("8839ef"),
        hex_to_hct("d20f39"),
        hex_to_hct("e64553"),
        hex_to_hct("fe640b"),
        hex_to_hct("df8e1d"),
        hex_to_hct("40a02b"),
        hex_to_hct("179299"),
        hex_to_hct("04a5e5"),
        hex_to_hct("209fb5"),
        hex_to_hct("1e66f5"),
        hex_to_hct("7287fd"),
    ]
}

pub fn get_dark_catppuccin() -> Vec<Hct> {
    vec![
        hex_to_hct("f5e0dc"),
        hex_to_hct("f2cdcd"),
        hex_to_hct("f5c2e7"),
        hex_to_hct("cba6f7"),
        hex_to_hct("f38ba8"),
        hex_to_hct("eba0ac"),
        hex_to_hct("fab387"),
        hex_to_hct("f9e2af"),
        hex_to_hct("a6e3a1"),
        hex_to_hct("94e2d5"),
        hex_to_hct("89dceb"),
        hex_to_hct("74c7ec"),
        hex_to_hct("89b4fa"),
        hex_to_hct("b4befe"),
    ]
}

pub struct KColor {
    pub name: &'static str,
    pub hct: Hct,
}

pub fn get_kcolors() -> Vec<KColor> {
    vec![
        KColor {
            name: "klink",
            hct: hex_to_hct("2980b9"),
        },
        KColor {
            name: "kvisited",
            hct: hex_to_hct("9b59b6"),
        },
        KColor {
            name: "knegative",
            hct: hex_to_hct("da4453"),
        },
        KColor {
            name: "kneutral",
            hct: hex_to_hct("f67400"),
        },
        KColor {
            name: "kpositive",
            hct: hex_to_hct("27ae60"),
        },
    ]
}

pub const COLOUR_NAMES: [&str; 14] = [
    "rosewater",
    "flamingo",
    "pink",
    "mauve",
    "red",
    "maroon",
    "peach",
    "yellow",
    "green",
    "teal",
    "sky",
    "sapphire",
    "blue",
    "lavender",
];
