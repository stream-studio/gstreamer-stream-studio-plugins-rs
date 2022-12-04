use gst::glib;
use gst::prelude::*;

mod imp;


glib::wrapper! {
    pub struct RTMPServer(ObjectSubclass<imp::RTMPServer>) @extends gst::Bin, gst::Element, gst::Object;
}




pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "rtmpserver",
        gst::Rank::None,
        RTMPServer::static_type(),
    )?;
    Ok(())
}