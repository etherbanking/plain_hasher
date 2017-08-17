#![feature(test)]

extern crate test;
extern crate plain_hasher;

use std::hash::Hasher;
use test::{Bencher, black_box};
use plain_hasher::PlainHasher;

#[bench]
fn u128_mul(b: &mut Bencher) {
	b.iter(|| {
		let n: u8 = black_box(100);
		(0..n).fold(PlainHasher::default(), |mut old, new| { 
			let bb = black_box([new; 32]);
			old.write(&bb as &[u8]);
			old
		});
	});
}
