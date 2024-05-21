<div align="center">

# `🎬 render-pipeline-shaders-rs`

**A very lightweight wrapper around the Render Pipeline Shaders library 🦀**

[projectkml-badge]: https://img.shields.io/badge/projectkml-open%20source-blueviolet.svg
[projectkml-url]: https://github.com/ProjectKML

[crates-badge]: https://img.shields.io/crates/v/render-pipeline-shaders-rs.svg
[crates-url]: https://crates.io/crates/render-pipeline-shaders-rs

[docs-badge]: https://docs.rs/render-pipeline-shaders-rs/badge.svg
[docs-url]: https://docs.rs/render-pipeline-shaders-rs

[dependency-badge]: https://deps.rs/repo/github/ProjectKML/render-pipeline-shaders-rs/status.svg
[dependency-url]: https://deps.rs/repo/github/ProjectKML/render-pipeline-shaders-rs

[build-badge]: https://github.com/ProjectKML/render-pipeline-shaders-rs/workflows/CI/badge.svg
[build-url]: https://github.com/ProjectKML/render-pipeline-shaders-rs/actions

[![ProjectKML][projectkml-badge]][projectkml-url]
[![Crates.io][crates-badge]][crates-url]
[![Docs][docs-badge]][docs-url]
[![Dependency status][dependency-badge]][dependency-url]
[![Build status][build-badge]][build-url]
</div>

## Basic example

```Rust
let device_create_info = rps::DeviceCreateInfo::default();
let runtime_create_info = rps::RuntimeDeviceCreateInfo::default();

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

## Contributing

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in an ProjectKML project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Credits
* [AMD team](https://github.com/GPUOpen-LibrariesAndSDKs/RenderPipelineShaders) for creating the Render Pipeline Shaders library
* [Embark Studios](https://github.com/EmbarkStudios/fsr-rs/blob/main/CONTRIBUTING.md) as my contributor guide is very similar to theirs.