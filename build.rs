use std::env;
#[cfg(feature = "generate_bindings")]
use std::path::Path;

#[cfg(feature = "generate_bindings")]
use bindgen::callbacks::ParseCallbacks;

#[cfg(feature = "generate_bindings")]
fn generate_bindings(output_path: impl AsRef<Path>) {
    let bindings = bindgen::Builder::default()
        .clang_arg("-DRPS_VK_RUNTIME")
        .clang_arg("-I./vendor/Vulkan-Headers/include")
        .clang_arg("-I./vendor/RenderPipelineShaders/include")
        .header("vendor/RenderPipelineShaders/include/rps/rps.h")
        .allowlist_function("rps.*")
        .allowlist_function("PFN_rps.*")
        .allowlist_type("Rps.*")
        .parse_callbacks(Box::new(FixAshTypes))
        .blocklist_type("Vk.*")
        .blocklist_type("PFN_vk.*")
        .blocklist_type("__.*")
        .blocklist_type("va_list")
        .raw_line("use ash::vk::*;")
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .trust_clang_mangling(false)
        .layout_tests(false)
        .generate()
        .expect("Failed to generate bindings");
    bindings.write_to_file(output_path.as_ref()).expect("Failed to write bindings to file");
}

#[cfg(feature = "generate_bindings")]
#[derive(Debug)]
struct FixAshTypes;

#[cfg(feature = "generate_bindings")]
impl ParseCallbacks for FixAshTypes {
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

fn main() {
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

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .flag("-Wno-nullability-completeness")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("ios") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("android") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    } else if target.contains("windows") && target.contains("gnu") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .flag("-Wno-type-limits")
            .flag("-Wno-unused-but-set-variable")
            .flag("-Wno-sign-compare")
            .flag("-Wno-unused-function")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    }

    build.compile("rps_cpp");

    #[cfg(feature = "generate_bindings")]
    generate_bindings("gen/bindings.rs");
}
