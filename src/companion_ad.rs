/// Companion Ads are secondary ads included in the VAST tag that accompany the
/// video/audio ad.
///
/// ```text
/// <xs:complexType name="CompanionAds">
///   <xs:attribute name="required" use="optional">
///   <xs:sequence>
///     <xs:element name="Companion" minOccurs="0" maxOccurs="unbounded" type="vast:CompanionAd_type">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "CompanionAds", strict(unknown_attribute, unknown_element))]
pub struct CompanionAds<'a> {
    /// How the player should treat a companion ad when multiple are supplied.
    #[xml(attr = "required", default)]
    pub required: Option<CompanionRequirement>,

    /// The container for one or more [`<Companion>`](Companion) elements.
    #[xml(child = "Companion", default)]
    pub companions: Vec<Companion<'a>>,
}

/// The required attribute for the [`<CompanionAds>`](CompanionAds) element provides information
/// about which Companion creative to display when multiple Companions are supplied and whether the
/// ad can be displayed without its [Companion] creative.
///
/// ```text
/// <xs:simpleType>
///   <xs:restriction base="xs:token">
///     <xs:enumeration value="all" />
///     <xs:enumeration value="any" />
///     <xs:enumeration value="none" />
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CompanionRequirement {
    All,
    Any,
    None,
}

impl std::str::FromStr for CompanionRequirement {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "all" => Self::All,
            "any" => Self::Any,
            "none" => Self::None,
            _ => {
                return Err(crate::VastParseError::new(format!(
                    "required attribute parsing error: '{s}'"
                )));
            }
        })
    }
}

impl std::fmt::Display for CompanionRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Any => write!(f, "any"),
            Self::None => write!(f, "none"),
        }
    }
}

/// The resource elements for providing creative resources.
///
/// ```text
/// <xs:complexType name="Companion">
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
///   <xs:attribute name="assetWidth" type="xs:integer" use="optional">
///   <xs:attribute name="assetHeight" type="xs:integer" use="optional">
///   <xs:attribute name="expandedWidth" type="xs:integer" use="optional">
///   <xs:attribute name="expandedHeight" type="xs:integer" use="optional">
///   <xs:attribute name="apiFramework" type="xs:string" use="optional">
///   <xs:attribute name="adSlotId" type="xs:string" use="optional">
///   <xs:attribute name="pxratio" type="xs:decimal">
///   <xs:attribute name="renderingMode" use="optional">
///   <xs:sequence>
///     <xs:element name="AdParameters" minOccurs="0" maxOccurs="1" type="vast:AdParameters_type">
///     <xs:element name="AltText" minOccurs="0" maxOccurs="1" type="xs:string">
///     <xs:element name="CompanionClickThrough" minOccurs="0" maxOccurs="1" type="xs:anyURI">
///     <xs:element name="CompanionClickTracking" minOccurs="0" maxOccurs="unbounded">
///     <xs:element name="CreativeExtensions" minOccurs="0" maxOccurs="1" type="vast:CreativeExtensions_type" />
///     <xs:element name="TrackingEvents" minOccurs="0" maxOccurs="1" type="vast:TrackingEvents_type">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Companion", strict(unknown_attribute, unknown_element))]
pub struct Companion<'a> {
    /// An optional identifier for the creative.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,
    /// An optional identifier for the creative.
    #[xml(attr = "width")]
    pub width: i32,
    /// The pixel height of the placement slot for which the creative is intended.
    #[xml(attr = "height")]
    pub height: i32,
    /// The pixel width of the creative.
    #[xml(attr = "assetWidth", default)]
    pub asset_width: Option<i32>,
    /// The pixel height of the creative.
    #[xml(attr = "assetHeight", default)]
    pub asset_height: Option<i32>,
    /// The maximum pixel width of the creative in its expanded state.
    #[xml(attr = "expandedWidth", default)]
    pub expanded_width: Option<i32>,
    /// The maximum pixel height of the creative in its expanded state.
    #[xml(attr = "expandedHeight", default)]
    pub expanded_height: Option<i32>,
    /// The API necessary to communicate with the creative if available.
    #[xml(attr = "apiFramework", default)]
    pub api_framework: Option<std::borrow::Cow<'a, str>>,
    /// Used to identify desired placement on a publisher’s page. Values to be used should be
    /// discussed between publishers and advertisers.
    #[xml(attr = "adSlotId", default)]
    pub ad_slot_id: Option<std::borrow::Cow<'a, str>>,
    /// The pixel ratio for which the companion creative is intended. The pixel ratio is the
    /// ratio of physical pixels on the device to the device-independent pixels. An ad intended
    /// for display on a device with a pixel ratio that is twice that of a standard 1:1 pixel ratio
    /// would use the value "2." Default value is "1."
    #[xml(attr = "pxratio", default)]
    pub pxratio: Option<f32>,
    /// Used to indicate when and where to use this companion ad. Values can be “default” or
    /// “end-card” or “concurrent”. If this field is empty or not given, “default” will be used.
    #[xml(attr = "renderingMode", default)]
    pub rendering_mode: Option<RenderingMode>,

    /// The container for zero or more `<HTMLResource>` elements.
    #[xml(flatten_text = "HTMLResource", cdata, default)]
    pub html_resources: Vec<std::borrow::Cow<'a, str>>,
    /// The container for zero or more `<IFrameResource>` elements.
    #[xml(flatten_text = "IFrameResource", cdata, default)]
    pub iframe_resources: Vec<std::borrow::Cow<'a, str>>,
    /// The container for zero or more [`<StaticResource>`](crate::StaticResource) elements.
    #[xml(child = "StaticResource", default)]
    pub static_resources: Vec<crate::StaticResource<'a>>,

    /// The container for zero or one [`<AdParameters>`](crate::AdParameters) element.
    #[xml(child = "AdParameters", default)]
    pub ad_parameters: Option<crate::AdParameters<'a>>,
    /// The container for zero or one `<AltText>` element.
    #[xml(flatten_text = "AltText", default)]
    pub alt_text: Option<std::borrow::Cow<'a, str>>,
    /// The container for zero or one `<CompanionClickThrough>` element.
    #[xml(flatten_text = "CompanionClickThrough", cdata, default)]
    pub companion_click_through: Option<std::borrow::Cow<'a, str>>,
    /// The container for zero or more [`<CompanionClickTracking>`](CompanionClickTracking)
    /// elements.
    #[xml(child = "CompanionClickTracking", default)]
    pub companion_click_trackings: Vec<CompanionClickTracking<'a>>,
    /// The container for zero or more [`<CreativeExtensions>`](crate::CreativeExtensions) element.
    #[xml(child = "CreativeExtensions", default)]
    pub creative_extensiosn: Option<crate::CreativeExtensions>,
    /// The container for zero or one `<TrackingEvents>`(TrackingEvents) element.
    #[xml(child = "TrackingEvents", default)]
    pub tracking_events: Option<crate::TrackingEvents<'a>>,
}

/// Used to indicate when and where to use this companion ad. If this field is empty or not given,
/// default will be used.
///
/// ```text
/// <xs:simpleType>
///   <xs:restriction base="xs:token">
///     <xs:enumeration value="default" />
///     <xs:enumeration value="end-card" />
///     <xs:enumeration value="concurrent" />
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum RenderingMode {
    #[default]
    Default,
    EndCard,
    Concurrent,
}

impl std::str::FromStr for RenderingMode {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "default" => Self::Default,
            "end-card" => Self::EndCard,
            "concurrent" => Self::Concurrent,
            _ => {
                return Err(crate::VastParseError::new(format!(
                    "rendering mode parsing error: '{s}'",
                )));
            }
        })
    }
}

impl std::fmt::Display for RenderingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderingMode::Default => write!(f, "default"),
            RenderingMode::EndCard => write!(f, "end-card"),
            RenderingMode::Concurrent => write!(f, "concurrent"),
        }
    }
}

/// The `<CompanionClickTracking>` element is used to track the click.
///
/// ```text
/// <xs:element name="CompanionClickTracking">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="id" type="xs:string" use="required">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "CompanionClickTracking", strict(unknown_attribute, unknown_element))]
pub struct CompanionClickTracking<'a> {
    /// An id provided by the ad server to track the click in reports.
    #[xml(attr = "id")]
    pub id: std::borrow::Cow<'a, str>,

    /// A URI to a tracking resource file used to track a companion clickthrough.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}
