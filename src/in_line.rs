/// Second-level element surrounding complete ad data for a single ad.
///
/// ```text
/// <xs:element name="InLine">
///   <xs:complexType>
///     <xs:sequence>
///       <!-- AdDefinitionBase_type -->
///       <xs:element name="AdSystem" minOccurs="1" maxOccurs="1">
///       <xs:element name="Error" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI">
///       <xs:element name="Extensions" minOccurs="0" maxOccurs="1">
///       <xs:element name="Impression" minOccurs="1" maxOccurs="unbounded" type="vast:Impression_type">
///       <xs:element name="Pricing" minOccurs="0" maxOccurs="1">
///       <xs:element name="ViewableImpression" minOccurs="0" maxOccurs="1" type="vast:ViewableImpression_type" />
///
///       <xs:element name="AdServingId" minOccurs="1" maxOccurs="1" type="xs:string">
///       <xs:element name="AdTitle" minOccurs="1" maxOccurs="1" type="xs:string">
///       <xs:element name="AdVerifications" minOccurs="0" maxOccurs="1" type="vast:AdVerifications_type" />
///       <xs:element name="Advertiser" minOccurs="0" maxOccurs="1" type="xs:string">
///       <xs:element name="Category" minOccurs="0" maxOccurs="unbounded">
///       <xs:element name="Creatives" minOccurs="1" maxOccurs="1">
///       <xs:element name="Description" minOccurs="0" maxOccurs="1" type="xs:string">
///       <xs:element name="Expires" minOccurs="0" maxOccurs="1" type="xs:integer">
///       <xs:element name="Survey" minOccurs="0" maxOccurs="1">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "InLine", strict(unknown_attribute, unknown_element))]
pub struct InLine<'a> {
    /// The [`<AdSystem>`](AdSystem) element.
    #[xml(child = "AdSystem")]
    pub ad_system: AdSystem<'a>,
    /// The container for zero or more `<Error>` elements.
    #[xml(flatten_text = "Error", cdata, default)]
    pub errors: Vec<std::borrow::Cow<'a, str>>,
    /// The container for zero or one [`<Extensions>`](crate::Extensions) element.
    #[xml(child = "Extensions", default)]
    pub extensions: Option<crate::Extensions>,
    /// The container for one or more [`<Impression>`](Impression) elements.
    #[xml(child = "Impression")]
    pub impressions: Vec<Impression<'a>>,
    /// The container for zero or one [`<Pricing>`](Pricing) element.
    #[xml(child = "Pricing", default)]
    pub pricing: Option<Pricing<'a>>,
    /// The container for zero or one [`<ViewableImpression>`](crate::ViewableImpression) element.
    #[xml(child = "ViewableImpression", default)]
    pub viewable_impression: Option<crate::ViewableImpression<'a>>,

    /// Any ad server that returns a VAST containing an `<InLine>` ad must generate a pseudo-unique
    /// identifier that is appropriate for all involved parties to track the lifecycle of that ad.
    /// This should be inserted into the `<AdServingId>` element, and also be included on all
    /// outgoing tracking pixels. The value should be different for each Inline in a VAST. Usage of
    /// a GUID is recommended.
    #[xml(flatten_text = "AdServingId")]
    pub ad_serving_id: std::borrow::Cow<'a, str>,
    /// Common name of ad
    #[xml(flatten_text = "AdTitle")]
    pub ad_title: std::borrow::Cow<'a, str>,
    /// The container for zero or one [`<AdVerifications>`](crate::AdVerifications) element.
    #[xml(child = "AdVerifications", default)]
    pub ad_verifications: Option<crate::AdVerifications<'a>>,
    /// Name of advertiser as defined by the ad serving party.
    #[xml(flatten_text = "Advertiser", default)]
    pub advertiser: Option<std::borrow::Cow<'a, str>>,
    /// The container for zero or more [`<Category>`](Category) elements.
    #[xml(child = "Category", default)]
    pub categories: Vec<Category<'a>>,
    /// The [`<Creatives>`](crate::Creatives) elements.
    #[xml(child = "Creatives")]
    pub creatives: crate::Creatives<'a>,
    /// Longer description of ad.
    #[xml(flatten_text = "Description", default)]
    pub description: Option<std::borrow::Cow<'a, str>>,
    /// An integer value that defines the expiry period (in seconds).
    #[xml(flatten_text = "Expires", default)]
    pub expires: Option<i32>,
    /// The container for zero or one [`<Survey>`](Survey) elements.
    #[xml(child = "Survey", default)]
    pub surveys: Option<Survey<'a>>,
}

/// Indicates source ad server.
///
/// ```text
/// <xs:element name="AdSystem" minOccurs="1" maxOccurs="1">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:string">
///         <xs:attribute name="version" type="xs:string" use="optional">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "AdSystem", strict(unknown_attribute, unknown_element))]
pub struct AdSystem<'a> {
    /// Internal version used by ad system.
    #[xml(attr = "version", default)]
    pub version: Option<std::borrow::Cow<'a, str>>,

    /// A string that provides the name of the ad server that returned the ad.
    #[xml(text)]
    pub name: std::borrow::Cow<'a, str>,
}

/// [Impression] is a URI that directs the video player to a tracking resource file that
/// the video player should request when the first frame of the ad is displayed.
///
/// ```xml
/// <xs:complexType name="Impression_type" >
///   <xs:simpleContent>
///     <xs:extension base="xs:anyURI">
///       <xs:attribute name="id" type="xs:string" use="optional">
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Impression", strict(unknown_attribute, unknown_element))]
pub struct Impression<'a> {
    /// Ad server ID for the impression.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// A URI that directs the media player to a tracking resource file that the media player
    /// must use to notify the ad server when the impression occurs. If there is no reason to
    /// include an Impression element, the placeholder "about:blank" should be used instead of
    /// a tracking URL. The player should disregard dispatching the tracking URI if it is set to
    /// "about:blank"..
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// Used in creative separation and for compliance in certain programs, a category field is
/// needed to categorize the ad’s content.
///
/// ```text
/// <xs:element name="Category">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:string">
///         <xs:attribute name="authority" type="xs:anyURI" use="required">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Category", strict(unknown_attribute, unknown_element))]
pub struct Category<'a> {
    /// A URL for the organizational authority that produced the list being used to identify
    /// ad content category.
    #[xml(attr = "authority")]
    pub authority: std::borrow::Cow<'a, str>,

    /// A string that provides a category code or label that identifies the ad content
    /// category.
    #[xml(text)]
    pub code: std::borrow::Cow<'a, str>,
}

/// Pricing provides a value that represents a price that can be used by real-time
/// bidding (RTB) systems. VAST is not designed to handle RTB since other methods
/// exist, but this element is offered for custom solutions if needed.
///
/// ```text
/// <xs:element name="Pricing" minOccurs="0" maxOccurs="1">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:decimal">
///         <xs:attribute name="model" use="required">
///         <xs:attribute name="currency" use="required">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Pricing", strict(unknown_attribute, unknown_element))]
pub struct Pricing<'a> {
    /// Identifies the pricing model as one of: CPM, CPC, CPE, or CPV.
    #[xml(attr = "model")]
    pub model: PriceModel,
    // TODO: typed value
    /// The three-letter ISO-4217 currency symbol that identifies the currency of the
    /// value provided (e.g. USD, GBP, etc.).
    #[xml(attr = "currency")]
    pub currency: std::borrow::Cow<'a, str>,

    /// A number that represents a price that can be used in real-time bidding systems.
    #[xml(text)]
    pub price: f64,
}

/// The pricing model used.
///
/// ```text
/// <xs:simpleType>
///   <xs:restriction base="xs:token">
///     <xs:enumeration value="CPC" />
///     <xs:enumeration value="CPM" />
///     <xs:enumeration value="CPE" />
///     <xs:enumeration value="CPV" />
///     <xs:enumeration value="cpc" />
///     <xs:enumeration value="cpm" />
///     <xs:enumeration value="cpe" />
///     <xs:enumeration value="cpv" />
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum PriceModel {
    /// CPM
    #[default]
    Cpm,
    /// CPC
    Cpc,
    /// CPE
    Cpe,
    /// CPV
    Cpv,
}

impl std::str::FromStr for PriceModel {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CPM" | "cpm" => Self::Cpm,
            "CPC" | "cpc" => Self::Cpc,
            "CPE" | "cpe" => Self::Cpe,
            "CPV" | "cpv" => Self::Cpv,
            _ => {
                return Err(crate::VastParseError::new(format!(
                    "price model parsing error: '{s}'",
                )));
            }
        })
    }
}

impl std::fmt::Display for PriceModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PriceModel::Cpm => write!(f, "cpm"),
            PriceModel::Cpc => write!(f, "cpc"),
            PriceModel::Cpe => write!(f, "cpe"),
            PriceModel::Cpv => write!(f, "cpv"),
        }
    }
}

/// The `<Survey>` element can be used to provide a URI to any resource file having to do
/// with collecting survey data.
///
/// ```text
/// <xs:element name="Survey" minOccurs="0" maxOccurs="1">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="type" type="xs:string" use="optional">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Survey", strict(unknown_attribute, unknown_element))]
pub struct Survey<'a> {
    /// The MIME type of the resource being served.
    #[xml(attr = "type", default)]
    pub mime_type: Option<std::borrow::Cow<'a, str>>,

    /// A URI to any resource relating to an integrated survey.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}
