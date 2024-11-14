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
use crate::qfb::*;
use crate::qqbar::*;
use crate::qsieve::*;
use crate::templates::*;
use crate::thread_pool::*;
use crate::thread_support::*;
use crate::ulong_extras::*;


extern "C" {
    pub fn fq_ctx_init(
        ctx: *mut fq_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    );
    pub fn _fq_ctx_init_conway_ui(
        ctx: *mut fq_ctx_struct,
        p: mp_limb_t,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    ) -> libc::c_int;
    pub fn fq_ctx_init_conway_ui(
        ctx: *mut fq_ctx_struct,
        p: mp_limb_t,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    );
    pub fn _fq_ctx_init_conway(
        ctx: *mut fq_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    ) -> libc::c_int;
    pub fn fq_ctx_init_conway(
        ctx: *mut fq_ctx_struct,
        p: *const fmpz,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    );
    pub fn fq_ctx_init_modulus(
        ctx: *mut fq_ctx_struct,
        modulus: *const fmpz_mod_poly_struct,
        ctxp: *const fmpz_mod_ctx_struct,
        var: *const libc::c_char,
    );
    pub fn fq_ctx_init_randtest(
        ctx: *mut fq_ctx_struct,
        state: *mut flint_rand_s,
        type_: libc::c_int,
    );
    pub fn fq_ctx_init_randtest_reducible(
        ctx: *mut fq_ctx_struct,
        state: *mut flint_rand_s,
        type_: libc::c_int,
    );
    pub fn fq_ctx_clear(ctx: *mut fq_ctx_struct);
    #[link_name = "fq_ctx_modulus__extern"]
    pub fn fq_ctx_modulus(ctx: *const fq_ctx_struct) -> *const fmpz_mod_poly_struct;
    #[link_name = "fq_ctx_degree__extern"]
    pub fn fq_ctx_degree(ctx: *const fq_ctx_struct) -> mp_limb_signed_t;
    #[link_name = "fq_ctx_prime__extern"]
    pub fn fq_ctx_prime(ctx: *const fq_ctx_struct) -> *const fmpz;
    pub fn fq_ctx_order(f: *mut fmpz, ctx: *const fq_ctx_struct);
    pub fn fq_ctx_fprint(file: *mut FILE, ctx: *const fq_ctx_struct) -> libc::c_int;
    pub fn fq_ctx_print(ctx: *const fq_ctx_struct);
    pub fn fq_init(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_init2(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_clear(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn _fq_sparse_reduce(R: *mut fmpz, lenR: mp_limb_signed_t, ctx: *const fq_ctx_struct);
    pub fn _fq_dense_reduce(R: *mut fmpz, lenR: mp_limb_signed_t, ctx: *const fq_ctx_struct);
    pub fn _fq_reduce(R: *mut fmpz, lenR: mp_limb_signed_t, ctx: *const fq_ctx_struct);
    pub fn fq_reduce(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_add(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_sub(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_sub_one(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_neg(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_mul(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_mul_fmpz(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        x: *const fmpz,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_mul_si(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        x: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_mul_ui(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        x: mp_limb_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_sqr(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_inv(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn _fq_pow(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: *const fmpz,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_pow(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        e: *const fmpz,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_pow_ui(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        e: mp_limb_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_sqrt(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_pth_root(
        rop: *mut fmpz_poly_struct,
        op1: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_is_square(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> libc::c_int;
    pub fn fq_randtest(
        rop: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_randtest_dense(
        rop: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_randtest_not_zero(
        rop: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_rand(rop: *mut fmpz_poly_struct, state: *mut flint_rand_s, ctx: *const fq_ctx_struct);
    pub fn fq_rand_not_zero(
        rop: *mut fmpz_poly_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_equal(
        op1: *const fmpz_poly_struct,
        op2: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_is_zero(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> libc::c_int;
    pub fn fq_is_one(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> libc::c_int;
    pub fn fq_set(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_set_ui(rop: *mut fmpz_poly_struct, x: mp_limb_t, ctx: *const fq_ctx_struct);
    pub fn fq_set_si(rop: *mut fmpz_poly_struct, x: mp_limb_signed_t, ctx: *const fq_ctx_struct);
    pub fn fq_set_fmpz(rop: *mut fmpz_poly_struct, x: *const fmpz, ctx: *const fq_ctx_struct);
    pub fn fq_set_fmpz_poly(
        a: *mut fmpz_poly_struct,
        b: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_set_fmpz_mod_poly(
        a: *mut fmpz_poly_struct,
        b: *const fmpz_mod_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_get_fmpz(
        a: *mut fmpz,
        b: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_get_fmpz_poly(
        a: *mut fmpz_poly_struct,
        b: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_get_fmpz_mod_poly(
        a: *mut fmpz_mod_poly_struct,
        b: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_zero(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_one(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_swap(
        op1: *mut fmpz_poly_struct,
        op2: *mut fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_gen(rop: *mut fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_fprint(
        file: *mut FILE,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_print(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_fprint_pretty(
        file: *mut FILE,
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_print_pretty(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> libc::c_int;
    pub fn fq_get_str(op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct) -> *mut libc::c_char;
    pub fn fq_get_str_pretty(
        op: *const fmpz_poly_struct,
        ctx: *const fq_ctx_struct,
    ) -> *mut libc::c_char;
    pub fn _fq_trace(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_trace(rop: *mut fmpz, op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn _fq_frobenius(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_frobenius(
        rop: *mut fmpz_poly_struct,
        op: *const fmpz_poly_struct,
        e: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn _fq_norm(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_norm(rop: *mut fmpz, op: *const fmpz_poly_struct, ctx: *const fq_ctx_struct);
    pub fn fq_bit_pack(
        f: *mut fmpz,
        op: *const fmpz_poly_struct,
        bit_size: mp_limb_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn fq_bit_unpack(
        rop: *mut fmpz_poly_struct,
        f: *const fmpz,
        bit_size: mp_limb_t,
        ctx: *const fq_ctx_struct,
    );
    pub fn __fq_ctx_prime(p: *mut fmpz, ctx: *mut fq_ctx_struct);
}
