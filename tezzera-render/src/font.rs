use fontdue::{Font, FontSettings};

pub struct FontCache {
    font: Font,
}

impl FontCache {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let font = Font::from_bytes(bytes, FontSettings::default())
            .expect("invalid font bytes");
        Self { font }
    }

    /// Try to load a system monospace font from common paths.
    pub fn system_mono() -> Option<Self> {
        let candidates = [
            // macOS — system fonts
            "/System/Library/Fonts/Menlo.ttc",
            "/System/Library/Fonts/Monaco.ttf",
            "/System/Library/Fonts/SFNSMono.ttf",
            "/System/Library/Fonts/Helvetica.ttc",
            "/System/Library/Fonts/Supplemental/Courier New.ttf",
            "/Library/Fonts/Arial.ttf",
            // Linux
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
            // Windows
            "C:\\Windows\\Fonts\\consola.ttf",
            "C:\\Windows\\Fonts\\arial.ttf",
        ];
        for path in &candidates {
            if let Ok(bytes) = std::fs::read(path) {
                return Some(Self::from_bytes(&bytes));
            }
        }
        None
    }

    /// Rasterize a single character at the given px size.
    /// Returns (metrics, coverage_bitmap) where coverage_bitmap is 1 byte per pixel, 0..255.
    pub fn rasterize(&self, c: char, px: f32) -> (fontdue::Metrics, Vec<u8>) {
        self.font.rasterize(c, px)
    }
}
