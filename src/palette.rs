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
            base_lightness_range: 0.17..0.988,  // Ensure 0.988 is the brightest lightness
            low_lightness: 0.17,
            high_lightness: 0.988,  // Ensure colors using "high_lightness" get the correct brightness
            low_chroma: 0.032,
            medium_chroma: 0.07,
            high_chroma: 0.1,
        }
    }
    
}

impl Palette {
    pub(crate) fn chroma() -> Self {
        Self {
            low_chroma: 0.06,
            medium_chroma: 0.09,
            high_chroma: 0.11,
            high_lightness: 0.86, // allows pushing chroma a little higher
            ..Self::default()
        }
    }

    pub(crate) fn soft() -> Self {
        Self {
            base_lightness_range: 0.25..0.95,
            ..Self::default()
        }
    }

    pub(crate) fn soft_chroma() -> Self {
        Self {
            base_lightness_range: 0.25..0.95,
            ..Self::chroma()
        }
    }

    pub(crate) fn light() -> Self {
        Self {
            base_lightness_range: 1.0..0.2,
            low_lightness: 0.65,
            high_lightness: 0.55,
            low_chroma: 0.04,
            medium_chroma: 0.06,
            high_chroma: 0.08,
        }
    }

    pub(crate) fn light_chroma() -> Self {
        Self {
            low_chroma: 0.09,
            medium_chroma: 0.1,
            high_chroma: 0.12,
            ..Self::light()
        }
    }

    pub(crate) fn light_soft() -> Self {
        Self {
            base_lightness_range: 0.96..0.3,
            ..Self::light()
        }
    }

    pub(crate) fn light_soft_chroma() -> Self {
        Self {
            base_lightness_range: 0.96..0.3,
            ..Self::light_chroma()
        }
    }

    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(
            lerp(scale.value(), self.base_lightness_range.clone()),
            0.0,
            0.0,
        )
    }

    pub(crate) fn pink(&self) -> Oklch {
        // oklch(self.high_lightness, self.low_chroma, 0.0)
        oklch(0.77, 0.132, 358.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        // oklch(self.low_lightness, self.high_chroma, 30.0)
        oklch(0.688,  0.195, 20.6)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        // oklch(self.high_lightness, self.low_chroma, 105.0)
        oklch(0.891, 0.0919, 85.1)
    }

    pub(crate) fn green(&self) -> Oklch {
        // oklch(self.high_lightness, self.medium_chroma, 130.0)
        oklch(0.882, 0.101, 130.0)
    }

    pub(crate) fn light_green(&self) -> Oklch {
        // oklch(self.high_lightness, self.low_chroma, 130.0)
        oklch(0.939, 0.0601, 127.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        // oklch(self.low_lightness, self.high_chroma, 230.0)
        oklch(0.824, 0.0857, 204.0)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        // oklch(self.high_lightness, self.low_chroma, 240.0)
        oklch(0.906, 0.0455, 193.0)
    }

    pub(crate) fn lavender(&self) -> Oklch {
        // oklch(self.high_lightness, self.low_chroma, 285.0)
        oklch(0.788, 0.0282, 293.0)
    }

    pub(crate) fn magenta(&self) -> Oklch {
        // oklch(self.low_lightness, self.high_chroma, 330.0)
        oklch(0.877, 0.0595, 3.95)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.7467,  0.1308, 43.76)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.8341, 0.0889, 175.67)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.572, 0.084, 291.0)
    }

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

    pub(crate) fn eight(&self) -> Oklch {
        oklch(0.7658, 0.0108, 48.58)
    }

    pub(crate) fn white(&self) -> Oklch {
        oklch(0.9947, 0.0184, 113.33)
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
