#![cfg_attr(not(feature = "std"), no_std)]

/// An enum that represents a PNM image
#[derive(Clone, Debug)]
pub enum PNMImage<'a> {
    /// Binary PPM (P6) image
    PPMBinary {
        /// The width of the image
        width: usize,
        /// The height of the image
        height: usize,
        /// The maximum pixel value of the image
        maximum_pixel: usize,
        /// The comment associated with the image
        comment: &'a str,
        /// The pixel data of the image
        pixel_data: &'a [u8],
    },
}

use PNMImage::*;

/// Error type that represents the different PNM parsing errors
#[derive(Debug)]
pub enum PNMError {
    /// The file is not in PNM format
    NotPNMFormat,
    /// The PNM format is not supported. Right now, only P6 is supported.
    UnsupportedPNMFormat,
    /// Error while parsing a UTF-8 encoded string
    UTF8Error,
    /// Error while parsing the image
    ParseError {
        /// The position of the error
        pos: usize,
        /// The byte that was encountered
        got: u8,
        /// Contextual information about the error
        ctx: &'static str,
    },
}

use PNMError::*;

impl<'a> PNMImage<'a> {

    /// Parses a PNM image from a byte array
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte array containing the PNM image data
    ///
    /// # Returns
    ///
    /// A Result object containing the parsed PNMImage if successful, otherwise a PNMError
    pub fn from_parse<const N: usize>(bytes: &'a [u8; N]) -> Result<Self, PNMError> {
        // magic number P6\n
        if bytes[0] != b'P' {
            return Err(NotPNMFormat);
        }
        match bytes[1] {
            b'1' ..= b'5' => return Err(UnsupportedPNMFormat),
            b'6' => (),
            _ => return Err(NotPNMFormat)
        }
        if bytes[2] != b'\n' {
            return Err(ParseError {
                pos: 2,
                got: bytes[2],
                ctx: "expected newline.",
            });
        }
        let mut idx = 3;

        // comments
        while bytes[idx] == b'#' {
            while bytes[idx] != b'\n' {
                idx += 1
            }
        }
        let comment = if idx == 3 {
            ""
        } else if let Ok(header) = core::str::from_utf8(&bytes[3..idx]) {
            header
        } else {
            return Err(UTF8Error);
        };
        idx += 1;

        macro_rules! parse_dec {
            ($stop:expr) => {{
                let mut acc = 0;
                while bytes[idx] != $stop {
                    if !bytes[idx].is_ascii_digit() {
                        return Err(ParseError {
                            pos: idx,
                            got: bytes[idx],
                            ctx: "expected digit.",
                        });
                    }
                    acc *= 10;
                    acc += (bytes[idx] - b'0') as usize;

                    idx += 1;
                }
                idx += 1;
                acc
            }};
        }

        // parse <width>SPC<height>\n
        let width = parse_dec!(b' ');
        let height = parse_dec!(b'\n');
        // parse <maximum_pixel>\n
        let maximum_pixel = parse_dec!(b'\n');

        // rest is raw data
        let pixel_data = &bytes[idx..N];

        Ok(Self::PPMBinary {
            width,
            height,
            maximum_pixel,
            comment,
            pixel_data,
        })
    }
}

impl PNMImage<'_> {
    /// Returns the width of the PNM image.
    pub fn width(&self) -> usize {
        let PPMBinary{width, ..} = *self;
        width
    }

    /// Returns the height of the PNM image.
    pub fn height(&self) -> usize {
        let PPMBinary{height, ..} = *self;
        height
    }

    /// Returns the maximum pixel value of the PNM image.
    pub fn maximum_pixel(&self) -> usize {
        let PPMBinary{maximum_pixel, ..} = *self;
        maximum_pixel
    }

    /// Returns the comment associated with the PNM image.
    pub fn comment(&self) -> &str {
        let PPMBinary{comment, ..} = *self;
        comment
    }

    /// Returns the raw pixel bytes data of the PNM image.
    fn pixel_data(&self) -> &[u8] {
        let PPMBinary{pixel_data, ..} = *self;
        pixel_data
    }

    /// Returns the RGB values of the pixel at the specified (x, y) coordinate.
    /// Returns `None` if the pixel is outside the bounds of the image.
    pub fn pixel_rgb(&self, x: usize, y: usize) -> Option<(u8, u8, u8)> {
        let idx = (x + y * self.width()) * 3;
        if idx >= self.pixel_data().len() {
            None
        } else {
            Some((
                self.pixel_data()[idx],
                self.pixel_data()[idx + 1],
                self.pixel_data()[idx + 2],
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let raw_img = include_bytes!("./binary.ppm");
        let ppm_img = PNMImage::from_parse(raw_img).unwrap();

        assert_eq!(ppm_img.comment(), "# Created by GIMP version 2.10.34 PNM plug-in");
        assert_eq!(ppm_img.width(), 64, "expecting image width 64");
        assert_eq!(ppm_img.height(), 64, "expecting image height 64");

        // overflows should be None
        assert_eq!(ppm_img.pixel_rgb(64, 63), None);

        // corners should be black
        assert_eq!(ppm_img.pixel_rgb(0, 0), Some((0,0,0)));
        assert_eq!(ppm_img.pixel_rgb(63, 63), Some((0,0,0)));
        assert_eq!(ppm_img.pixel_rgb(63, 0), Some((0,0,0)));
        assert_eq!(ppm_img.pixel_rgb(0, 63), Some((0,0,0)));

        // with an offset, they should be:
        // GREEN | RED | YELLOW
        // RED | WHITE | RED
        // CYAN | RED | BLUE
        assert_eq!(ppm_img.pixel_rgb(7, 7), Some((0,255,0)));
        assert_eq!(ppm_img.pixel_rgb(31, 7), Some((255,0,0)));
        assert_eq!(ppm_img.pixel_rgb(56, 7), Some((255,255,0)));

        assert_eq!(ppm_img.pixel_rgb(7, 31), Some((255,0,0)));
        assert_eq!(ppm_img.pixel_rgb(31, 31), Some((255,255,255)));
        assert_eq!(ppm_img.pixel_rgb(56, 31), Some((255,0,0)));

        assert_eq!(ppm_img.pixel_rgb(7, 56), Some((0,255,255)));
        assert_eq!(ppm_img.pixel_rgb(31, 56), Some((255,0,0)));
        assert_eq!(ppm_img.pixel_rgb(56, 56), Some((0,0,255)));
    }
}
