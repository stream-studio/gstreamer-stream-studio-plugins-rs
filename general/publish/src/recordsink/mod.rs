use gst::glib;
use gst::prelude::*;

mod imp;


glib::wrapper! {
    pub struct RecordSink(ObjectSubclass<imp::RecordSink>) @extends gst::Bin, gst::Element, gst::Object;
}




pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "recordsink",
        gst::Rank::None,
        RecordSink::static_type(),
    )?;
    Ok(())
}