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
use crate::qfb::*;
use crate::qqbar::*;
use crate::qsieve::*;
use crate::templates::*;
use crate::thread_pool::*;
use crate::thread_support::*;
use crate::ulong_extras::*;


#[repr(C)]
pub struct hypgeom_struct {
    pub A: fmpz_poly_t,
    pub B: fmpz_poly_t,
    pub P: fmpz_poly_t,
    pub Q: fmpz_poly_t,
    pub have_precomputed: libc::c_int,
    pub r: mp_limb_signed_t,
    pub boundC: mp_limb_signed_t,
    pub boundD: mp_limb_signed_t,
    pub boundK: mp_limb_signed_t,
    pub MK: mag_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hypgeom_struct"][::std::mem::size_of::<hypgeom_struct>() - 152usize];
    ["Alignment of hypgeom_struct"][::std::mem::align_of::<hypgeom_struct>() - 8usize];
    ["Offset of field: hypgeom_struct::A"][::std::mem::offset_of!(hypgeom_struct, A) - 0usize];
    ["Offset of field: hypgeom_struct::B"][::std::mem::offset_of!(hypgeom_struct, B) - 24usize];
    ["Offset of field: hypgeom_struct::P"][::std::mem::offset_of!(hypgeom_struct, P) - 48usize];
    ["Offset of field: hypgeom_struct::Q"][::std::mem::offset_of!(hypgeom_struct, Q) - 72usize];
    ["Offset of field: hypgeom_struct::have_precomputed"]
        [::std::mem::offset_of!(hypgeom_struct, have_precomputed) - 96usize];
    ["Offset of field: hypgeom_struct::r"][::std::mem::offset_of!(hypgeom_struct, r) - 104usize];
    ["Offset of field: hypgeom_struct::boundC"]
        [::std::mem::offset_of!(hypgeom_struct, boundC) - 112usize];
    ["Offset of field: hypgeom_struct::boundD"]
        [::std::mem::offset_of!(hypgeom_struct, boundD) - 120usize];
    ["Offset of field: hypgeom_struct::boundK"]
        [::std::mem::offset_of!(hypgeom_struct, boundK) - 128usize];
    ["Offset of field: hypgeom_struct::MK"][::std::mem::offset_of!(hypgeom_struct, MK) - 136usize];
};
impl Default for hypgeom_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type hypgeom_t = [hypgeom_struct; 1usize];
extern "C" {
    pub fn hypgeom_init(hyp: *mut hypgeom_struct);
    pub fn hypgeom_clear(hyp: *mut hypgeom_struct);
    pub fn hypgeom_precompute(hyp: *mut hypgeom_struct);
    pub fn hypgeom_estimate_terms(
        z: *const mag_struct,
        r: libc::c_int,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn hypgeom_bound(
        error: *mut mag_struct,
        r: libc::c_int,
        C: mp_limb_signed_t,
        D: mp_limb_signed_t,
        K: mp_limb_signed_t,
        TK: *const mag_struct,
        z: *const mag_struct,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn arb_hypgeom_sum(
        P: *mut arb_struct,
        Q: *mut arb_struct,
        hyp: *const hypgeom_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_infsum(
        P: *mut arb_struct,
        Q: *mut arb_struct,
        hyp: *mut hypgeom_struct,
        target_prec: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
}
