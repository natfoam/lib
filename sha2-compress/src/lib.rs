use fixed_array::Array;
use uints::UInt;

pub struct BigSigma(u32, u32, u32);

impl BigSigma {
    fn get<I: UInt>(&self, v: I) -> I {
        v.ror(self.0) ^ v.ror(self.1) ^ v.ror(self.2)
    }
}

pub struct SmallSigma(u32, u32, u8);

impl SmallSigma {
    fn get<I: UInt>(&self, v: I) -> I {
        v.ror(self.0) ^ v.ror(self.1) ^ (v >> self.2)
    }
}

pub type Hash<I> = [I; 8];

pub trait Item: UInt + Copy + Sized {
    type W: Array<Output = Self>;
    const K: Self::W;
    const BIG_S0: BigSigma;
    const BIG_S1: BigSigma;
    const SMALL_S0: SmallSigma;
    const SMALL_S1: SmallSigma;
    fn w(a: &Hash<Self>, b: &Hash<Self>) -> Self::W;
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
    fn w(a: &Hash<Self>, b: &Hash<Self>) -> Self::W {
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
    fn w(a: &Hash<Self>, b: &Hash<Self>) -> Self::W {
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

pub trait Sha2 {
    fn compress(&self, h1: &Self, h2: &Self) -> Self;
}

impl<I: Item> Sha2 for Hash<I> {
    fn compress(&self, h1: &Hash<I>, h2: &Hash<I>) -> Hash<I> {
        let mut w = I::w(h1, h2);
        for i in 16..I::W::SIZE {
            w[i] = w[i - 16]
                .overflow_add(I::SMALL_S0.get(w[i - 15]))
                .overflow_add(w[i - 7])
                .overflow_add(I::SMALL_S1.get(w[i - 2]));
        }
        let mut a = self[0];
        let mut b = self[1];
        let mut c = self[2];
        let mut d = self[3];
        let mut e = self[4];
        let mut f = self[5];
        let mut g = self[6];
        let mut h = self[7];
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
            self[0].overflow_add(a),
            self[1].overflow_add(b),
            self[2].overflow_add(c),
            self[3].overflow_add(d),
            self[4].overflow_add(e),
            self[5].overflow_add(f),
            self[6].overflow_add(g),
            self[7].overflow_add(h),
        ]
    }
}

pub const SHA256: Hash<u32> = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

pub const SHA224: Hash<u32> = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
];

pub const SHA512: Hash<u64> = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

pub const SHA384: Hash<u64> = [
    0xcbbb9d5dc1059ed8,
    0x629a292a367cd507,
    0x9159015a3070dd17,
    0x152fecd8f70e5939,
    0x67332667ffc00b31,
    0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7,
    0x47b5481dbefa4fa4,
];

pub const SHA512_256: Hash<u64> = [
    0x22312194FC2BF72C,
    0x9F555FA3C84C64C2,
    0x2393B86B6F53B151,
    0x963877195940EABD,
    0x96283EE2A88EFFE3,
    0xBE5E1E2553863992,
    0x2B0199FC2C85B8AA,
    0x0EB72DDC81C52CA2,
];

pub const SHA512_224: Hash<u64> = [
    0x8C3D37C819544DA2,
    0x73E1996689DCD4D6,
    0x1DFAB7AE32FF9C82,
    0x679DD514582F9FCF,
    0x0F6D2B697BD44DA8,
    0x77E36F7304C48942,
    0x3F9D85A86A1D36C8,
    0x1112E6AD91D692A1,
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sha256() {
        let result = SHA256.compress(
            &[0x8000_0000, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            result,
            [
                0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, // 4
                0x27ae41e4, 0x649b934c, 0xa495991b, 0x7852b855, // 8
            ]
        );
    }
    #[test]
    fn sha224() {
        let result = SHA224.compress(
            &[0x8000_0000, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            result[0..7],
            [0xd14a028c, 0x2a3a2bc9, 0x476102bb, 0x288234c4, 0x15a2b01f, 0x828ea62a, 0xc5b3e42f]
        );
    }
    #[test]
    fn sha512() {
        let result = SHA512.compress(
            &[0x8000_0000_0000_0000, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            result,
            [
                0xcf83_e135_7eef_b8bd,
                0xf154_2850_d66d_8007,
                0xd620_e405_0b57_15dc,
                0x83f4_a921_d36c_e9ce,
                0x47d0_d13c_5d85_f2b0,
                0xff83_18d2_877e_ec2f,
                0x63b9_31bd_4741_7a81,
                0xa538_327a_f927_da3e,
            ]
        );
    }
    #[test]
    fn sha384() {
        let result = SHA384.compress(
            &[0x8000_0000_0000_0000, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            result[0..6],
            [
                0x38b0_60a7_51ac_9638,
                0x4cd9_327e_b1b1_e36a,
                0x21fd_b711_14be_0743,
                0x4c0c_c7bf_63f6_e1da,
                0x274e_debf_e76f_65fb,
                0xd51a_d2f1_4898_b95b,
            ]
        );
    }
    #[test]
    fn sha512_256() {
        let result = SHA512_256.compress(
            &[0x8000_0000_0000_0000, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            result[0..4],
            [
                0xc672_b8d1_ef56_ed28,
                0xab87_c362_2c51_1406,
                0x9bdd_3ad7_b8f9_7374,
                0x98d0_c01e_cef0_967a,
            ]
        );
    }
    #[test]
    fn sha512_224() {
        let result = SHA512_224.compress(
            &[0x8000_0000_0000_0000, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            result[0..3],
            [
                0x6ed0_dd02_806f_a89e,
                0x25de_060c_19d3_ac86,
                0xcabb_87d6_a0dd_d05c,
            ]
        );
        assert_eq!(
            result[3] >> 32,
            0x333b_84f4,
        );
    }
}
