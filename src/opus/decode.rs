
use ::CumulativeDistributionFrequency;

use opus::imported_decode;

use std;

pub struct Reader {
    source: Box<[u8]>,
    state: imported_decode::ec_dec,
}

impl Reader {
    pub fn from_boxed_slice(mut source: Box<[u8]>) -> Self {
        let state = unsafe {
            let mut state : imported_decode::ec_dec = std::mem::uninitialized();
            imported_decode::ec_dec_init(&mut state, source.as_mut_ptr(), source.len() as u32);
            state
        };
        Reader {
            source,
            state
        }
    }

    /// Decode the next symbol in line.
    pub fn symbol(&mut self, icdf: &CumulativeDistributionFrequency) -> Result<u32, std::io::Error> {
        let index = unsafe {
            let frequency = imported_decode::ec_decode(&mut self.state, icdf.width());
            self.check_status()?;
            let indexed= icdf.find(frequency)
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid probability"))?;
            imported_decode::ec_dec_update(&mut self.state, indexed.segment.low, indexed.segment.next, icdf.width());
            self.check_status()?;
            indexed.index
        };
        Ok(index as u32)
    }

    pub fn done(self) {
        // FIXME: Nothing to do?
    }

    unsafe fn check_status(&mut self) -> Result<(), std::io::Error> {
        let status = imported_decode::ec_get_error(&mut self.state);
        if status != 0 {
            unimplemented!()
        }
        Ok(())
    }
}