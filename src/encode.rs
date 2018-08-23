
use ::CumulativeDistributionFrequency;

use opus_encode;

use std;

pub struct Writer {
    finalized: bool,
    destination: Box<[u8]>,
    state: opus_encode::ec_enc,
}

impl Writer {
    pub fn new(mut destination: Box<[u8]>) -> Self {
        let state = unsafe {
            let mut state : opus_encode::ec_enc = std::mem::uninitialized();
            opus_encode::ec_enc_init(&mut state, destination.as_mut_ptr(), destination.len() as u32);
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
            opus_encode::ec_encode(&mut self.state, segment.low, segment.next, icdf.width());
            self.check_status()?;
        };
        Ok(())
    }

    pub fn done(mut self) {
        debug_assert_eq!(self.finalized, false);
        self.finalized = true;
        unsafe {
            opus_encode::ec_enc_done(&mut self.state);
        }
    }

    unsafe fn check_status(&mut self) -> Result<(), std::io::Error> {
        let status = opus_encode::ec_get_error(&mut self.state);
        if status != 0 {
            unimplemented!()
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
            opus_encode::ec_enc_done(&mut self.state);
        }
    }
}