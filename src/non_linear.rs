/// A container for the [`<NonLinear>`](NonLinear) creative files and tracking resources.
///
/// ```text
/// <xs:element name="NonLinearAds">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="TrackingEvents" minOccurs="0" maxOccurs="1" type="vast:TrackingEvents_type" />
///       <xs:element name="NonLinear" minOccurs="0" maxOccurs="unbounded" type="vast:NonLinearAd_Inline_type">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "NonLinearAds", strict(unknown_attribute, unknown_element))]
pub struct NonLinearAds<'a> {
    /// The container for zero or one `<TrackingEvents>`(crate::TrackingEvents) element.
    #[xml(child = "TrackingEvents", default)]
    pub tracking_events: Option<crate::TrackingEvents<'a>>,
    /// The container for zero or more [`<NonLinear>`](NonLinear) elements.
    #[xml(child = "NonLinear", default)]
    pub non_linears: Vec<NonLinear<'a>>,
}

/// Each `<NonLinear>` element may provide different versions of the same creative using the
/// [`<StaticResource>`](crate::StaticResource), `<IFrameResource>`, and `<HTMLResource>` elements
/// in the [`InLine`](crate::InLine) VAST response.
///
/// ## NonLinear in [`InLine`](crate::InLine):
/// ```text
/// <xs:complexType name="NonLinear">
///   <!-- CreativeResource_type -->
///   <xs:sequence>
///     <xs:element name="HTMLResource" minOccurs="0" maxOccurs="unbounded" type="vast:HTMLResource_type">
///     <xs:element name="IFrameResource" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI">
///     <xs:element name="StaticResource" minOccurs="0" maxOccurs="unbounded">
///   </xs:sequence>
///
///   <xs:attribute name="id" type="xs:string" use="optional">
///   <xs:attribute name="width" type="xs:integer" use="required">
///   <xs:attribute name="height" type="xs:integer" use="required">
///   <xs:attribute name="expandedWidth" type="xs:integer" use="optional">
///   <xs:attribute name="expandedHeight" type="xs:integer" use="optional">
///   <xs:attribute name="scalable" type="xs:boolean" use="optional">
///   <xs:attribute name="maintainAspectRatio" type="xs:boolean" use="optional">
///   <xs:attribute name="minSuggestedDuration" type="xs:time" use="optional">
///   <xs:attribute name="apiFramework" type="xs:string" use="optional">
///   <xs:sequence>
///     <xs:element name="AdParameters" minOccurs="0" maxOccurs="1" type="vast:AdParameters_type">
///     <xs:element name="NonLinearClickThrough" minOccurs="0" maxOccurs="1" type="xs:anyURI">
///     <xs:element name="NonLinearClickTracking" minOccurs="0" maxOccurs="unbounded">
///   </xs:sequence>
/// </xs:complexType>
/// ```
///
/// ## NonLinear in [`Wrapper`](crate::Wrapper):
/// ```text
/// <xs:complexType name="NonLinear">
///   <xs:element name="NonLinearClickTracking" minOccurs="0" maxOccurs="1">
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "NonLinear", strict(unknown_attribute, unknown_element))]
pub struct NonLinear<'a> {
    /// InLine: An optional identifier for the creative.
    /// Wrapper: No use this field.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,
    /// InLine: The pixel width of the placement slot for which the creative is intended.
    /// **Required this field inside [`InLine`](crate::InLine) element.**
    /// Wrapper: No use this field.
    #[xml(attr = "width", default)]
    pub width: Option<i32>,
    /// InLine: The pixel height of the placement slot for which the creative is intended.
    /// **Required this field inside [`InLine`](crate::InLine) element.**
    /// Wrapper: No use this field.
    #[xml(attr = "height", default)]
    pub height: Option<i32>,
    /// InLine: The maximum pixel width of the creative in its expanded state.
    /// Wrapper: No use this field.
    #[xml(attr = "expandedWidth", default)]
    pub expanded_width: Option<i32>,
    /// InLine: The maximum pixel height of the creative in its expanded state.
    /// Wrapper: No use this field.
    #[xml(attr = "expandedHeight", default)]
    pub expanded_height: Option<i32>,
    /// InLine: Identifies whether the creative can scale to new dimensions relative to the video
    /// player when the video player is resized.
    /// Wrapper: No use this field.
    #[xml(attr = "scalable", default)]
    pub scalable: Option<bool>,
    /// InLine: Identifies whether the aspect ratio of the creative should be maintained when it is
    /// scaled to new dimensions as the video player is resized.
    /// Wrapper: No use this field.
    #[xml(attr = "maintainAspectRatio", default)]
    pub maintain_aspect_ratio: Option<bool>,
    /// InLine: The minimum suggested duration that the creative should be displayed.
    /// Wrapper: No use this field.
    #[xml(attr = "minSuggestedDuration", default)]
    pub min_suggested_duration: Option<crate::Duration>,
    /// InLine: The API necessary to communicate with the creative if available.
    /// Wrapper: No use this field.
    #[xml(attr = "apiFramework", default)]
    pub api_framework: Option<std::borrow::Cow<'a, str>>,

    /// InLine: The container for zero or more `<HTMLResource>` elements.
    /// Wrapper: No use this field.
    #[xml(flatten_text = "HTMLResource", cdata, default)]
    pub html_resources: Vec<std::borrow::Cow<'a, str>>,
    /// InLine: The container for zero or more `<IFrameResource>` elements.
    /// Wrapper: No use this field.
    #[xml(flatten_text = "IFrameResource", cdata, default)]
    pub iframe_resources: Vec<std::borrow::Cow<'a, str>>,
    /// InLine: The container for zero or more [`<StaticResource>`](crate::StaticResource)
    /// elements. Wrapper: No use this field.
    #[xml(child = "StaticResource", default)]
    pub static_resources: Vec<crate::StaticResource<'a>>,

    /// InLine: The container for zero or one [`<AdParameters>`](crate::AdParameters) element.
    /// Wrapper: No use this field.
    #[xml(child = "AdParameters", default)]
    pub ad_parameters: Option<crate::AdParameters<'a>>,
    /// InLine: The container for zero or one `<NonLinearClickThrough>` element.
    /// Wrapper: No use this field.
    #[xml(flatten_text = "NonLinearClickThrough", cdata, default)]
    pub non_linear_click_through: Option<std::borrow::Cow<'a, str>>,
    /// InLine: The container for zero or one `<NonLinearClickTracking>` element.
    /// Wrapper: The container for zero or more `<NonLinearClickTracking>` elements.
    #[xml(child = "NonLinearClickTracking", default)]
    pub non_linear_click_trackings: Vec<NonLinearClickTracking<'a>>,
}

/// The `<NonLinearClickTracking>` element is used to track the click for [NonLinear] ad creative.
///
/// ```text
/// <xs:element name="NonLinearClickTracking">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="id" type="xs:string" use="optional">
///         </xs:attribute>
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "NonLinearClickTracking", strict(unknown_attribute, unknown_element))]
pub struct NonLinearClickTracking<'a> {
    /// An id provided by the ad server to track the click in reports.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// A URI to a tracking resource file used to track a [NonLinear] clickthrough.
    #[xml(text, cdata, default)]
    pub uri: std::borrow::Cow<'a, str>,
}
