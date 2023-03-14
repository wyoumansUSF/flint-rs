use flint_sys::{self, deps, fmpz::*, fmpz_mod::*, fmpz_mod_poly::*};
use rug::Integer;
use rug_fft;
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use std::cmp::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::mem::MaybeUninit;
use std::ops::*;

mod flint_rug_bridge;

/// A polynomial with modular integer coefficients.
///
/// That is, a member of `Z/nZ[X]`.
///
/// # Examples
///
/// See constructors:
///
///    * [`new`](fn.ModPoly.new)
///    * [`interpolate_from_mul_subgroup`](fn.ModPoly.interpolate_from_mul_subgroup)
///
pub struct ModPoly {
    raw: fmpz_mod_poly_struct,
    ctx: fmpz_mod_ctx,
    modulus: Integer,
}

impl ModPoly {
    /// A new polynomial, equal to zero.
    pub fn new(modulus: Integer) -> Self {
        unsafe {
            let mut raw = MaybeUninit::uninit();
            let mut ctx = MaybeUninit::uninit();
            let mut flint_modulus = flint_rug_bridge::int_to_fmpz(&modulus);
            fmpz_mod_ctx_init(ctx.as_mut_ptr(), &flint_modulus);
            fmpz_clear(&mut flint_modulus);
            let ctx = ctx.assume_init();
            fmpz_mod_poly_init(raw.as_mut_ptr(), &ctx as *const _ as *mut _);
            ModPoly {
                raw: raw.assume_init(),
                ctx,
                modulus,
            }
        }
    }

    /// A new polynomial, equal to `constant`.
    pub fn from_int(modulus: Integer, mut constant: Integer) -> Self {
        constant %= &modulus;
        let mut this = ModPoly::new(modulus);
        this.set_coefficient(0, &constant);
        this
    }

    /// A new polynomial, equal to zero, with room for `n` coefficients.
    pub fn with_capacity(modulus: Integer, n: usize) -> Self {
        unsafe {
            let mut raw = MaybeUninit::uninit();
            let mut flint_modulus = flint_rug_bridge::int_to_fmpz(&modulus);
            let mut ctx = MaybeUninit::uninit();
            fmpz_mod_ctx_init(ctx.as_mut_ptr(), &flint_modulus);
            fmpz_clear(&mut flint_modulus);
            let ctx = ctx.assume_init();
            fmpz_mod_poly_init2(
                raw.as_mut_ptr(),
                n as deps::slong,
                &ctx as *const _ as *mut _,
            );
            ModPoly {
                raw: raw.assume_init(),
                modulus,
                ctx,
            }
        }
    }

    /// Interpolate a polynomial which agrees with the given values over a multiplicative subgroup
    /// of the prime field with modulus `m`.
    ///
    /// Let `n` be a power of two and the order of the multiplicative subgroup generated by `w`
    /// modulo `m`. Let `ys` be a vector of values at `1`, `w`, `w^2`, ...
    ///
    /// Returns a polynomial `f`, such that for `i` in `0..n`, `f(w^i) = ys[i] mod m`.
    ///
    /// # Panics
    ///
    /// If `n` is not a power of two, or if `w` does not generate a subgroup of order `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rug_polynomial::*;
    /// use rug::Integer;
    ///
    /// let m = Integer::from(5);
    /// let w = Integer::from(2);
    /// let ys: Vec<Integer> = vec![2, 3, 0, 4].into_iter().map(Integer::from).collect();
    /// let p = ModPoly::interpolate_from_mul_subgroup(ys, m, &w);
    /// debug_assert_eq!(p.len(), 2);
    /// debug_assert_eq!(p.get_coefficient(0), Integer::from(1));
    /// debug_assert_eq!(p.get_coefficient(1), Integer::from(1));
    /// ```
    pub fn interpolate_from_mul_subgroup(mut ys: Vec<Integer>, m: Integer, w: &Integer) -> Self {
        rug_fft::cooley_tukey_radix_2_intt(&mut ys, &m, w);
        let mut p = ModPoly::with_capacity(m, ys.len());
        for (i, c) in ys.iter().enumerate() {
            p.set_coefficient(i, c);
        }
        p
    }

    /// Evaluate this polynomial over the multiplicative subgroup generated by `w`, of size `n`.
    ///
    /// Returns list of evaluations, over `{1, w, w^2, ... w^(2^n-1)}`.
    ///
    /// # Panics
    ///
    /// If `n` is not a power of two, or if `w` does not generate a subgroup of order `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rug_polynomial::*;
    /// use rug::Integer;
    ///
    /// let m = Integer::from(5);
    /// let w = Integer::from(2);
    /// let ys: Vec<Integer> = vec![2, 3, 0, 4].into_iter().map(Integer::from).collect();
    /// let mut p = ModPoly::new(m);
    /// p.set_coefficient_ui(0, 1);
    /// p.set_coefficient_ui(1, 1);
    /// let vs = p.evaluate_over_mul_subgroup(&Integer::from(2), 4);
    /// let vs: Vec<usize> = vs.into_iter().map(|i| i.to_usize().unwrap()).collect();
    /// debug_assert_eq!(vs, vec![2, 3, 0, 4]);
    /// ```
    pub fn evaluate_over_mul_subgroup(&self, w: &Integer, n: usize) -> Vec<Integer> {
        let mut cs: Vec<Integer> = (0..n)
            .into_iter()
            .map(|i| self.get_coefficient(i))
            .collect();
        rug_fft::cooley_tukey_radix_2_ntt(&mut cs, &self.modulus, w);
        cs
    }

    /// Returns the minimal-degree monic polynomial with the given roots.
    ///
    /// # Example
    ///
    /// ```
    /// use rug_polynomial::*;
    /// use rug::Integer;
    ///
    /// let p = ModPoly::with_roots(vec![0, 1].into_iter().map(Integer::from), &Integer::from(5));
    /// debug_assert_eq!(p.len(), 3);
    /// debug_assert_eq!(p.get_coefficient(0), Integer::from(0));
    /// debug_assert_eq!(p.get_coefficient(1), Integer::from(4));
    /// debug_assert_eq!(p.get_coefficient(2), Integer::from(1));
    /// ```
    pub fn with_roots(xs: impl IntoIterator<Item = Integer>, m: &Integer) -> Self {
        let mut ps = xs
            .into_iter()
            .map(|x| {
                let mut p = ModPoly::new(m.clone());
                p.set_coefficient_ui(1, 1);
                p.set_coefficient(0, &-x);
                p
            })
            .collect::<Vec<_>>();
        while ps.len() > 1 {
            for i in 0..(ps.len() / 2) {
                let back = ps.pop().unwrap();
                ps[i] *= &back;
            }
        }
        ps.pop().unwrap_or_else(|| {
            let mut p = ModPoly::new(m.clone());
            p.set_coefficient_ui(0, 1);
            p
        })
    }

    /// Reallocates the polynomial to have room for `n` coefficients. Truncates the polynomial if
    /// it has more than `n` coefficients.
    pub fn reserve(&mut self, n: usize) {
        unsafe {
            fmpz_mod_poly_realloc(&mut self.raw, n as deps::slong, &mut self.ctx);
        }
    }

    /// Evaluate the polynomial at the given input.
    ///
    /// # Example
    ///
    /// ```
    /// use rug_polynomial::*;
    /// use rug::Integer;
    ///
    /// let p = ModPoly::with_roots(vec![0, 1].into_iter().map(Integer::from), &Integer::from(5));
    /// let y = p.evaluate(&Integer::from(3));
    /// debug_assert_eq!(y, Integer::from(1));
    /// ```
    pub fn evaluate(&self, i: &Integer) -> Integer {
        unsafe {
            let mut in_ = flint_rug_bridge::int_to_fmpz(i);

            let mut out = fmpz::default();
            fmpz_init(&mut out);
            fmpz_mod_poly_evaluate_fmpz(
                &mut out,
                &self.raw as *const _ as *mut _,
                &mut in_,
                &self.ctx as *const _ as *mut _,
            );

            let out_rug = flint_rug_bridge::fmpz_to_int(&out);
            fmpz_clear(&mut in_);
            fmpz_clear(&mut out);
            out_rug
        }
    }

    /// Get the modulus of this polynomial.
    pub fn modulus(&self) -> &Integer {
        &self.modulus
    }

    /// Get the `i`th coefficient
    pub fn get_coefficient(&self, i: usize) -> Integer {
        unsafe {
            let mut c = fmpz::default();
            fmpz_init(&mut c);
            fmpz_mod_poly_get_coeff_fmpz(
                &mut c,
                &self.raw as *const _ as *mut _,
                i as deps::slong,
                &self.ctx as *const _ as *mut _,
            );
            let c_gmp = flint_rug_bridge::fmpz_to_int(&c);
            fmpz_clear(&mut c);
            c_gmp % &self.modulus
        }
    }

    /// Set the `i`th coefficient to be `c`
    pub fn set_coefficient(&mut self, i: usize, c: &Integer) {
        unsafe {
            let mut c_flint = flint_rug_bridge::int_to_fmpz(c);
            fmpz_mod_poly_set_coeff_fmpz(
                &mut self.raw,
                i as deps::slong,
                &mut c_flint,
                &mut self.ctx,
            );
            fmpz_clear(&mut c_flint);
        }
    }

    /// Set the `i`th coefficient to be `c`
    pub fn set_coefficient_ui(&mut self, i: usize, c: usize) {
        unsafe {
            fmpz_mod_poly_set_coeff_ui(
                &mut self.raw,
                i as deps::slong,
                c as deps::ulong,
                &mut self.ctx,
            );
        }
    }

    /// The number of coefficients in the polynomial. One more than the degree.
    pub fn len(&self) -> usize {
        unsafe {
            fmpz_mod_poly_length(
                &self.raw as *const _ as *mut _,
                &self.ctx as *const _ as *mut _,
            ) as usize
        }
    }

    /// `self = -self`
    pub fn neg(&mut self) {
        unsafe {
            fmpz_mod_poly_neg(&mut self.raw, &mut self.raw, &mut self.ctx);
        }
    }

    /// `self = self + other`
    pub fn add(&mut self, other: &Self) {
        assert_eq!(self.modulus, other.modulus);
        unsafe {
            fmpz_mod_poly_add(
                &mut self.raw,
                &mut self.raw,
                &other.raw as *const _ as *mut _,
                &mut self.ctx,
            );
        }
    }

    /// `self = self - other`
    pub fn sub(&mut self, other: &Self) {
        assert_eq!(self.modulus, other.modulus);
        unsafe {
            fmpz_mod_poly_sub(
                &mut self.raw,
                &mut self.raw,
                &other.raw as *const _ as *mut _,
                &mut self.ctx,
            );
        }
    }

    /// `self = other - self`
    pub fn sub_from(&mut self, other: &Self) {
        assert_eq!(self.modulus, other.modulus);
        unsafe {
            fmpz_mod_poly_sub(
                &mut self.raw,
                &other.raw as *const _ as *mut _,
                &mut self.raw,
                &mut self.ctx,
            );
        }
    }

    /// `self = self * other`
    pub fn mul(&mut self, other: &Self) {
        assert_eq!(self.modulus, other.modulus);
        unsafe {
            fmpz_mod_poly_mul(
                &mut self.raw,
                &mut self.raw,
                &other.raw as *const _ as *mut _,
                &mut self.ctx,
            );
        }
    }

    /// Find `q` and `r` such that `self = other * q + r` and `r` has degree less than `other`.
    ///
    /// ## Returns
    ///
    /// `(q, r)`
    pub fn divrem(&self, other: &Self) -> (ModPoly, ModPoly) {
        assert_eq!(self.modulus, other.modulus);
        let mut q = ModPoly::new(self.modulus.clone());
        let mut r = ModPoly::new(self.modulus.clone());
        unsafe {
            fmpz_mod_poly_divrem(
                &mut q.raw,
                &mut r.raw,
                &self.raw as *const _ as *mut _,
                &other.raw as *const _ as *mut _,
                &self.ctx as *const _ as *mut _,
            );
        }
        (q, r)
    }

    /// `self = self / other`
    pub fn div(&mut self, other: &Self) {
        assert_eq!(self.modulus, other.modulus);
        let mut r = ModPoly::new(self.modulus.clone());
        unsafe {
            fmpz_mod_poly_divrem(
                &mut self.raw,
                &mut r.raw,
                &mut self.raw,
                &other.raw as *const _ as *mut _,
                &mut self.ctx,
            );
        }
    }

    /// `self = other / self`
    pub fn div_from(&mut self, other: &Self) {
        assert_eq!(self.modulus, other.modulus);
        let mut r = ModPoly::new(self.modulus.clone());
        unsafe {
            fmpz_mod_poly_divrem(
                &mut self.raw,
                &mut r.raw,
                &other.raw as *const _ as *mut _,
                &mut self.raw,
                &mut self.ctx,
            );
        }
    }

    /// `self = self % other`
    pub fn rem(&mut self, other: &Self) {
        assert_eq!(self.modulus, other.modulus);
        let mut q = ModPoly::new(self.modulus.clone());
        unsafe {
            fmpz_mod_poly_divrem(
                &mut q.raw,
                &mut self.raw,
                &mut self.raw,
                &other.raw as *const _ as *mut _,
                &mut self.ctx,
            );
        }
    }

    /// `self = other % self`
    pub fn rem_from(&mut self, other: &Self) {
        assert_eq!(self.modulus, other.modulus);
        let mut q = ModPoly::new(self.modulus.clone());
        unsafe {
            fmpz_mod_poly_divrem(
                &mut q.raw,
                &mut self.raw,
                &other.raw as *const _ as *mut _,
                &mut self.raw,
                &mut self.ctx,
            );
        }
    }

    /// `self = self * self`
    pub fn sqr(&mut self) {
        unsafe {
            fmpz_mod_poly_sqr(&mut self.raw, &mut self.raw, &mut self.ctx);
        }
    }

    /// From `(a, b)`, returns `(g, s, t)` such that `g | a`, `g | b` and `g = a*s + b*t`.
    pub fn xgcd(&self, other: &Self) -> (Self, Self, Self) {
        assert_eq!(self.modulus, other.modulus);
        let mut g = ModPoly::new(self.modulus.clone());
        let mut s = ModPoly::new(self.modulus.clone());
        let mut t = ModPoly::new(self.modulus.clone());
        unsafe {
            fmpz_mod_poly_xgcd(
                &mut g.raw,
                &mut s.raw,
                &mut t.raw,
                &self.raw as *const _ as *mut _,
                &other.raw as *const _ as *mut _,
                &self.ctx as *const _ as *mut _,
            );
        }
        (g, s, t)
    }

    /// Give the formal derivative of `self`.
    pub fn derivative(&self) -> Self {
        let mut d_self = ModPoly::new(self.modulus.clone());
        unsafe {
            fmpz_mod_poly_derivative(
                &mut d_self.raw,
                &self.raw as *const _ as *mut _,
                &self.ctx as *const _ as *mut _,
            );
        }
        d_self
    }
}

impl Clone for ModPoly {
    fn clone(&self) -> Self {
        let mut this = ModPoly::new(self.modulus.clone());
        unsafe {
            fmpz_mod_poly_set(
                &mut this.raw,
                &self.raw as *const _ as *mut _,
                &self.ctx as *const _ as *mut _,
            );
        }
        this
    }
}

impl Drop for ModPoly {
    fn drop(&mut self) {
        unsafe {
            fmpz_mod_poly_clear(&mut self.raw, &mut self.ctx);
            fmpz_mod_ctx_clear(&mut self.ctx);
        }
    }
}

impl PartialEq<ModPoly> for ModPoly {
    fn eq(&self, other: &ModPoly) -> bool {
        unsafe {
            fmpz_mod_poly_equal(
                &self.raw as *const _ as *mut _,
                &other.raw as *const _ as *mut _,
                &self.ctx as *const _ as *mut _,
            ) != 0
        }
    }
}
impl Eq for ModPoly {}

impl Debug for ModPoly {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("ModPoly")
            .field("modulus", &self.modulus)
            .field(
                "coefficients",
                &(0..self.len())
                    .map(|i| self.get_coefficient(i))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl Display for ModPoly {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let n = self.len();
        let mut first = true;
        for i in 0..n {
            let j = n - i - 1;
            let c = self.get_coefficient(j);
            if c != 0 {
                if !first {
                    write!(f, " + ")?;
                }
                write!(f, "{}", c)?;
                if j != 0 {
                    write!(f, "x^{}", j)?;
                }
                first = false;
            }
        }
        if first {
            write!(f, "0")?;
        }
        Ok(())
    }
}

macro_rules! impl_self_binary {
    ($Big:ty,
     $func:ident,
     $from_func:ident,
     $Trait:ident { $method:ident },
     $TraitAssign:ident { $method_assign:ident }
    ) => {
        // Big + Big
        impl $Trait<$Big> for $Big {
            type Output = $Big;
            #[inline]
            fn $method(mut self, rhs: $Big) -> $Big {
                self.$method_assign(rhs);
                self
            }
        }
        // Big + &Big
        impl $Trait<&$Big> for $Big {
            type Output = $Big;
            #[inline]
            fn $method(mut self, rhs: &$Big) -> $Big {
                self.$method_assign(rhs);
                self
            }
        }
        // &Big + Big
        impl $Trait<$Big> for &$Big {
            type Output = $Big;
            #[inline]
            fn $method(self, mut rhs: $Big) -> $Big {
                <$Big>::$from_func(&mut rhs, self);
                rhs
            }
        }
        // Big += Big
        impl $TraitAssign<$Big> for $Big {
            #[inline]
            fn $method_assign(&mut self, rhs: $Big) {
                <$Big>::$func(self, &rhs)
            }
        }
        // Big += &Big
        impl $TraitAssign<&$Big> for $Big {
            #[inline]
            fn $method_assign(&mut self, rhs: &$Big) {
                <$Big>::$func(self, rhs)
            }
        }
    };
}

macro_rules! impl_int_binary {
    ($Big:ty,
     $Base:ty,
     $func:ident,
     $from_func:ident,
     $lift_func:ident,
     $Trait:ident { $method:ident },
     $TraitAssign:ident { $method_assign:ident }
    ) => {
        // Big + &Base
        impl $Trait<$Base> for $Big {
            type Output = $Big;
            #[inline]
            fn $method(mut self, rhs: $Base) -> $Big {
                let rhs = <$Big>::$lift_func(self.modulus.clone(), rhs);
                <$Big>::$func(&mut self, &rhs);
                self
            }
        }
        // &Base + Big
        impl $Trait<$Big> for $Base {
            type Output = $Big;
            #[inline]
            fn $method(self, mut rhs: $Big) -> $Big {
                let lhs = <$Big>::$lift_func(rhs.modulus.clone(), self);
                <$Big>::$from_func(&mut rhs, &lhs);
                rhs
            }
        }
        // Big += &Base
        impl $TraitAssign<$Base> for $Big {
            #[inline]
            fn $method_assign(&mut self, rhs: $Base) {
                let rhs = <$Big>::$lift_func(self.modulus.clone(), rhs);
                <$Big>::$func(self, &rhs)
            }
        }
    };
}

impl_self_binary!(ModPoly, add, add, Add { add }, AddAssign { add_assign });
impl_int_binary!(
    ModPoly,
    Integer,
    add,
    add,
    from_int,
    Add { add },
    AddAssign { add_assign }
);
impl_self_binary!(
    ModPoly,
    sub,
    sub_from,
    Sub { sub },
    SubAssign { sub_assign }
);
impl_int_binary!(
    ModPoly,
    Integer,
    sub,
    sub_from,
    from_int,
    Sub { sub },
    SubAssign { sub_assign }
);
impl_self_binary!(ModPoly, mul, mul, Mul { mul }, MulAssign { mul_assign });
impl_int_binary!(
    ModPoly,
    Integer,
    mul,
    mul,
    from_int,
    Mul { mul },
    MulAssign { mul_assign }
);
impl_self_binary!(
    ModPoly,
    div,
    div_from,
    Div { div },
    DivAssign { div_assign }
);
impl_int_binary!(
    ModPoly,
    Integer,
    div,
    div_from,
    from_int,
    Div { div },
    DivAssign { div_assign }
);
impl_self_binary!(
    ModPoly,
    rem,
    rem_from,
    Rem { rem },
    RemAssign { rem_assign }
);
impl_int_binary!(
    ModPoly,
    Integer,
    rem,
    rem_from,
    from_int,
    Rem { rem },
    RemAssign { rem_assign }
);

use std::convert::From;

/// Serializable modular polynomial
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModPolySer {
    pub modulus: Integer,
    pub coefficients: Vec<Integer>,
}

impl From<ModPolySer> for ModPoly {
    fn from(other: ModPolySer) -> ModPoly {
        let mut inner = ModPoly::new(other.modulus.clone());
        for (i, c) in other.coefficients.into_iter().enumerate() {
            inner.set_coefficient(i, &c);
        }
        inner
    }
}

impl From<&ModPoly> for ModPolySer {
    fn from(other: &ModPoly) -> ModPolySer {
        let modulus = other.modulus().clone();
        let coefficients = (0..(other.len()))
            .into_iter()
            .map(|i| other.get_coefficient(i).clone())
            .collect();
        ModPolySer {
            modulus,
            coefficients,
        }
    }
}

impl Serialize for ModPoly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ModPolySer::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ModPoly {
    fn deserialize<D>(deserializer: D) -> Result<ModPoly, D::Error>
    where
        D: Deserializer<'de>,
    {
        ModPolySer::deserialize(deserializer).map(ModPoly::from)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck;
    use quickcheck_macros;

    #[test]
    fn init() {
        let p = Integer::from(17);
        let f = ModPoly::new(p);
        assert_eq!(f.len(), 0);
    }

    #[test]
    fn from_const() {
        let p = Integer::from(17);
        let f = ModPoly::from_int(p, Integer::from(0));
        assert_eq!(f.len(), 0);
        assert_eq!(f.evaluate(&Integer::from(0)), Integer::from(0));
    }

    #[test]
    fn just_set() {
        let p = Integer::from(17);
        let mut f = ModPoly::new(p);
        f.set_coefficient_ui(0, 1);
        assert_eq!(f.len(), 1);
        f.set_coefficient_ui(5, 1);
        assert_eq!(f.len(), 6);
        f.set_coefficient_ui(5, 0);
        assert_eq!(f.len(), 1);
    }

    #[test]
    fn set_get() {
        let p = Integer::from(17);
        let mut f = ModPoly::new(p);
        f.set_coefficient_ui(0, 1);
        assert_eq!(f.get_coefficient(0), Integer::from(1));
        f.set_coefficient(5, &Integer::from(5));
        for i in 1..5 {
            assert_eq!(f.get_coefficient(i), Integer::from(0));
        }
        assert_eq!(f.get_coefficient(5), Integer::from(5));
    }

    #[test]
    fn add() {
        let p = Integer::from(17);
        let mut f = ModPoly::new(p.clone());
        f.set_coefficient_ui(0, 1);
        let mut g = ModPoly::new(p);
        g.set_coefficient_ui(3, 1);
        let h = f.clone() + g.clone();
        assert_eq!(h.get_coefficient(0), Integer::from(1));
        assert_eq!(h.get_coefficient(1), Integer::from(0));
        assert_eq!(h.get_coefficient(2), Integer::from(0));
        assert_eq!(h.get_coefficient(3), Integer::from(1));
        assert_eq!(h.len(), 4);
        assert_eq!(h, f.clone() + &g);
        assert_eq!(h, &f + g.clone());
        assert_eq!(h, g.clone() + Integer::from(1));
        assert_eq!(h, Integer::from(1) + g.clone());
    }

    #[test]
    fn sub() {
        let p = Integer::from(17);
        let mut f = ModPoly::new(p.clone());
        f.set_coefficient_ui(0, 1);
        let mut g = ModPoly::new(p);
        g.set_coefficient_ui(3, 1);
        let h = f.clone() - g.clone();
        assert_eq!(h.get_coefficient(0), Integer::from(1));
        assert_eq!(h.get_coefficient(1), Integer::from(0));
        assert_eq!(h.get_coefficient(2), Integer::from(0));
        assert_eq!(h.get_coefficient(3), Integer::from(16));
        assert_eq!(h.len(), 4);
        assert_eq!(h, f.clone() - &g);
        assert_eq!(h, &f - g.clone());
        assert_eq!(h, Integer::from(1) - g.clone());
    }

    #[test]
    fn mul() {
        let p = Integer::from(17);
        let mut f = ModPoly::new(p.clone());
        f.set_coefficient_ui(1, 2);
        let mut g = ModPoly::new(p);
        g.set_coefficient_ui(3, 1);
        let h = f.clone() * g.clone();
        assert_eq!(h.get_coefficient(0), Integer::from(0));
        assert_eq!(h.get_coefficient(1), Integer::from(0));
        assert_eq!(h.get_coefficient(2), Integer::from(0));
        assert_eq!(h.get_coefficient(3), Integer::from(0));
        assert_eq!(h.get_coefficient(4), Integer::from(2));
        assert_eq!(h.len(), 5);
        assert_eq!(h, f.clone() * &g);
        assert_eq!(h, &f * g.clone());
        assert_eq!(h, h.clone() * Integer::from(1));
        assert_eq!(h, Integer::from(1) * h.clone());
    }
    #[test]
    fn mul_wrap() {
        let p = Integer::from(17);
        let mut g = ModPoly::new(p);
        g.set_coefficient_ui(3, 1);
        g.set_coefficient_ui(0, 5);
        let h = g.clone() * Integer::from(4);
        assert_eq!(h.get_coefficient(0), Integer::from(3));
        assert_eq!(h.get_coefficient(1), Integer::from(0));
        assert_eq!(h.get_coefficient(2), Integer::from(0));
        assert_eq!(h.get_coefficient(3), Integer::from(4));
        assert_eq!(h.len(), 4);
    }

    #[test]
    fn div() {
        let p = Integer::from(17);
        let mut f = ModPoly::new(p.clone());
        f.set_coefficient_ui(1, 1);
        let mut g = ModPoly::new(p);
        g.set_coefficient_ui(3, 1);
        let h = g.clone() / f.clone();
        assert_eq!(h.get_coefficient(0), Integer::from(0));
        assert_eq!(h.get_coefficient(1), Integer::from(0));
        assert_eq!(h.get_coefficient(2), Integer::from(1));
        assert_eq!(h.len(), 3);
        assert_eq!(h, g.clone() / &f);
        assert_eq!(h, &g / f.clone());
        assert_eq!(h, h.clone() / Integer::from(1));
    }

    fn test_interpolate_from_mul_subgroup(
        ys: Vec<isize>,
        m: usize,
        w: usize,
        expected_cs: Vec<isize>,
    ) {
        let n = ys.len();
        let p = ModPoly::interpolate_from_mul_subgroup(
            ys.into_iter().map(Integer::from).collect(),
            Integer::from(m),
            &Integer::from(w),
        );
        for i in 0..n {
            assert_eq!(
                p.get_coefficient(i),
                expected_cs[i],
                "Difference in coefficient {}: expected {} but got {}",
                i,
                expected_cs[i],
                p.get_coefficient(i)
            );
        }
    }

    #[test]
    fn interpolate_zero_mod_5() {
        test_interpolate_from_mul_subgroup(vec![0, 0, 0, 0], 5, 2, vec![0, 0, 0, 0]);
    }
    #[test]
    fn interpolate_const_mod_5() {
        test_interpolate_from_mul_subgroup(vec![3, 3, 3, 3], 5, 2, vec![3, 0, 0, 0]);
    }
    #[test]
    fn interpolate_line_mod_5() {
        test_interpolate_from_mul_subgroup(vec![1, 0, 3, 4], 5, 2, vec![2, 4, 0, 0]);
    }
    #[test]
    fn interpolate_poly_mod_5() {
        test_interpolate_from_mul_subgroup(vec![4, 0, 0, 0], 5, 2, vec![1, 1, 1, 1]);
    }

    #[derive(Debug, Clone)]
    struct Usize16([u32; 16]);

    impl quickcheck::Arbitrary for Usize16 {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let mut a = [0u32; 16];
            for i in &mut a {
                *i = g.next_u32();
            }
            Usize16(a)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn test_interpolate_rountrip(ys: Usize16) -> bool {
        let m = Integer::from(17);
        let w = Integer::from(3);
        let Usize16(mut ys) = ys;
        for i in &mut ys {
            *i %= 17;
        }
        let ys: Vec<Integer> = ys.iter().cloned().map(Integer::from).collect();
        let p = ModPoly::interpolate_from_mul_subgroup(ys.clone(), m.clone(), &w);
        let ys2 = p.evaluate_over_mul_subgroup(&w, 16);
        ys == ys2
    }

    fn test_derivative_xgcd(roots: Vec<isize>, m: Integer) {
        let p = ModPoly::with_roots(roots.into_iter().map(Integer::from), &m);
        let dp = p.derivative();
        let (g, s, t) = p.xgcd(&dp);
        assert_eq!(g.len(), 1);
        assert_eq!(g, p * s + dp * t);
    }

    #[test]
    fn test_xgcd() {
        test_derivative_xgcd(vec![0], Integer::from(17));
        test_derivative_xgcd(vec![0, 1], Integer::from(17));
        test_derivative_xgcd(vec![0, 1, 2], Integer::from(17));
        test_derivative_xgcd(vec![0, 4, 5], Integer::from(17));
    }

    #[test]
    #[ignore]
    fn bench_xgcd() {
        let bls_12_381_r = Integer::from_str_radix(
            "52435875175126190479447740508185965837690552500527637822603658699938581184513",
            10,
        )
        .unwrap();
        for log_n in 4..16 {
            let n = 1 << log_n;
            let roots: Vec<usize> = (0..n).collect();
            let p = ModPoly::with_roots(roots.into_iter().map(Integer::from), &bls_12_381_r);
            let dp = p.derivative();
            let start = std::time::Instant::now();
            let (g, _s, _t) = p.xgcd(&dp);
            let duration = start.elapsed();
            let nanos_per = duration.as_nanos() / n as u128;
            println!("{log_n:>2}: {n:>8}: {duration:>8.1?} {nanos_per}ns/deg");
            assert_eq!(g.len(), 1);
        }
    }
}
