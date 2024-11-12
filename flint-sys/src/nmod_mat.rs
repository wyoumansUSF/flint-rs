/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use crate::flint::*;
use crate::nmod_types::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


pub const NMOD_MAT_MUL_TRANSPOSE_CUTOFF: u32 = 20;
pub const NMOD_MAT_SOLVE_TRI_ROWS_CUTOFF: u32 = 64;
pub const NMOD_MAT_SOLVE_TRI_COLS_CUTOFF: u32 = 64;
pub const NMOD_MAT_OPTIMAL_MODULUS_BITS: u32 = 59;
extern "C" {
    pub fn nmod_mat_set_mod(mat: *mut nmod_mat_struct, n: mp_limb_t);
    pub fn nmod_mat_init(
        mat: *mut nmod_mat_struct,
        rows: mp_limb_signed_t,
        cols: mp_limb_signed_t,
        n: mp_limb_t,
    );
    pub fn nmod_mat_init_set(mat: *mut nmod_mat_struct, src: *const nmod_mat_struct);
    pub fn nmod_mat_clear(mat: *mut nmod_mat_struct);
    pub fn nmod_mat_one(mat: *mut nmod_mat_struct);
    pub fn nmod_mat_swap(mat1: *mut nmod_mat_struct, mat2: *mut nmod_mat_struct);
    pub fn nmod_mat_window_init(
        window: *mut nmod_mat_struct,
        mat: *const nmod_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
    pub fn nmod_mat_window_clear(window: *mut nmod_mat_struct);
    pub fn nmod_mat_concat_horizontal(
        res: *mut nmod_mat_struct,
        mat1: *const nmod_mat_struct,
        mat2: *const nmod_mat_struct,
    );
    pub fn nmod_mat_concat_vertical(
        res: *mut nmod_mat_struct,
        mat1: *const nmod_mat_struct,
        mat2: *const nmod_mat_struct,
    );
    pub fn nmod_mat_randtest(mat: *mut nmod_mat_struct, state: *mut flint_rand_s);
    pub fn nmod_mat_randfull(mat: *mut nmod_mat_struct, state: *mut flint_rand_s);
    pub fn nmod_mat_randpermdiag(
        mat: *mut nmod_mat_struct,
        state: *mut flint_rand_s,
        diag: mp_srcptr,
        n: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_randrank(
        arg1: *mut nmod_mat_struct,
        state: *mut flint_rand_s,
        rank: mp_limb_signed_t,
    );
    pub fn nmod_mat_randops(
        mat: *mut nmod_mat_struct,
        state: *mut flint_rand_s,
        count: mp_limb_signed_t,
    );
    pub fn nmod_mat_randtril(
        mat: *mut nmod_mat_struct,
        state: *mut flint_rand_s,
        unit: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_randtriu(
        mat: *mut nmod_mat_struct,
        state: *mut flint_rand_s,
        unit: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_fprint_pretty(
        file: *mut FILE,
        mat: *const nmod_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_fprint(f: *mut FILE, mat: *const nmod_mat_struct) -> ::std::os::raw::c_int;
    pub fn nmod_mat_print_pretty(mat: *const nmod_mat_struct);
    pub fn nmod_mat_print(mat: *const nmod_mat_struct) -> ::std::os::raw::c_int;
    pub fn nmod_mat_equal(
        mat1: *const nmod_mat_struct,
        mat2: *const nmod_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_zero(mat: *mut nmod_mat_struct);
    pub fn nmod_mat_is_zero(mat: *const nmod_mat_struct) -> ::std::os::raw::c_int;
    pub fn nmod_mat_is_one(mat: *const nmod_mat_struct) -> ::std::os::raw::c_int;
    pub fn nmod_mat_is_zero_row(
        mat: *const nmod_mat_struct,
        i: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_set(B: *mut nmod_mat_struct, A: *const nmod_mat_struct);
    pub fn nmod_mat_transpose(B: *mut nmod_mat_struct, A: *const nmod_mat_struct);
    pub fn nmod_mat_add(
        C: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    );
    pub fn nmod_mat_sub(
        C: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    );
    pub fn nmod_mat_neg(B: *mut nmod_mat_struct, A: *const nmod_mat_struct);
    pub fn nmod_mat_scalar_mul(B: *mut nmod_mat_struct, A: *const nmod_mat_struct, c: mp_limb_t);
    pub fn nmod_mat_scalar_addmul_ui(
        dest: *mut nmod_mat_struct,
        X: *const nmod_mat_struct,
        Y: *const nmod_mat_struct,
        b: mp_limb_t,
    );
    pub fn nmod_mat_scalar_mul_fmpz(
        res: *mut nmod_mat_struct,
        M: *const nmod_mat_struct,
        c: *const fmpz,
    );
    pub fn nmod_mat_mul(
        C: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    );
    pub fn nmod_mat_mul_blas(
        C: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_mul_classical(
        C: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    );
    pub fn _nmod_mat_mul_classical_threaded_pool_op(
        D: *mut nmod_mat_struct,
        C: *const nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
        op: ::std::os::raw::c_int,
        threads: *mut thread_pool_handle,
        num_threads: mp_limb_signed_t,
    );
    pub fn nmod_mat_mul_classical_threaded(
        C: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    );
    pub fn nmod_mat_mul_strassen(
        C: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    );
    pub fn _nmod_mat_mul_classical_op(
        D: *mut nmod_mat_struct,
        C: *const nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
        op: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_addmul(
        D: *mut nmod_mat_struct,
        C: *const nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    );
    pub fn nmod_mat_submul(
        D: *mut nmod_mat_struct,
        C: *const nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    );
    pub fn nmod_mat_mul_nmod_vec(
        c: *mut mp_limb_t,
        A: *const nmod_mat_struct,
        b: *const mp_limb_t,
        blen: mp_limb_signed_t,
    );
    pub fn nmod_mat_mul_nmod_vec_ptr(
        c: *const *mut mp_limb_t,
        A: *const nmod_mat_struct,
        b: *const *const mp_limb_t,
        blen: mp_limb_signed_t,
    );
    pub fn nmod_mat_nmod_vec_mul(
        c: *mut mp_limb_t,
        a: *const mp_limb_t,
        alen: mp_limb_signed_t,
        B: *const nmod_mat_struct,
    );
    pub fn nmod_mat_nmod_vec_mul_ptr(
        c: *const *mut mp_limb_t,
        a: *const *const mp_limb_t,
        alen: mp_limb_signed_t,
        B: *const nmod_mat_struct,
    );
    pub fn _nmod_mat_pow(dest: *mut nmod_mat_struct, mat: *const nmod_mat_struct, pow: mp_limb_t);
    pub fn nmod_mat_pow(dest: *mut nmod_mat_struct, mat: *const nmod_mat_struct, pow: mp_limb_t);
    pub fn nmod_mat_trace(mat: *const nmod_mat_struct) -> mp_limb_t;
    pub fn _nmod_mat_det(A: *mut nmod_mat_struct) -> mp_limb_t;
    pub fn nmod_mat_det(A: *const nmod_mat_struct) -> mp_limb_t;
    pub fn _nmod_mat_det_howell(A: *mut nmod_mat_struct) -> mp_limb_t;
    pub fn nmod_mat_det_howell(A: *const nmod_mat_struct) -> mp_limb_t;
    pub fn nmod_mat_rank(A: *const nmod_mat_struct) -> mp_limb_signed_t;
    pub fn nmod_mat_inv(
        B: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_permute_rows(
        mat: *mut nmod_mat_struct,
        perm_act: *const mp_limb_signed_t,
        perm_store: *mut mp_limb_signed_t,
    );
    pub fn nmod_mat_solve_tril(
        X: *mut nmod_mat_struct,
        L: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
        unit: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_solve_tril_recursive(
        X: *mut nmod_mat_struct,
        L: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
        unit: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_solve_tril_classical(
        X: *mut nmod_mat_struct,
        L: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
        unit: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_solve_triu(
        X: *mut nmod_mat_struct,
        U: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
        unit: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_solve_triu_recursive(
        X: *mut nmod_mat_struct,
        U: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
        unit: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_solve_triu_classical(
        X: *mut nmod_mat_struct,
        U: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
        unit: ::std::os::raw::c_int,
    );
    pub fn nmod_mat_lu(
        P: *mut mp_limb_signed_t,
        A: *mut nmod_mat_struct,
        rank_check: ::std::os::raw::c_int,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_lu_classical(
        P: *mut mp_limb_signed_t,
        A: *mut nmod_mat_struct,
        rank_check: ::std::os::raw::c_int,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_lu_classical_delayed(
        P: *mut mp_limb_signed_t,
        A: *mut nmod_mat_struct,
        rank_check: ::std::os::raw::c_int,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_lu_recursive(
        P: *mut mp_limb_signed_t,
        A: *mut nmod_mat_struct,
        rank_check: ::std::os::raw::c_int,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_solve(
        X: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_solve_vec(
        x: mp_ptr,
        A: *const nmod_mat_struct,
        b: mp_srcptr,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_can_solve_inner(
        rank: *mut mp_limb_signed_t,
        prm: *mut mp_limb_signed_t,
        piv: *mut mp_limb_signed_t,
        X: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_can_solve(
        X: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
        B: *const nmod_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_mat_rref(A: *mut nmod_mat_struct) -> mp_limb_signed_t;
    pub fn _nmod_mat_rref(
        A: *mut nmod_mat_struct,
        pivots_nonpivots: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_rref_classical(A: *mut nmod_mat_struct) -> mp_limb_signed_t;
    pub fn _nmod_mat_rref_classical(
        A: *mut nmod_mat_struct,
        pivots_nonpivots: *mut mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_rref_storjohann(A: *mut nmod_mat_struct) -> mp_limb_signed_t;
    pub fn _nmod_mat_rref_storjohann(
        A: *mut nmod_mat_struct,
        pivots_nonpivots: *mut mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_reduce_row(
        M: *mut nmod_mat_struct,
        P: *mut mp_limb_signed_t,
        L: *mut mp_limb_signed_t,
        m: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_nullspace(
        X: *mut nmod_mat_struct,
        A: *const nmod_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn nmod_mat_strong_echelon_form(A: *mut nmod_mat_struct);
    pub fn nmod_mat_howell_form(A: *mut nmod_mat_struct) -> mp_limb_signed_t;
    pub fn nmod_mat_similarity(M: *mut nmod_mat_struct, r: mp_limb_signed_t, d: mp_limb_t);
    pub fn nmod_mat_set_entry(
        mat: *mut nmod_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        x: mp_limb_t,
    );
}
