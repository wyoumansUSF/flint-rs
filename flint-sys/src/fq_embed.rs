/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use crate::fmpz_types::*;
use crate::fmpz_mod_types::*;
use crate::fq_types::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


extern "C" {
    pub fn fq_modulus_pow_series_inv(
        res: *mut fmpz_mod_poly_struct,
        ctx: *const fq_ctx_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn fq_modulus_derivative_inv(
        m_prime: *mut fmpz_poly_struct,
        m_prime_inv: *mut fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
}
