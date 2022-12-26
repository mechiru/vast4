/// The `<VideoClicks>` element provides URIs for clickthroughs, clicktracking, and custom
/// clicks and is available for [`Linear`](crate::Linear) Ads in both the [`InLine`](crate::InLine)
/// and [`Wrapper`](crate::Wrapper) formats.
///
/// ```text
/// <xs:complexType name="VideoClicks_type" >
///   <xs:sequence>
///     <xs:element name="ClickTracking" minOccurs="0" maxOccurs="unbounded">
///     <xs:element name="ClickThrough" minOccurs="0" maxOccurs="1">
///     <xs:element name="CustomClick" minOccurs="0" maxOccurs="unbounded">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "VideoClicks", strict(unknown_attribute, unknown_element))]
pub struct VideoClicks<'a> {
    /// The container for zero or more [`<ClickTracking>`](ClickTracking) elements.
    #[xml(child = "ClickTracking")]
    pub click_trackings: Vec<ClickTracking<'a>>,
    /// The container for zero or one [`<ClickThrough>`](ClickThrough) element.
    #[xml(child = "ClickThrough", default)]
    pub click_through: Option<ClickThrough<'a>>,
    /// The container for zero or more [`<CustomClick>`](CustomClick) elements.
    #[xml(child = "CustomClick", default)]
    pub custom_clicks: Vec<CustomClick<'a>>,
}

/// The `<ClickThrough>` is a URI to the advertiser’s site that the media player opens when a
/// viewer clicks the ad.
///
/// ```text
/// <xs:element name="ClickThrough">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="id" type="xs:string" use="optional" />
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "ClickThrough", strict(unknown_attribute, unknown_element))]
pub struct ClickThrough<'a> {
    /// A unique ID for the clickthrough.
    #[xml(attr = "id")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// A URI to the advertiser’s site that the media player opens when a viewer clicks the ad.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// Multiple `<ClickTracking>` elements can be used in the case where multiple parties would
/// like to track the Linear ad clickthrough.
///
/// ```text
/// <xs:element name="ClickTracking">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="id" type="xs:string" use="optional" />
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "ClickTracking", strict(unknown_attribute, unknown_element))]
pub struct ClickTracking<'a> {
    /// A unique ID for the click to be tracked.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// A URI for tracking when the [ClickThrough] is triggered.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// The `<CustomClick>` is used to track any interactions with the linear ad that do not include
/// the clickthrough click and do not take the viewer away from the media player.
///
/// ```text
/// <xs:element name="CustomClick">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="id" type="xs:string" use="optional" />
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "CustomClick", strict(unknown_attribute, unknown_element))]
pub struct CustomClick<'a> {
    /// A unique ID for the custom click to be tracked.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// A URI for tracking custom interactions.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}
