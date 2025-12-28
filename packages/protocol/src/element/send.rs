use crate::impl_from_trait;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use puniyu_protocol::element::send as puniyu_protocol_element;
use puniyu_types::element::send as puniyu_element;

#[napi(object, js_name = "SendTextElement")]
pub struct TextElement {
    /// 文本元素内容
    pub text: String,
}

impl_from_trait!(TextElement, puniyu_element::TextElement, {
    text => text,
});
impl_from_trait!(TextElement, puniyu_protocol_element::TextElement, {
    text => text,
});

#[napi(object, js_name = "SendAtElement")]
pub struct AtElement {
    /// at元素目标id
    pub target_id: String,
}

impl_from_trait!(AtElement, puniyu_element::AtElement, {
    target_id => target_id,
});
impl_from_trait!(AtElement, puniyu_protocol_element::AtElement, {
    target_id => target_id,
});

#[napi(object, js_name = "SendReplyElement")]
pub struct ReplyElement {
    /// 回复元素id
    pub message_id: String,
}

impl_from_trait!(ReplyElement, puniyu_element::ReplyElement, {
    message_id => message_id,
});
impl_from_trait!(ReplyElement, puniyu_protocol_element::ReplyElement, {
    message_id => message_id,
});

#[napi(object, js_name = "SendFaceElement")]
pub struct FaceElement {
    /// 表情元素id
    pub id: u32,
}

impl From<FaceElement> for puniyu_element::FaceElement {
    fn from(element: FaceElement) -> Self {
        Self {
            id: element.id as u64,
        }
    }
}

impl From<puniyu_element::FaceElement> for FaceElement {
    fn from(element: puniyu_element::FaceElement) -> Self {
        Self {
            id: element.id as u32,
        }
    }
}

impl From<FaceElement> for puniyu_protocol_element::FaceElement {
    fn from(element: FaceElement) -> Self {
        Self {
            id: element.id as u64,
        }
    }
}

impl From<puniyu_protocol_element::FaceElement> for FaceElement {
    fn from(element: puniyu_protocol_element::FaceElement) -> Self {
        Self {
            id: element.id as u32,
        }
    }
}

#[napi(object, js_name = "SendImageElement")]
pub struct ImageElement {
    /// 图片元素
    pub file: Buffer,
    /// 图片外显
    pub summary: Option<String>,
}

impl From<ImageElement> for puniyu_element::ImageElement {
    fn from(element: ImageElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            summary: element.summary,
        }
    }
}

impl From<puniyu_element::ImageElement> for ImageElement {
    fn from(element: puniyu_element::ImageElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            summary: element.summary,
        }
    }
}

impl From<ImageElement> for puniyu_protocol_element::ImageElement {
    fn from(element: ImageElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            summary: element.summary,
        }
    }
}

impl From<puniyu_protocol_element::ImageElement> for ImageElement {
    fn from(element: puniyu_protocol_element::ImageElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            summary: element.summary,
        }
    }
}

#[napi(object, js_name = "SendFileElement")]
pub struct FileElement {
    /// 文件元素
    pub file: Buffer,
    /// 文件名
    pub file_name: String,
}

impl From<FileElement> for puniyu_element::FileElement {
    fn from(element: FileElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_name: element.file_name,
        }
    }
}

impl From<puniyu_element::FileElement> for FileElement {
    fn from(element: puniyu_element::FileElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_name: element.file_name,
        }
    }
}
impl From<FileElement> for puniyu_protocol_element::FileElement {
    fn from(element: FileElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_name: element.file_name,
        }
    }
}

impl From<puniyu_protocol_element::FileElement> for FileElement {
    fn from(element: puniyu_protocol_element::FileElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_name: element.file_name,
        }
    }
}

#[napi(object, js_name = "SendVideoElement")]
pub struct VideoElement {
    /// 视频元素
    pub file: Buffer,
    /// 视频文件名
    pub file_name: Option<String>,
}

impl From<VideoElement> for puniyu_element::VideoElement {
    fn from(element: VideoElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_name: element.file_name,
        }
    }
}
impl From<puniyu_element::VideoElement> for VideoElement {
    fn from(element: puniyu_element::VideoElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_name: element.file_name,
        }
    }
}
impl From<VideoElement> for puniyu_protocol_element::VideoElement {
    fn from(element: VideoElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_name: element.file_name,
        }
    }
}
impl From<puniyu_protocol_element::VideoElement> for VideoElement {
    fn from(element: puniyu_protocol_element::VideoElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_name: element.file_name,
        }
    }
}
#[napi(object, js_name = "SendRecordElement")]
pub struct RecordElement {
    /// 语言元素
    pub file: Buffer,
}

impl From<RecordElement> for puniyu_element::RecordElement {
    fn from(element: RecordElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
        }
    }
}

impl From<puniyu_element::RecordElement> for RecordElement {
    fn from(element: puniyu_element::RecordElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
        }
    }
}
impl From<RecordElement> for puniyu_protocol_element::RecordElement {
    fn from(element: RecordElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
        }
    }
}

impl From<puniyu_protocol_element::RecordElement> for RecordElement {
    fn from(element: puniyu_protocol_element::RecordElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
        }
    }
}

#[napi(object, js_name = "SendJsonElement")]
pub struct JsonElement {
    /// Json数据，未序列化
    pub data: String,
}

impl_from_trait!(JsonElement, puniyu_element::JsonElement, {
    data => data,
});
impl_from_trait!(JsonElement, puniyu_protocol_element::JsonElement, {
    data => data,
});
#[napi(object, js_name = "SendXmlElement")]
pub struct XmlElement {
    /// Xml数据，未序列化
    pub data: String,
}

impl_from_trait!(XmlElement, puniyu_element::XmlElement, {
    data => data,
});
impl_from_trait!(XmlElement, puniyu_protocol_element::XmlElement, {
    data => data,
});
#[napi(js_name = "SendElements")]
pub enum Elements {
    Text(TextElement),
    At(AtElement),
    Reply(ReplyElement),
    Face(FaceElement),
    Image(ImageElement),
    File(FileElement),
    Video(VideoElement),
    Record(RecordElement),
    Json(JsonElement),
    Xml(XmlElement),
}

impl From<Elements> for puniyu_element::Elements {
    fn from(element: Elements) -> Self {
        match element {
            Elements::Text(element) => puniyu_element::Elements::Text(element.into()),
            Elements::At(element) => puniyu_element::Elements::At(element.into()),
            Elements::Reply(element) => puniyu_element::Elements::Reply(element.into()),
            Elements::Face(element) => puniyu_element::Elements::Face(element.into()),
            Elements::Image(element) => puniyu_element::Elements::Image(element.into()),
            Elements::File(element) => puniyu_element::Elements::File(element.into()),
            Elements::Video(element) => puniyu_element::Elements::Video(element.into()),
            Elements::Record(element) => puniyu_element::Elements::Record(element.into()),
            Elements::Json(element) => puniyu_element::Elements::Json(element.into()),
            Elements::Xml(element) => puniyu_element::Elements::Xml(element.into()),
        }
    }
}

impl From<Elements> for puniyu_protocol_element::elements::Element {
    fn from(element: Elements) -> Self {
        match element {
            Elements::Text(element) => {
                puniyu_protocol_element::elements::Element::TextElement(element.into())
            }
            Elements::At(element) => {
                puniyu_protocol_element::elements::Element::AtElement(element.into())
            }
            Elements::Reply(element) => {
                puniyu_protocol_element::elements::Element::ReplyElement(element.into())
            }
            Elements::Face(element) => {
                puniyu_protocol_element::elements::Element::FaceElement(element.into())
            }
            Elements::Image(element) => {
                puniyu_protocol_element::elements::Element::ImageElement(element.into())
            }
            Elements::File(element) => {
                puniyu_protocol_element::elements::Element::FileElement(element.into())
            }
            Elements::Video(element) => {
                puniyu_protocol_element::elements::Element::VideoElement(element.into())
            }
            Elements::Record(element) => {
                puniyu_protocol_element::elements::Element::RecordElement(element.into())
            }
            Elements::Json(element) => {
                puniyu_protocol_element::elements::Element::JsonElement(element.into())
            }
            Elements::Xml(element) => {
                puniyu_protocol_element::elements::Element::XmlElement(element.into())
            }
        }
    }
}

impl From<puniyu_element::Elements> for Elements {
    fn from(element: puniyu_element::Elements) -> Self {
        match element {
            puniyu_element::Elements::Text(element) => Elements::Text(element.into()),
            puniyu_element::Elements::At(element) => Elements::At(element.into()),
            puniyu_element::Elements::Reply(element) => Elements::Reply(element.into()),
            puniyu_element::Elements::Face(element) => Elements::Face(element.into()),
            puniyu_element::Elements::Image(element) => Elements::Image(element.into()),
            puniyu_element::Elements::File(element) => Elements::File(element.into()),
            puniyu_element::Elements::Video(element) => Elements::Video(element.into()),
            puniyu_element::Elements::Record(element) => Elements::Record(element.into()),
            puniyu_element::Elements::Json(element) => Elements::Json(element.into()),
            puniyu_element::Elements::Xml(element) => Elements::Xml(element.into()),
        }
    }
}

impl From<puniyu_protocol_element::elements::Element> for Elements {
    fn from(element: puniyu_protocol_element::elements::Element) -> Self {
        match element {
            puniyu_protocol_element::elements::Element::TextElement(element) => {
                Elements::Text(element.into())
            }
            puniyu_protocol_element::elements::Element::AtElement(element) => {
                Elements::At(element.into())
            }
            puniyu_protocol_element::elements::Element::ReplyElement(element) => {
                Elements::Reply(element.into())
            }
            puniyu_protocol_element::elements::Element::FaceElement(element) => {
                Elements::Face(element.into())
            }
            puniyu_protocol_element::elements::Element::ImageElement(element) => {
                Elements::Image(element.into())
            }
            puniyu_protocol_element::elements::Element::FileElement(element) => {
                Elements::File(element.into())
            }
            puniyu_protocol_element::elements::Element::VideoElement(element) => {
                Elements::Video(element.into())
            }
            puniyu_protocol_element::elements::Element::RecordElement(element) => {
                Elements::Record(element.into())
            }
            puniyu_protocol_element::elements::Element::JsonElement(element) => {
                Elements::Json(element.into())
            }
            puniyu_protocol_element::elements::Element::XmlElement(element) => {
                Elements::Xml(element.into())
            }
        }
    }
}

impl From<Elements> for puniyu_protocol_element::Elements {
    fn from(element: Elements) -> Self {
        Self {
            element: Some(element.into()),
        }
    }
}

impl From<puniyu_protocol_element::Elements> for Elements {
    fn from(element: puniyu_protocol_element::Elements) -> Self {
        element.element.unwrap().into()
    }
}
