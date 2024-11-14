/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::bindgen::*;
use crate::fmpz_mod_types::*;
use crate::fq_default::*;
use crate::fq_default_poly::*;
use crate::fq_nmod_types::*;
use crate::fq_types::*;
use crate::fq_zech_types::*;
use crate::nmod_types::*;

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
