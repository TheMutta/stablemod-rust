use crate::spec::crt_objects;
use crate::spec::{Os, Cc, LinkerFlavor, Lld, RelroLevel, RelocModel, StackProbeType, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: Os::Stablemod,
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        stack_probes: StackProbeType::Inline,
        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        relocation_model: RelocModel::Static,
        // Following two only if using external CRT to provide _start
        pre_link_objects: crt_objects::pre_stablemod(),
        post_link_objects: crt_objects::post_stablemod(),
        ..Default::default()
    }
}

