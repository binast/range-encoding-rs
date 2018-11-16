extern crate libc;

/// Implementation of the Opus range encoder.
pub mod opus {
    /// A c2rust-ified version of the Opus range decoder.
    mod imported_decode;
    mod imported_encode;

    mod encode;
    mod decode;

    pub use self::encode::Writer;
    pub use self::decode::Reader;
}

/// Determine whether the symbol needs to be somehow
/// injected in a dictionary.
pub struct DefinitionRequirement {
    symbol: bool,
    distribution_total: bool,
}
impl DefinitionRequirement {
    pub fn symbol(&self) -> bool {
        self.symbol
    }
    pub fn distribution_total(&self) -> bool {
        self.distribution_total
    }
}


#[derive(Clone)]
pub struct Segment {
    /// First value part of the segment.
    low: u32,

    /// First value >= low **not** part of the segment.
    next: u32,

    /// `true` if we have already returned at least one instance of this Segment.
    already_encountered: bool,
}
impl Segment {
    pub fn new(low: u32, next: u32) -> Segment {
        Segment {
            low,
            next,
            already_encountered: false
        }
    }
    pub fn width(&self) -> u32 {
        self.next - self.low
    }
}

pub struct IndexedSegment {
    pub segment: Segment,
    pub index: usize,
}

pub struct CumulativeDistributionFrequency {
    /// Ordered, contiguous list of segments, starting at 0.
    segments: Box<[Segment]>,

    /// The width, which is exactly `segments.last.width`.
    width: u32,

    already_encountered: bool,
}
impl CumulativeDistributionFrequency {
    pub fn new(probabilities: Vec<u32>) -> Self {
        let mut segments = Vec::with_capacity(probabilities.len());
        let mut start = 0;
        for probability in probabilities {
            let next = start + probability;
            segments.push(Segment::new(start, next));
            start = next;
        }
        Self {
            segments: segments
                .into_boxed_slice(),
            width: start,
            already_encountered: false,
        }
    }

    /// Return the total frequency of symbols in this distribution.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Iterate through the widths of the symbols.
    pub fn widths<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        self.segments.iter()
            .map(Segment::width)
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
            segment: self.segments[index].clone()
        })
    }

    /// Find a value from its index
    pub fn at_index<'a>(&'a mut self, index: usize) -> Option<&'a mut Segment> {
        if index >= self.segments.len() {
            return None;
        }
        Some(&mut self.segments[index])
    }

    pub fn requirements_for_index(&self, index: usize) -> Option<DefinitionRequirement> {
        if index >= self.segments.len() {
            return None;
        }
        Some(DefinitionRequirement {
            distribution_total: !self.already_encountered,
            symbol: !self.segments[index].already_encountered
        })
    }

    /// Return the number of values in this CDF
    pub fn len(&self) -> usize {
        self.segments.len()
    }
}