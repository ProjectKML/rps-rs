<!-- markdownlint-disable-file MD041 -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🎬 render-pipeline-shaders-rs`

**A very lightweight wrapper around the Render Pipeline Shaders library 🦀**

[![crates][crates-badge]][crates-url]
[![license][license-badge]][license-url]
[![rps][rps-badge]][rps-url]
[![dependency-status][dependency-badge]][dependency-url]

[crates-badge]: https://img.shields.io/crates/v/render-pipeline-shaders.svg
[crates-url]: https://crates.io/crates/render-pipeline-shaders

[license-badge]: https://img.shields.io/badge/License-MIT/Apache_2.0-blue.svg
[license-url]: LICENSE-MIT

[rps-badge]: https://img.shields.io/badge/Render%20Pipeline%20Shaders-1.0%20Beta-orange
[rps-url]: https://github.com/GPUOpen-LibrariesAndSDKs/RenderPipelineShaders

[dependency-badge]: https://deps.rs/repo/github/projectkml/render-pipeline-shaders-rs/status.svg
[dependency-url]: https://deps.rs/repo/github/projectkml/render-pipeline-shaders-rs

</div>

## 🚨 Warning 🚨

This library is still experimental and should not be used for anything serious, yet. It is also not yet published to crates.io.

```Rust
let device_create_info = rps::DeviceCreateInfo {
    allocator: rps::Allocator {
        pfn_alloc: Some(alloc),
        pfn_realloc: Some(realloc),
        pfn_free: Some(free),
        ..Default::default()
    },
    printer: rps::Printer {
        pfn_printf: Some(printf),
        pfn_vprintf: Some(vprintf),
        ..Default::default()
    },
    ..Default::default()
};

let runtime_create_info = rps::RuntimeDeviceCreateInfo {
    callbacks: rps::RuntimeCallbacks {
        pfn_record_debug_marker: Some(record_debug_marker),
        pfn_set_debug_name: Some(set_debug_name),
        ..Default::default()
    },
    ..Default::default()
};

let vk_functions = rps::VKFunctions::new(device.instance().loader(), device.loader());

let runtime_device_create_info = rps::VKRuntimeDeviceCreateInfo {
    device_create_info: &device_create_info,
    runtime_create_info: &runtime_create_info,
    vk_device: *device,
    vk_physical_device: *device.physical_device(),
    flags: rps::VKRuntimeFlags::DONT_FLIP_VIEWPORT,
    vk_functions: &vk_functions,
};

let rps_device = unsafe { rps::vk_runtime_device_create(&runtime_device_create_info) }?;
```

### Credits
* [AMD](https://gpuopen.com/learn/rps_1_0/) for creating the Render Pipeline Shaders library.
* [The Ash community](https://github.com/ash-rs/ash) for creating such an awesome rust wrapper around Vulkan.
