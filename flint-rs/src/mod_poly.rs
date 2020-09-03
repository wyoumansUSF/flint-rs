use flint_sys::{self, fmpz_mod_poly};
use rug::Integer;

use std::cmp::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::mem::MaybeUninit;
use std::ops::*;

pub struct ModPoly {
    raw: fmpz_mod_poly,
    modulus: Integer,
}

impl ModPoly {
    /// A new polynomial, equal to zero.
    pub fn new(modulus: Integer) -> Self {
        unsafe {
            let mut raw = MaybeUninit::uninit();
            let flint_modulus = flint_sys::gmp_to_flint(modulus.as_raw());
            flint_sys::fmpz_mod_poly_init(raw.as_mut_ptr(), &flint_modulus);
            ModPoly {
                raw: raw.assume_init(),
                modulus,
            }
        }
    }

    /// A new polynomial, equal to zero, with room for `n` coefficients.
    pub fn with_capacity(modulus: Integer, n: usize) -> Self {
        unsafe {
            let mut raw = MaybeUninit::uninit();
            let flint_modulus = flint_sys::gmp_to_flint(modulus.as_raw());
            flint_sys::fmpz_mod_poly_init2(raw.as_mut_ptr(), &flint_modulus, n as flint_sys::slong);
            ModPoly {
                raw: raw.assume_init(),
                modulus,
            }
        }
    }

    /// Reallocates the polynomial to have room for `n` coefficients. Truncates the polynomial if
    /// it has more than `n` coefficients.
    pub fn reserve(&mut self, n: usize) {
        unsafe {
            flint_sys::fmpz_mod_poly_realloc(&mut self.raw, n as flint_sys::slong);
        }
    }

    /// Get the modulus of this polynomial.
    pub fn modulus(&self) -> &Integer {
        &self.modulus
    }

    /// Get the `i`th coefficient
    pub fn get_coefficient(&self, i: usize) -> Integer {
        unsafe {
            let mut c = Integer::new();
            flint_sys::fmpz_mod_poly_get_coeff_mpz(
                c.as_raw_mut(),
                &self.raw,
                i as flint_sys::slong,
            );
            c
        }
    }

    /// Set the `i`th coefficient to be `c`
    pub fn set_coefficient(&mut self, i: usize, c: &Integer) {
        unsafe {
            flint_sys::fmpz_mod_poly_set_coeff_mpz(
                &mut self.raw,
                i as flint_sys::slong,
                c.as_raw(),
            );
        }
    }

    /// Set the `i`th coefficient to be `c`
    pub fn set_coefficient_ui(&mut self, i: usize, c: usize) {
        unsafe {
            flint_sys::fmpz_mod_poly_set_coeff_ui(
                &mut self.raw,
                i as flint_sys::slong,
                c as flint_sys::ulong,
            );
        }
    }

    /// The number of coefficients in the polynomial. One more than the degree.
    pub fn len(&self) -> usize {
        unsafe { flint_sys::fmpz_mod_poly_length(&self.raw) as usize }
    }

    /// `self = self + other`
    pub fn add(&mut self, other: &Self) {
        debug_assert_eq!(self.modulus, other.modulus);
        unsafe {
            flint_sys::fmpz_mod_poly_add(&mut self.raw, &self.raw, &other.raw);
        }
    }

    /// `self = self - other`
    pub fn sub(&mut self, other: &Self) {
        debug_assert_eq!(self.modulus, other.modulus);
        unsafe {
            flint_sys::fmpz_mod_poly_sub(&mut self.raw, &self.raw, &other.raw);
        }
    }

    /// `self = other - self`
    pub fn sub_from(&mut self, other: &Self) {
        debug_assert_eq!(self.modulus, other.modulus);
        unsafe {
            flint_sys::fmpz_mod_poly_sub(&mut self.raw, &other.raw, &self.raw);
        }
    }

    /// `self = self * other`
    pub fn mul(&mut self, other: &Self) {
        debug_assert_eq!(self.modulus, other.modulus);
        unsafe {
            flint_sys::fmpz_mod_poly_mul(&mut self.raw, &self.raw, &other.raw);
        }
    }

    /// Find `q` and `r` such that `self = other * q + r` and `r` has degree less than `other`.
    ///
    /// ## Returns
    ///
    /// `(q, r)`
    pub fn divrem(&self, other: &Self) -> (ModPoly, ModPoly) {
        debug_assert_eq!(self.modulus, other.modulus);
        let mut q = ModPoly::new(self.modulus.clone());
        let mut r = ModPoly::new(self.modulus.clone());
        unsafe {
            flint_sys::fmpz_mod_poly_divrem(&mut q.raw, &mut r.raw, &self.raw, &other.raw);
        }
        (q, r)
    }

    /// `self = self / other`
    pub fn div(&mut self, other: &Self) {
        debug_assert_eq!(self.modulus, other.modulus);
        let mut r = ModPoly::new(self.modulus.clone());
        unsafe {
            flint_sys::fmpz_mod_poly_divrem(&mut self.raw, &mut r.raw, &self.raw, &other.raw);
        }
    }

    /// `self = other / self`
    pub fn div_from(&mut self, other: &Self) {
        debug_assert_eq!(self.modulus, other.modulus);
        let mut r = ModPoly::new(self.modulus.clone());
        unsafe {
            flint_sys::fmpz_mod_poly_divrem(&mut self.raw, &mut r.raw, &other.raw, &self.raw);
        }
    }

    /// `self = self % other`
    pub fn rem(&mut self, other: &Self) {
        debug_assert_eq!(self.modulus, other.modulus);
        let mut q = ModPoly::new(self.modulus.clone());
        unsafe {
            flint_sys::fmpz_mod_poly_divrem(&mut q.raw, &mut self.raw, &self.raw, &other.raw);
        }
    }

    /// `self = other % self`
    pub fn rem_from(&mut self, other: &Self) {
        debug_assert_eq!(self.modulus, other.modulus);
        let mut q = ModPoly::new(self.modulus.clone());
        unsafe {
            flint_sys::fmpz_mod_poly_divrem(&mut q.raw, &mut self.raw, &other.raw, &self.raw);
        }
    }

    /// `self = self * self`
    pub fn sqr(&mut self) {
        unsafe {
            flint_sys::fmpz_mod_poly_sqr(&mut self.raw, &self.raw);
        }
    }
}

impl Clone for ModPoly {
    fn clone(&self) -> Self {
        let mut this = ModPoly::new(self.modulus.clone());
        unsafe {
            flint_sys::fmpz_mod_poly_set(&mut this.raw, &self.raw);
        }
        this
    }
}

impl PartialEq<ModPoly> for ModPoly {
    fn eq(&self, other: &ModPoly) -> bool {
        unsafe { flint_sys::fmpz_mod_poly_equal(&self.raw, &other.raw) != 0 }
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

impl_self_binary!(ModPoly, add, add, Add { add }, AddAssign { add_assign });
impl_self_binary!(
    ModPoly,
    sub,
    sub_from,
    Sub { sub },
    SubAssign { sub_assign }
);
impl_self_binary!(ModPoly, mul, mul, Mul { mul }, MulAssign { mul_assign });
impl_self_binary!(
    ModPoly,
    div,
    div_from,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let p = Integer::from(17);
        let f = ModPoly::new(p);
        assert_eq!(f.len(), 0);
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
    }
}
