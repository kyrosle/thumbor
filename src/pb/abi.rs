/// a ImageSpec is a array which has order, server will handle by the spec order
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSpec {
    #[prost(message, repeated, tag="1")]
    pub specs: ::prost::alloc::vec::Vec<Spec>,
}
/// change the image size
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resize {
    #[prost(uint32, tag="1")]
    pub width: u32,
    #[prost(uint32, tag="2")]
    pub height: u32,
    #[prost(enumeration="resize::ResizeType", tag="3")]
    pub rtype: i32,
    #[prost(enumeration="resize::SampleFilter", tag="4")]
    pub filter: i32,
}
/// Nested message and enum types in `Resize`.
pub mod resize {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResizeType {
        Normal = 0,
        /// seam carve
        SeamCarve = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SampleFilter {
        /// undefined
        Undefined = 0,
        /// nearest
        Nearest = 1,
        /// traingle
        Triangle = 2,
        /// catmull rom
        CatmullRom = 3,
        /// gaussian
        Gaussian = 4,
        /// lanczos3
        Lanczos3 = 5,
    }
}
/// Image crop
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Crop {
    #[prost(uint32, tag="1")]
    pub x1: u32,
    #[prost(uint32, tag="2")]
    pub y1: u32,
    #[prost(uint32, tag="3")]
    pub x2: u32,
    #[prost(uint32, tag="4")]
    pub y2: u32,
}
/// handler heritical flip
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fliph {
}
/// handler vertical flip
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flipv {
}
/// image contrast
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contrast {
    #[prost(float, tag="1")]
    pub contrast: f32,
}
/// handle filter
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(enumeration="filter::Filter", tag="1")]
    pub filter: i32,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Filter {
        /// unspecified
        Unspecified = 0,
        /// oceanic
        Oceanic = 1,
        /// islands
        Islands = 2,
        /// marine
        Marine = 3,
    }
}
/// handler watermark
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watermark {
    #[prost(uint32, tag="1")]
    pub x: u32,
    #[prost(uint32, tag="2")]
    pub y: u32,
}
/// contain above solve way
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spec {
    #[prost(oneof="spec::Data", tags="1, 2, 3, 4, 5, 6, 7")]
    pub data: ::core::option::Option<spec::Data>,
}
/// Nested message and enum types in `Spec`.
pub mod spec {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="1")]
        Resize(super::Resize),
        #[prost(message, tag="2")]
        Crop(super::Crop),
        #[prost(message, tag="3")]
        Flipv(super::Flipv),
        #[prost(message, tag="4")]
        Fliph(super::Fliph),
        #[prost(message, tag="5")]
        Contrast(super::Contrast),
        #[prost(message, tag="6")]
        Filter(super::Filter),
        #[prost(message, tag="7")]
        Watermark(super::Watermark),
    }
}
