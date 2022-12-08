use gst::{glib, subclass::prelude::*, ffi::GstElement, prelude::{ElementExtManual, GstBinExtManual, ParamSpecBuilderExt, ObjectExt}, traits::GstBinExt};
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


pub struct RecordSink {
    settings: Mutex<Settings>,
    aqueue: gst::Element,
    vqueue: gst::Element,
    muxer: gst::Element,
    sink: gst::Element,
}

impl Default for RecordSink {
    fn default() -> Self {
        let aqueue = gst::ElementFactory::make("queue")
            .build()
            .expect("Failed to create audio queue");

        let vqueue = gst::ElementFactory::make("queue")
            .build()
            .expect("Failed to create video queue");     
            
        let muxer = gst::ElementFactory::make("mp4mux")
            .build()
            .expect("Failed to create muxer");                 

        let sink = gst::ElementFactory::make("filesink")
            .build()
            .expect("Failed to create filesink");     

        Self {
            settings: Mutex::new(Settings::default()),
            aqueue: aqueue,
            vqueue: vqueue,
            muxer: muxer,
            sink: sink
        }
    }
}




static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "RecordSink",
        gst::DebugColorFlags::empty(),
        Some("Record Sink Element"),
    )
});

#[glib::object_subclass]
impl ObjectSubclass for RecordSink {
    const NAME: &'static str = "GstRecordSink";
    type Type = super::RecordSink;
    type ParentType = gst::Bin;
}

impl ObjectImpl for RecordSink {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::builder("location")
                    .nick("Output file location")
                    .blurb("Output file location")
                    .build(),
            ]});
        
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "location" => {
                self.sink.set_property("location", value
                .get::<Option<String>>()
                .expect("Location value is expected"));
            }
            _ => unimplemented!(),
        }
    }


    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "location" => {
                self.sink.property("location")
            }
            _ => unimplemented!(),
        }
    }



    fn constructed(&self) {
        self.parent_constructed();
        self.obj()
            .set_suppressed_flags(gst::ElementFlags::SINK | gst::ElementFlags::SOURCE);
        self.obj().set_element_flags(gst::ElementFlags::SINK);
        
        self.obj().add_many(&[&self.aqueue, &self.vqueue, &self.muxer, &self.sink]).unwrap();

        self.aqueue.link(&self.muxer).unwrap();
        self.vqueue.link(&self.muxer).unwrap();
        self.muxer.link(&self.sink).unwrap();

    }
}

impl GstObjectImpl for RecordSink {}

impl BinImpl for RecordSink {}

impl ElementImpl for RecordSink {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Record Sink",
                "General",
                "Record Sink",
                "Ludovic Bouguerra <ludovic.bouguerra@stream.studio>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }
}