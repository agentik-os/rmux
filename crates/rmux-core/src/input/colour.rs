//! Colour representation matching tmux colour model.

/// A terminal colour value.
///
/// - Values 0–7 are standard colours.
/// - Value 8 is the default colour.
/// - `COLOUR_FLAG_256 | idx` for 256-colour palette.
/// - `COLOUR_FLAG_RGB | (r << 16) | (g << 8) | b` for RGB.
pub type Colour = i32;

/// Default colour sentinel.
pub const COLOUR_DEFAULT: Colour = 8;

/// No-colour sentinel used by tmux `colour_tostring`.
pub const COLOUR_NONE: Colour = -1;

/// Terminal colour sentinel.
pub const COLOUR_TERMINAL: Colour = 9;

/// Flag for 256-colour palette indices.
pub const COLOUR_FLAG_256: Colour = 0x0100_0000;

/// Flag for true-colour RGB values.
pub const COLOUR_FLAG_RGB: Colour = 0x0200_0000;

/// Compose an RGB colour from r, g, b components.
#[must_use]
pub fn colour_join_rgb(r: u8, g: u8, b: u8) -> Colour {
    COLOUR_FLAG_RGB | (i32::from(r) << 16) | (i32::from(g) << 8) | i32::from(b)
}

/// Resolves a colour to RGB components when it has a concrete value.
///
/// ANSI and 256-palette indices resolve through the standard xterm palette;
/// the default/terminal/none sentinels have no concrete RGB value. Needed by
/// OSC 10/11 colour-query replies, which must always answer in `rgb:` form.
#[must_use]
pub fn colour_to_rgb(colour: Colour) -> Option<(u8, u8, u8)> {
    if matches!(colour, COLOUR_NONE | COLOUR_DEFAULT | COLOUR_TERMINAL) {
        return None;
    }
    if (colour & COLOUR_FLAG_RGB) != 0 {
        return Some((
            ((colour >> 16) & 0xff) as u8,
            ((colour >> 8) & 0xff) as u8,
            (colour & 0xff) as u8,
        ));
    }
    if (colour & COLOUR_FLAG_256) != 0 {
        return xterm_palette_rgb((colour & 0xff) as u8);
    }
    match colour {
        0..=7 => XTERM_BASIC_PALETTE.get(colour as usize).copied(),
        90..=97 => XTERM_BASIC_PALETTE.get((colour - 82) as usize).copied(),
        _ => None,
    }
}

fn xterm_palette_rgb(index: u8) -> Option<(u8, u8, u8)> {
    if usize::from(index) < XTERM_BASIC_PALETTE.len() {
        return XTERM_BASIC_PALETTE.get(index as usize).copied();
    }
    if (16..=231).contains(&index) {
        let cube = index - 16;
        return Some((
            XTERM_CUBE_LEVELS[usize::from(cube / 36)],
            XTERM_CUBE_LEVELS[usize::from((cube % 36) / 6)],
            XTERM_CUBE_LEVELS[usize::from(cube % 6)],
        ));
    }
    // 232-255: the 24-step grayscale ramp.
    Some((8 + 10 * (index - 232), 8 + 10 * (index - 232), 8 + 10 * (index - 232)))
}

const XTERM_BASIC_PALETTE: &[(u8, u8, u8)] = &[
    (0, 0, 0),
    (205, 0, 0),
    (0, 205, 0),
    (205, 205, 0),
    (0, 0, 238),
    (205, 0, 205),
    (0, 205, 205),
    (229, 229, 229),
    (127, 127, 127),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (92, 92, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
];

const XTERM_CUBE_LEVELS: [u8; 6] = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff];
