use rmux_core::{
    Colour, COLOUR_DEFAULT, COLOUR_FLAG_256, COLOUR_FLAG_RGB, COLOUR_NONE, COLOUR_TERMINAL,
};

pub(super) fn colour_to_rgb_string(colour: Colour) -> Option<String> {
    let (red, green, blue) = colour_to_rgb(colour)?;
    Some(format!("rgb:{red:02x}/{green:02x}/{blue:02x}"))
}

pub(super) fn colour_to_rgb(colour: Colour) -> Option<(u8, u8, u8)> {
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
        0..=7 => BASIC_PALETTE.get(colour as usize).copied(),
        90..=97 => BASIC_PALETTE.get((colour - 82) as usize).copied(),
        _ => None,
    }
}

fn xterm_palette_rgb(index: u8) -> Option<(u8, u8, u8)> {
    if usize::from(index) < BASIC_PALETTE.len() {
        return BASIC_PALETTE.get(index as usize).copied();
    }
    if (16..=231).contains(&index) {
        let cube = index - 16;
        let red = cube / 36;
        let green = (cube % 36) / 6;
        let blue = cube % 6;
        return Some((
            CUBE_LEVELS[red as usize],
            CUBE_LEVELS[green as usize],
            CUBE_LEVELS[blue as usize],
        ));
    }
    if (232..=255).contains(&index) {
        let value = 8 + 10 * (index - 232);
        return Some((value, value, value));
    }
    None
}

const BASIC_PALETTE: &[(u8, u8, u8)] = &[
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
const CUBE_LEVELS: &[u8] = &[0, 95, 135, 175, 215, 255];

/// Finds the closest 256-palette index for an RGB value, comparing the 6x6x6
/// colour cube against the grayscale ramp (port of tmux `colour_find_rgb`).
/// Used to downgrade truecolor SGR for clients without RGB support.
pub(super) fn nearest_256_index(r: u8, g: u8, b: u8) -> u8 {
    fn to_6cube(value: u8) -> usize {
        if value < 48 {
            0
        } else if value < 114 {
            1
        } else {
            usize::from((value - 35) / 40)
        }
    }
    fn dist_sq(ar: u8, ag: u8, ab: u8, br: u8, bg: u8, bb: u8) -> i32 {
        let dr = i32::from(ar) - i32::from(br);
        let dg = i32::from(ag) - i32::from(bg);
        let db = i32::from(ab) - i32::from(bb);
        dr * dr + dg * dg + db * db
    }

    let (qr, qg, qb) = (to_6cube(r), to_6cube(g), to_6cube(b));
    let (cr, cg, cb) = (CUBE_LEVELS[qr], CUBE_LEVELS[qg], CUBE_LEVELS[qb]);
    let cube_index = (16 + 36 * qr + 6 * qg + qb) as u8;
    if (cr, cg, cb) == (r, g, b) {
        return cube_index;
    }

    let gray_avg = (u16::from(r) + u16::from(g) + u16::from(b)) / 3;
    let gray_index = if gray_avg > 238 {
        23
    } else {
        gray_avg.saturating_sub(3) / 10
    };
    let gray = (8 + 10 * gray_index) as u8;

    if dist_sq(gray, gray, gray, r, g, b) < dist_sq(cr, cg, cb, r, g, b) {
        232 + gray_index as u8
    } else {
        cube_index
    }
}
