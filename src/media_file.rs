/// The container for [`<MediaFile>`](MediaFile), [`<Mezzanine>`](Mezzanine),
/// [`<InteractiveCreativeFile>`](InteractiveCreativeFile) and
/// [`ClosedCaptionFiles`](ClosedCaptionFiles).
///
/// ```text
/// <xs:element name="MediaFiles" minOccurs="1" maxOccurs="1">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="ClosedCaptionFiles" minOccurs="0" maxOccurs="1">
///       <xs:element name="MediaFile" minOccurs="1" maxOccurs="unbounded">
///       <xs:element name="Mezzanine" minOccurs="0" maxOccurs="unbounded">
///       <xs:element name="InteractiveCreativeFile" minOccurs="0" maxOccurs="unbounded">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "MediaFiles", strict(unknown_attribute, unknown_element))]
pub struct MediaFiles<'a> {
    /// The container for zero or one [`<ClosedCaptionFiles>`](ClosedCaptionFiles) element.
    #[xml(child = "ClosedCaptionFiles", default)]
    pub closed_caption_files: Option<ClosedCaptionFiles<'a>>,
    /// The container for one or more [`<MediaFile>`](MediaFile) element.
    #[xml(child = "MediaFile")]
    pub media_files: Vec<MediaFile<'a>>,
    /// The container for zero or more [`<Mezzanine>`](Mezzanine) elements.
    #[xml(child = "Mezzanine", default)]
    pub mezzanines: Vec<Mezzanine<'a>>,
    /// The container for zero or more [`<InteractiveCreativeFile>`](InteractiveCreativeFile)
    /// elements.
    #[xml(child = "InteractiveCreativeFile", default)]
    pub interactive_creative_files: Vec<InteractiveCreativeFile<'a>>,
}

/// In VAST 4.x `<MediaFile>` should only be used to contain the video or audio file for a
/// [`Linear`](crate::Linear) ad.
///
/// ```text
/// <xs:element name="MediaFile">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="id" type="xs:string" use="optional">
///         <xs:attribute name="delivery" use="required">
///         <xs:attribute name="type" type="xs:string" use="required">
///         <xs:attribute name="width" type="xs:integer" use="required">
///         <xs:attribute name="height" type="xs:integer" use="required">
///         <xs:attribute name="codec" type="xs:string" use="optional">
///         <xs:attribute name="bitrate" type="xs:integer" use="optional">
///         <xs:attribute name="minBitrate" type="xs:integer" use="optional">
///         <xs:attribute name="maxBitrate" type="xs:integer" use="optional">
///         <xs:attribute name="scalable" type="xs:boolean" use="optional">
///         <xs:attribute name="maintainAspectRatio" type="xs:boolean" use="optional">
///         <xs:attribute name="fileSize" type="xs:integer" use="optional">
///         <xs:attribute name="mediaType" type="xs:string" use="optional">
///         <xs:attribute name="apiFramework" type="xs:string" use="optional">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "MediaFile", strict(unknown_attribute, unknown_element))]
pub struct MediaFile<'a> {
    /// An identifier for the media file.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,
    /// Either “progressive” for progressive download protocols (such as HTTP) or
    /// “streaming” for streaming protocols.
    #[xml(attr = "delivery")]
    pub delivery: DeliveryProtocol,
    // TODO: typed value
    /// MIME type for the file container. Popular MIME types include, but are not
    /// limited to “video/mp4” for MP4, “audio/mpeg” and "audio/aac" for audio ads.
    #[xml(attr = "type")]
    pub mime_type: std::borrow::Cow<'a, str>,
    /// The native width of the video file, in pixels. (0 for audio ads)
    #[xml(attr = "width")]
    pub width: i32,
    /// The native height of the video file, in pixels. (0 for audio ads)
    #[xml(attr = "height")]
    pub height: i32,
    // TODO: typed value
    /// The codec used to encode the file which can take values as specified by
    /// [RFC 4281](http://tools.ietf.org/html/rfc4281).
    #[xml(attr = "codec", default)]
    pub codec: Option<std::borrow::Cow<'a, str>>,
    /// For progressive load video or audio, the `bitrate` value specifies the average
    /// bitrate for the media file; otherwise the `minBitrate` and `maxBitrate` can be
    /// used together to specify the minimum and maximum bitrates for streaming
    /// videos or audio files.
    #[xml(attr = "bitrate", default)]
    pub bitrate: Option<i32>,
    /// The `minBitrate` is minimum bitrate for streaming videos or audio files.
    /// Can be used with `maxBitrate` to specify bitrate.
    #[xml(attr = "minBitrate", default)]
    pub min_bitrate: Option<i32>,
    /// The `maxBitrate` is maximum bitrate for streaming videos or audio files.
    /// Can be used with `minBitrate` to specify bitrate.
    #[xml(attr = "maxBitrate", default)]
    pub max_bitrate: Option<i32>,
    /// A Boolean value that indicates whether the media file is meant to scale to larger
    /// dimensions.
    #[xml(attr = "scalable", default)]
    pub scalable: Option<bool>,
    /// A Boolean value that indicates whether aspect ratio for media file dimensions
    /// should be maintained when scaled to new dimensions.
    #[xml(attr = "maintainAspectRatio", default)]
    pub maintain_aspect_ratio: Option<bool>,
    /// Optional field that helps eliminate the need to calculate the size based on
    /// bitrate and duration.
    /// Units - Bytes
    #[xml(attr = "fileSize", default)]
    pub file_size: Option<i32>,
    // TODO: typed value
    /// Type of media file (2D / 3D / 360 / etc).
    /// Optional. Default value = 2D
    #[xml(attr = "mediaType", default)]
    pub media_type: Option<std::borrow::Cow<'a, str>>,
    /// Identifies the API needed to execute an interactive media file,
    /// but current support is for backward compatibility. Please use the
    /// [`<InteractiveCreativeFile>`](InteractiveCreativeFile) element to
    /// include files that require an API for execution.
    #[deprecated(since = "VAST 4.1")]
    #[xml(attr = "apiFramework", default)]
    pub api_framework: Option<std::borrow::Cow<'a, str>>,

    /// A CDATA-wrapped URI to a media file.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// Represents the video delivery protocol type.
///
/// ```text
/// <xs:simpleType>
///   <xs:restriction base="xs:token">
///     <xs:enumeration value="streaming" />
///     <xs:enumeration value="progressive" />
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum DeliveryProtocol {
    /// Progressive download protocols (such as HTTP).
    #[default]
    Progressive,
    /// Streaming protocols.
    Streaming,
}

impl std::str::FromStr for DeliveryProtocol {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "progressive" => Self::Progressive,
            "streaming" => Self::Streaming,
            _ => {
                return Err(crate::VastParseError::new(format!(
                    "delivery protocol parsing error: '{s}'"
                )));
            }
        })
    }
}

impl std::fmt::Display for DeliveryProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Progressive => write!(f, "progressive"),
            Self::Streaming => write!(f, "streaming"),
        }
    }
}

/// The mezzanine file specifications are defined in the [Digital Video Ad Format Guidelines](https://www.iab.com/wp-content/uploads/2016/01/DVAFG_2015-01-08.pdf).
///
/// ```text
/// <xs:element name="Mezzanine" minOccurs="0" maxOccurs="unbounded">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="id" type="xs:string" use="optional">
///         <xs:attribute name="delivery" use="required">
///         <xs:attribute name="type" type="xs:string" use="required">
///         <xs:attribute name="width" type="xs:integer" use="required">
///         <xs:attribute name="height" type="xs:integer" use="required">
///         <xs:attribute name="codec" type="xs:string" use="optional">
///         <xs:attribute name="fileSize" type="xs:integer" use="optional">
///         <xs:attribute name="mediaType" type="xs:string" use="optional">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Mezzanine", strict(unknown_attribute, unknown_element))]
pub struct Mezzanine<'a> {
    /// An identifier for the media file.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,
    /// Either “progressive” for progressive download protocols (such as HTTP) or
    /// “streaming” for streaming protocols.
    #[xml(attr = "delivery")]
    pub delivery: DeliveryProtocol,
    /// MIME type for the file container. Popular MIME types include, but are not
    /// limited to “video/mp4” for MP4, “audio/mpeg” and "audio/aac" for audio ads.
    #[xml(attr = "type")]
    pub mime_type: std::borrow::Cow<'a, str>,
    /// The native width of the video file, in pixels. (0 for audio ads)
    #[xml(attr = "width")]
    pub width: i32,
    /// The native height of the video file, in pixels. (0 for audio ads)
    #[xml(attr = "height")]
    pub height: i32,
    /// The codec used to encode the file which can take values as specified by
    /// [RFC 4281](http://tools.ietf.org/html/rfc4281).
    #[xml(attr = "codec", default)]
    pub codec: Option<std::borrow::Cow<'a, str>>,
    /// Optional field that helps eliminate the need to calculate the size based on
    /// bitrate and duration.
    /// Units - Bytes
    #[xml(attr = "fileSize", default)]
    pub file_size: Option<i32>,
    // TODO: typed value
    /// Type of media file (2D / 3D / 360 / etc).
    /// Optional. Default value = 2D
    #[xml(attr = "mediaType", default)]
    pub media_type: Option<std::borrow::Cow<'a, str>>,

    /// A CDATA-wrapped URI to a raw, high-quality media file.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// The `<InteractiveCreativeFile>` element is used to identify the file and the framework
/// needed for execution.
///
/// ```text
/// <xs:element name="InteractiveCreativeFile" minOccurs="0" maxOccurs="unbounded">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="type" type="xs:string" use="optional">
///         <xs:attribute name="apiFramework" type="xs:string" use="optional">
///         <xs:attribute name="variableDuration" type="xs:boolean" use="optional">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "InteractiveCreativeFile", strict(unknown_attribute, unknown_element))]
pub struct InteractiveCreativeFile<'a> {
    /// Identifies the MIME type of the file provided.
    #[xml(attr = "type", default)]
    pub mime_type: Option<std::borrow::Cow<'a, str>>,
    /// Identifies the API needed to execute the resource file if applicable.
    #[xml(attr = "apiFramework", default)]
    pub api_framework: Option<std::borrow::Cow<'a, str>>,
    /// Useful for interactive use cases. Identifies whether the ad always drops when the
    /// duration is reached, or if it can potentially extend the duration by pausing the
    /// underlying video or delaying the adStopped call after adVideoComplete.
    #[xml(attr = "variableDuration", default)]
    pub variable_duration: Option<bool>,

    /// A CDATA-wrapped URI to a file providing creative functions for the media file.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// Optional node that enables closed caption sidecar files associated with the ad media (video or
/// audio) to be provided to the player.
///
/// ```text
/// <xs:element name="ClosedCaptionFiles">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="ClosedCaptionFile" minOccurs="0" maxOccurs="unbounded">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "ClosedCaptionFiles", strict(unknown_attribute, unknown_element))]
pub struct ClosedCaptionFiles<'a> {
    /// The container for zero or more [`<ClosedCaptionFile>`](ClosedCaptionFile) element.
    #[xml(child = "ClosedCaptionFile")]
    pub closed_caption_files: Vec<ClosedCaptionFile<'a>>,
}

/// Individual closed caption files for various languages.
///
/// ```text
/// <xs:element name="ClosedCaptionFile">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="type" type="xs:string" use="optional">
///         <xs:attribute name="language" type="xs:string" use="optional">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "ClosedCaptionFile", strict(unknown_attribute, unknown_element))]
pub struct ClosedCaptionFile<'a> {
    /// Identifies the MIME type of the file provided.
    #[xml(attr = "type", default)]
    pub mime_type: Option<std::borrow::Cow<'a, str>>,
    // TODO: typed value
    /// Language of the Closed Caption File using ISO 631-1 codes. An optional locale
    /// suffix can also be provided.
    /// Example:- “en”, “en-US”, “zh-TW”
    #[xml(attr = "language", default)]
    pub language: Option<std::borrow::Cow<'a, str>>,

    /// A CDATA-wrapped URI to a file providing Closed Caption info for the media file.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}
