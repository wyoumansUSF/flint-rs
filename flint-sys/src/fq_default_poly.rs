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
pub struct fq_default_poly_struct {
    pub fq: __BindgenUnionField<fq_poly_t>,
    pub fq_nmod: __BindgenUnionField<fq_nmod_poly_t>,
    pub fq_zech: __BindgenUnionField<fq_zech_poly_t>,
    pub nmod: __BindgenUnionField<nmod_poly_t>,
    pub fmpz_mod: __BindgenUnionField<fmpz_mod_poly_t>,
    pub bindgen_union_field: [u64; 6usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fq_default_poly_struct"][::std::mem::size_of::<fq_default_poly_struct>() - 48usize];
    ["Alignment of fq_default_poly_struct"]
        [::std::mem::align_of::<fq_default_poly_struct>() - 8usize];
    ["Offset of field: fq_default_poly_struct::fq"]
        [::std::mem::offset_of!(fq_default_poly_struct, fq) - 0usize];
    ["Offset of field: fq_default_poly_struct::fq_nmod"]
        [::std::mem::offset_of!(fq_default_poly_struct, fq_nmod) - 0usize];
    ["Offset of field: fq_default_poly_struct::fq_zech"]
        [::std::mem::offset_of!(fq_default_poly_struct, fq_zech) - 0usize];
    ["Offset of field: fq_default_poly_struct::nmod"]
        [::std::mem::offset_of!(fq_default_poly_struct, nmod) - 0usize];
    ["Offset of field: fq_default_poly_struct::fmpz_mod"]
        [::std::mem::offset_of!(fq_default_poly_struct, fmpz_mod) - 0usize];
};
impl Default for fq_default_poly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fq_default_poly_t = [fq_default_poly_struct; 1usize];
extern "C" {
    #[link_name = "fq_default_poly_init__extern"]
    pub fn fq_default_poly_init(
        poly: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_init2__extern"]
    pub fn fq_default_poly_init2(
        poly: *mut fq_default_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_realloc__extern"]
    pub fn fq_default_poly_realloc(
        poly: *mut fq_default_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_truncate__extern"]
    pub fn fq_default_poly_truncate(
        poly: *mut fq_default_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_set_trunc__extern"]
    pub fn fq_default_poly_set_trunc(
        poly1: *mut fq_default_poly_struct,
        poly2: *mut fq_default_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_fit_length__extern"]
    pub fn fq_default_poly_fit_length(
        poly: *mut fq_default_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "_fq_default_poly_set_length__extern"]
    pub fn _fq_default_poly_set_length(
        poly: *mut fq_default_poly_struct,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_clear__extern"]
    pub fn fq_default_poly_clear(
        poly: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_length__extern"]
    pub fn fq_default_poly_length(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    #[link_name = "fq_default_poly_degree__extern"]
    pub fn fq_default_poly_degree(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    #[link_name = "fq_default_poly_randtest__extern"]
    pub fn fq_default_poly_randtest(
        f: *mut fq_default_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_randtest_not_zero__extern"]
    pub fn fq_default_poly_randtest_not_zero(
        f: *mut fq_default_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_randtest_monic__extern"]
    pub fn fq_default_poly_randtest_monic(
        f: *mut fq_default_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_randtest_irreducible__extern"]
    pub fn fq_default_poly_randtest_irreducible(
        f: *mut fq_default_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_set__extern"]
    pub fn fq_default_poly_set(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_set_fq_default__extern"]
    pub fn fq_default_poly_set_fq_default(
        poly: *mut fq_default_poly_struct,
        c: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_swap__extern"]
    pub fn fq_default_poly_swap(
        op1: *mut fq_default_poly_struct,
        op2: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_zero__extern"]
    pub fn fq_default_poly_zero(
        poly: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_one__extern"]
    pub fn fq_default_poly_one(
        poly: *mut fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_gen__extern"]
    pub fn fq_default_poly_gen(f: *mut fq_default_poly_struct, ctx: *const fq_default_ctx_struct);
    #[link_name = "fq_default_poly_make_monic__extern"]
    pub fn fq_default_poly_make_monic(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_reverse__extern"]
    pub fn fq_default_poly_reverse(
        res: *mut fq_default_poly_struct,
        poly: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_deflation__extern"]
    pub fn fq_default_poly_deflation(
        input: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_t;
    #[link_name = "fq_default_poly_deflate__extern"]
    pub fn fq_default_poly_deflate(
        result: *mut fq_default_poly_struct,
        input: *const fq_default_poly_struct,
        deflation: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_inflate__extern"]
    pub fn fq_default_poly_inflate(
        result: *mut fq_default_poly_struct,
        input: *const fq_default_poly_struct,
        inflation: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_get_coeff__extern"]
    pub fn fq_default_poly_get_coeff(
        x: *mut fq_default_struct,
        poly: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_set_coeff__extern"]
    pub fn fq_default_poly_set_coeff(
        poly: *mut fq_default_poly_struct,
        n: mp_limb_signed_t,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_set_coeff_fmpz__extern"]
    pub fn fq_default_poly_set_coeff_fmpz(
        poly: *mut fq_default_poly_struct,
        n: mp_limb_signed_t,
        x: *const fmpz,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_set_nmod_poly__extern"]
    pub fn fq_default_poly_set_nmod_poly(
        rop: *mut fq_default_poly_struct,
        op: *const nmod_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_set_fmpz_mod_poly__extern"]
    pub fn fq_default_poly_set_fmpz_mod_poly(
        rop: *mut fq_default_poly_struct,
        op: *const fmpz_mod_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_set_fmpz_poly(
        rop: *mut fq_default_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_equal__extern"]
    pub fn fq_default_poly_equal(
        poly1: *const fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_equal_trunc__extern"]
    pub fn fq_default_poly_equal_trunc(
        poly1: *const fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_is_zero__extern"]
    pub fn fq_default_poly_is_zero(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_is_one__extern"]
    pub fn fq_default_poly_is_one(
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_is_unit__extern"]
    pub fn fq_default_poly_is_unit(
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_is_gen__extern"]
    pub fn fq_default_poly_is_gen(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_equal_fq_default__extern"]
    pub fn fq_default_poly_equal_fq_default(
        poly: *const fq_default_poly_struct,
        c: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_add__extern"]
    pub fn fq_default_poly_add(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_add_si__extern"]
    pub fn fq_default_poly_add_si(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        c: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_add_series__extern"]
    pub fn fq_default_poly_add_series(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_sub__extern"]
    pub fn fq_default_poly_sub(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_sub_series__extern"]
    pub fn fq_default_poly_sub_series(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_neg__extern"]
    pub fn fq_default_poly_neg(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_scalar_mul_fq_default__extern"]
    pub fn fq_default_poly_scalar_mul_fq_default(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_scalar_div_fq_default__extern"]
    pub fn fq_default_poly_scalar_div_fq_default(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_scalar_addmul_fq_default__extern"]
    pub fn fq_default_poly_scalar_addmul_fq_default(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_scalar_submul_fq_default__extern"]
    pub fn fq_default_poly_scalar_submul_fq_default(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        x: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_mul__extern"]
    pub fn fq_default_poly_mul(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_mullow__extern"]
    pub fn fq_default_poly_mullow(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_mulhigh__extern"]
    pub fn fq_default_poly_mulhigh(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        start: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_mulmod__extern"]
    pub fn fq_default_poly_mulmod(
        res: *mut fq_default_poly_struct,
        poly1: *const fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_sqr__extern"]
    pub fn fq_default_poly_sqr(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_pow__extern"]
    pub fn fq_default_poly_pow(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        e: mp_limb_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_pow_trunc__extern"]
    pub fn fq_default_poly_pow_trunc(
        res: *mut fq_default_poly_struct,
        poly: *const fq_default_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_powmod_fmpz_binexp__extern"]
    pub fn fq_default_poly_powmod_fmpz_binexp(
        res: *mut fq_default_poly_struct,
        poly: *const fq_default_poly_struct,
        e: *const fmpz,
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_powmod_ui_binexp__extern"]
    pub fn fq_default_poly_powmod_ui_binexp(
        res: *mut fq_default_poly_struct,
        poly: *const fq_default_poly_struct,
        e: mp_limb_t,
        f: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_shift_left__extern"]
    pub fn fq_default_poly_shift_left(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_shift_right__extern"]
    pub fn fq_default_poly_shift_right(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_hamming_weight__extern"]
    pub fn fq_default_poly_hamming_weight(
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_signed_t;
    #[link_name = "fq_default_poly_gcd__extern"]
    pub fn fq_default_poly_gcd(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_xgcd__extern"]
    pub fn fq_default_poly_xgcd(
        G: *mut fq_default_poly_struct,
        S: *mut fq_default_poly_struct,
        T: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_remove__extern"]
    pub fn fq_default_poly_remove(
        f: *mut fq_default_poly_struct,
        g: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> mp_limb_t;
    #[link_name = "fq_default_poly_divrem__extern"]
    pub fn fq_default_poly_divrem(
        Q: *mut fq_default_poly_struct,
        R: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_rem__extern"]
    pub fn fq_default_poly_rem(
        R: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_inv_series__extern"]
    pub fn fq_default_poly_inv_series(
        Qinv: *mut fq_default_poly_struct,
        Q: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_div_series__extern"]
    pub fn fq_default_poly_div_series(
        Q: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_divides__extern"]
    pub fn fq_default_poly_divides(
        Q: *mut fq_default_poly_struct,
        A: *const fq_default_poly_struct,
        B: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_derivative__extern"]
    pub fn fq_default_poly_derivative(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_invsqrt_series__extern"]
    pub fn fq_default_poly_invsqrt_series(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_sqrt_series__extern"]
    pub fn fq_default_poly_sqrt_series(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_sqrt__extern"]
    pub fn fq_default_poly_sqrt(
        rop: *mut fq_default_poly_struct,
        op: *const fq_default_poly_struct,
        ctx: *mut fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_evaluate_fq_default__extern"]
    pub fn fq_default_poly_evaluate_fq_default(
        res: *mut fq_default_struct,
        f: *const fq_default_poly_struct,
        a: *const fq_default_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_compose__extern"]
    pub fn fq_default_poly_compose(
        rop: *mut fq_default_poly_struct,
        op1: *const fq_default_poly_struct,
        op2: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_poly_compose_mod__extern"]
    pub fn fq_default_poly_compose_mod(
        res: *mut fq_default_poly_struct,
        poly1: *const fq_default_poly_struct,
        poly2: *const fq_default_poly_struct,
        poly3: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    );
    pub fn fq_default_poly_fprint(
        file: *mut FILE,
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_default_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fq_default_poly_struct,
        x: *const libc::c_char,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_default_poly_print(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_default_poly_print_pretty(
        poly: *const fq_default_poly_struct,
        x: *const libc::c_char,
        ctx: *const fq_default_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_default_poly_get_str_pretty__extern"]
    pub fn fq_default_poly_get_str_pretty(
        poly: *const fq_default_poly_struct,
        x: *const libc::c_char,
        ctx: *const fq_default_ctx_struct,
    ) -> *mut libc::c_char;
    #[link_name = "fq_default_poly_get_str__extern"]
    pub fn fq_default_poly_get_str(
        poly: *const fq_default_poly_struct,
        ctx: *const fq_default_ctx_struct,
    ) -> *mut libc::c_char;
    #[link_name = "fq_default_mat_charpoly__extern"]
    pub fn fq_default_mat_charpoly(
        p: *mut fq_default_poly_struct,
        M: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
    #[link_name = "fq_default_mat_minpoly__extern"]
    pub fn fq_default_mat_minpoly(
        p: *mut fq_default_poly_struct,
        X: *const fq_default_mat_struct,
        ctx: *const fq_default_ctx_struct,
    );
}
