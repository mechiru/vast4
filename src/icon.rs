/// The container for zero or one [`<Icon>`](Icon) element.
///
/// ```text
/// <xs:element name="Icons">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="Icon" minOccurs="1" maxOccurs="unbounded" type="vast:Icon_type">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Icons", strict(unknown_attribute, unknown_element))]
pub struct Icons<'a> {
    /// The container for one or more [`<Icon>`](Icon) elements.
    #[xml(child = "Icon")]
    pub icons: Vec<Icon<'a>>,
}

/// The Icon is used to provide one or more creative files for the icon that represents
/// the program being implemented along with any icon tracking elements.
///
/// ```text
/// <xs:complexType name="Icon">
///   <!-- CreativeResource_type -->
///   <xs:sequence>
///     <xs:element name="HTMLResource" minOccurs="0" maxOccurs="unbounded" type="vast:HTMLResource_type">
///     <xs:element name="IFrameResource" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI">
///     <xs:element name="StaticResource" minOccurs="0" maxOccurs="unbounded">
///   </xs:sequence>
///
///   <xs:attribute name="program" type="xs:string">
///   <xs:attribute name="width" type="xs:integer">
///   <xs:attribute name="height" type="xs:integer">
///   <xs:attribute name="xPosition">
///   <xs:attribute name="yPosition">
///   <xs:attribute name="duration" type="xs:time">
///   <xs:attribute name="offset" type="xs:time">
///   <xs:attribute name="apiFramework" type="xs:string">
///   <xs:attribute name="pxratio" type="xs:decimal">
///   <xs:sequence>
///     <xs:element name="IconClicks" minOccurs="0" maxOccurs="1">
///     <xs:element name="IconViewTracking" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Icon", strict(unknown_attribute, unknown_element))]
pub struct Icon<'a> {
    /// The program represented in the icon (e.g. "AdChoices").
    #[xml(attr = "program", default)]
    pub program: Option<std::borrow::Cow<'a, str>>,
    /// Pixel width of the icon asset.
    #[xml(attr = "width", default)]
    pub width: Option<i32>,
    /// Pixel height of the icon asset.
    #[xml(attr = "height", default)]
    pub height: Option<i32>,
    /// The x-coordinate of the top, left corner of the icon asset relative to the ad display
    /// area. Values of "left" or "right" also accepted and indicate the leftmost or rightmost
    /// available position for the icon asset.
    #[xml(attr = "xPosition", default)]
    pub x_position: Option<XPosition>,
    /// The y-coordinate of the top left corner of the icon asset relative to the ad display
    /// area; values of "top" or "bottom" also accepted and indicate the topmost or
    /// bottommost available position for the icon asset.
    #[xml(attr = "yPosition", default)]
    pub y_position: Option<YPosition>,
    /// The duration the icon should be displayed unless clicked or ad is finished playing.
    #[xml(attr = "duration", default)]
    pub duration: Option<crate::Duration>,
    /// The time of delay from when the associated linear creative begins playing to when
    /// the icon should be displayed.
    #[xml(attr = "offset", default)]
    pub offset: Option<crate::Duration>,
    /// Identifies the API needed to execute the icon resource file if applicable.
    #[xml(attr = "apiFramework", default)]
    pub api_framework: Option<std::borrow::Cow<'a, str>>,
    /// The pixel ratio for which the icon creative is intended. The pixel ratio is the ratio of
    /// physical pixels on the device to the device-independent pixels. An ad intended for
    /// display on a device with a pixel ratio that is twice that of a standard 1:1 pixel ratio
    /// would use the value "2 " Default value is "1".
    #[xml(attr = "pxratio", default)]
    pub pxratio: Option<f32>,

    /// The container for zero or more `<HTMLResource>` elements.
    #[xml(flatten_text = "HTMLResource", cdata, default)]
    pub html_resources: Vec<std::borrow::Cow<'a, str>>,
    /// The container for zero or more `<IFrameResource>` elements.
    #[xml(flatten_text = "IFrameResource", cdata, default)]
    pub iframe_resources: Vec<std::borrow::Cow<'a, str>>,
    /// The container for zero or more [`<StaticResource>`](crate::StaticResource) elements.
    #[xml(child = "StaticResource", default)]
    pub static_resources: Vec<crate::StaticResource<'a>>,

    /// The container for zero or one [`<IconClicks>`](IconClicks) element.
    #[xml(child = "IconClicks", default)]
    pub icon_clicks: Option<IconClicks<'a>>,
    /// The container for zero or more `<IconViewTracking>` elements.
    #[xml(flatten_text = "IconViewTracking", cdata, default)]
    pub icon_view_trackings: Vec<std::borrow::Cow<'a, str>>,
}

/// The x-cooridinate of the top, left corner of the icon asset relative to the ad display area.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum XPosition {
    Coordinate(i32),
    Left,
    Right,
}

impl std::str::FromStr for XPosition {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "left" => Self::Left,
            "right" => Self::Right,
            _ => match s.parse::<i32>() {
                Ok(x) => Self::Coordinate(x),
                Err(_) => {
                    return Err(crate::VastParseError::new(format!(
                        "x position parsing error: '{s}'",
                    )));
                }
            },
        })
    }
}

impl std::fmt::Display for XPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XPosition::Coordinate(x) => write!(f, "{x}"),
            XPosition::Left => write!(f, "left"),
            XPosition::Right => write!(f, "right"),
        }
    }
}

/// The y-cooridinate of the top left corner of the icon asset relative to the ad display area.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum YPosition {
    Coordinate(i32),
    Top,
    Bottom,
}

impl std::str::FromStr for YPosition {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "top" => Self::Top,
            "buttom" => Self::Bottom,
            _ => match s.parse::<i32>() {
                Ok(y) => Self::Coordinate(y),
                Err(_) => {
                    return Err(crate::VastParseError::new(format!(
                        "y position parsing error: '{s}'",
                    )));
                }
            },
        })
    }
}

impl std::fmt::Display for YPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YPosition::Coordinate(y) => write!(f, "{y}"),
            YPosition::Top => write!(f, "top"),
            YPosition::Bottom => write!(f, "bottom"),
        }
    }
}

/// The `<IconClicks>` element is a container for `<IconClickThrough>` and
/// [`<ClickTracking>`](crate::ClickTracking).
///
/// ```text
/// <xs:element name="IconClicks">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="IconClickFallbackImages" minOccurs="0" maxOccurs="1">
///       <xs:element name="IconClickThrough" minOccurs="0" maxOccurs="1" type="xs:anyURI">
///       <xs:element name="IconClickTracking" minOccurs="0" maxOccurs="unbounded" type="vast:IconClickTracking_type">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "IconClicks", strict(unknown_attribute, unknown_element))]
pub struct IconClicks<'a> {
    /// The container for zero or one [`<IconClickFallbackImages>`](IconClickFallbackImages)
    /// element.
    #[xml(child = "IconClickFallbackImages", default)]
    pub icon_click_fallback_images: Option<IconClickFallbackImages<'a>>,
    /// The container for zero or one `<IconClickThrough>` element.
    #[xml(flatten_text = "IconClickThrough", cdata, default)]
    pub icon_click_through: Option<std::borrow::Cow<'a, str>>,
    /// The container for zero or one [`<IconClickTracking>`](IconClickTracking) element.
    #[xml(child = "IconClickTracking", default)]
    pub icon_click_trackings: Vec<IconClickTracking<'a>>,
}

/// `<IconClickTracking>` is used to track click activity within the icon.
///
/// ```text
/// <xs:complexType name="IconClickTracking_type">
///   <xs:simpleContent>
///     <xs:extension base="xs:anyURI">
///       <xs:attribute name="id" type="xs:string" />
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "IconClickTracking", strict(unknown_attribute, unknown_element))]
pub struct IconClickTracking<'a> {
    /// An id for the click to be measured.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// A URI to the tracking resource file to be called when a click corresponding to the
    /// id attribute (if provided) occurs.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// The `<IconClickFallbackImages>` element is used to provide information disclosure for
/// platforms which do not support HTML rendering, by baking the information into an image.
///
/// ```text
/// <xs:element name="IconClickFallbackImages" minOccurs="0" maxOccurs="1">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="IconClickFallbackImage" minOccurs="1" maxOccurs="unbounded">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "IconClickFallbackImages", strict(unknown_attribute, unknown_element))]
pub struct IconClickFallbackImages<'a> {
    /// The container for one or more [`<IconClickFallbackImage>`](IconClickFallbackImage) element.
    #[xml(child = "IconClickFallbackImage")]
    pub icon_click_fallback_images: Vec<IconClickFallbackImage<'a>>,
}

/// The `<IconClickFallbackImage>` element is used to display information when an icon clicka
/// occurs.
///
/// ```text
/// <xs:element name="IconClickFallbackImage">
///   <xs:complexType>
///     <xs:attribute name="height" type="xs:integer" use="optional">
///     <xs:attribute name="width" type="xs:integer" use="optional">
///     <xs:sequence>
///       <xs:element name="AltText" minOccurs="0" maxOccurs="1" type="xs:string" >
///       <xs:element name="StaticResource" minOccurs="0" maxOccurs="1" type="xs:anyURI">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "IconClickFallbackImage", strict(unknown_attribute, unknown_element))]
pub struct IconClickFallbackImage<'a> {
    /// Pixel height of the image asset.
    #[xml(attr = "height", default)]
    pub height: Option<i32>,
    /// Pixel width of the image asset.
    #[xml(attr = "width", default)]
    pub width: Option<i32>,

    /// The container for zero or one `<AltText>` element.
    #[xml(flatten_text = "AltText", default)]
    pub alt_text: Option<std::borrow::Cow<'a, str>>,
    /// The container for zero or one `<StaticResource>` element.
    #[xml(flatten_text = "StaticResource", cdata, default)]
    pub static_resource: Option<std::borrow::Cow<'a, str>>,
}
