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
use crate::qfb::*;
use crate::qqbar::*;
use crate::qsieve::*;
use crate::templates::*;
use crate::thread_pool::*;
use crate::thread_support::*;
use crate::ulong_extras::*;


#[repr(C)]
pub struct ca_mat_struct {
    pub entries: ca_ptr,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut ca_ptr,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ca_mat_struct"][::std::mem::size_of::<ca_mat_struct>() - 32usize];
    ["Alignment of ca_mat_struct"][::std::mem::align_of::<ca_mat_struct>() - 8usize];
    ["Offset of field: ca_mat_struct::entries"]
        [::std::mem::offset_of!(ca_mat_struct, entries) - 0usize];
    ["Offset of field: ca_mat_struct::r"][::std::mem::offset_of!(ca_mat_struct, r) - 8usize];
    ["Offset of field: ca_mat_struct::c"][::std::mem::offset_of!(ca_mat_struct, c) - 16usize];
    ["Offset of field: ca_mat_struct::rows"][::std::mem::offset_of!(ca_mat_struct, rows) - 24usize];
};
impl Default for ca_mat_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ca_mat_t = [ca_mat_struct; 1usize];
extern "C" {
    #[link_name = "ca_mat_entry_ptr__extern"]
    pub fn ca_mat_entry_ptr(
        mat: *mut ca_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> ca_ptr;
    pub fn ca_mat_init(
        mat: *mut ca_mat_struct,
        r: mp_limb_signed_t,
        c: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_clear(mat: *mut ca_mat_struct, ctx: *mut ca_ctx_struct);
    #[link_name = "ca_mat_swap__extern"]
    pub fn ca_mat_swap(mat1: *mut ca_mat_struct, mat2: *mut ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_window_init(
        window: *mut ca_mat_struct,
        mat: *const ca_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_window_clear__extern"]
    pub fn ca_mat_window_clear(window: *mut ca_mat_struct, ctx: *mut ca_ctx_struct);
    #[link_name = "ca_mat_is_empty__extern"]
    pub fn ca_mat_is_empty(mat: *const ca_mat_struct) -> libc::c_int;
    #[link_name = "ca_mat_is_square__extern"]
    pub fn ca_mat_is_square(mat: *const ca_mat_struct) -> libc::c_int;
    pub fn ca_mat_set(dest: *mut ca_mat_struct, src: *const ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_set_fmpz_mat(
        dest: *mut ca_mat_struct,
        src: *const fmpz_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_set_fmpq_mat(
        dest: *mut ca_mat_struct,
        src: *const fmpq_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_set_ca(y: *mut ca_mat_struct, x: *const ca_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_transfer(
        res: *mut ca_mat_struct,
        res_ctx: *mut ca_ctx_struct,
        src: *const ca_mat_struct,
        src_ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_randtest(
        mat: *mut ca_mat_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_randtest_rational(
        mat: *mut ca_mat_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_randops(
        mat: *mut ca_mat_struct,
        state: *mut flint_rand_s,
        count: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_print(mat: *const ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_printn(
        mat: *const ca_mat_struct,
        digits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_zero(mat: *mut ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_one(mat: *mut ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_ones(mat: *mut ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_pascal(mat: *mut ca_mat_struct, triangular: libc::c_int, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_stirling(mat: *mut ca_mat_struct, kind: libc::c_int, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_hilbert(mat: *mut ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_dft(res: *mut ca_mat_struct, type_: libc::c_int, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_check_equal(
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_check_is_zero(A: *const ca_mat_struct, ctx: *mut ca_ctx_struct) -> truth_t;
    pub fn ca_mat_check_is_one(A: *const ca_mat_struct, ctx: *mut ca_ctx_struct) -> truth_t;
    pub fn ca_mat_transpose(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_conj(B: *mut ca_mat_struct, A: *const ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_conj_transpose(
        mat1: *mut ca_mat_struct,
        mat2: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_add_ca(
        y: *mut ca_mat_struct,
        a: *const ca_mat_struct,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_sub_ca(
        y: *mut ca_mat_struct,
        a: *const ca_mat_struct,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_addmul_ca(
        y: *mut ca_mat_struct,
        a: *const ca_mat_struct,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_submul_ca(
        y: *mut ca_mat_struct,
        a: *const ca_mat_struct,
        x: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_neg(dest: *mut ca_mat_struct, src: *const ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_add(
        res: *mut ca_mat_struct,
        mat1: *const ca_mat_struct,
        mat2: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_sub(
        res: *mut ca_mat_struct,
        mat1: *const ca_mat_struct,
        mat2: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_mul(
        C: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_mul_classical(
        C: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_mul_same_nf(
        C: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        K: *mut ca_field_struct,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_mul_si__extern"]
    pub fn ca_mat_mul_si(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        c: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_mul_fmpz__extern"]
    pub fn ca_mat_mul_fmpz(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        c: *const fmpz,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_mul_fmpq__extern"]
    pub fn ca_mat_mul_fmpq(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        c: *const fmpq,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_mul_ca__extern"]
    pub fn ca_mat_mul_ca(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        c: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_div_si__extern"]
    pub fn ca_mat_div_si(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        c: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_div_fmpz__extern"]
    pub fn ca_mat_div_fmpz(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        c: *const fmpz,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_div_fmpq__extern"]
    pub fn ca_mat_div_fmpq(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        c: *const fmpq,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_div_ca__extern"]
    pub fn ca_mat_div_ca(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        c: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_mat_sqr__extern"]
    pub fn ca_mat_sqr(res: *mut ca_mat_struct, A: *const ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_pow_ui_binexp(
        B: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        exp: mp_limb_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_mat_ca_poly_evaluate(
        y: *mut ca_mat_struct,
        poly: ca_srcptr,
        len: mp_limb_signed_t,
        x: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_ca_poly_evaluate(
        res: *mut ca_mat_struct,
        f: *const ca_poly_struct,
        a: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_trace(trace: *mut ca_struct, mat: *const ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_find_pivot(
        pivot_row: *mut mp_limb_signed_t,
        mat: *mut ca_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        column: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    #[link_name = "_ca_mat_swap_rows__extern"]
    pub fn _ca_mat_swap_rows(
        mat: *mut ca_mat_struct,
        perm: *mut mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
    );
    pub fn ca_mat_lu_classical(
        rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        rank_check: libc::c_int,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_lu_recursive(
        rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        rank_check: libc::c_int,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_lu(
        rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        rank_check: libc::c_int,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_fflu(
        rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut ca_mat_struct,
        den: *mut ca_struct,
        A: *const ca_mat_struct,
        rank_check: libc::c_int,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_rref_fflu(
        rank: *mut mp_limb_signed_t,
        R: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_rref_lu(
        rank: *mut mp_limb_signed_t,
        R: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_rref(
        rank: *mut mp_limb_signed_t,
        R: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_nonsingular_lu(
        P: *mut mp_limb_signed_t,
        LU: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_nonsingular_fflu(
        P: *mut mp_limb_signed_t,
        LU: *mut ca_mat_struct,
        den: *mut ca_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_nonsingular_solve_adjugate(
        X: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_nonsingular_solve_fflu(
        X: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_nonsingular_solve_lu(
        X: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_nonsingular_solve(
        X: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_inv(
        X: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_solve_tril_classical(
        X: *mut ca_mat_struct,
        L: *const ca_mat_struct,
        B: *const ca_mat_struct,
        unit: libc::c_int,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_solve_tril_recursive(
        X: *mut ca_mat_struct,
        L: *const ca_mat_struct,
        B: *const ca_mat_struct,
        unit: libc::c_int,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_solve_tril(
        X: *mut ca_mat_struct,
        L: *const ca_mat_struct,
        B: *const ca_mat_struct,
        unit: libc::c_int,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_solve_triu_classical(
        X: *mut ca_mat_struct,
        U: *const ca_mat_struct,
        B: *const ca_mat_struct,
        unit: libc::c_int,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_solve_triu_recursive(
        X: *mut ca_mat_struct,
        U: *const ca_mat_struct,
        B: *const ca_mat_struct,
        unit: libc::c_int,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_solve_triu(
        X: *mut ca_mat_struct,
        U: *const ca_mat_struct,
        B: *const ca_mat_struct,
        unit: libc::c_int,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_solve_lu_precomp(
        X: *mut ca_mat_struct,
        perm: *const mp_limb_signed_t,
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_solve_fflu_precomp(
        X: *mut ca_mat_struct,
        perm: *const mp_limb_signed_t,
        A: *const ca_mat_struct,
        den: *const ca_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_rank(
        rank: *mut mp_limb_signed_t,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_right_kernel(
        X: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_det_berkowitz(
        det: *mut ca_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_det_lu(
        det: *mut ca_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_det_bareiss(
        det: *mut ca_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_det_cofactor(
        det: *mut ca_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_det(det: *mut ca_struct, A: *const ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_adjugate_cofactor(
        adj: *mut ca_mat_struct,
        det: *mut ca_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_adjugate_charpoly(
        adj: *mut ca_mat_struct,
        det: *mut ca_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_adjugate(
        adj: *mut ca_mat_struct,
        det: *mut ca_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_mat_charpoly_berkowitz(
        cp: ca_ptr,
        mat: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_charpoly_berkowitz(
        cp: *mut ca_poly_struct,
        mat: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_mat_charpoly_danilevsky(
        p: ca_ptr,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_charpoly_danilevsky(
        cp: *mut ca_poly_struct,
        mat: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn _ca_mat_charpoly(cp: ca_ptr, mat: *const ca_mat_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_mat_charpoly(
        cp: *mut ca_poly_struct,
        mat: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_companion(
        A: *mut ca_mat_struct,
        poly: *const ca_poly_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_eigenvalues(
        lambda: *mut ca_vec_struct,
        exp: *mut mp_limb_t,
        mat: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_diagonalization(
        D: *mut ca_mat_struct,
        P: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn ca_mat_set_jordan_blocks(
        mat: *mut ca_mat_struct,
        lambda: *const ca_vec_struct,
        num_blocks: mp_limb_signed_t,
        block_lambda: *mut mp_limb_signed_t,
        block_size: *mut mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_mat_jordan_blocks(
        lambda: *mut ca_vec_struct,
        num_blocks: *mut mp_limb_signed_t,
        block_lambda: *mut mp_limb_signed_t,
        block_size: *mut mp_limb_signed_t,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_jordan_transformation(
        mat: *mut ca_mat_struct,
        lambda: *const ca_vec_struct,
        num_blocks: mp_limb_signed_t,
        block_lambda: *mut mp_limb_signed_t,
        block_size: *mut mp_limb_signed_t,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_jordan_form(
        J: *mut ca_mat_struct,
        P: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_exp(
        res: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    pub fn ca_mat_log(
        res: *mut ca_mat_struct,
        A: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    pub fn _ca_mat_same_field(A: *const ca_mat_struct, ctx: *mut ca_ctx_struct) -> ca_field_ptr;
    pub fn _ca_mat_same_field2(
        A: *const ca_mat_struct,
        B: *const ca_mat_struct,
        ctx: *mut ca_ctx_struct,
    ) -> ca_field_ptr;
}
