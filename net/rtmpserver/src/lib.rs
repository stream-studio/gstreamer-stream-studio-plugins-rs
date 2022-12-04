use gst::glib;
mod rtmpserver;


fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    rtmpserver::register(plugin);
    Ok(())
}

gst::plugin_define!(
    rtmpserver,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    // FIXME: MPL-2.0 is only allowed since 1.18.3 (as unknown) and 1.20 (as known)
    "MPL",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
