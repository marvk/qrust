use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use image::{ImageBuffer, Rgb, RgbImage};

pub use version::QrVersion;

mod alignment;
mod version;

struct QrRaw {
    pub version: QrVersion,
    bit_vec: BitVec,
}

impl From<QrRaw> for RgbImage {
    fn from(value: QrRaw) -> Self {
        ImageBuffer::from_fn(value.side_length().as_u32(), value.side_length().as_u32(), |x, y| {
            if value.get_unchecked(x as usize, y as usize) {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            }
        })
    }
}

impl QrRaw {
    pub fn new(version: QrVersion) -> Self {
        let len = version.side_length() * version.side_length();
        Self { version, bit_vec: bitvec![0; len] }
    }

    pub fn side_length(&self) -> usize {
        self.version.side_length()
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.side_length() + x
    }

    fn to_coords(&self, index: usize) -> (usize, usize) {
        let x = index % self.side_length();
        let y = index / self.side_length();
        (x, y)
    }

    fn draw_square(&mut self, x: usize, y: usize, n: usize, value: bool) {
        if n == 0 {
            return;
        }
        let n = n.saturating_sub(1);
        self.draw_horizontal_line(x, x + n, y, value);
        self.draw_horizontal_line(x, x + n, y + n, value);
        self.draw_vertical_line(x, y + 1, (y + n).saturating_sub(1), value);
        self.draw_vertical_line(x + n, y + 1, (y + n).saturating_sub(1), value);
    }

    fn draw_horizontal_line(&mut self, x1: usize, x2: usize, y: usize, value: bool) {
        for x in x1..=x2 {
            self.set(x, y, value);
        }
    }

    fn draw_vertical_line(&mut self, x: usize, y1: usize, y2: usize, value: bool) {
        for y in y1..=y2 {
            self.set(x, y, value);
        }
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        let i = self.to_index(x, y);
        self.bit_vec.set(i, value);
    }

    fn get_unchecked(&self, x: usize, y: usize) -> bool {
        self.bit_vec[self.to_index(x, y)]
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.bit_vec.get(self.to_index(x, y)).as_deref().copied()
    }
}

const FINDER_PATTER_SIZE: usize = 7;

#[allow(clippy::identity_op)]
fn qr_mask(version: QrVersion) -> (QrRaw, QrRaw) {
    let side_length = version.side_length();
    let mut function_patterns = QrRaw::new(version);
    let mut mask = QrRaw::new(version);

    for (x, y) in [(0, FINDER_PATTER_SIZE + 1), (side_length - FINDER_PATTER_SIZE - 2, FINDER_PATTER_SIZE + 1)] {
        function_patterns.draw_horizontal_line(x, x + FINDER_PATTER_SIZE + 1, y, false);
        mask.draw_horizontal_line(x, x + FINDER_PATTER_SIZE + 1, y, true);
    }

    for (x, y) in [(FINDER_PATTER_SIZE + 1, 0), (FINDER_PATTER_SIZE + 1, side_length - FINDER_PATTER_SIZE - 1)] {
        function_patterns.draw_horizontal_line(x, x + FINDER_PATTER_SIZE + 1, y, true);
        mask.draw_vertical_line(x, y, y + FINDER_PATTER_SIZE, true);
    }

    for c in 0..side_length {
        let value = c % 2 == 0;
        function_patterns.set(c, 6, value);
        function_patterns.set(6, c, value);

        mask.set(c, 6, true);
        mask.set(6, c, true);
    }

    for (x, y) in [(0, 0), (0, side_length - FINDER_PATTER_SIZE), (side_length - FINDER_PATTER_SIZE, 0)] {
        function_patterns.draw_square(x + 0, y + 0, 7, true);
        function_patterns.draw_square(x + 1, y + 1, 5, false);
        function_patterns.draw_square(x + 2, y + 2, 3, true);
        function_patterns.draw_square(x + 3, y + 3, 1, true);


        mask.draw_square(x + 0, y + 0, 7, true);
        mask.draw_square(x + 1, y + 1, 5, true);
        mask.draw_square(x + 2, y + 2, 3, true);
        mask.draw_square(x + 3, y + 3, 1, true);
    }

    for (x, y) in [(0, FINDER_PATTER_SIZE), (0, side_length - FINDER_PATTER_SIZE - 1), (side_length - FINDER_PATTER_SIZE - 1, FINDER_PATTER_SIZE)] {
        mask.draw_horizontal_line(x, x + FINDER_PATTER_SIZE, y, true);
    }

    for (x, y) in [(FINDER_PATTER_SIZE, 0), (side_length - FINDER_PATTER_SIZE - 1, 0), (FINDER_PATTER_SIZE, side_length - FINDER_PATTER_SIZE)] {
        mask.draw_vertical_line(x, y, y + FINDER_PATTER_SIZE - 1, true);
    }

    for &y in alignment::alignment_coords(version) {
        for &x in alignment::alignment_coords(version) {
            if ![(x - 2, y - 2), (x - 2, y + 2), (x + 2, y - 2), (x + 2, y + 2)].iter().any(|&(x_t, y_t)| mask.get_unchecked(x_t, y_t)) {
                function_patterns.draw_square(x - 0, y - 0, 1, true);
                function_patterns.draw_square(x - 1, y - 1, 3, false);
                function_patterns.draw_square(x - 2, y - 2, 5, true);

                mask.draw_square(x - 0, y - 0, 1, true);
                mask.draw_square(x - 1, y - 1, 3, true);
                mask.draw_square(x - 2, y - 2, 5, true);
            }
        }
    }

    (function_patterns, mask)
}

pub fn img() {
    let (mut function_patterns, mask) = qr_mask(QrVersion::V8);

    let mut buffer = RgbImage::from(function_patterns);

    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if !mask.get_unchecked(x as usize, y as usize) {
                buffer.put_pixel(x, y, Rgb([127, 127, 127]));
            }
        }
    }

    buffer.save("function_patterns.png").unwrap();
    RgbImage::from(mask).save("mask.png").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        img();
    }
}
