/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


pub const ACB_THETA_LOW_PREC: u32 = 32;
pub const ACB_THETA_G2_COV_NB: u32 = 26;
#[repr(C)]
pub struct acb_theta_eld_struct {
    pub dim: mp_limb_signed_t,
    pub ambient_dim: mp_limb_signed_t,
    pub last_coords: *mut mp_limb_signed_t,
    pub min: mp_limb_signed_t,
    pub mid: mp_limb_signed_t,
    pub max: mp_limb_signed_t,
    pub nr: mp_limb_signed_t,
    pub nl: mp_limb_signed_t,
    pub rchildren: *mut acb_theta_eld_struct,
    pub lchildren: *mut acb_theta_eld_struct,
    pub nb_pts: mp_limb_signed_t,
    pub nb_border: mp_limb_signed_t,
    pub box_: *mut mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of acb_theta_eld_struct"][::std::mem::size_of::<acb_theta_eld_struct>() - 104usize];
    ["Alignment of acb_theta_eld_struct"][::std::mem::align_of::<acb_theta_eld_struct>() - 8usize];
    ["Offset of field: acb_theta_eld_struct::dim"]
        [::std::mem::offset_of!(acb_theta_eld_struct, dim) - 0usize];
    ["Offset of field: acb_theta_eld_struct::ambient_dim"]
        [::std::mem::offset_of!(acb_theta_eld_struct, ambient_dim) - 8usize];
    ["Offset of field: acb_theta_eld_struct::last_coords"]
        [::std::mem::offset_of!(acb_theta_eld_struct, last_coords) - 16usize];
    ["Offset of field: acb_theta_eld_struct::min"]
        [::std::mem::offset_of!(acb_theta_eld_struct, min) - 24usize];
    ["Offset of field: acb_theta_eld_struct::mid"]
        [::std::mem::offset_of!(acb_theta_eld_struct, mid) - 32usize];
    ["Offset of field: acb_theta_eld_struct::max"]
        [::std::mem::offset_of!(acb_theta_eld_struct, max) - 40usize];
    ["Offset of field: acb_theta_eld_struct::nr"]
        [::std::mem::offset_of!(acb_theta_eld_struct, nr) - 48usize];
    ["Offset of field: acb_theta_eld_struct::nl"]
        [::std::mem::offset_of!(acb_theta_eld_struct, nl) - 56usize];
    ["Offset of field: acb_theta_eld_struct::rchildren"]
        [::std::mem::offset_of!(acb_theta_eld_struct, rchildren) - 64usize];
    ["Offset of field: acb_theta_eld_struct::lchildren"]
        [::std::mem::offset_of!(acb_theta_eld_struct, lchildren) - 72usize];
    ["Offset of field: acb_theta_eld_struct::nb_pts"]
        [::std::mem::offset_of!(acb_theta_eld_struct, nb_pts) - 80usize];
    ["Offset of field: acb_theta_eld_struct::nb_border"]
        [::std::mem::offset_of!(acb_theta_eld_struct, nb_border) - 88usize];
    ["Offset of field: acb_theta_eld_struct::box_"]
        [::std::mem::offset_of!(acb_theta_eld_struct, box_) - 96usize];
};
impl Default for acb_theta_eld_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type acb_theta_eld_t = [acb_theta_eld_struct; 1usize];
pub type acb_theta_naive_worker_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: acb_ptr,
        arg2: acb_srcptr,
        arg3: acb_srcptr,
        arg4: *const mp_limb_signed_t,
        arg5: mp_limb_signed_t,
        arg6: *const acb_struct,
        arg7: *const mp_limb_signed_t,
        arg8: mp_limb_signed_t,
        arg9: mp_limb_signed_t,
        arg10: mp_limb_signed_t,
        arg11: mp_limb_signed_t,
    ),
>;
pub type acb_theta_ql_worker_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: acb_ptr,
        arg2: acb_srcptr,
        arg3: acb_srcptr,
        arg4: arb_srcptr,
        arg5: arb_srcptr,
        arg6: *const acb_mat_struct,
        arg7: mp_limb_signed_t,
        arg8: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn sp2gz_set_blocks(
        mat: *mut fmpz_mat_struct,
        alpha: *const fmpz_mat_struct,
        beta: *const fmpz_mat_struct,
        gamma: *const fmpz_mat_struct,
        delta: *const fmpz_mat_struct,
    );
    pub fn sp2gz_j(mat: *mut fmpz_mat_struct);
    pub fn sp2gz_block_diag(mat: *mut fmpz_mat_struct, U: *const fmpz_mat_struct);
    pub fn sp2gz_trig(mat: *mut fmpz_mat_struct, S: *const fmpz_mat_struct);
    pub fn sp2gz_embed(res: *mut fmpz_mat_struct, mat: *const fmpz_mat_struct);
    pub fn sp2gz_restrict(res: *mut fmpz_mat_struct, mat: *const fmpz_mat_struct);
    pub fn sp2gz_nb_fundamental(g: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn sp2gz_fundamental(mat: *mut fmpz_mat_struct, j: mp_limb_signed_t);
    pub fn sp2gz_is_correct(mat: *const fmpz_mat_struct) -> ::std::os::raw::c_int;
    pub fn sp2gz_is_j(mat: *const fmpz_mat_struct) -> ::std::os::raw::c_int;
    pub fn sp2gz_is_block_diag(mat: *const fmpz_mat_struct) -> ::std::os::raw::c_int;
    pub fn sp2gz_is_trig(mat: *const fmpz_mat_struct) -> ::std::os::raw::c_int;
    pub fn sp2gz_is_embedded(
        res: *mut fmpz_mat_struct,
        mat: *const fmpz_mat_struct,
    ) -> ::std::os::raw::c_int;
    pub fn sp2gz_inv(inv: *mut fmpz_mat_struct, mat: *const fmpz_mat_struct);
    pub fn sp2gz_decompose(
        nb: *mut mp_limb_signed_t,
        mat: *const fmpz_mat_struct,
    ) -> *mut fmpz_mat_struct;
    pub fn sp2gz_randtest(
        mat: *mut fmpz_mat_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
    );
    pub fn acb_siegel_cocycle(
        c: *mut acb_mat_struct,
        mat: *const fmpz_mat_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_siegel_transform_cocycle_inv(
        w: *mut acb_mat_struct,
        c: *mut acb_mat_struct,
        cinv: *mut acb_mat_struct,
        mat: *const fmpz_mat_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_siegel_transform(
        w: *mut acb_mat_struct,
        mat: *const fmpz_mat_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_siegel_transform_z(
        r: acb_ptr,
        w: *mut acb_mat_struct,
        mat: *const fmpz_mat_struct,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_siegel_cho(
        C: *mut arb_mat_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_siegel_yinv(
        Yinv: *mut arb_mat_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_siegel_reduce(
        mat: *mut fmpz_mat_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_siegel_is_reduced(
        tau: *const acb_mat_struct,
        tol_exp: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_siegel_randtest(
        tau: *mut acb_mat_struct,
        state: *mut flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn acb_siegel_randtest_reduced(
        tau: *mut acb_mat_struct,
        state: *mut flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn acb_siegel_randtest_vec(
        z: acb_ptr,
        state: *mut flint_rand_s,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_char_get_slong(n: *mut mp_limb_signed_t, a: mp_limb_t, g: mp_limb_signed_t);
    pub fn acb_theta_char_get_a(n: *const mp_limb_signed_t, g: mp_limb_signed_t) -> mp_limb_t;
    pub fn acb_theta_char_get_arb(v: arb_ptr, a: mp_limb_t, g: mp_limb_signed_t);
    pub fn acb_theta_char_get_acb(v: acb_ptr, a: mp_limb_t, g: mp_limb_signed_t);
    pub fn acb_theta_char_dot(a: mp_limb_t, b: mp_limb_t, g: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn acb_theta_char_dot_slong(
        a: mp_limb_t,
        n: *const mp_limb_signed_t,
        g: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_theta_char_dot_acb(
        x: *mut acb_struct,
        a: mp_limb_t,
        z: acb_srcptr,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_char_is_even(ab: mp_limb_t, g: mp_limb_signed_t) -> ::std::os::raw::c_int;
    pub fn acb_theta_char_is_goepel(
        ch1: mp_limb_t,
        ch2: mp_limb_t,
        ch3: mp_limb_t,
        ch4: mp_limb_t,
        g: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_theta_char_is_syzygous(
        ch1: mp_limb_t,
        ch2: mp_limb_t,
        ch3: mp_limb_t,
        g: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_theta_eld_init(
        E: *mut acb_theta_eld_struct,
        d: mp_limb_signed_t,
        g: mp_limb_signed_t,
    );
    pub fn acb_theta_eld_clear(E: *mut acb_theta_eld_struct);
    pub fn acb_theta_eld_set(
        E: *mut acb_theta_eld_struct,
        C: *const arb_mat_struct,
        R2: *const arf_struct,
        v: arb_srcptr,
    ) -> ::std::os::raw::c_int;
    pub fn acb_theta_eld_points(pts: *mut mp_limb_signed_t, E: *const acb_theta_eld_struct);
    pub fn acb_theta_eld_border(pts: *mut mp_limb_signed_t, E: *const acb_theta_eld_struct);
    pub fn acb_theta_eld_contains(
        E: *const acb_theta_eld_struct,
        pt: *const mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_theta_eld_print(E: *const acb_theta_eld_struct);
    pub fn acb_theta_naive_radius(
        R2: *mut arf_struct,
        eps: *mut arf_struct,
        C: *const arb_mat_struct,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_naive_reduce(
        v: arb_ptr,
        new_zs: acb_ptr,
        as_: arb_ptr,
        cs: acb_ptr,
        us: arb_ptr,
        zs: acb_srcptr,
        nb: mp_limb_signed_t,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_naive_term(
        res: *mut acb_struct,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        tup: *const mp_limb_signed_t,
        n: *const mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_naive_worker(
        th: acb_ptr,
        len: mp_limb_signed_t,
        zs: acb_srcptr,
        nb: mp_limb_signed_t,
        tau: *const acb_mat_struct,
        E: *const acb_theta_eld_struct,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        worker: acb_theta_naive_worker_t,
    );
    pub fn acb_theta_naive_00(
        th: acb_ptr,
        zs: acb_srcptr,
        nb: mp_limb_signed_t,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_naive_0b(
        th: acb_ptr,
        zs: acb_srcptr,
        nb: mp_limb_signed_t,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_naive_fixed_ab(
        th: acb_ptr,
        ab: mp_limb_t,
        zs: acb_srcptr,
        nb: mp_limb_signed_t,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_naive_fixed_a(
        th: acb_ptr,
        a: mp_limb_t,
        zs: acb_srcptr,
        nb: mp_limb_signed_t,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_naive_all(
        th: acb_ptr,
        zs: acb_srcptr,
        nb: mp_limb_signed_t,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_nb(ord: mp_limb_signed_t, g: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn acb_theta_jet_total_order(
        tup: *const mp_limb_signed_t,
        g: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_theta_jet_tuples(
        tups: *mut mp_limb_signed_t,
        ord: mp_limb_signed_t,
        g: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_index(
        tup: *const mp_limb_signed_t,
        g: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_theta_jet_mul(
        res: acb_ptr,
        v1: acb_srcptr,
        v2: acb_srcptr,
        ord: mp_limb_signed_t,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_compose(
        res: acb_ptr,
        v: acb_srcptr,
        N: *const acb_mat_struct,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_exp_pi_i(
        res: acb_ptr,
        a: arb_srcptr,
        ord: mp_limb_signed_t,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_naive_radius(
        R2: *mut arf_struct,
        eps: *mut arf_struct,
        C: *const arb_mat_struct,
        v: arb_srcptr,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_naive_00(
        dth: acb_ptr,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_naive_fixed_ab(
        dth: acb_ptr,
        ab: mp_limb_t,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_naive_all(
        dth: acb_ptr,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_error_bounds(
        err: arb_ptr,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        dth: acb_srcptr,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_dist_pt(
        d: *mut arb_struct,
        v: arb_srcptr,
        C: *const arb_mat_struct,
        n: *const mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_dist_lat(
        d: *mut arb_struct,
        v: arb_srcptr,
        C: *const arb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_dist_a0(
        d: arb_ptr,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_dist_addprec(d: *const arb_struct) -> mp_limb_signed_t;
    pub fn acb_theta_agm_hadamard(
        res: acb_ptr,
        a: acb_srcptr,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_agm_sqrt(
        res: acb_ptr,
        a: acb_srcptr,
        roots: acb_srcptr,
        nb: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_agm_mul(
        res: acb_ptr,
        a1: acb_srcptr,
        a2: acb_srcptr,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_agm_mul_tight(
        res: acb_ptr,
        a0: acb_srcptr,
        a: acb_srcptr,
        d0: arb_srcptr,
        d: arb_srcptr,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_ql_a0_naive(
        th: acb_ptr,
        t: acb_srcptr,
        z: acb_srcptr,
        d0: arb_srcptr,
        d: arb_srcptr,
        tau: *const acb_mat_struct,
        guard: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_theta_ql_a0_split(
        th: acb_ptr,
        t: acb_srcptr,
        z: acb_srcptr,
        d: arb_srcptr,
        tau: *const acb_mat_struct,
        s: mp_limb_signed_t,
        guard: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        worker: acb_theta_ql_worker_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_theta_ql_a0_steps(
        th: acb_ptr,
        t: acb_srcptr,
        z: acb_srcptr,
        d0: arb_srcptr,
        d: arb_srcptr,
        tau: *const acb_mat_struct,
        nb_steps: mp_limb_signed_t,
        s: mp_limb_signed_t,
        guard: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        worker: acb_theta_ql_worker_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_theta_ql_a0_nb_steps(
        C: *const arb_mat_struct,
        s: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_theta_ql_a0(
        th: acb_ptr,
        t: acb_srcptr,
        z: acb_srcptr,
        d0: arb_srcptr,
        d: arb_srcptr,
        tau: *const acb_mat_struct,
        guard: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_theta_ql_reduce(
        x: acb_ptr,
        c: *mut acb_struct,
        u: *mut arb_struct,
        n1: *mut mp_limb_signed_t,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_theta_ql_all(
        th: acb_ptr,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        sqr: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_ql_bounds(
        c: *mut arb_struct,
        rho: *mut arb_struct,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        ord: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_ql_radius(
        eps: *mut arf_struct,
        err: *mut arf_struct,
        c: *const arb_struct,
        rho: *const arb_struct,
        ord: mp_limb_signed_t,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_ql_finite_diff(
        dth: acb_ptr,
        eps: *const arf_struct,
        err: *const arf_struct,
        rho: *const arb_struct,
        val: acb_srcptr,
        ord: mp_limb_signed_t,
        g: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_ql_all(
        dth: acb_ptr,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_transform_char(
        e: *mut mp_limb_signed_t,
        mat: *const fmpz_mat_struct,
        ab: mp_limb_t,
    ) -> mp_limb_t;
    pub fn acb_theta_transform_sqrtdet(
        res: *mut acb_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_transform_kappa(
        sqrtdet: *mut acb_struct,
        mat: *const fmpz_mat_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_theta_transform_kappa2(mat: *const fmpz_mat_struct) -> mp_limb_signed_t;
    pub fn acb_theta_transform_proj(
        res: acb_ptr,
        mat: *const fmpz_mat_struct,
        th: acb_srcptr,
        sqr: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_all(
        th: acb_ptr,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        sqr: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_jet_all(
        dth: acb_ptr,
        z: acb_srcptr,
        tau: *const acb_mat_struct,
        ord: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_g2_jet_naive_1(
        dth: acb_ptr,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_g2_detk_symj(
        res: *mut acb_poly_struct,
        m: *const acb_mat_struct,
        f: *const acb_poly_struct,
        k: mp_limb_signed_t,
        j: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_g2_transvectant(
        res: *mut acb_poly_struct,
        g: *const acb_poly_struct,
        h: *const acb_poly_struct,
        m: mp_limb_signed_t,
        n: mp_limb_signed_t,
        k: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_g2_transvectant_lead(
        r: *mut acb_struct,
        g: *const acb_poly_struct,
        h: *const acb_poly_struct,
        m: mp_limb_signed_t,
        n: mp_limb_signed_t,
        k: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_g2_character(mat: *const fmpz_mat_struct) -> mp_limb_signed_t;
    pub fn acb_theta_g2_psi4(res: *mut acb_struct, th2: acb_srcptr, prec: mp_limb_signed_t);
    pub fn acb_theta_g2_psi6(res: *mut acb_struct, th2: acb_srcptr, prec: mp_limb_signed_t);
    pub fn acb_theta_g2_chi10(res: *mut acb_struct, th2: acb_srcptr, prec: mp_limb_signed_t);
    pub fn acb_theta_g2_chi12(res: *mut acb_struct, th2: acb_srcptr, prec: mp_limb_signed_t);
    pub fn acb_theta_g2_chi5(res: *mut acb_struct, th: acb_srcptr, prec: mp_limb_signed_t);
    pub fn acb_theta_g2_chi35(res: *mut acb_struct, th: acb_srcptr, prec: mp_limb_signed_t);
    pub fn acb_theta_g2_chi3_6(res: *mut acb_poly_struct, dth: acb_srcptr, prec: mp_limb_signed_t);
    pub fn acb_theta_g2_sextic(
        res: *mut acb_poly_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_g2_sextic_chi5(
        res: *mut acb_poly_struct,
        chi5: *mut acb_struct,
        tau: *const acb_mat_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_g2_covariants(
        res: *mut acb_poly_struct,
        f: *const acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_theta_g2_covariants_lead(
        res: acb_ptr,
        f: *const acb_poly_struct,
        prec: mp_limb_signed_t,
    );
}
