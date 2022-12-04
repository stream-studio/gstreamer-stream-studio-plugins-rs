use gst::{glib, subclass::prelude::*};
use once_cell::sync::Lazy;
use std::sync::Mutex;




struct Settings {

}


impl Default for Settings {
    fn default() -> Self {
        Self {
        }
    }
}


#[derive(Default)]
pub struct RTMPServer {
    settings: Mutex<Settings>,


}

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "rtmpserver",
        gst::DebugColorFlags::empty(),
        Some("RTMP Server Element"),
    )
});

#[glib::object_subclass]
impl ObjectSubclass for RTMPServer {
    const NAME: &'static str = "GstRTMPServer";
    type Type = super::RTMPServer;
    type ParentType = gst::Bin;


    fn with_class(_klass: &Self::Class) -> Self {
        Self {
            settings: Mutex::new(Settings::default()),
        }
    }


}

impl ObjectImpl for RTMPServer {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

#[repr(C)]
pub(crate) struct Class {
    parent: gst::ffi::GstBinClass,
}

impl GstObjectImpl for RTMPServer {}

impl BinImpl for RTMPServer {}

impl ElementImpl for RTMPServer {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "RTMP Server",
                "Generic",
                "RTMP Server",
                "Ludovic Bouguerra <ludovic.bouguerra@stream.studio>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }
}