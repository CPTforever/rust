use crate::spec::{
    crt_objects, LinkArgs, LinkOutputKind, LinkerFlavor, LldFlavor, PanicStrategy, TargetOptions,
    TlsModel,
};

pub fn opts() -> TargetOptions {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(
        LinkerFlavor::Lld(LldFlavor::Ld),
        vec!["--build-id".into(), "--hash-style=gnu".into(), "--Bstatic".into()],
    );

    TargetOptions {
        os: "twizzler".into(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".into()),
        executables: true,
        pre_link_args,
        pre_link_objects: crt_objects::new(&[
            (LinkOutputKind::DynamicNoPicExe, &["crti.o", "crtbegin.o"]),
            (LinkOutputKind::DynamicPicExe, &["crti.o", "crtbeginS.o"]),
            (LinkOutputKind::StaticNoPicExe, &["crti.o", "crtbegin.o"]),
            (LinkOutputKind::StaticPicExe, &["crti.o", "crtbeginS.o"]),
        ]),
        post_link_objects: crt_objects::new(&[
            (LinkOutputKind::DynamicNoPicExe, &["crtend.o", "crtn.o"]),
            (LinkOutputKind::DynamicPicExe, &["crtendS.o", "crtn.o"]),
            (LinkOutputKind::StaticNoPicExe, &["crtend.o", "crtn.o"]),
            (LinkOutputKind::StaticPicExe, &["crtendS.o", "crtn.o"]),
        ]),
        panic_strategy: PanicStrategy::Unwind,
        position_independent_executables: false,
        static_position_independent_executables: false,
        tls_model: TlsModel::InitialExec,
        crt_static_default: true,
        crt_static_respected: true,
        dynamic_linking: false,
        has_thread_local: true,
        ..Default::default()
    }
}
