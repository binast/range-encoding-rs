
use ::{ CumulativeDistributionFrequency };

use opus::imported_encode;

use std;

pub struct Writer<W> where W: std::io::Write {
    state: imported_encode::ec_enc<W>,
}

impl<W> Writer<W> where W: std::io::Write {
    pub fn new(out: W) -> Self {
        Self {
            state: imported_encode::ec_enc {
                out,
                end_window: 0,
                nend_bits: 0,
                nbits_total: 33,
                offs: 0,
                rng: std::u32::MAX / 2 + 1,
                rem: -1,
                val: 0,
                ext: 0,
                end_buffer: vec![],
            }
        }
    }

    /// Encode the next symbol in line.
    pub fn symbol(&mut self, index: usize, icdf: &CumulativeDistributionFrequency) -> Result<(), std::io::Error> {
        let width = icdf.width();
        let segment = icdf.at_index(index).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid symbol")
        })?;
        unsafe {
            imported_encode::ec_encode(&mut self.state, segment.low, segment.next, width)?;
        };
        Ok(())
    }

/*
// FIXME: I actually don't understand `bits()` well enough
// to publish it.    /// Encode a sequence of raw bits, without any frequency information.
    pub fn bits(&mut self, bits: u16, size: usize) -> Result<(), std::io::Error> {
        unsafe {
            imported_encode::ec_enc_bits(&mut self.state,
                bits as u32,
                size as u32);
            self.check_status()?;
        }
        Ok(())
    }
*/

    /// Flush and return the underlying stream.
    pub fn done(mut self) -> Result<W, std::io::Error> {
        unsafe {
            imported_encode::ec_enc_done(&mut self.state)?;
        };
        Ok(self.state.out)
    }
}
