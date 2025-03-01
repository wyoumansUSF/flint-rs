// Adapted from https://gitlab.com/tspiteri/gmp-mpfr-sys/-/blob/master/build.rs
// Also see https://github.com/rust-lang/rust-bindgen/discussions/2405


use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    io::Result as IoResult,
    path::{Path, PathBuf},
    process::Command,
    str,
};

const FLINT_DIR: &str = "flint-3.1.2";
const FLINT_LIB: &str = "libflint.a";
const FLINT_VER: &str = "3.1.2";
const FLINT_HEADERS: &[&str] = &[
    "acb.h",
    "acb_calc.h",
    "acb_dft.h",
    "acb_dirichlet.h",
    "acb_elliptic.h",
    "acb_hypgeom.h",
    "acb_mat.h",
    "acb_modular.h",
    "acb_poly.h",
    "acb_theta.h",
    "acb_types.h",
    "acf.h",
    "acf_types.h",
    "aprcl.h",
    "arb.h",
    "arb_calc.h",
    "arb_fmpz_poly.h",
    "arb_fpwrap.h",
    "arb_hypgeom.h",
    "arb_mat.h",
    "arb_poly.h",
    "arb_types.h",
    "arf.h",
    "arf_types.h",
    "arith.h",
    "bernoulli.h",
    "bool_mat.h",
    "ca.h",
    "ca_ext.h",
    "ca_field.h",
    "ca_mat.h",
    "ca_poly.h",
    "ca_vec.h",
    "calcium.h",
    "d_mat.h",
    "d_vec.h",
    "dirichlet.h",
    "dlog.h",
    "double_extras.h",
    "double_interval.h",
    "fexpr.h",
    "fexpr_builtin.h",
    "fft.h",
    "fft_tuning.h",
    "flint-config.h",
    "flint.h",
    "fmpq.h",
    "fmpq_mat.h",
    "fmpq_mpoly.h",
    "fmpq_mpoly_factor.h",
    "fmpq_poly.h",
    "fmpq_types.h",
    "fmpq_vec.h",
    "fmpz.h",
    "fmpz_extras.h",
    "fmpz_factor.h",
    "fmpz_lll.h",
    "fmpz_mat.h",
    "fmpz_mod.h",
    "fmpz_mod_mat.h",
    "fmpz_mod_mpoly.h",
    "fmpz_mod_mpoly_factor.h",
    "fmpz_mod_poly.h",
    "fmpz_mod_poly_factor.h",
    "fmpz_mod_types.h",
    "fmpz_mod_vec.h",
    "fmpz_mpoly.h",
    "fmpz_mpoly_factor.h",
    "fmpz_mpoly_q.h",
    "fmpz_poly.h",
    "fmpz_poly_factor.h",
    "fmpz_poly_mat.h",
    "fmpz_poly_q.h",
    "fmpz_types.h",
    "fmpz_vec.h",
    "fmpzi.h",
    "fq.h",
    "fq_default.h",
    "fq_default_mat.h",
    "fq_default_poly.h",
    "fq_default_poly_factor.h",
    "fq_embed.h",
    "fq_embed_templates.h",
    "fq_mat.h",
    "fq_mat_templates.h",
    "fq_nmod.h",
    "fq_nmod_embed.h",
    "fq_nmod_mat.h",
    "fq_nmod_mpoly.h",
    "fq_nmod_mpoly_factor.h",
    "fq_nmod_poly.h",
    "fq_nmod_poly_factor.h",
    "fq_nmod_types.h",
    "fq_nmod_vec.h",
    "fq_poly.h",
    "fq_poly_factor.h",
    "fq_poly_factor_templates.h",
    "fq_poly_templates.h",
    "fq_templates.h",
    "fq_types.h",
    "fq_vec.h",
    "fq_vec_templates.h",
    "fq_zech.h",
    "fq_zech_embed.h",
    "fq_zech_mat.h",
    "fq_zech_mpoly.h",
    "fq_zech_mpoly_factor.h",
    "fq_zech_poly.h",
    "fq_zech_poly_factor.h",
    "fq_zech_types.h",
    "fq_zech_vec.h",
    "gmpcompat.h",
    "gr.h",
    "gr_generic.h",
    "gr_mat.h",
    "gr_mpoly.h",
    "gr_poly.h",
    "gr_special.h",
    "gr_vec.h",
    "hypgeom.h",
    "limb_types.h",
    "long_extras.h",
    "longlong.h",
    "longlong_asm_clang.h",
    "longlong_div_gnu.h",
    "mag.h",
    "mpf-impl.h",
    "mpfr_mat.h",
    "mpfr_vec.h",
    "mpoly.h",
    "mpoly_types.h",
    "n_poly.h",
    "n_poly_types.h",
    "nf.h",
    "nf_elem.h",
    "nmod.h",
    "nmod_mat.h",
    "nmod_mpoly.h",
    "nmod_mpoly_factor.h",
    "nmod_poly.h",
    "nmod_poly_factor.h",
    "nmod_poly_mat.h",
    "nmod_types.h",
    "nmod_vec.h",
    "padic.h",
    "padic_mat.h",
    "padic_poly.h",
    "padic_types.h",
    "partitions.h",
    "perm.h",
    "qadic.h",
    "qfb.h",
    "qqbar.h",
    "qsieve.h",
    "templates.h",
    "thread_pool.h",
    "thread_support.h",
    "ulong_extras.h",
];

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Mingw,
    Msvc,
    Other,
}

struct Environment {
    target: Target,
    gmp_mpfr_dir: PathBuf,
    src_dir: PathBuf,
    out_dir: PathBuf,
    lib_dir: PathBuf,
    include_dir: PathBuf,
    build_dir: PathBuf,
    cache_dir: Option<PathBuf>,
    jobs: OsString,
}

fn main() {
    let cc = env::var_os("CC");
    let cc_cache_dir = cc.as_ref().map(|cc| {
        let mut dir = OsString::from("CC-");
        dir.push(cc);
        dir
    });

    let raw_target = cargo_env("TARGET")
        .into_string()
        .expect("env var TARGET having sensible characters");

    let target = if raw_target.contains("-windows-msvc") {
        Target::Msvc
    } else if raw_target.contains("-windows-gnu") {
        Target::Mingw
    } else {
        Target::Other
    };

    let gmp_mpfr_dir = PathBuf::from(cargo_env("DEP_GMP_OUT_DIR"));
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));

    println!("cargo:rerun-if-env-changed=FLINT_SYS_CACHE");
    let cache_dir = if env::var("DOCS_RS").is_ok() {
        None
    } else {
        match env::var_os("FLINT_SYS_CACHE") {
            Some(ref c) if c.is_empty() || c == "_" => None,
            Some(c) => Some(PathBuf::from(c)),
            None => system_cache_dir().map(|c| c.join("flint-sys")),
        }
    };
    let cache_dir = cache_dir
        .map(|cache| cache.join(&FLINT_VER))
        .map(|cache| match cc_cache_dir {
            Some(dir) => cache.join(dir),
            None => cache,
        });

    let env = Environment {
        target,
        gmp_mpfr_dir,
        src_dir,
        out_dir: out_dir.clone(),
        lib_dir: out_dir.join("lib"),
        include_dir: out_dir.join("include"),
        build_dir: out_dir.join("build"),
        cache_dir,
        jobs: cargo_env("NUM_JOBS"),
    };

    // make sure we have target directories
    create_dir_or_panic(&env.lib_dir);
    create_dir_or_panic(&env.include_dir);

    compile(&env);
}

fn compile(env: &Environment) {
    if need_compile(env) {
        check_for_msvc(env);
        remove_dir_or_panic(&env.build_dir);
        copy_dir_or_panic(&env.src_dir.join(FLINT_DIR), &env.build_dir);
        build(env);
        build_extern(env);
        remove_dir_or_panic(&env.build_dir);
        save_cache(env);
    }
    write_link_info(env);
}

fn need_compile(env: &Environment) -> bool {
    let mut ok = env.lib_dir.join(FLINT_LIB).is_file();
    ok = ok && env.lib_dir.join("libextern.a").is_file();

    for h in FLINT_HEADERS {
        ok = ok && env.include_dir.join(h).is_file();
    }

    if ok {
        if should_save_cache(env) {
            save_cache(env);
        }
        return false;
    } else if load_cache(env) {
        // if loading cache works, we're done
        return false;
    }
    true
}

fn save_cache(env: &Environment) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let mut ok = create_dir(&cache_dir).is_ok();
    ok = ok && create_dir(&cache_dir.join("lib")).is_ok();
    ok = ok && create_dir(&cache_dir.join("include")).is_ok();

    ok = ok && copy_file(&env.lib_dir.join(FLINT_LIB), &cache_dir.join(FLINT_LIB)).is_ok();
    ok = ok && copy_file(&env.lib_dir.join("libextern.a"), &cache_dir.join("lib").join("libextern.a")).is_ok();

    for h in FLINT_HEADERS {
        ok = ok && copy_file(&env.include_dir.join(h), &cache_dir.join("include").join(h)).is_ok();
    }
    ok
}

fn load_cache(env: &Environment) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let mut ok = true;
    ok = ok && copy_file(&cache_dir.join(FLINT_LIB), &env.lib_dir.join(FLINT_LIB)).is_ok();
    ok = ok && copy_file(&cache_dir.join("lib").join("libextern.a"), &env.lib_dir.join("libextern.a")).is_ok();

    for h in FLINT_HEADERS {
        ok = ok && copy_file(&cache_dir.join("include").join(h), &env.include_dir.join(h)).is_ok();
    }
    ok
}

fn should_save_cache(env: &Environment) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let mut ok = true;
    ok = ok && cache_dir.join(FLINT_LIB).is_file();
    ok = ok && cache_dir.join("lib").join("libextern.a").is_file();

    for h in FLINT_HEADERS {
        ok = ok && cache_dir.join("include").join(h).is_file();
    }
    !ok
}

fn build(env: &Environment) {
    println!("$ cd {:?}", &env.build_dir);
    let conf = String::from(format!(
        r#"./configure --disable-shared --with-mpfr={} --with-gmp={} CFLAGS="-fPIC""#,
        env.gmp_mpfr_dir.display(),
        env.gmp_mpfr_dir.display(),
    ));

    configure(&env.build_dir, &OsString::from(conf));
    make_and_check(env, &env.build_dir);

    let build_lib = &env.build_dir.join(FLINT_LIB);
    copy_file_or_panic(&build_lib, &env.lib_dir.join(FLINT_LIB));

    let src = env.build_dir.join("src");
    for h in FLINT_HEADERS {
        copy_file_or_panic(&src.join(h), &env.include_dir.join(h));
    }
}

fn build_extern(env: &Environment) {
    // compile wrapped static inlined functions
    let obj_path = &env.build_dir.join("extern.o");
    // This is the path to the static library file.
    let lib_path = &env.lib_dir.join("libextern.a");

    // Compile the generated wrappers into an object file.
    let clang_output = std::process::Command::new("clang")
        // TODO: -flto=thin causes a lot of problems, needs lld linker isntead of ld for some 
        // reason and setting this in .cargo/config doesn't seem to be respected by cargo test, so 
        // tests break. See https://github.com/rust-lang/rust-bindgen/discussions/2405
        //.arg("-flto=thin")
        .arg("-O")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(env.src_dir.join("C").join("extern.c"))
        .arg(format!("-I{}", env.gmp_mpfr_dir.join("include").display()))
        .arg(format!("-I{}", env.include_dir.display()))
        .arg("-fPIC")
        .output()
        .expect("Could not compile object file.");
    
    if !clang_output.status.success() {
        panic!(
            "Could not compile object file:\n{}",
            String::from_utf8_lossy(&clang_output.stderr)
        );
    }
    
    // Turn the object file into a static library
    #[cfg(not(target_os = "windows"))]
    let lib_output = Command::new("ar")
        .arg("crus")
        .arg(&lib_path)
        .arg(obj_path)
        .output()
        .expect("Could not build static library extern.");
    #[cfg(target_os = "windows")]
    let lib_output = Command::new("LIB")
        .arg(obj_path)
        .arg(format!("/OUT:{}", env.lib_dir.join("libextern.a").display()))
        .output()
        .expect("Could not build static library extern.");
    if !lib_output.status.success() {
        panic!(
            "Could not emit library file:\n{}",
            String::from_utf8_lossy(&lib_output.stderr)
        );
    }
}



fn write_link_info(env: &Environment) {
    let out_str = env.out_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.out_dir.display()
        )
    });
    let lib_str = env.lib_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.lib_dir.display()
        )
    });
    let include_str = env.include_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.include_dir.display()
        )
    });

    println!("cargo:out_dir={}", out_str);
    println!("cargo:lib_dir={}", lib_str);
    println!("cargo:include_dir={}", include_str);
    println!("cargo:rustc-link-search=native={}", lib_str);
    println!("cargo:rustc-link-lib=static=extern");
    println!("cargo:rustc-link-lib=static=flint");
    println!("cargo:rustc-link-lib=static=mpfr");
    println!("cargo:rustc-link-lib=static=gmp");
}

fn cargo_env(name: &str) -> OsString {
    env::var_os(name)
        .unwrap_or_else(|| panic!("environment variable not found: {}, please use cargo", name))
}

fn check_for_msvc(env: &Environment) {
    if env.target == Target::Msvc {
        panic!("Windows MSVC target is not supported (linking would fail)");
    }
}

fn remove_dir(dir: &Path) -> IoResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    assert!(dir.is_dir(), "Not a directory: {:?}", dir);
    println!("$ rm -r {:?}", dir);
    fs::remove_dir_all(dir)
}

fn remove_dir_or_panic(dir: &Path) {
    remove_dir(dir).unwrap_or_else(|_| panic!("Unable to remove directory: {:?}", dir));
}

fn create_dir(dir: &Path) -> IoResult<()> {
    println!("$ mkdir -p {:?}", dir);
    fs::create_dir_all(dir)
}

fn create_dir_or_panic(dir: &Path) {
    create_dir(dir).unwrap_or_else(|_| panic!("Unable to create directory: {:?}", dir));
}

pub fn copy_dir(from: &Path, to: &Path) -> IoResult<()> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from));

    let output_root = PathBuf::from(to);
    let input_root = PathBuf::from(from).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!("$ mkdir {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

fn copy_dir_or_panic(src: &Path, dst: &Path) {
    copy_dir(src, dst).unwrap_or_else(|_| {
        panic!("Unable to copy {:?} -> {:?}", src, dst);
    });
}

fn copy_file(src: &Path, dst: &Path) -> IoResult<u64> {
    println!("$ cp {:?} {:?}", src, dst);
    fs::copy(src, dst)
}

fn copy_file_or_panic(src: &Path, dst: &Path) {
    copy_file(src, dst).unwrap_or_else(|_| {
        panic!("Unable to copy {:?} -> {:?}", src, dst);
    });
}

fn configure(build_dir: &Path, conf_line: &OsStr) {
    let mut conf = Command::new("sh");
    conf.current_dir(&build_dir).arg("-c").arg(conf_line);
    execute(conf);
}

fn make_and_check(env: &Environment, build_dir: &Path) {
    let mut make = Command::new("make");
    make.current_dir(build_dir).arg("-j").arg(&env.jobs);
    execute(make);

    if !cfg!(feature = "disable-make-check") {
        let mut make_check = Command::new("make");
        make_check
            .current_dir(build_dir)
            .arg("-j")
            .arg(&env.jobs)
            .arg("check");
        execute(make_check);
    }
}

fn execute(mut command: Command) {
    println!("$ {:?}", command);
    let status = command
        .status()
        .unwrap_or_else(|_| panic!("Unable to execute: {:?}", command));
    if !status.success() {
        if let Some(code) = status.code() {
            panic!("Program failed with code {}: {:?}", code, command);
        } else {
            panic!("Program failed: {:?}", command);
        }
    }
}

fn system_cache_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        use core::{mem::MaybeUninit, ptr, slice};
        use std::os::windows::ffi::OsStringExt;
        use winapi::{
            shared::winerror::S_OK,
            um::{combaseapi, knownfolders::FOLDERID_LocalAppData, shlobj, winbase},
        };
        let id = &FOLDERID_LocalAppData;
        let flags = 0;
        let access = ptr::null_mut();
        let mut path = MaybeUninit::uninit();
        unsafe {
            if shlobj::SHGetKnownFolderPath(id, flags, access, path.as_mut_ptr()) == S_OK {
                let path = path.assume_init();
                let slice = slice::from_raw_parts(path, winbase::lstrlenW(path) as usize);
                let string = OsString::from_wide(slice);
                combaseapi::CoTaskMemFree(path as _);
                Some(string.into())
            } else {
                None
            }
        }
    }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        env::var_os("HOME")
            .filter(|x| !x.is_empty())
            .map(|x| AsRef::<Path>::as_ref(&x).join("Library").join("Caches"))
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios")))]
    {
        env::var_os("XDG_CACHE_HOME")
            .filter(|x| !x.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME")
                    .filter(|x| !x.is_empty())
                    .map(|x| AsRef::<Path>::as_ref(&x).join(".cache"))
            })
    }
}
