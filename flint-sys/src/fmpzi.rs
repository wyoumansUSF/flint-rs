/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::flint::*;
use crate::fmpz_types::*;


extern "C" {
    #[link_name = "fmpzi_init__extern"]
    pub fn fmpzi_init(x: *mut fmpzi_struct);
    #[link_name = "fmpzi_clear__extern"]
    pub fn fmpzi_clear(x: *mut fmpzi_struct);
    #[link_name = "fmpzi_equal__extern"]
    pub fn fmpzi_equal(x: *const fmpzi_struct, y: *const fmpzi_struct) -> libc::c_int;
    #[link_name = "fmpzi_zero__extern"]
    pub fn fmpzi_zero(x: *mut fmpzi_struct);
    #[link_name = "fmpzi_one__extern"]
    pub fn fmpzi_one(x: *mut fmpzi_struct);
    #[link_name = "fmpzi_set__extern"]
    pub fn fmpzi_set(res: *mut fmpzi_struct, x: *const fmpzi_struct);
    #[link_name = "fmpzi_conj__extern"]
    pub fn fmpzi_conj(res: *mut fmpzi_struct, x: *const fmpzi_struct);
    #[link_name = "fmpzi_swap__extern"]
    pub fn fmpzi_swap(x: *mut fmpzi_struct, y: *mut fmpzi_struct);
    pub fn fmpzi_print(x: *const fmpzi_struct);
    #[link_name = "fmpzi_set_si_si__extern"]
    pub fn fmpzi_set_si_si(res: *mut fmpzi_struct, a: mp_limb_signed_t, b: mp_limb_signed_t);
    #[link_name = "fmpzi_randtest__extern"]
    pub fn fmpzi_randtest(res: *mut fmpzi_struct, state: *mut flint_rand_s, bits: mp_bitcnt_t);
    #[link_name = "fmpzi_is_unit__extern"]
    pub fn fmpzi_is_unit(x: *const fmpzi_struct) -> libc::c_int;
    #[link_name = "fmpzi_is_zero__extern"]
    pub fn fmpzi_is_zero(x: *const fmpzi_struct) -> libc::c_int;
    #[link_name = "fmpzi_is_one__extern"]
    pub fn fmpzi_is_one(x: *const fmpzi_struct) -> libc::c_int;
    pub fn fmpzi_bits(x: *const fmpzi_struct) -> mp_limb_signed_t;
    #[link_name = "fmpzi_norm__extern"]
    pub fn fmpzi_norm(res: *mut fmpz, x: *const fmpzi_struct);
    #[link_name = "fmpzi_neg__extern"]
    pub fn fmpzi_neg(res: *mut fmpzi_struct, x: *const fmpzi_struct);
    #[link_name = "fmpzi_add__extern"]
    pub fn fmpzi_add(res: *mut fmpzi_struct, x: *const fmpzi_struct, y: *const fmpzi_struct);
    #[link_name = "fmpzi_sub__extern"]
    pub fn fmpzi_sub(res: *mut fmpzi_struct, x: *const fmpzi_struct, y: *const fmpzi_struct);
    pub fn fmpzi_sqr(res: *mut fmpzi_struct, x: *const fmpzi_struct);
    pub fn fmpzi_mul(res: *mut fmpzi_struct, x: *const fmpzi_struct, y: *const fmpzi_struct);
    pub fn fmpzi_pow_ui(res: *mut fmpzi_struct, x: *const fmpzi_struct, exp: mp_limb_t);
    pub fn fmpzi_mul_i(z: *mut fmpzi_struct, x: *const fmpzi_struct);
    pub fn fmpzi_div_i(z: *mut fmpzi_struct, x: *const fmpzi_struct);
    pub fn fmpzi_mul_i_pow_si(res: *mut fmpzi_struct, z: *const fmpzi_struct, k: mp_limb_signed_t);
    pub fn fmpzi_canonical_unit_i_pow(x: *const fmpzi_struct) -> mp_limb_signed_t;
    #[link_name = "fmpzi_canonicalise_unit__extern"]
    pub fn fmpzi_canonicalise_unit(res: *mut fmpzi_struct, x: *const fmpzi_struct);
    pub fn fmpzi_divrem(
        q: *mut fmpzi_struct,
        r: *mut fmpzi_struct,
        x: *const fmpzi_struct,
        y: *const fmpzi_struct,
    );
    pub fn fmpzi_divrem_approx(
        q: *mut fmpzi_struct,
        r: *mut fmpzi_struct,
        x: *const fmpzi_struct,
        y: *const fmpzi_struct,
    );
    pub fn fmpzi_divexact(q: *mut fmpzi_struct, x: *const fmpzi_struct, y: *const fmpzi_struct);
    pub fn fmpzi_remove_one_plus_i(
        res: *mut fmpzi_struct,
        x: *const fmpzi_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpzi_gcd_euclidean(
        res: *mut fmpzi_struct,
        x: *const fmpzi_struct,
        y: *const fmpzi_struct,
    );
    pub fn fmpzi_gcd_euclidean_improved(
        res: *mut fmpzi_struct,
        x: *const fmpzi_struct,
        y: *const fmpzi_struct,
    );
    pub fn fmpzi_gcd_binary(res: *mut fmpzi_struct, x: *const fmpzi_struct, y: *const fmpzi_struct);
    pub fn fmpzi_gcd_shortest(g: *mut fmpzi_struct, x: *const fmpzi_struct, y: *const fmpzi_struct);
    pub fn fmpzi_gcd(g: *mut fmpzi_struct, x: *const fmpzi_struct, y: *const fmpzi_struct);
    pub fn fmpzi_is_prime(n: *const fmpzi_struct) -> libc::c_int;
    pub fn fmpzi_is_probabprime(n: *const fmpzi_struct) -> libc::c_int;
}
