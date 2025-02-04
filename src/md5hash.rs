/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

use serde::Serializer;
use std::convert::TryInto;

#[derive(Clone, Copy, Debug)]
pub enum Variant {
    Original,
    MD5F,
    MD5FC,
}

#[derive(Clone, Copy)]
struct RoundOp {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    k: usize,
    s: u32,
    i: u32,
    t: u32,
    op: fn(u32, u32, u32) -> u32,
}

#[inline(always)]
fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | ((!x) & z)
}
#[inline(always)]
fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & (!z))
}
#[inline(always)]
fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}
#[inline(always)]
fn i_func(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | (!z))
}

const BASE_ROUND_OPS: [RoundOp; 64] = [
    // Round 1 (function f)
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 0,
        s: 7,
        i: 1,
        t: 0xd76aa478,
        op: f,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 1,
        s: 12,
        i: 2,
        t: 0xe8c7b756,
        op: f,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 2,
        s: 17,
        i: 3,
        t: 0x242070db,
        op: f,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 3,
        s: 22,
        i: 4,
        t: 0xc1bdceee,
        op: f,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 4,
        s: 7,
        i: 5,
        t: 0xf57c0faf,
        op: f,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 5,
        s: 12,
        i: 6,
        t: 0x4787c62a,
        op: f,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 6,
        s: 17,
        i: 7,
        t: 0xa8304613,
        op: f,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 7,
        s: 22,
        i: 8,
        t: 0xfd469501,
        op: f,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 8,
        s: 7,
        i: 9,
        t: 0x698098d8,
        op: f,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 9,
        s: 12,
        i: 10,
        t: 0x8b44f7af,
        op: f,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 10,
        s: 17,
        i: 11,
        t: 0xffff5bb1,
        op: f,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 11,
        s: 22,
        i: 12,
        t: 0x895cd7be,
        op: f,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 12,
        s: 7,
        i: 13,
        t: 0x6b901122,
        op: f,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 13,
        s: 12,
        i: 14,
        t: 0xfd987193,
        op: f,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 14,
        s: 17,
        i: 15,
        t: 0xa679438e,
        op: f,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 15,
        s: 22,
        i: 16,
        t: 0x49b40821,
        op: f,
    },
    // Round 2 (g)
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 1,
        s: 5,
        i: 17,
        t: 0xf61e2562,
        op: g,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 6,
        s: 9,
        i: 18,
        t: 0xc040b340,
        op: g,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 11,
        s: 14,
        i: 19,
        t: 0x265e5a51,
        op: g,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 0,
        s: 20,
        i: 20,
        t: 0xe9b6c7aa,
        op: g,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 5,
        s: 5,
        i: 21,
        t: 0xd62f105d,
        op: g,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 10,
        s: 9,
        i: 22,
        t: 0x2441453,
        op: g,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 15,
        s: 14,
        i: 23,
        t: 0xd8a1e681,
        op: g,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 4,
        s: 20,
        i: 24,
        t: 0xe7d3fbc8,
        op: g,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 9,
        s: 5,
        i: 25,
        t: 0x21e1cde6,
        op: g,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 14,
        s: 9,
        i: 26,
        t: 0xc33707d6,
        op: g,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 3,
        s: 14,
        i: 27,
        t: 0xf4d50d87,
        op: g,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 8,
        s: 20,
        i: 28,
        t: 0x455a14ed,
        op: g,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 13,
        s: 5,
        i: 29,
        t: 0xa9e3e905,
        op: g,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 2,
        s: 9,
        i: 30,
        t: 0xfcefa3f8,
        op: g,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 7,
        s: 14,
        i: 31,
        t: 0x676f02d9,
        op: g,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 12,
        s: 20,
        i: 32,
        t: 0x8d2a4c8a,
        op: g,
    },
    // Round 3 (h)
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 5,
        s: 4,
        i: 33,
        t: 0xfffa3942,
        op: h,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 8,
        s: 11,
        i: 34,
        t: 0x8771f681,
        op: h,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 11,
        s: 16,
        i: 35,
        t: 0x6d9d6122,
        op: h,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 14,
        s: 23,
        i: 36,
        t: 0xfde5380c,
        op: h,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 1,
        s: 4,
        i: 37,
        t: 0xa4beea44,
        op: h,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 4,
        s: 11,
        i: 38,
        t: 0x4bdecfa9,
        op: h,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 7,
        s: 16,
        i: 39,
        t: 0xf6bb4b60,
        op: h,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 10,
        s: 23,
        i: 40,
        t: 0xbebfbc70,
        op: h,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 13,
        s: 4,
        i: 41,
        t: 0x289b7ec6,
        op: h,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 0,
        s: 11,
        i: 42,
        t: 0xeaa127fa,
        op: h,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 3,
        s: 16,
        i: 43,
        t: 0xd4ef3085,
        op: h,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 6,
        s: 23,
        i: 44,
        t: 0x4881d05,
        op: h,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 9,
        s: 4,
        i: 45,
        t: 0xd9d4d039,
        op: h,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 12,
        s: 11,
        i: 46,
        t: 0xe6db99e5,
        op: h,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 15,
        s: 16,
        i: 47,
        t: 0x1fa27cf8,
        op: h,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 2,
        s: 23,
        i: 48,
        t: 0xc4ac5665,
        op: h,
    },
    // Round 4 (i)
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 0,
        s: 6,
        i: 49,
        t: 0xf4292244,
        op: i_func,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 7,
        s: 10,
        i: 50,
        t: 0x432aff97,
        op: i_func,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 14,
        s: 15,
        i: 51,
        t: 0xab9423a7,
        op: i_func,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 5,
        s: 21,
        i: 52,
        t: 0xfc93a039,
        op: i_func,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 12,
        s: 6,
        i: 53,
        t: 0x655b59c3,
        op: i_func,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 3,
        s: 10,
        i: 54,
        t: 0x8f0ccc92,
        op: i_func,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 10,
        s: 15,
        i: 55,
        t: 0xffeff47d,
        op: i_func,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 1,
        s: 21,
        i: 56,
        t: 0x85845dd1,
        op: i_func,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 8,
        s: 6,
        i: 57,
        t: 0x6fa87e4f,
        op: i_func,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 15,
        s: 10,
        i: 58,
        t: 0xfe2ce6e0,
        op: i_func,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 6,
        s: 15,
        i: 59,
        t: 0xa3014314,
        op: i_func,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 13,
        s: 21,
        i: 60,
        t: 0x4e0811a1,
        op: i_func,
    },
    RoundOp {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        k: 4,
        s: 6,
        i: 61,
        t: 0xf7537e82,
        op: i_func,
    },
    RoundOp {
        a: 3,
        b: 0,
        c: 1,
        d: 2,
        k: 11,
        s: 10,
        i: 62,
        t: 0xbd3af235,
        op: i_func,
    },
    RoundOp {
        a: 2,
        b: 3,
        c: 0,
        d: 1,
        k: 2,
        s: 15,
        i: 63,
        t: 0x2ad7d2bb,
        op: i_func,
    },
    RoundOp {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
        k: 9,
        s: 21,
        i: 64,
        t: 0xeb86d391,
        op: i_func,
    },
];

const MD5F_PATCHES: &[(usize, RoundOp)] = &[
    (
        1,
        RoundOp {
            a: 3,
            b: 0,
            c: 1,
            d: 2,
            k: 1,
            s: 12,
            i: 2,
            t: 0xe8d7b756,
            op: f,
        },
    ),
    (
        6,
        RoundOp {
            a: 2,
            b: 3,
            c: 0,
            d: 1,
            k: 6,
            s: 17,
            i: 7,
            t: 0xa8304623,
            op: f,
        },
    ),
    (
        12,
        RoundOp {
            a: 0,
            b: 1,
            c: 2,
            d: 3,
            k: 12,
            s: 7,
            i: 13,
            t: 0x6b9f1122,
            op: f,
        },
    ),
    (
        15,
        RoundOp {
            a: 1,
            b: 2,
            c: 3,
            d: 0,
            k: 15,
            s: 22,
            i: 16,
            t: 0x39b40821,
            op: f,
        },
    ),
    (
        19,
        RoundOp {
            a: 1,
            b: 2,
            c: 3,
            d: 0,
            k: 0,
            s: 20,
            i: 20,
            t: 0xc9b6c7aa,
            op: g,
        },
    ),
    (
        21,
        RoundOp {
            a: 3,
            b: 0,
            c: 1,
            d: 2,
            k: 10,
            s: 9,
            i: 22,
            t: 0x2443453,
            op: g,
        },
    ),
    (
        24,
        RoundOp {
            a: 0,
            b: 1,
            c: 2,
            d: 3,
            k: 9,
            s: 5,
            i: 25,
            t: 0x21f1cde6,
            op: g,
        },
    ),
    (
        27,
        RoundOp {
            a: 1,
            b: 2,
            c: 3,
            d: 0,
            k: 8,
            s: 20,
            i: 28,
            t: 0x475a14ed,
            op: g,
        },
    ),
];

const MD5FC_PATCHES: &[(usize, RoundOp)] = &[
    (
        1,
        RoundOp {
            a: 3,
            b: 0,
            c: 1,
            d: 2,
            k: 1,
            s: 12,
            i: 2,
            t: 0xe8d7b756,
            op: f,
        },
    ),
    (
        3,
        RoundOp {
            a: 1,
            b: 2,
            c: 3,
            d: 0,
            k: 3,
            s: 22,
            i: 4,
            t: 0xc1bdceef,
            op: f,
        },
    ),
    (
        6,
        RoundOp {
            a: 2,
            b: 3,
            c: 0,
            d: 1,
            k: 6,
            s: 17,
            i: 7,
            t: 0xa8304623,
            op: f,
        },
    ),
    (
        12,
        RoundOp {
            a: 0,
            b: 1,
            c: 2,
            d: 3,
            k: 12,
            s: 7,
            i: 13,
            t: 0x6b9f1122,
            op: f,
        },
    ),
    (
        15,
        RoundOp {
            a: 1,
            b: 2,
            c: 3,
            d: 0,
            k: 15,
            s: 22,
            i: 16,
            t: 0x39b40821,
            op: f,
        },
    ),
    (
        19,
        RoundOp {
            a: 1,
            b: 2,
            c: 3,
            d: 0,
            k: 0,
            s: 20,
            i: 20,
            t: 0xc9b6c7aa,
            op: g,
        },
    ),
    (
        21,
        RoundOp {
            a: 3,
            b: 0,
            c: 1,
            d: 2,
            k: 10,
            s: 9,
            i: 22,
            t: 0x2443453,
            op: g,
        },
    ),
    (
        24,
        RoundOp {
            a: 0,
            b: 1,
            c: 2,
            d: 3,
            k: 9,
            s: 5,
            i: 25,
            t: 0x23f1cde6,
            op: g,
        },
    ),
    (
        27,
        RoundOp {
            a: 1,
            b: 2,
            c: 3,
            d: 0,
            k: 8,
            s: 20,
            i: 28,
            t: 0x475a14ed,
            op: g,
        },
    ),
    (
        34,
        RoundOp {
            a: 2,
            b: 3,
            c: 0,
            d: 1,
            k: 11,
            s: 16,
            i: 35,
            t: 0x6d9d6121,
            op: h,
        },
    ),
];

pub struct DysonSphereMD5 {
    state: [u32; 4],
    buffer: Vec<u8>,
    length: u64,
    digest: Option<[u8; 16]>,
    variant: Variant,
    is_finalized: bool,
}

#[derive(Debug)]
pub struct MD5NotFinalized;
impl ::std::fmt::Display for MD5NotFinalized {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.serialize_unit_struct("MD5NotFinalized")
    }
}

impl ::std::error::Error for MD5NotFinalized {}

#[cfg(debug_assertions)]
impl std::fmt::Debug for DysonSphereMD5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DysonSphereMD5")
            .field("state", &self.state)
            .field("digest", &self.digest)
            .field("variant", &self.variant)
            .field("length", &self.length)
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl DysonSphereMD5 {
    pub fn new(variant: Variant) -> Self {
        let state = match variant {
            Variant::Original => [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            Variant::MD5F | Variant::MD5FC => [0x67452301, 0xefdcab89, 0x98badcfe, 0x10325746],
        };
        Self {
            state,
            buffer: Vec::new(),
            length: 0,
            digest: None,
            variant,
            is_finalized: false,
        }
    }

    pub fn update(&mut self, data: &[u8]) -> &mut Self {
        if self.digest.is_some() {
            panic!("Digest already finalized");
        }
        self.length = self.length.wrapping_add(data.len() as u64);
        self.buffer.extend_from_slice(data);
        while self.buffer.len() >= 64 {
            let block = self.buffer.drain(..64).collect::<Vec<_>>();
            self.update_block(&block);
        }
        self
    }

    fn update_block(&mut self, block: &[u8]) {
        assert_eq!(block.len(), 64);
        let mut local_state = self.state;
        let x: [u32; 16] = std::array::from_fn(|i| {
            u32::from_le_bytes(block[i * 4..i * 4 + 4].try_into().unwrap())
        });
        for (i, &base_op) in BASE_ROUND_OPS.iter().enumerate() {
            let op = self.get_patch(i).unwrap_or(base_op);
            let a = local_state[op.a];
            let b = local_state[op.b];
            let c = local_state[op.c];
            let d = local_state[op.d];
            let sum = a
                .wrapping_add((op.op)(b, c, d))
                .wrapping_add(x[op.k])
                .wrapping_add(op.t);
            local_state[op.a] = b.wrapping_add(sum.rotate_left(op.s));
        }
        self.state[0] = self.state[0].wrapping_add(local_state[0]);
        self.state[1] = self.state[1].wrapping_add(local_state[1]);
        self.state[2] = self.state[2].wrapping_add(local_state[2]);
        self.state[3] = self.state[3].wrapping_add(local_state[3]);
    }

    fn get_patch(&self, index: usize) -> Option<RoundOp> {
        let patches = match self.variant {
            Variant::Original => return None,
            Variant::MD5F => MD5F_PATCHES,
            Variant::MD5FC => MD5FC_PATCHES,
        };
        for &(idx, op) in patches {
            if idx == index {
                return Some(op);
            }
        }
        None
    }

    pub fn finalize(&mut self) -> &mut Self {
        if self.digest.is_some() {
            return self;
        }
        let orig_length = self.length;
        let bit_len = orig_length.wrapping_mul(8);

        let r = self.buffer.len(); // 0 ≤ r < 64
        let pad_len = if r < 56 { 56 - r } else { 64 + 56 - r }; // (r + pad_len + 8) % 64 == 0

        let mut padding = Vec::with_capacity(pad_len + 8);
        padding.push(0x80);
        padding.extend(std::iter::repeat(0).take(pad_len - 1));
        padding.extend(&bit_len.to_le_bytes());

        self.buffer.extend_from_slice(&padding);

        let chunks = self.buffer.clone();
        let chunks = chunks.chunks(64);
        for chunk in chunks {
            self.update_block(chunk);
        }
        self.buffer.clear();

        let mut out = [0u8; 16];
        for (i, &s) in self.state.iter().enumerate() {
            out[i * 4..(i + 1) * 4].copy_from_slice(&s.to_le_bytes());
        }
        self.digest = Some(out);
        self.is_finalized = true;
        self
    }

    pub fn digest(&self) -> Result<[u8; 16], MD5NotFinalized> {
        if !self.is_finalized {
            Err(MD5NotFinalized)
        } else {
            Ok(self.digest.unwrap())
        }
    }

    pub fn hexdigest(&self) -> Result<String, MD5NotFinalized> {
        let digest = self.digest()?;
        let mut s = String::with_capacity(32);
        for byte in digest.iter() {
            use std::fmt::Write;
            write!(&mut s, "{:02x}", byte).unwrap();
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut md = DysonSphereMD5::new(Variant::MD5F);
        md.update(b"");
        md.finalize();
        assert_eq!(md.hexdigest().unwrap(), "84d1ce3bd68f49ab26eb0f96416617cf");
    }

    #[test]
    fn test_single_character() {
        let mut md = DysonSphereMD5::new(Variant::MD5F);
        md.update(b"a");
        md.finalize();
        assert_eq!(md.hexdigest().unwrap(), "f10bddaecb62e5a92433757867ee06db");
    }

    #[test]
    fn test_abcd() {
        let mut md = DysonSphereMD5::new(Variant::MD5F);
        md.update(b"abcd");
        md.finalize();
        assert_eq!(md.hexdigest().unwrap(), "fa27c78b6ec31559f0e760ce3f2b03f6");
    }

    #[test]
    fn test_long_string() {
        let mut md = DysonSphereMD5::new(Variant::MD5F);
        md.update(b"Some random words blablablablablabla");
        md.finalize();
        assert_eq!(md.hexdigest().unwrap(), "ffe3de11cdddb9ccecfef7089b420218");
    }
}
