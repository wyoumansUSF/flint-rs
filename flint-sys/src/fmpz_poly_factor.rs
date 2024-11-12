/* automatically generated by rust-bindgen 0.70.1 */

#![allow(non_camel_case_types)]
use crate::deps::*;
use crate::flint::*;
use crate::fmpz_types::*;
use crate::nmod_types::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zassenhaus_prune_struct {
    pub deg: mp_limb_signed_t,
    pub pos_degs: *mut ::std::os::raw::c_uchar,
    pub new_length: mp_limb_signed_t,
    pub new_total: mp_limb_signed_t,
    pub new_degs: *mut mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of zassenhaus_prune_struct"][::std::mem::size_of::<zassenhaus_prune_struct>() - 48usize];
    ["Alignment of zassenhaus_prune_struct"]
        [::std::mem::align_of::<zassenhaus_prune_struct>() - 8usize];
    ["Offset of field: zassenhaus_prune_struct::deg"]
        [::std::mem::offset_of!(zassenhaus_prune_struct, deg) - 0usize];
    ["Offset of field: zassenhaus_prune_struct::pos_degs"]
        [::std::mem::offset_of!(zassenhaus_prune_struct, pos_degs) - 8usize];
    ["Offset of field: zassenhaus_prune_struct::new_length"]
        [::std::mem::offset_of!(zassenhaus_prune_struct, new_length) - 16usize];
    ["Offset of field: zassenhaus_prune_struct::new_total"]
        [::std::mem::offset_of!(zassenhaus_prune_struct, new_total) - 24usize];
    ["Offset of field: zassenhaus_prune_struct::new_degs"]
        [::std::mem::offset_of!(zassenhaus_prune_struct, new_degs) - 32usize];
    ["Offset of field: zassenhaus_prune_struct::alloc"]
        [::std::mem::offset_of!(zassenhaus_prune_struct, alloc) - 40usize];
};
impl Default for zassenhaus_prune_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type zassenhaus_prune_t = [zassenhaus_prune_struct; 1usize];
extern "C" {
    pub fn fmpz_poly_factor_init(fac: *mut fmpz_poly_factor_struct);
    pub fn fmpz_poly_factor_init2(fac: *mut fmpz_poly_factor_struct, alloc: mp_limb_signed_t);
    pub fn fmpz_poly_factor_realloc(fac: *mut fmpz_poly_factor_struct, alloc: mp_limb_signed_t);
    pub fn fmpz_poly_factor_fit_length(fac: *mut fmpz_poly_factor_struct, len: mp_limb_signed_t);
    pub fn fmpz_poly_factor_clear(fac: *mut fmpz_poly_factor_struct);
    pub fn fmpz_poly_factor_set(
        res: *mut fmpz_poly_factor_struct,
        fac: *const fmpz_poly_factor_struct,
    );
    pub fn fmpz_poly_factor_insert(
        fac: *mut fmpz_poly_factor_struct,
        p: *const fmpz_poly_struct,
        exp: mp_limb_signed_t,
    );
    pub fn fmpz_poly_factor_concat(
        res: *mut fmpz_poly_factor_struct,
        fac: *const fmpz_poly_factor_struct,
    );
    pub fn fmpz_poly_factor_print(fac: *const fmpz_poly_factor_struct);
    pub fn fmpz_poly_factor_zassenhaus_recombination(
        final_fac: *mut fmpz_poly_factor_struct,
        lifted_fac: *const fmpz_poly_factor_struct,
        F: *const fmpz_poly_struct,
        P: *const fmpz,
        exp: mp_limb_signed_t,
    );
    pub fn fmpz_poly_factor_squarefree(
        fac: *mut fmpz_poly_factor_struct,
        F: *const fmpz_poly_struct,
    );
    pub fn fmpz_poly_factor_mignotte(B: *mut fmpz, f: *const fmpz_poly_struct);
    pub fn _fmpz_poly_factor_zassenhaus(
        final_fac: *mut fmpz_poly_factor_struct,
        exp: mp_limb_signed_t,
        f: *const fmpz_poly_struct,
        cutoff: mp_limb_signed_t,
        use_van_hoeij: ::std::os::raw::c_int,
    );
    pub fn fmpz_poly_factor_zassenhaus(
        fac: *mut fmpz_poly_factor_struct,
        G: *const fmpz_poly_struct,
    );
    pub fn _fmpz_poly_factor_quadratic(
        fac: *mut fmpz_poly_factor_struct,
        f: *const fmpz_poly_struct,
        exp: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_factor_cubic(
        fac: *mut fmpz_poly_factor_struct,
        f: *const fmpz_poly_struct,
        exp: mp_limb_signed_t,
    );
    pub fn _fmpz_poly_factor_CLD_mat(
        res: *mut fmpz_mat_struct,
        f: *const fmpz_poly_struct,
        lifted_fac: *mut fmpz_poly_factor_struct,
        P: *mut fmpz,
        k: mp_limb_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_poly_factor_van_hoeij_check_if_solved(
        M: *mut fmpz_mat_struct,
        final_fac: *mut fmpz_poly_factor_struct,
        lifted_fac: *mut fmpz_poly_factor_struct,
        f: *const fmpz_poly_struct,
        P: *mut fmpz,
        exp: mp_limb_signed_t,
        lc: *mut fmpz,
    ) -> ::std::os::raw::c_int;
    pub fn fmpz_poly_factor_van_hoeij(
        final_fac: *mut fmpz_poly_factor_struct,
        fac: *const nmod_poly_factor_struct,
        f: *const fmpz_poly_struct,
        exp: mp_limb_signed_t,
        p: mp_limb_t,
    );
    pub fn fmpz_poly_factor(fac: *mut fmpz_poly_factor_struct, G: *const fmpz_poly_struct);
    pub fn fmpz_poly_factor_get_fmpz_poly(
        z: *mut fmpz_poly_struct,
        F: *const fmpz_poly_factor_struct,
        i: mp_limb_signed_t,
    );
    pub fn fmpz_poly_factor_get_fmpz(z: *mut fmpz, F: *const fmpz_poly_factor_struct);
    pub fn zassenhaus_subset_first(
        s: *mut mp_limb_signed_t,
        r: mp_limb_signed_t,
        m: mp_limb_signed_t,
    );
    pub fn zassenhaus_subset_next(
        s: *mut mp_limb_signed_t,
        r: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn zassenhaus_subset_next_disjoint(
        s: *mut mp_limb_signed_t,
        r: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn zassenhaus_prune_clear(Z: *mut zassenhaus_prune_struct);
    pub fn zassenhaus_prune_set_degree(Z: *mut zassenhaus_prune_struct, d: mp_limb_signed_t);
    pub fn zassenhaus_prune_add_factor(
        Z: *mut zassenhaus_prune_struct,
        deg: mp_limb_signed_t,
        exp: mp_limb_signed_t,
    );
    pub fn zassenhaus_prune_end_add_factors(Z: *mut zassenhaus_prune_struct);
    pub fn zassenhaus_prune_must_be_irreducible(
        Z: *const zassenhaus_prune_struct,
    ) -> ::std::os::raw::c_int;
    pub fn fmpz_poly_factor_zassenhaus_recombination_with_prune(
        final_fac: *mut fmpz_poly_factor_struct,
        lifted_fac: *const fmpz_poly_factor_struct,
        F: *const fmpz_poly_struct,
        P: *const fmpz,
        exp: mp_limb_signed_t,
        Z: *const zassenhaus_prune_struct,
    );
}
