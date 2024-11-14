/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::flint::*;
use crate::gr::*;


pub const GR_GENERIC_DEBUG_RINGS: u32 = 0;
pub const GR_PARSE_BALANCE_ADDITIONS: u32 = 1;
pub const GR_PARSE_RING_EXPONENTS: u32 = 2;
extern "C" {
    pub fn gr_generic_ctx_predicate(ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_generic_ctx_predicate_true(ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_generic_ctx_predicate_false(ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_generic_ctx_clear(ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_set_shallow(res: gr_ptr, x: gr_srcptr, ctx: *const gr_ctx_struct);
    pub fn gr_generic_write_n(
        out: *mut gr_stream_struct,
        x: gr_srcptr,
        n: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_randtest_not_zero(
        x: gr_ptr,
        state: *mut flint_rand_s,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_randtest_small(
        x: gr_ptr,
        state: *mut flint_rand_s,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_gens(vec: *mut gr_vec_struct, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_gens_single(vec: *mut gr_vec_struct, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_gens_recursive(
        vec: *mut gr_vec_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_is_zero(x: gr_srcptr, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_generic_is_one(x: gr_srcptr, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_generic_is_neg_one(x: gr_srcptr, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_generic_neg_one(res: gr_ptr, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_set_other(
        res: gr_ptr,
        x: gr_srcptr,
        xctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_set_fmpq(res: gr_ptr, y: *const fmpq, ctx: *mut gr_ctx_struct)
        -> libc::c_int;
    pub fn gr_generic_set_str_expr(
        res: gr_ptr,
        s: *const libc::c_char,
        flags: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_set_str(
        res: gr_ptr,
        s: *const libc::c_char,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_set_str_balance_additions(
        res: gr_ptr,
        s: *const libc::c_char,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_set_str_ring_exponents(
        res: gr_ptr,
        s: *const libc::c_char,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_add_fmpz(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_add_ui(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_add_si(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_add_fmpq(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_add_other(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_add(
        res: gr_ptr,
        x: gr_srcptr,
        x_ctx: *mut gr_ctx_struct,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_sub_ui(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_sub_si(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_sub_fmpz(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_sub_fmpq(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_sub_other(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_sub(
        res: gr_ptr,
        x: gr_srcptr,
        x_ctx: *mut gr_ctx_struct,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_mul_fmpz(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_mul_ui(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_mul_si(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_mul_fmpq(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_mul_other(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_mul(
        res: gr_ptr,
        x: gr_srcptr,
        x_ctx: *mut gr_ctx_struct,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_addmul(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_addmul_ui(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_addmul_si(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_addmul_fmpz(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_addmul_fmpq(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_addmul_other(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_submul(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_submul_ui(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_submul_si(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_submul_fmpz(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_submul_fmpq(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_submul_other(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_mul_two(res: gr_ptr, x: gr_srcptr, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_sqr(res: gr_ptr, x: gr_srcptr, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_mul_2exp_si(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_mul_2exp_fmpz(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_set_fmpz_2exp_fmpz(
        res: gr_ptr,
        x: *const fmpz,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_get_fmpz_2exp_fmpz(
        res1: *mut fmpz,
        res2: *mut fmpz,
        x: gr_ptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_set_fmpz_10exp_fmpz(
        res: gr_ptr,
        x: *const fmpz,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_inv(res: gr_ptr, x: gr_srcptr, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_is_invertible(x: gr_srcptr, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_generic_div_fmpz(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_div_ui(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_div_si(
        res: gr_ptr,
        x: gr_srcptr,
        y: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_div_fmpq(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_div_other(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_div(
        res: gr_ptr,
        x: gr_srcptr,
        x_ctx: *mut gr_ctx_struct,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_divexact(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_fmpz_sliding(
        f: gr_ptr,
        g: gr_srcptr,
        pow: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_ui_sliding(
        f: gr_ptr,
        g: gr_srcptr,
        pow: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_fmpz_binexp(
        res: gr_ptr,
        x: gr_srcptr,
        exp: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_ui_binexp(
        res: gr_ptr,
        x: gr_srcptr,
        e: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_fmpz(
        res: gr_ptr,
        x: gr_srcptr,
        e: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_si(
        res: gr_ptr,
        x: gr_srcptr,
        e: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_ui(
        res: gr_ptr,
        x: gr_srcptr,
        e: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_fmpq(
        res: gr_ptr,
        x: gr_srcptr,
        y: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_pow_other(
        res: gr_ptr,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_pow(
        res: gr_ptr,
        x: gr_srcptr,
        x_ctx: *mut gr_ctx_struct,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_numerator(res: gr_ptr, x: gr_srcptr, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_denominator(
        res: gr_ptr,
        x: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_is_square(x: gr_srcptr, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_generic_sqrt(res: gr_ptr, x: gr_srcptr, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_rsqrt(res: gr_ptr, x: gr_srcptr, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_generic_cmp(
        res: *mut libc::c_int,
        x: gr_srcptr,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_cmpabs(
        res: *mut libc::c_int,
        x: gr_srcptr,
        y: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_cmp_other(
        res: *mut libc::c_int,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_cmpabs_other(
        res: *mut libc::c_int,
        x: gr_srcptr,
        y: gr_srcptr,
        y_ctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_bernoulli_ui(
        res: gr_ptr,
        n: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_bernoulli_fmpz(
        res: gr_ptr,
        n: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_bernoulli_vec(
        res: gr_ptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_eulernum_ui(
        res: gr_ptr,
        n: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_eulernum_fmpz(
        res: gr_ptr,
        n: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_eulernum_vec(
        res: gr_ptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_stirling_s1u_uiui(
        res: gr_ptr,
        x: mp_limb_t,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_stirling_s1_uiui(
        res: gr_ptr,
        x: mp_limb_t,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_stirling_s2_uiui(
        res: gr_ptr,
        x: mp_limb_t,
        y: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_stirling_s1u_ui_vec(
        res: gr_ptr,
        x: mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_stirling_s1_ui_vec(
        res: gr_ptr,
        x: mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_stirling_s2_ui_vec(
        res: gr_ptr,
        x: mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_init(vec: gr_ptr, len: mp_limb_signed_t, ctx: *mut gr_ctx_struct);
    pub fn gr_generic_vec_clear(vec: gr_ptr, len: mp_limb_signed_t, ctx: *mut gr_ctx_struct);
    pub fn gr_generic_vec_swap(
        vec1: gr_ptr,
        vec2: gr_ptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    );
    pub fn gr_generic_vec_zero(
        vec: gr_ptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_set(
        res: gr_ptr,
        src: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_neg(
        res: gr_ptr,
        src: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_normalise(
        res: *mut mp_limb_signed_t,
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_normalise_weak(
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> mp_limb_signed_t;
    pub fn gr_generic_vec_mul_scalar_2exp_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_scalar_addmul(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_scalar_submul(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_scalar_addmul_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_scalar_submul_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_equal(
        vec1: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> truth_t;
    pub fn gr_generic_vec_is_zero(
        vec: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_dot(
        res: gr_ptr,
        initial: gr_srcptr,
        subtract: libc::c_int,
        vec1: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_dot_rev(
        res: gr_ptr,
        initial: gr_srcptr,
        subtract: libc::c_int,
        vec1: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_dot_ui(
        res: gr_ptr,
        initial: gr_srcptr,
        subtract: libc::c_int,
        vec1: gr_srcptr,
        vec2: *const mp_limb_t,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_dot_si(
        res: gr_ptr,
        initial: gr_srcptr,
        subtract: libc::c_int,
        vec1: gr_srcptr,
        vec2: *const mp_limb_signed_t,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_dot_fmpz(
        res: gr_ptr,
        initial: gr_srcptr,
        subtract: libc::c_int,
        vec1: gr_srcptr,
        vec2: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_set_powers(
        res: gr_ptr,
        x: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_reciprocals(
        res: gr_ptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_add(
        res: gr_ptr,
        src1: gr_srcptr,
        src2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_sub(
        res: gr_ptr,
        src1: gr_srcptr,
        src2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_mul(
        res: gr_ptr,
        src1: gr_srcptr,
        src2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_div(
        res: gr_ptr,
        src1: gr_srcptr,
        src2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_divexact(
        res: gr_ptr,
        src1: gr_srcptr,
        src2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_pow(
        res: gr_ptr,
        src1: gr_srcptr,
        src2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_add_scalar(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_sub_scalar(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_mul_scalar(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_div_scalar(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_divexact_scalar(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_pow_scalar(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_add_scalar_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_sub_scalar_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_mul_scalar_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_div_scalar_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_divexact_scalar_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_pow_scalar_si(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_add_scalar_ui(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_sub_scalar_ui(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_mul_scalar_ui(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_div_scalar_ui(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_divexact_scalar_ui(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_pow_scalar_ui(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_add_scalar_fmpz(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_sub_scalar_fmpz(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_mul_scalar_fmpz(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_div_scalar_fmpz(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_divexact_scalar_fmpz(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_pow_scalar_fmpz(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_add_scalar_fmpq(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_sub_scalar_fmpq(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_mul_scalar_fmpq(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_div_scalar_fmpq(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_divexact_scalar_fmpq(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_pow_scalar_fmpq(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_add_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_sub_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_mul_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_div_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_divexact_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_pow_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_add_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        vec3: gr_srcptr,
        ctx3: *mut gr_ctx_struct,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_sub_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        vec3: gr_srcptr,
        ctx3: *mut gr_ctx_struct,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_mul_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        vec3: gr_srcptr,
        ctx3: *mut gr_ctx_struct,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_div_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        vec3: gr_srcptr,
        ctx3: *mut gr_ctx_struct,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_divexact_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        vec3: gr_srcptr,
        ctx3: *mut gr_ctx_struct,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_pow_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        vec3: gr_srcptr,
        ctx3: *mut gr_ctx_struct,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_add_vec(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        ctx2: *mut gr_ctx_struct,
        vec3: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_sub_vec(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        ctx2: *mut gr_ctx_struct,
        vec3: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_mul_vec(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        ctx2: *mut gr_ctx_struct,
        vec3: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_div_vec(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        ctx2: *mut gr_ctx_struct,
        vec3: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_divexact_vec(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        ctx2: *mut gr_ctx_struct,
        vec3: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_other_pow_vec(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        ctx2: *mut gr_ctx_struct,
        vec3: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_add_scalar_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_sub_scalar_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_mul_scalar_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_div_scalar_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_divexact_scalar_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_vec_pow_scalar_other(
        vec1: gr_ptr,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_other_add_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_other_sub_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_other_mul_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_other_div_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_other_divexact_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_generic_scalar_other_pow_vec(
        vec1: gr_ptr,
        c: gr_srcptr,
        cctx: *mut gr_ctx_struct,
        vec2: gr_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
}
