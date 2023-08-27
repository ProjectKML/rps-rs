use std::{env, fs, path::Path};

use bindgen::callbacks::ParseCallbacks;
use bindgen::Formatter;

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
        .formatter(Formatter::Rustfmt)
        .size_t_is_usize(true)
        .allowlist_function("rps.*")
        .allowlist_function("PFN_rps.*")
        .allowlist_type("Rps.*")
        .parse_callbacks(Box::new(BindgenCallbacks))
        .blocklist_type("Vk.*")
        .blocklist_type("PFN_vk.*")
        .blocklist_type("va_list")
        .raw_line("pub type va_list = *mut u8;")
        .layout_tests(false);

    let bindings_result = builder.generate();

    //HACK: restore original file
    fs::write("vendor/RenderPipelineShaders/include/rps/runtime/vk/rps_vk_runtime.h", &rps_vk_runtime_str).unwrap();

    let bindings = bindings_result.expect("Failed to generate bindings");

    let bindings_str = bindings.to_string();

    fs::create_dir_all("gen").unwrap();
    fs::write(Path::new("gen/bindings.rs"), bindings_str).expect("Failed to write bindings to file");
}

fn main() {
    #[cfg(feature = "d3d11")]
    panic!("Feature D3D11 is not supported at the moment");

    #[cfg(feature = "d3d12")]
    panic!("Feature D3D12 is not supported at the moment");

    generate_bindings();

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .include("vendor/Vulkan-Headers/include")
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
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_rpsl_host.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_runtime_backend.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_runtime_device.cpp")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_subprogram.cpp");

    let target = env::var("TARGET").unwrap();

    if target.contains("gnu") || target.contains("darwin") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-sign-compare")
            .flag("-Wno-unused-function")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-but-set-variable")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-unused-variable")
            .flag("-Wno-extra");
    } else if target.contains("msvc") {
        build
            .flag("/wd4100")
            .flag("/wd4189")
            .flag("/wd4201")
            .flag("/wd4244")
            .flag("/wd4389")
            .flag("/wd4463")
            .flag("/wd4205")
            .flag("/wd4505");
    }

    #[cfg(feature = "vulkan")]
    {
        build
            .define("RPS_VK_RUNTIME", "1")
            .define("RPS_VK_DYNAMIC_LOADING", "1")
            .file("vendor/RenderPipelineShaders/src/runtime/vk/rps_vk_built_in_nodes.cpp")
            .file("vendor/RenderPipelineShaders/src/runtime/vk/rps_vk_formats.cpp")
            .file("vendor/RenderPipelineShaders/src/runtime/vk/rps_vk_runtime_backend.cpp")
            .file("vendor/RenderPipelineShaders/src/runtime/vk/rps_vk_runtime_device.cpp");
    }

    build.cpp(true).compile("render_pipeline_shaders_sys_cc");

    cc::Build::new()
        .include("vendor/RenderPipelineShaders/include")
        .include("vendor/RenderPipelineShaders/src")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_rpsl_host_dll.c")
        .file("vendor/RenderPipelineShaders/src/runtime/common/rps_rpsl_host_intrinsics.c")
        .compile("render_pipeline_shaders_sys_dll_cc");
}
