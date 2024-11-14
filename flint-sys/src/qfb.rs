/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::bindgen::*;
use crate::acb::*;
use crate::acb_calc::*;
use crate::acb_dft::*;
use crate::acb_dirichlet::*;
use crate::acb_elliptic::*;
use crate::acb_hypgeom::*;
use crate::acb_mat::*;
use crate::acb_modular::*;
use crate::acb_poly::*;
use crate::acb_theta::*;
use crate::acb_types::*;
use crate::acf::*;
use crate::acf_types::*;
use crate::aprcl::*;
use crate::arb::*;
use crate::arb_calc::*;
use crate::arb_fmpz_poly::*;
use crate::arb_fpwrap::*;
use crate::arb_hypgeom::*;
use crate::arb_mat::*;
use crate::arb_poly::*;
use crate::arb_types::*;
use crate::arf::*;
use crate::arf_types::*;
use crate::arith::*;
use crate::bernoulli::*;
use crate::bool_mat::*;
use crate::ca::*;
use crate::ca_ext::*;
use crate::ca_field::*;
use crate::ca_mat::*;
use crate::ca_poly::*;
use crate::ca_vec::*;
use crate::calcium::*;
use crate::d_mat::*;
use crate::d_vec::*;
use crate::dirichlet::*;
use crate::dlog::*;
use crate::double_extras::*;
use crate::double_interval::*;
use crate::fexpr::*;
use crate::fexpr_builtin::*;
use crate::fft::*;
use crate::fft_tuning::*;
use crate::flint_config::*;
use crate::flint::*;
use crate::fmpq::*;
use crate::fmpq_mat::*;
use crate::fmpq_mpoly::*;
use crate::fmpq_mpoly_factor::*;
use crate::fmpq_poly::*;
use crate::fmpq_types::*;
use crate::fmpq_vec::*;
use crate::fmpz::*;
use crate::fmpz_extras::*;
use crate::fmpz_factor::*;
use crate::fmpz_lll::*;
use crate::fmpz_mat::*;
use crate::fmpz_mod::*;
use crate::fmpz_mod_mat::*;
use crate::fmpz_mod_mpoly::*;
use crate::fmpz_mod_mpoly_factor::*;
use crate::fmpz_mod_poly::*;
use crate::fmpz_mod_poly_factor::*;
use crate::fmpz_mod_types::*;
use crate::fmpz_mod_vec::*;
use crate::fmpz_mpoly::*;
use crate::fmpz_mpoly_factor::*;
use crate::fmpz_mpoly_q::*;
use crate::fmpz_poly::*;
use crate::fmpz_poly_factor::*;
use crate::fmpz_poly_mat::*;
use crate::fmpz_poly_q::*;
use crate::fmpz_types::*;
use crate::fmpz_vec::*;
use crate::fmpzi::*;
use crate::fq::*;
use crate::fq_default::*;
use crate::fq_default_mat::*;
use crate::fq_default_poly::*;
use crate::fq_default_poly_factor::*;
use crate::fq_embed::*;
use crate::fq_embed_templates::*;
use crate::fq_mat::*;
use crate::fq_mat_templates::*;
use crate::fq_nmod::*;
use crate::fq_nmod_embed::*;
use crate::fq_nmod_mat::*;
use crate::fq_nmod_mpoly::*;
use crate::fq_nmod_mpoly_factor::*;
use crate::fq_nmod_poly::*;
use crate::fq_nmod_poly_factor::*;
use crate::fq_nmod_types::*;
use crate::fq_nmod_vec::*;
use crate::fq_poly::*;
use crate::fq_poly_factor::*;
use crate::fq_poly_factor_templates::*;
use crate::fq_poly_templates::*;
use crate::fq_templates::*;
use crate::fq_types::*;
use crate::fq_vec::*;
use crate::fq_vec_templates::*;
use crate::fq_zech::*;
use crate::fq_zech_embed::*;
use crate::fq_zech_mat::*;
use crate::fq_zech_mpoly::*;
use crate::fq_zech_mpoly_factor::*;
use crate::fq_zech_poly::*;
use crate::fq_zech_poly_factor::*;
use crate::fq_zech_types::*;
use crate::fq_zech_vec::*;
use crate::gmpcompat::*;
use crate::gr::*;
use crate::gr_generic::*;
use crate::gr_mat::*;
use crate::gr_mpoly::*;
use crate::gr_poly::*;
use crate::gr_special::*;
use crate::gr_vec::*;
use crate::hypgeom::*;
use crate::limb_types::*;
use crate::long_extras::*;
use crate::longlong::*;
use crate::mag::*;
use crate::mpf_impl::*;
use crate::mpfr_mat::*;
use crate::mpfr_vec::*;
use crate::mpoly::*;
use crate::mpoly_types::*;
use crate::n_poly::*;
use crate::n_poly_types::*;
use crate::nf::*;
use crate::nf_elem::*;
use crate::nmod::*;
use crate::nmod_mat::*;
use crate::nmod_mpoly::*;
use crate::nmod_mpoly_factor::*;
use crate::nmod_poly::*;
use crate::nmod_poly_factor::*;
use crate::nmod_poly_mat::*;
use crate::nmod_types::*;
use crate::nmod_vec::*;
use crate::padic::*;
use crate::padic_mat::*;
use crate::padic_poly::*;
use crate::padic_types::*;
use crate::partitions::*;
use crate::perm::*;
use crate::qadic::*;
use crate::qqbar::*;
use crate::qsieve::*;
use crate::templates::*;
use crate::thread_pool::*;
use crate::thread_support::*;
use crate::ulong_extras::*;


#[repr(C)]
pub struct qfb {
    pub a: fmpz_t,
    pub b: fmpz_t,
    pub c: fmpz_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of qfb"][::std::mem::size_of::<qfb>() - 24usize];
    ["Alignment of qfb"][::std::mem::align_of::<qfb>() - 8usize];
    ["Offset of field: qfb::a"][::std::mem::offset_of!(qfb, a) - 0usize];
    ["Offset of field: qfb::b"][::std::mem::offset_of!(qfb, b) - 8usize];
    ["Offset of field: qfb::c"][::std::mem::offset_of!(qfb, c) - 16usize];
};
impl Default for qfb {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type qfb_t = [qfb; 1usize];
#[repr(C)]
pub struct qfb_hash_t {
    pub q: qfb_t,
    pub q2: qfb_t,
    pub iter: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of qfb_hash_t"][::std::mem::size_of::<qfb_hash_t>() - 56usize];
    ["Alignment of qfb_hash_t"][::std::mem::align_of::<qfb_hash_t>() - 8usize];
    ["Offset of field: qfb_hash_t::q"][::std::mem::offset_of!(qfb_hash_t, q) - 0usize];
    ["Offset of field: qfb_hash_t::q2"][::std::mem::offset_of!(qfb_hash_t, q2) - 24usize];
    ["Offset of field: qfb_hash_t::iter"][::std::mem::offset_of!(qfb_hash_t, iter) - 48usize];
};
impl Default for qfb_hash_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "qfb_init__extern"]
    pub fn qfb_init(q: *mut qfb);
    #[link_name = "qfb_clear__extern"]
    pub fn qfb_clear(q: *mut qfb);
    #[link_name = "qfb_equal__extern"]
    pub fn qfb_equal(f: *mut qfb, g: *mut qfb) -> libc::c_int;
    #[link_name = "qfb_set__extern"]
    pub fn qfb_set(f: *mut qfb, g: *mut qfb);
    #[link_name = "qfb_discriminant__extern"]
    pub fn qfb_discriminant(D: *mut fmpz, f: *mut qfb);
    pub fn qfb_print(q: *mut qfb);
    #[link_name = "qfb_array_clear__extern"]
    pub fn qfb_array_clear(forms: *mut *mut qfb, num: mp_limb_signed_t);
    pub fn qfb_hash_init(depth: mp_limb_signed_t) -> *mut qfb_hash_t;
    pub fn qfb_hash_clear(qhash: *mut qfb_hash_t, depth: mp_limb_signed_t);
    pub fn qfb_hash_insert(
        qhash: *mut qfb_hash_t,
        q: *mut qfb,
        q2: *mut qfb,
        iter: mp_limb_signed_t,
        depth: mp_limb_signed_t,
    );
    pub fn qfb_hash_find(
        qhash: *mut qfb_hash_t,
        q: *mut qfb,
        depth: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn qfb_reduce(r: *mut qfb, f: *mut qfb, D: *mut fmpz);
    pub fn qfb_is_reduced(r: *mut qfb) -> libc::c_int;
    pub fn qfb_reduced_forms(forms: *mut *mut qfb, d: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn qfb_reduced_forms_large(forms: *mut *mut qfb, d: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn qfb_nucomp(r: *mut qfb, f: *const qfb, g: *const qfb, D: *mut fmpz, L: *mut fmpz);
    pub fn qfb_nudupl(r: *mut qfb, f: *const qfb, D: *mut fmpz, L: *mut fmpz);
    pub fn qfb_pow_ui(r: *mut qfb, f: *mut qfb, D: *mut fmpz, exp: mp_limb_t);
    pub fn qfb_pow(r: *mut qfb, f: *mut qfb, D: *mut fmpz, exp: *mut fmpz);
    pub fn qfb_pow_with_root(r: *mut qfb, f: *mut qfb, D: *mut fmpz, e: *mut fmpz, L: *mut fmpz);
    #[link_name = "qfb_inverse__extern"]
    pub fn qfb_inverse(r: *mut qfb, f: *mut qfb);
    #[link_name = "qfb_is_principal_form__extern"]
    pub fn qfb_is_principal_form(f: *mut qfb, D: *mut fmpz) -> libc::c_int;
    #[link_name = "qfb_principal_form__extern"]
    pub fn qfb_principal_form(f: *mut qfb, D: *mut fmpz);
    #[link_name = "qfb_is_primitive__extern"]
    pub fn qfb_is_primitive(f: *mut qfb) -> libc::c_int;
    pub fn qfb_prime_form(r: *mut qfb, D: *mut fmpz, p: *mut fmpz);
    pub fn qfb_exponent_element(
        exponent: *mut fmpz,
        f: *mut qfb,
        n: *mut fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
    ) -> libc::c_int;
    pub fn qfb_exponent(
        exponent: *mut fmpz,
        n: *mut fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
        c: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn qfb_exponent_grh(
        exponent: *mut fmpz,
        n: *mut fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
    ) -> libc::c_int;
}
