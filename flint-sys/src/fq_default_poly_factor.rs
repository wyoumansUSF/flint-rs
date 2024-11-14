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
use crate::qfb::*;
use crate::qqbar::*;
use crate::qsieve::*;
use crate::templates::*;
use crate::thread_pool::*;
use crate::thread_support::*;
use crate::ulong_extras::*;


#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
pub struct fq_default_poly_factor_struct {
    pub fq: __BindgenUnionField<fq_poly_factor_t>,
    pub fq_nmod: __BindgenUnionField<fq_nmod_poly_factor_t>,
    pub fq_zech: __BindgenUnionField<fq_zech_poly_factor_t>,
    pub nmod: __BindgenUnionField<nmod_poly_factor_t>,
    pub fmpz_mod: __BindgenUnionField<fmpz_mod_poly_factor_t>,
    pub bindgen_union_field: [u64; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_default_poly_factor_struct"]
        [::std::mem::size_of::<fq_default_poly_factor_struct>() - 32usize];
    ["Alignment of fq_default_poly_factor_struct"]
        [::std::mem::align_of::<fq_default_poly_factor_struct>() - 8usize];
    ["Offset of field: fq_default_poly_factor_struct::fq"]
        [::std::mem::offset_of!(fq_default_poly_factor_struct, fq) - 0usize];
    ["Offset of field: fq_default_poly_factor_struct::fq_nmod"]
        [::std::mem::offset_of!(fq_default_poly_factor_struct, fq_nmod) - 0usize];
    ["Offset of field: fq_default_poly_factor_struct::fq_zech"]
        [::std::mem::offset_of!(fq_default_poly_factor_struct, fq_zech) - 0usize];
    ["Offset of field: fq_default_poly_factor_struct::nmod"]
        [::std::mem::offset_of!(fq_default_poly_factor_struct, nmod) - 0usize];
    ["Offset of field: fq_default_poly_factor_struct::fmpz_mod"]
        [::std::mem::offset_of!(fq_default_poly_factor_struct, fmpz_mod) - 0usize];
};
impl Default for fq_default_poly_factor_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_default_poly_factor_t = [fq_default_poly_factor_struct; 1usize];
extern "C" {
    #[link_name = "fq_default_poly_factor_init__extern"]
    pub fn fq_default_poly_factor_init(
        fac: *mut fq_default_poly_factor_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_clear__extern"]
    pub fn fq_default_poly_factor_clear(
        fac: *mut fq_default_poly_factor_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_realloc__extern"]
    pub fn fq_default_poly_factor_realloc(
        fac: *mut fq_default_poly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_fit_length__extern"]
    pub fn fq_default_poly_factor_fit_length(
        fac: *mut fq_default_poly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_length__extern"]
    pub fn fq_default_poly_factor_length(
        fac: *mut fq_default_poly_factor_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    #[link_name = "fq_default_poly_factor_exp__extern"]
    pub fn fq_default_poly_factor_exp(
        fac: *mut fq_default_poly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    #[link_name = "fq_default_poly_factor_set__extern"]
    pub fn fq_default_poly_factor_set(
        res: *mut fq_default_poly_factor_struct,
        fac: *const fq_default_poly_factor_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_insert__extern"]
    pub fn fq_default_poly_factor_insert(
        fac: *mut fq_default_poly_factor_struct,
        poly: *const fq_default_poly_struct,
        exp: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_get_poly__extern"]
    pub fn fq_default_poly_factor_get_poly(
        poly: *mut fq_default_poly_struct,
        fac: *const fq_default_poly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_print__extern"]
    pub fn fq_default_poly_factor_print(
        fac: *const fq_default_poly_factor_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_print_pretty__extern"]
    pub fn fq_default_poly_factor_print_pretty(
        fac: *const fq_default_poly_factor_struct,
        var: *const libc::c_char,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_concat__extern"]
    pub fn fq_default_poly_factor_concat(
        res: *mut fq_default_poly_factor_struct,
        fac: *const fq_default_poly_factor_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_pow__extern"]
    pub fn fq_default_poly_factor_pow(
        fac: *mut fq_default_poly_factor_struct,
        exp: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_is_squarefree__extern"]
    pub fn fq_default_poly_is_squarefree(
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_factor_squarefree__extern"]
    pub fn fq_default_poly_factor_squarefree(
        res: *mut fq_default_poly_factor_struct,
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_is_irreducible__extern"]
    pub fn fq_default_poly_is_irreducible(
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_factor_distinct_deg__extern"]
    pub fn fq_default_poly_factor_distinct_deg(
        res: *mut fq_default_poly_factor_struct,
        poly: *const fq_default_poly_struct,
        degs: *const *mut mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_equal_deg__extern"]
    pub fn fq_default_poly_factor_equal_deg(
        factors: *mut fq_default_poly_factor_struct,
        pol: *const fq_default_poly_struct,
        d: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor__extern"]
    pub fn fq_default_poly_factor(
        result: *mut fq_default_poly_factor_struct,
        leading_coeff: *mut fq_default_struct,
        input: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_factor_split_single__extern"]
    pub fn fq_default_poly_factor_split_single(
        linfactor: *mut fq_default_poly_struct,
        input: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_roots__extern"]
    pub fn fq_default_poly_roots(
        r: *mut fq_default_poly_factor_struct,
        f: *const fq_default_poly_struct,
        with_multiplicity: libc::c_int,
        ctx: *const fq_default_ctx_struct,
    );
}
