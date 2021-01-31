
#![feature(test)]
#![feature(box_syntax)]

extern crate test;
// extern crate jh;
// extern crate jhffi;

use rand::prelude::*;
use rand::Fill;
use jhffi;
use jh;
use test::{Bencher};

#[bench]
fn bench_hash_0200(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 200] = [0; 200];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result = [0; 32];
    b.iter(|| {
        jh::hash(256, &keccak_state, &mut result).unwrap();
    });
}

#[bench]
fn bench_hash_0200_ffi(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 200] = [0; 200];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result_jhffi = [0; 32];
    b.iter(|| {
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
    });
}

#[bench]
fn bench_hash_0400(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 400] = [0; 400];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result = [0; 32];
    b.iter(|| {
        jh::hash(256, &keccak_state, &mut result).unwrap();
    });
}

#[bench]
fn bench_hash_0400_ffi(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 400] = [0; 400];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result_jhffi = [0; 32];
    b.iter(|| {
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
    });
}

#[bench]
fn bench_hash_0800(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 800] = [0; 800];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result = [0; 32];
    b.iter(|| {
        jh::hash(256, &keccak_state, &mut result).unwrap();
    });
}

#[bench]
fn bench_hash_0800_ffi(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 800] = [0; 800];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result_jhffi = [0; 32];
    b.iter(|| {
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
    });
}

#[bench]
fn bench_hash_1600(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 1600] = [0; 1600];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result = [0; 32];
    b.iter(|| {
        jh::hash(256, &keccak_state, &mut result).unwrap();
    });
}

#[bench]
fn bench_hash_1600_ffi(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 1600] = [0; 1600];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result_jhffi = [0; 32];
    b.iter(|| {
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
    });
}

#[bench]
fn bench_hash_3200(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 3200] = [0; 3200];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result = [0; 32];
    b.iter(|| {
        jh::hash(256, &keccak_state, &mut result).unwrap();
    });
}

#[bench]
fn bench_hash_3200_ffi(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 3200] = [0; 3200];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result_jhffi = [0; 32];
    b.iter(|| {
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
    });
}

#[bench]
fn bench_hash_6400(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 6400] = [0; 6400];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result = [0; 32];
    b.iter(|| {
        jh::hash(256, &keccak_state, &mut result).unwrap();
    });
}

#[bench]
fn bench_hash_6400_ffi(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut keccak_state: [u8; 6400] = [0; 6400];
    keccak_state.try_fill(&mut rng).unwrap();
    let mut result_jhffi = [0; 32];
    b.iter(|| {
        jhffi::hash(256, &keccak_state, &mut result_jhffi).unwrap();
    });
}