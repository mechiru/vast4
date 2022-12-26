/// Top-level element, wraps each ad in the response or ad unit in an ad pod. This MUST be present
/// unless an `Error` element is present.
///
/// ```text
/// <xs:element name="Ad">
///   <xs:complexType>
///     <xs:attribute name="id" type="xs:string" use="optional" />
///     <xs:attribute name="sequence" type="xs:integer" use="optional">
///     <xs:attribute name="conditionalAd" type="xs:boolean" use="optional">
///     <xs:attribute name="adType" use="optional">
///     <xs:choice>
///       <xs:element name="InLine" minOccurs="0" maxOccurs="1" type="vast:Inline_type">
///       <xs:element name="Wrapper" minOccurs="0" maxOccurs="1" type="vast:Wrapper_type">
///     </xs:choice>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Ad", strict(unknown_attribute, unknown_element))]
pub struct Ad<'a> {
    /// An ad server-defined identifier string for the ad.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,
    /// Identifies the sequence of multiple Ads that are part of an Ad Pod.
    #[xml(attr = "sequence", default)]
    pub sequence: Option<i32>,
    /// A Boolean value that identifies a conditional ad.
    #[deprecated(since = "VAST 4.1")]
    #[xml(attr = "conditionalAd", default)]
    pub conditional_ad: Option<bool>,
    /// An optional string that identifies the type of ad. This allows VAST to support audio ad
    /// scenarios. The default value is video.
    #[xml(attr = "adType", default)]
    pub ad_type: Option<std::borrow::Cow<'a, str>>,

    /// The container for zero or one [`<InLine>`](crate::InLine) element.
    #[xml(child = "InLine", default)]
    pub in_line: Option<crate::InLine<'a>>,
    /// Second-level element surrounding wrapper ad pointing to Secondary ad server.
    #[xml(child = "Wrapper", default)]
    pub wrapper: Option<crate::Wrapper<'a>>,
}

/// Identifier for type of ad.
///
/// ```text
/// <xs:simpleType>
///  <xs:restriction base="xs:token">
///     <xs:enumeration value="video" />
///     <xs:enumeration value="audio" />
///     <xs:enumeration value="hybrid" />
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum AdType {
    #[default]
    Video,
    Audio,
    Hybrid,
}

impl std::str::FromStr for AdType {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "video" => Self::Video,
            "audio" => Self::Audio,
            "hybrid" => Self::Hybrid,
            _ => return Err(crate::VastParseError::new(format!("ad type parsing error: '{s}'"))),
        })
    }
}

impl std::fmt::Display for AdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdType::Video => write!(f, "video"),
            AdType::Audio => write!(f, "audio"),
            AdType::Hybrid => write!(f, "hybrid"),
        }
    }
}
