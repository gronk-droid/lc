use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette {
    base_lightness_range: Range<f32>,
    low_lightness: f32,
    high_lightness: f32,
    low_chroma: f32,
    medium_chroma: f32,
    high_chroma: f32,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            base_lightness_range: 0.17..0.988,
            low_lightness: 0.8,
            high_lightness: 0.988,
            low_chroma: 0.032,
            medium_chroma: 0.07,
            high_chroma: 0.1,
        }
    }
}

impl Palette {

    /*
    .88b  d88.  .d88b.  d8b   db  .d88b.
    88'YbdP`88 .8P  Y8. 888o  88 .8P  Y8.
    88  88  88 88    88 88V8o 88 88    88
    88  88  88 88    88 88 V8o88 88    88
    88  88  88 `8b  d8' 88  V888 `8b  d8'
    YP  YP  YP  `Y88P'  VP   V8P  `Y88P'


    */

    pub(crate) fn black(&self) -> Oklch {
        oklch(0.2898, 0.0083, 317.72)
    }

    pub(crate) fn one(&self) -> Oklch {
        oklch(0.3554, 0.0109, 325.8)
    }

    pub(crate) fn two(&self) -> Oklch {
        oklch(0.4181, 0.0134, 330.4)
    }

    pub(crate) fn four(&self) -> Oklch {
        oklch(0.5374, 0.0128, 348.76)
    }

    pub(crate) fn six(&self) -> Oklch {
        oklch(0.6497, 0.0132, 17.5)
    }

    pub(crate) fn seven(&self) -> Oklch {
        oklch(0.7072, 0.0121, 31.08)
    }

    pub(crate) fn eight(&self) -> Oklch {
        oklch(0.7658, 0.0108, 48.58)
    }

    pub(crate) fn nine(&self) -> Oklch {
        oklch(0.8263, 0.0118, 71.88)
    }

    pub(crate) fn ten(&self) -> Oklch {
        oklch(0.8849, 0.0139, 93.0)
    }

    pub(crate) fn eleven(&self) -> Oklch {
        oklch(0.9433, 0.0173, 103.15)
    }

    pub(crate) fn white(&self) -> Oklch {
        oklch(0.9947, 0.0184, 113.33)
    }

    /*
     .o88b.  .d88b.  db       .d88b.  d8888b.
    d8P  Y8 .8P  Y8. 88      .8P  Y8. 88  `8D
    8P      88    88 88      88    88 88oobY'
    8b      88    88 88      88    88 88`8b
    Y8b  d8 `8b  d8' 88booo. `8b  d8' 88 `88.
     `Y88P'  `Y88P'  Y88888P  `Y88P'  88   YD


    */

    pub(crate) fn red(&self) -> Oklch {
        // oklch(self.low_lightness, self.high_chroma, 30.0)
        oklch(0.688,  0.195, 20.6)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.7467,  0.1308, 43.76)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.891, 0.0919, 85.1)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.882, 0.101, 130.0)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.8341, 0.0889, 175.67)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.824, 0.0857, 204.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.572, 0.084, 291.0)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.7699, 0.1315, 358.49)
    }

    /*
    d888888b d888888b d8b   db d888888b .d8888.
    `~~88~~'   `88'   888o  88 `~~88~~' 88'  YP
       88       88    88V8o 88    88    `8bo.
       88       88    88 V8o88    88      `Y8b.
       88      .88.   88  V888    88    db   8D
       YP    Y888888P VP   V8P    YP    `8888Y'


    */

    pub(crate) fn light_red(&self) -> Oklch {
        oklch(0.8257, 0.0934, 21.69)
    }

    pub(crate) fn light_orange(&self) -> Oklch {
        oklch(0.8648, 0.0666, 51.8)
    }

    pub(crate) fn light_yellow(&self) -> Oklch {
        oklch(0.942, 0.0545, 89.91)
    }

    pub(crate) fn light_green(&self) -> Oklch {
        oklch(0.9391, 0.0601, 126.95)
    }

    pub(crate) fn light_teal(&self) -> Oklch {
        oklch(0.9108, 0.0508, 167.88)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(0.9056, 0.0455, 192.96)
    }

    pub(crate) fn light_purple(&self) -> Oklch {
        oklch(0.7881, 0.0282, 292.63)
    }

    pub(crate) fn light_pink(&self) -> Oklch {
        oklch(0.8774, 0.0595, 3.95)
    }

    /*
    .d8888. db   db  .d8b.  d8888b. d88888b .d8888.
    88'  YP 88   88 d8' `8b 88  `8D 88'     88'  YP
    `8bo.   88ooo88 88ooo88 88   88 88ooooo `8bo.
      `Y8b. 88~~~88 88~~~88 88   88 88~~~~~   `Y8b.
    db   8D 88   88 88   88 88  .8D 88.     db   8D
    `8888Y' YP   YP YP   YP Y8888D' Y88888P `8888Y'


    */

    pub(crate) fn dark_red(&self) -> Oklch {
        oklch(0.4897, 0.1093, 16.68)
    }

    pub(crate) fn dark_orange(&self) -> Oklch {
        oklch(0.5281, 0.0722, 41.57)
    }

    pub(crate) fn dark_yellow(&self) -> Oklch {
        oklch(0.6104, 0.05, 83.05)
    }

    pub(crate) fn dark_green(&self) -> Oklch {
        oklch(0.6046, 0.0518, 128.9)
    }

    pub(crate) fn dark_teal(&self) -> Oklch {
        oklch(0.5769, 0.0477, 179.31)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.5711, 0.0485, 208.7)
    }

    pub(crate) fn dark_purple(&self) -> Oklch {
        oklch(0.4368, 0.0476, 292.22)
    }

    pub(crate) fn dark_pink(&self) -> Oklch {
        oklch(0.5425, 0.0741, 355.62)
    }

}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    LightBg,
    LighterBg,
    DarkFg,
    DimFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::LightBg => 0.1,
            Self::LighterBg => 0.25,
            Self::DarkFg => 0.35,
            Self::DimFg => 0.6,
            Self::Fg => 0.85,
            Self::BrightFg => 1.0,
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
