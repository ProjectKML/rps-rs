use bindgen::callbacks::ParseCallbacks;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct BindgenCallbacks;

impl ParseCallbacks for BindgenCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.starts_with("Vk") {
            Some(original_item_name.trim_start_matches("Vk").to_string())
        } else if original_item_name.starts_with("PFN_vk") && original_item_name.ends_with("KHR") {
            Some(original_item_name.trim_end_matches("KHR").to_string())
        } else {
            None
        }
    }
}

fn generate_bindings() {
    let mut builder = bindgen::Builder::default();

    #[cfg(feature = "vulkan")]
    {
        builder = builder.raw_line("use ash::vk::*;");
        builder = builder.clang_arg("-DRPS_VK_RUNTIME");
    }

    //HACK: replace file and restore it, because it does not work otherwise
    let rps_vk_runtime_str = fs::read_to_string("vendor/RenderPipelineShaders/include/rps/runtime/vk/rps_vk_runtime.h").unwrap();
    let rps_vk_runtime_bindgen = rps_vk_runtime_str.replace("RPS_IMPL_OPAQUE_HANDLE", "//");
    fs::write("vendor/RenderPipelineShaders/include/rps/runtime/vk/rps_vk_runtime.h", rps_vk_runtime_bindgen).unwrap();

    builder = builder
        .clang_arg("-I./vendor/Vulkan-Headers/include")
        .clang_arg("-I./vendor/RenderPipelineShaders/include")
        .header("vendor/RenderPipelineShaders/include/rps/rps.h")
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .allowlist_function("rps.*")
        .allowlist_function("PFN_rps.*")
        .allowlist_type("Rps.*")
        .parse_callbacks(Box::new(BindgenCallbacks))
        .blocklist_type("Vk.*")
        .blocklist_type("PFN_vk.*")
        .blocklist_type("__.*")
        .blocklist_type("va_list")
        .raw_line(r#"pub type va_list = *mut u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___rpsl_runtime_procs {
    _unused: [u8; 0]
}
        "#)
        .layout_tests(false);

    let bindings = builder
        .generate()
        .expect("Failed to generate bindings");

    let bindings_str = bindings.to_string();

    fs::create_dir_all("gen").unwrap();
    fs::write(Path::new("gen/bindings.rs"), bindings_str).expect("Failed to write bindings to file");

    //HACK: restore original file
    fs::write("vendor/RenderPipelineShaders/include/rps/runtime/vk/rps_vk_runtime.h", rps_vk_runtime_str).unwrap();
}

fn main() {
    #[cfg(feature = "d3d11")]
    panic!("Feature D3D11 is not supported at the moment");

    #[cfg(feature = "d3d12")]
    panic!("Feature D3D12 is not supported at the moment");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .include("vendor/RenderPipelineShaders/include")
        .include("vendor/RenderPipelineShaders/src")
        .file("vendor/RenderPipelineShaders/src/core/rps_core.cpp")
        .file("vendor/RenderPipelineShaders/src/core/rps_device.cpp")
        .file("vendor/RenderPipelineShaders/src/core/rps_graph.cpp")
        .file("vendor/RenderPipelineShaders/src/core/rps_result.cpp")
        .file("vendor/RenderPipelineShaders/src/frontend/rps_builder.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_access.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_format.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_null_runtime_backend.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_null_runtime_device.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_render_graph.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_render_graph_builder.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_render_graph_diagnostics.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_runtime_backend.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_runtime_device.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_subprogram.cpp");

    #[cfg(feature = "vulkan")]
    {
        build
            .file("vendor/RenderPipelineShaders/src/runtime/vk/rps_vk_built_in_nodes.cpp")
            .file("vendor/RenderPipelineShaders/src/runtime/vk/rps_vk_formats.cpp")
            .file("vendor/RenderPipelineShaders/src/runtime/vk/rps_vk_runtime_backend.cpp")
            .file("vendor/RenderPipelineShaders/src/runtime/vk/rps_vk_runtime_device.cpp");
    }

    build.cpp(true)
        .flag("-Wno-missing-field-initializers")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-function")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-unused-private-field")
        .flag("-Wno-unused-variable");

    build.compile("render_pipeline_shaders_sys_cc");

    cc::Build::new()
        .include("vendor/RenderPipelineShaders/include")
        .include("vendor/RenderPipelineShaders/src")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_rpsl_host.c")
        .compile("render_pipeline_shaders_sys_rpsl_host_cc");

    generate_bindings();
}