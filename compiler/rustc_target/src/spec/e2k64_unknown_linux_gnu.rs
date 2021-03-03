use crate::spec::{LinkArgs, LinkerFlavor, Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(
        LinkerFlavor::Gcc,
        vec!["-fPIC".to_string()],
    );

    let mut post_link_args = LinkArgs::new();
    post_link_args.insert(
        LinkerFlavor::Gcc,
        vec![
            "-L/usr/lib/lccrt/lib/e2k64/".to_string(),
            "-Wl,-rpath=/usr/lib/lccrt/lib/e2k64/".to_string(),
            "-llccrt_s".to_string(),
            "-llcc".to_string(),
            "-lm".to_string(),
        ],
    );

    Ok(Target {
        llvm_target: "e2k64-unknown-linux-gnu".to_string(),
        target_pointer_width: "64".to_string(),
        target_endian: "little".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-f80:128:128-f128:128:128-v128:128:128-n32:64".to_string(),
        arch: "e2k64".to_string(),
        target_os: "linux".to_string(),
        target_env: "gnu".to_string(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        
        options: TargetOptions {
            executables: true,
            linker: Some(option_env!("ECC").unwrap_or("/usr/bin/gcc").to_string()),
            max_atomic_width: Some(64),
            pre_link_args,
            post_link_args, 

            position_independent_executables: false,

            ..Default::default()
        },
    })
}
