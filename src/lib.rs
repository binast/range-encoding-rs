// Used for opus_decode.
#![feature(extern_types)]

extern crate bitstream_io;
extern crate num_traits;

pub mod encode;
pub mod decode;

/// A c2rust-ified version of the Opus range decoder.
mod opus_decode;
mod opus_encode;

#[derive(Clone, Copy)]
pub struct Segment {
    /// First value part of the segment.
    pub low: u32,

    /// First value >= low **not** part of the segment.
    pub next: u32,
}

pub struct CumulativeDistributionFrequency {
    /// Ordered, non-empty, contiguous list of segments, starting at 0.
    segments: Box<[Segment]>,

    /// The width, which is exactly `segments.last.width`.
    width: u32,
}
impl CumulativeDistributionFrequency {
    // FIXME: Better errors
    pub fn new(mut segments: Vec<Segment>) -> Result<Self, ()> {
        if segments.len() == 0 {
            return Err(())
        }
        if segments[0].low != 0 {
            return Err(())
        }
        segments.sort_by(|a, b| {
            u32::cmp(&a.low, &b.low)
        });
        for i in 0 .. segments.len() - 1 {
            if segments[i].next != segments[i + 1].low {
                return Err(())
            }
        }
        let width = segments.last()
            .unwrap() // Checked when entering function.
            .next;
        Ok(Self {
            segments: segments.into_boxed_slice(),
            width
        })
    }

    /// Return the total frequency of symbols in this distribution.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Find a value from its frequency.
    pub fn find(&self, probability: u32) -> Option<(usize, Segment)> {
        if probability >= self.width {
            return None
        }
        let index = self.segments.binary_search_by(|segment| {
            use std::cmp::Ordering;
            if segment.low > probability {
                return Ordering::Greater
            }
            if segment.next <= probability {
                return Ordering::Less
            }
            Ordering::Equal
        }).ok()?;
        Some((index, self.segments[index]))
    }

    /// Find a value from its index
    pub fn at_index(&self, index: usize) -> Option<Segment> {
        self.segments.get(index)
            .map(Segment::clone)
    }
}