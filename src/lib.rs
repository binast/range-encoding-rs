extern crate libc;

pub mod opus {
    /// A c2rust-ified version of the Opus range decoder.
    mod imported_decode;
    mod imported_encode;

    mod encode;
    mod decode;

    pub use self::encode::Writer;
    pub use self::decode::Reader;
}


#[derive(Clone, Copy)]
pub struct Segment {
    /// First value part of the segment.
    pub low: u32,

    /// First value >= low **not** part of the segment.
    pub next: u32,
}

pub struct IndexedSegment {
    pub segment: Segment,
    pub index: usize,
}

pub struct CumulativeDistributionFrequency {
    /// Ordered, non-empty, contiguous list of segments, starting at 0.
    segments: Box<[Segment]>,

    /// The width, which is exactly `segments.last.width`.
    width: u32,
}
impl CumulativeDistributionFrequency {
    // FIXME: Better errors
    pub fn new(probabilities: Vec<u32>) -> Result<Self, ()> {
        if probabilities.len() == 0 {
            return Err(())
        }

        let mut segments = Vec::with_capacity(probabilities.len());
        let mut start = 0;
        for probability in probabilities {
            let next = start + probability;
            segments.push(Segment {
                low: start,
                next,
            });
            start = next;
        }
        Ok(Self {
            segments: segments
                .into_boxed_slice(),
            width: start,
        })
    }

    /// Return the total frequency of symbols in this distribution.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Find a value from its frequency.
    pub fn find(&self, probability: u32) -> Option<IndexedSegment> {
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
        Some(IndexedSegment {
            index,
            segment: self.segments[index]
        })
    }

    /// Find a value from its index
    pub fn at_index(&self, index: usize) -> Option<Segment> {
        self.segments.get(index)
            .map(|indexed| indexed.clone())
    }
}