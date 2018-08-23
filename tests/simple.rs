extern crate range_encoding;
extern crate rand;

use range_encoding::*;
use rand::*;

#[test]
fn probabilities_roundtrip() {
    let symbols = ['g', 'a', 't', 'c'];
    let probabilities = CumulativeDistributionFrequency::new(vec![1, 30, 5, 20])
        .unwrap();

    let test_with_sample = |sample: &str| {
        eprintln!("Writing...");

        let mut writer = opus::Writer::with_capacity(128);
        for c in sample.chars() {
            let index = symbols.iter()
                .cloned()
                .position(|x| x == c)
                .unwrap();
            writer.symbol(index, &probabilities)
                .expect("Could not write symbol");
        }
        let encoded = writer.done();
        eprintln!("Wrote {} bytes to {} bytes",
            sample.len(), encoded.len());

        eprintln!("Reading...");
        let mut reader = opus::Reader::from_boxed_slice(encoded.into_boxed_slice());
        for reference in sample.chars() {
            let index = reader.symbol(&probabilities)
                .expect("Could not find symbol");
            assert_eq!(symbols[index as usize], reference);
        }

        eprintln!("Reading succeeded");
    };

    test_with_sample("gattaca");

    let mut small_rng = rand::rngs::SmallRng::from_entropy();
    let larger_sample : String = (0..32)
        .map(|_| {
            let index = small_rng.gen_range(0, symbols.len());
            symbols[index]
        })
        .collect();
    test_with_sample(&larger_sample);
}

/*
#[test]
// FIXME: I actually don't understand `bits()` well enough
// to publish it.

fn bits_roundtrip() {
    eprintln!("Initializing sample");
    let mut buf = vec![];
    let mut small_rng = rand::rngs::SmallRng::from_entropy();
    let mut total_bits = 0;
    for _ in 0..small_rng.gen_range(20, 40) {
        let byte : u16 = small_rng.gen_range(0, std::u16::MAX);
        total_bits += small_rng.gen_range(1, 16);
        buf.push((byte, total_bits));
    }


    eprintln!("Writing...");
    let mut writer = opus::Writer::with_capacity(128);
    for (byte, bits) in &buf {
        eprintln!("Writing {} / {}", byte, bits);
        writer.bits(*byte, *bits)
            .expect("Could not write bits");
    }
    let data = writer.done();
    eprintln!("Wrote {} bytes to {}.", buf.len(), data.len());


    eprintln!("Reading...");
    let mut reader = opus::Reader::from_boxed_slice(data.into_boxed_slice());
    for i in 0 .. buf.len() {
        let (expected, bits) = buf[i].clone();
        let extracted = reader.bits(bits)
            .expect("Could not read bits");
        assert_eq!(expected, extracted);
    }
    eprintln!("Read complete")
}
*/