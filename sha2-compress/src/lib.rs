use uints::{Array, UInt};

struct BigSigma(u32, u32, u32);

impl BigSigma {
    fn get<I: UInt>(&self, v: I) -> I {
        v.ror(self.0) ^ v.ror(self.1) ^ v.ror(self.2)
    }
}

struct SmallSigma(u32, u32, u8);

impl SmallSigma {
    fn get<I: UInt>(&self, v: I) -> I {
        v.ror(self.0) ^ v.ror(self.1) ^ (v >> self.2)
    }
}

trait Item: UInt + Copy + Sized {
    type W: Array<Output = Self>;
    const K: Self::W;
    const BIG_S0: BigSigma;
    const BIG_S1: BigSigma;
    const SMALL_S0: SmallSigma;
    const SMALL_S1: SmallSigma;
    fn w(a: &[Self; 8], b: &[Self; 8]) -> Self::W;
}

impl Item for u32 {
    type W = [Self; 64];
    const K: Self::W = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, // 4
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, // 8
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, // 12
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, // 16
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, // 20
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, // 24
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, // 28
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, // 32
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, // 36
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, // 40
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, // 44
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, // 48
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, // 52
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, // 56
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, // 60
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2, // 64
    ];
    const BIG_S0: BigSigma = BigSigma(2, 13, 22);
    const BIG_S1: BigSigma = BigSigma(6, 11, 25);
    const SMALL_S0: SmallSigma = SmallSigma(7, 18, 3);
    const SMALL_S1: SmallSigma = SmallSigma(17, 19, 10);
    fn w(a: &[Self; 8], b: &[Self; 8]) -> Self::W {
        [
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], // a
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], // b
            0, 0, 0, 0, 0, 0, 0, 0, // 2
            0, 0, 0, 0, 0, 0, 0, 0, // 3
            0, 0, 0, 0, 0, 0, 0, 0, // 4
            0, 0, 0, 0, 0, 0, 0, 0, // 5
            0, 0, 0, 0, 0, 0, 0, 0, // 6
            0, 0, 0, 0, 0, 0, 0, 0, // 7
        ]
    }
}

impl Item for u64 {
    type W = [Self; 80];
    const K: Self::W = [
        0x428a2f98d728ae22,
        0x7137449123ef65cd,
        0xb5c0fbcfec4d3b2f,
        0xe9b5dba58189dbbc,
        0x3956c25bf348b538,
        0x59f111f1b605d019,
        0x923f82a4af194f9b,
        0xab1c5ed5da6d8118,
        0xd807aa98a3030242,
        0x12835b0145706fbe,
        0x243185be4ee4b28c,
        0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f,
        0x80deb1fe3b1696b1,
        0x9bdc06a725c71235,
        0xc19bf174cf692694,
        0xe49b69c19ef14ad2,
        0xefbe4786384f25e3,
        0x0fc19dc68b8cd5b5,
        0x240ca1cc77ac9c65,
        0x2de92c6f592b0275,
        0x4a7484aa6ea6e483,
        0x5cb0a9dcbd41fbd4,
        0x76f988da831153b5,
        0x983e5152ee66dfab,
        0xa831c66d2db43210,
        0xb00327c898fb213f,
        0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2,
        0xd5a79147930aa725,
        0x06ca6351e003826f,
        0x142929670a0e6e70,
        0x27b70a8546d22ffc,
        0x2e1b21385c26c926,
        0x4d2c6dfc5ac42aed,
        0x53380d139d95b3df,
        0x650a73548baf63de,
        0x766a0abb3c77b2a8,
        0x81c2c92e47edaee6,
        0x92722c851482353b,
        0xa2bfe8a14cf10364,
        0xa81a664bbc423001,
        0xc24b8b70d0f89791,
        0xc76c51a30654be30,
        0xd192e819d6ef5218,
        0xd69906245565a910,
        0xf40e35855771202a,
        0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8,
        0x1e376c085141ab53,
        0x2748774cdf8eeb99,
        0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63,
        0x4ed8aa4ae3418acb,
        0x5b9cca4f7763e373,
        0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc,
        0x78a5636f43172f60,
        0x84c87814a1f0ab72,
        0x8cc702081a6439ec,
        0x90befffa23631e28,
        0xa4506cebde82bde9,
        0xbef9a3f7b2c67915,
        0xc67178f2e372532b,
        0xca273eceea26619c,
        0xd186b8c721c0c207,
        0xeada7dd6cde0eb1e,
        0xf57d4f7fee6ed178,
        0x06f067aa72176fba,
        0x0a637dc5a2c898a6,
        0x113f9804bef90dae,
        0x1b710b35131c471b,
        0x28db77f523047d84,
        0x32caab7b40c72493,
        0x3c9ebe0a15c9bebc,
        0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6,
        0x597f299cfc657e2a,
        0x5fcb6fab3ad6faec,
        0x6c44198c4a475817,
    ];
    const BIG_S0: BigSigma = BigSigma(28, 34, 39);
    const BIG_S1: BigSigma = BigSigma(14, 18, 41);
    const SMALL_S0: SmallSigma = SmallSigma(1, 8, 7);
    const SMALL_S1: SmallSigma = SmallSigma(19, 61, 6);
    fn w(a: &[Self; 8], b: &[Self; 8]) -> Self::W {
        [
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], // a
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], // b
            0, 0, 0, 0, 0, 0, 0, 0, // 2
            0, 0, 0, 0, 0, 0, 0, 0, // 3
            0, 0, 0, 0, 0, 0, 0, 0, // 4
            0, 0, 0, 0, 0, 0, 0, 0, // 5
            0, 0, 0, 0, 0, 0, 0, 0, // 6
            0, 0, 0, 0, 0, 0, 0, 0, // 7
            0, 0, 0, 0, 0, 0, 0, 0, // 8
            0, 0, 0, 0, 0, 0, 0, 0, // 9
        ]
    }
}

fn compress<I: Item>(h0: &[I; 8], h1: &[I; 8], h2: &[I; 8]) -> [I; 8] {
    let mut w = I::w(h1, h2);
    for i in 16..I::W::SIZE {
        w[i] = w[i - 16]
            .overflow_add(I::SMALL_S0.get(w[i - 15]))
            .overflow_add(w[i - 7])
            .overflow_add(I::SMALL_S1.get(w[i - 2]));
    }
    let mut a = h0[0];
    let mut b = h0[1];
    let mut c = h0[2];
    let mut d = h0[3];
    let mut e = h0[4];
    let mut f = h0[5];
    let mut g = h0[6];
    let mut h = h0[7];
    for i in 0..I::W::SIZE {
        let big_s1 = I::BIG_S1.get(e);
        let ch = (e & f) ^ (!e & g);
        let temp1 = h
            .overflow_add(big_s1)
            .overflow_add(ch)
            .overflow_add(I::K[i])
            .overflow_add(w[i]);
        let big_s0 = I::BIG_S0.get(a);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = big_s0.overflow_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.overflow_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.overflow_add(temp2);
    }
    [
        h0[0].overflow_add(a),
        h0[1].overflow_add(b),
        h0[2].overflow_add(c),
        h0[3].overflow_add(d),
        h0[4].overflow_add(e),
        h0[5].overflow_add(f),
        h0[6].overflow_add(g),
        h0[7].overflow_add(h),
    ]
}

pub const SHA256: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = compress(
            &SHA256,
            &[0x80000000, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            result,
            [0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, 0x27ae41e4, 0x649b934c, 0xa495991b, 0x7852b855]
        );
    }
}
