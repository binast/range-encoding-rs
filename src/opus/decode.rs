use CumulativeDistributionFrequency;

use opus::imported_decode;

use std;

pub struct Reader<R>
where
    R: std::io::Read,
{
    state: imported_decode::ec_dec<R>,
}

impl<R> Reader<R>
where
    R: std::io::Read,
{
    /*
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
    */
    pub fn new(input: R) -> Result<Self, std::io::Error> {
        let mut state = imported_decode::ec_dec {
            inp: input,
            // The rest will be initialized by `ec_dec_init`.
            end_window: 0,
            nend_bits: 0,
            nbits_total: 0,
            rng: 0,
            rem: 0,
            val: 0,
            ext: 0,
        };
        unsafe {
            imported_decode::ec_dec_init(&mut state)?;
        }
        Ok(Reader { state })
    }

    /// Decode the next symbol in line.
    pub fn symbol(
        &mut self,
        icdf: &CumulativeDistributionFrequency,
    ) -> Result<u32, std::io::Error> {
        let index = unsafe {
            let frequency = imported_decode::ec_decode(&mut self.state, icdf.width());
            let indexed = icdf.find(frequency).ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid probability")
            })?;
            imported_decode::ec_dec_update(
                &mut self.state,
                indexed.segment.low,
                indexed.segment.next,
                icdf.width(),
            )?;
            indexed.index
        };
        Ok(index as u32)
    }

    /*
    // FIXME: I actually don't understand `bits()` well enough
    // to publish it.    /// Encode a sequence of raw bits, without any frequency information.
        pub fn bits(&mut self, size: usize) -> Result<u16, std::io::Error> {
            let result = unsafe {
                let result = imported_decode::ec_dec_bits(&mut self.state,
                    size as u32);
                self.check_status()?;
                result as u16
            };
            Ok(result)
        }
    */

    pub fn done(self) {
        // FIXME: Nothing to do?
    }
}
