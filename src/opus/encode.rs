
use ::{ CumulativeDistributionFrequency };

use opus::imported_encode;

use std;

pub struct Writer {
    finalized: bool,
    // Note: Length is **invalid** until we call `done()`.
    destination: Vec<u8>,
    state: imported_encode::ec_enc,
}

impl Writer {
    // FIXME: The destination should really be a `std::io::Write`.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut destination = Vec::with_capacity(capacity);
        let state = unsafe {
            let mut state : imported_encode::ec_enc = std::mem::uninitialized();
            imported_encode::ec_enc_init(&mut state, destination.as_mut_ptr(), capacity as u32);
            state
        };
        Writer {
            destination,
            state,
            finalized: false,
        }
    }

    /// Encode the next symbol in line.
    pub fn symbol(&mut self, index: usize, icdf: &CumulativeDistributionFrequency) -> Result<(), std::io::Error> {
        let segment = icdf.at_index(index)
          .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid symbol"))?;
        unsafe {
            imported_encode::ec_encode(&mut self.state, segment.low, segment.next, icdf.width());
            self.check_status()?;
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

    pub fn done(mut self) -> Vec<u8> {
        debug_assert_eq!(self.finalized, false);
        self.finalized = true;
        let length = unsafe {
            imported_encode::ec_enc_done(&mut self.state);
            imported_encode::ec_range_bytes(&mut self.state)
        };
        let mut bytes : Vec<u8> = vec![];
        std::mem::swap(&mut bytes, &mut self.destination);
        unsafe {
            bytes.set_len(length as usize);
        }
        bytes
    }

    unsafe fn check_status(&mut self) -> Result<(), std::io::Error> {
        let status = imported_encode::ec_get_error(&mut self.state);
        if status != 0 {
            debug!(target: "opus", "Writer: check_status => {}", status);
            unimplemented!();
        }
        Ok(())
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        if self.finalized {
            return
        }
        unsafe {
            imported_encode::ec_enc_done(&mut self.state);
        }
    }
}