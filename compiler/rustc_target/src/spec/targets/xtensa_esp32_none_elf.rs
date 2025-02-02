use crate::spec::{base::xtensa, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-v1:8:8-i64:64-i128:128-n32".into(),
        arch: "xtensa".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },

        options: TargetOptions {
            cpu: "esp32".into(),
            linker: Some("xtensa-esp32-elf-gcc".into()),
            max_atomic_width: Some(32),
            atomic_cas: true,
            ..xtensa::opts()
        },
    }
}
