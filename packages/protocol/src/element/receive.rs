use crate::impl_from_trait;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use puniyu_types::element::receive as puniyu_element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object, js_name = "ReceiveTextElement")]
pub struct TextElement {
    /// 文本元素内容
    pub text: String,
}

impl_from_trait!(TextElement,puniyu_element::TextElement, {
    text => text
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object, js_name = "ReceiveAtElement")]
pub struct AtElement {
    /// at元素目标id
    pub target_id: String,
}


impl_from_trait!(AtElement,puniyu_element::AtElement, {
    target_id => target_id
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object, js_name = "ReceiveReplyElement")]
pub struct ReplyElement {
    /// 回复元素id
    #[serde(rename = "messageId")]
    pub message_id: String,
}

impl_from_trait!(ReplyElement,puniyu_element::ReplyElement, {
    message_id => message_id
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object, js_name = "ReceiveFaceElement")]
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

#[napi(object, js_name = "ReceiveImageElement")]
pub struct ImageElement {
    /// 图片元素
    pub file: Buffer,
    /// 图片外显
    pub summary: String,
    /// 图片宽度
    pub width: u32,
    /// 图片高度
    pub height: u32,
}

impl From<ImageElement> for puniyu_element::ImageElement {
    fn from(element: ImageElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            summary: element.summary,
            width: element.width as u64,
            height: element.height as u64,
        }
    }
}

impl From<puniyu_element::ImageElement> for ImageElement {
    fn from(element: puniyu_element::ImageElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            summary: element.summary,
            width: element.width as u32,
            height: element.height as u32,
        }
    }
}

#[napi(object, js_name = "ReceiveFileElement")]
pub struct FileElement {
    /// 文件元素
    pub file: Buffer,
    /// 文件id
    pub file_id: String,
    /// 文件大小, 单位字节
    pub file_size: u32,
    /// 文件名称
    pub file_name: String,
}

impl From<FileElement> for puniyu_element::FileElement {
    fn from(element: FileElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_id: element.file_id,
            file_size: element.file_size as u64,
            file_name: element.file_name,
        }
    }
}

impl From<puniyu_element::FileElement> for FileElement {
    fn from(element: puniyu_element::FileElement) -> Self {
        Self {
            file: element.file.to_vec().into(),
            file_id: element.file_id,
            file_size: element.file_size as u32,
            file_name: element.file_name,
        }
    }
}
#[napi(object, js_name = "ReceiveVideoElement")]
pub struct VideoElement {
    /// 视频元素
    pub file: Buffer,
    /// 视频文件名
    pub file_name: String,
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

#[napi(object, js_name = "ReceiveRecordElement")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object, js_name = "ReceiveJsonElement")]
pub struct JsonElement {
    /// Json数据，未序列化
    pub data: String,
}

impl_from_trait!(JsonElement,puniyu_element::JsonElement, {
    data => data
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object, js_name = "ReceiveXmlElement")]
pub struct XmlElement {
    /// Xml数据，未序列化
    pub data: String,
}

impl_from_trait!(XmlElement,puniyu_element::XmlElement, {
    data => data
});

#[napi(js_name = "ReceiveElements")]
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
