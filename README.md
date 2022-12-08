
![Logo Kalyzee](docs/assets/banner.png)

StreamStudio Plugins RS
========================

ELements used by Stream Studio - Low Latency Server 

Based on gst-plugins-rs by Sebastian Dröge


# How to add new elements 

Create rust lib in general, net or in a new folder.

```
cd general 
cargo new --lib superlib
```

In general/superlib
Add build.rs in your plugin path with the following content.

```
fn main() {
    gst_plugin_version_helper::info()
}
```

Replate cargo.toml by this template by replacing the project name

```
[package]
name = "rtmpserver"
authors = ["Ludovic Bouguerra <ludovic.bouguerra@stream.studio>"]
repository = "https://github.com/stream-studio/gstreamer-stream-studio-plugins-rs"
description = "RTMP Server"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
gst = { package = "gstreamer", git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", features = ["v1_16"] }
gst-base = { package = "gstreamer-base", git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", features = ["v1_16"] }
anyhow = "1"
once_cell = "1.0"

[build-dependencies]
gst-plugin-version-helper = "0.7.5"

[dev-dependencies]
gst-check = { package = "gstreamer-check",  git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", features = ["v1_20"] }


[lib]
name = "gstrtmpserver"
crate-type = ["cdylib", "rlib"]
path = "src/lib.rs"

[features]
static = []
capi = []
doc = []

[package.metadata.capi]
min_version = "0.8.0"

[package.metadata.capi.header]
enabled = false

[package.metadata.capi.library]
install_subdir = "gstreamer-1.0"
versioning = false

[package.metadata.capi.pkg_config]
requires_private = "gstreamer-1.0, gstreamer-base-1.0, gobject-2.0, glib-2.0, gmodule-2.0"

```

Your plugin path in both members and default member

```
members = [
    'general/publish',
    'net/rtmpserver'
]

# Only plugins without external dependencies
default-members = [
    'general/publish',
    'net/rtmpserver'
]
```

In meson.build add a line in plugins part
```
plugins = {
  'rtmpserver': 'libgstrtmpserver',
  'publish': 'libgstpublish',
}
```
