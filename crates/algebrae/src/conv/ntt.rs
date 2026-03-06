use crate::{
    modular::{Gf, Gf32},
    num_theory::primitive_root,
};

macro_rules! impl_ntt {
    ($gf:ident, $ntt:ident, $intt:ident, $multiply:ident, $t:ty) => {
        /// Computes the Number Theoretic Transform over (Z/pZ, +) in place.
        /// Let w = g^((P-1)/n) and br is bit-reverse transform,
        /// a'(br(i)) = sum a(j) w^(ij).
        ///
        /// # Complexity
        /// Time: O(n log n), Space: O(1)
        pub fn $ntt<const P: $t>(a: &mut [$gf<P>]) {
            let n = a.len();
            debug_assert!(n.is_power_of_two(), "n must be power of two");
            debug_assert!(
                (P - 1) % n as $t == 0,
                "n must divide P - 1: no primitive n-th root of unity exists for n={}",
                n
            );

            let rank = (P - 1).trailing_zeros() as usize;
            let g = primitive_root(P as u64) as $t;
            let mut root: [std::mem::MaybeUninit<$gf<P>>; std::mem::size_of::<$t>() * 8] =
                unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            unsafe {
                let r = root.as_mut_ptr() as *mut $gf<P>;
                *r.add(rank) = $gf::new(g).pow(((P - 1) as u64) >> rank);
                for i in (0..rank).rev() {
                    *r.add(i) = *r.add(i + 1) * *r.add(i + 1);
                }
            }
            let root = root.as_ptr() as *const $gf<P>;

            unsafe {
                let ptr = a.as_mut_ptr();
                let mut m = n;
                while m > 1 {
                    let h = m >> 1;
                    let wm = *root.add(m.trailing_zeros() as usize);
                    for b in (0..n).step_by(m) {
                        let mut w = $gf::new(1);
                        for i in 0..h {
                            let u = *ptr.add(b + i);
                            let v = *ptr.add(b + i + h);
                            *ptr.add(b + i) = u + v;
                            *ptr.add(b + i + h) = (u - v) * w;
                            w *= wm;
                        }
                    }
                    m = h;
                }
            }
        }

        /// Computes the inverse Number Theoretic Transform over (Z/pZ, +) in place.
        /// Let w = g^((P-1)/n) and br is bit-reverse transform,
        /// a'(i) = sum a(br(j)) w^(-ij).
        ///
        /// # Complexity
        /// Time: O(n log n), Space: O(1)
        pub fn $intt<const P: $t>(a: &mut [$gf<P>]) {
            let n = a.len();
            debug_assert!(n.is_power_of_two(), "n must be power of two");
            debug_assert!(
                (P - 1) % n as $t == 0,
                "n must divide P - 1: no primitive n-th root of unity exists for n={}",
                n
            );

            let rank = (P - 1).trailing_zeros() as usize;
            let g = primitive_root(P as u64) as $t;
            let mut iroot: [std::mem::MaybeUninit<$gf<P>>; std::mem::size_of::<$t>() * 8] =
                unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            unsafe {
                let r = iroot.as_mut_ptr() as *mut $gf<P>;
                *r.add(rank) = $gf::new(g).pow(((P - 1) as u64) >> rank).inv();
                for i in (0..rank).rev() {
                    *r.add(i) = *r.add(i + 1) * *r.add(i + 1);
                }
            }
            let iroot = iroot.as_ptr() as *const $gf<P>;

            unsafe {
                let ptr = a.as_mut_ptr();
                let mut m = 2;
                while m <= n {
                    let h = m >> 1;
                    let wm = *iroot.add(m.trailing_zeros() as usize);
                    for b in (0..n).step_by(m) {
                        let mut w = $gf::new(1);
                        for i in 0..h {
                            let t = *ptr.add(b + i + h) * w;
                            *ptr.add(b + i + h) = *ptr.add(b + i) - t;
                            *ptr.add(b + i) = *ptr.add(b + i) + t;
                            w *= wm;
                        }
                    }
                    m <<= 1;
                }
            }
        }

        /// Computes convolution over (Z/pZ, +).
        ///
        /// # Complexity
        /// Time: O(n log n), Space: O(n), where n = a.len() + b.len().
        pub fn $multiply<const P: $t>(mut a: Vec<$gf<P>>, mut b: Vec<$gf<P>>) -> Vec<$gf<P>> {
            if a.is_empty() || b.is_empty() {
                return Vec::new();
            }
            let n = a.len() + b.len() - 1;
            let z = n.next_power_of_two();
            a.reserve(z - a.len());
            b.reserve(z - b.len());
            unsafe {
                std::ptr::write_bytes(a.as_mut_ptr().add(a.len()), 0, z - a.len());
                a.set_len(z);
                std::ptr::write_bytes(b.as_mut_ptr().add(b.len()), 0, z - b.len());
                b.set_len(z);
            }

            $ntt(&mut a);
            $ntt(&mut b);

            unsafe {
                let a = a.as_mut_ptr();
                let b = b.as_ptr();
                for i in 0..z {
                    *a.add(i) = *a.add(i) * *b.add(i);
                }
            }

            $intt(&mut a);

            let iz = $gf::new(z as $t).inv();
            unsafe {
                let a = a.as_mut_ptr();
                for i in 0..n {
                    *a.add(i) = *a.add(i) * iz;
                }
            }

            a.truncate(n);
            a
        }
    };
}

impl_ntt!(Gf32, ntt32, intt32, multiply32, u32);
impl_ntt!(Gf, ntt, intt, multiply, u64);
