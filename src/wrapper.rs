/// Second-level element surrounding wrapper ad pointing to Secondary ad server.
///
/// ```text
/// <xs:element name="Wrapper">
///   <xs:complexType>
///     <xs:sequence>
///       <!-- AdDefinitionBase_type -->
///       <xs:element name="AdSystem" minOccurs="1" maxOccurs="1">
///       <xs:element name="Error" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI">
///       <xs:element name="Extensions" minOccurs="0" maxOccurs="1">
///       <xs:element name="Impression" minOccurs="1" maxOccurs="unbounded" type="vast:Impression_type">
///       <xs:element name="Pricing" minOccurs="0" maxOccurs="1">
///       <xs:element name="ViewableImpression" minOccurs="0" maxOccurs="1" type="vast:ViewableImpression_type" />
///     </xs:sequence>
///
///     <xs:attribute name="followAdditionalWrappers" type="xs:boolean">
///     <xs:attribute name="allowMultipleAds" type="xs:boolean">
///     <xs:attribute name="fallbackOnNoAd" type="xs:boolean">
///     <xs:sequence>
///       <xs:element name="AdVerifications" minOccurs="0" maxOccurs="1" type="vast:AdVerifications_type" />
///       <xs:element name="BlockedAdCategories" minOccurs="0" maxOccurs="unbounded">
///       <xs:element name="Creatives" minOccurs="0" maxOccurs="1">
///       <xs:element name="VASTAdTagURI" minOccurs="1" maxOccurs="1" type="xs:anyURI">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Wrapper", strict(unknown_attribute, unknown_element))]
pub struct Wrapper<'a> {
    /// A Boolean value that identifies whether subsequent Wrappers after a
    /// requested VAST response is allowed. If false, any Wrappers received (i.e. not
    /// an Inline VAST response) should be ignored. Otherwise, VAST Wrappers
    /// received should be accepted (default value is “true.”)
    #[xml(attr = "followAdditionalWrappers", default)]
    pub follow_additional_wrappers: Option<bool>,
    /// A Boolean value that identifies whether multiple ads are allowed in the
    /// requested VAST response. If true, both Pods and stand-alone ads are allowed.
    /// If false, only the first stand-alone Ad (with no sequence values) in the
    /// requested VAST response is allowed. Default value is “false.”
    #[xml(attr = "allowMultipleAds", default)]
    pub allow_multiple_ads: Option<bool>,
    /// A Boolean value that provides instruction for using an available Ad when the
    /// requested VAST response returns no ads. If true, the media player should
    /// select from any stand-alone ads available. If false and the Wrapper represents
    /// an Ad in a Pod, the media player should move on to the next Ad in a Pod;
    /// otherwise, the media player can follow through at its own discretion where
    /// no-ad responses are concerned.
    #[xml(attr = "fallbackOnNoAd", default)]
    pub fallback_on_no_ad: Option<bool>,

    /// The [`<AdSystem>`](crate::AdSystem) element.
    #[xml(child = "AdSystem")]
    pub ad_system: crate::AdSystem<'a>,
    /// The container for zero or more `<Error>` elements.
    #[xml(flatten_text = "Error", cdata, default)]
    pub errors: Vec<std::borrow::Cow<'a, str>>,
    /// The container for zero or one [`<Extensions>`](crate::Extensions) element.
    #[xml(child = "Extensions", default)]
    pub extensions: Option<crate::Extensions>,
    /// The container for one or more [`<Impression>`](crate::Impression) elements.
    #[xml(child = "Impression")]
    pub impressions: Vec<crate::Impression<'a>>,
    /// The container for zero or one [`<Pricing>`](crate::Pricing) element.
    #[xml(child = "Pricing", default)]
    pub pricing: Option<crate::Pricing<'a>>,
    /// The container for zero or one [`<ViewableImpression>`](crate::ViewableImpression) element.
    #[xml(child = "ViewableImpression", default)]
    pub viewable_impression: Option<crate::ViewableImpression<'a>>,

    /// The container for zero or one [`<AdVerifications>`](crate::AdVerifications) element.
    #[xml(child = "AdVerifications", default)]
    pub ad_verifications: Option<crate::AdVerifications<'a>>,
    /// The container for zero or more [`<BlockedAdCategories>`](BlockedAdCategories) elements.
    #[xml(child = "BlockedAdCategories", default)]
    pub blocked_ad_categories: Vec<BlockedAdCategories<'a>>,
    /// The container for zero or one [`<Creative>`](crate::Creative) element.
    #[xml(child = "Creatives", default)]
    pub creatives: Option<crate::Creatives<'a>>,
    /// A URI to another VAST response that may be another VAST [Wrapper] or a VAST
    /// [InLine](crate::InLine) ad.
    #[xml(flatten_text = "VASTAdTagURI")]
    pub vast_ad_tag_uri: std::borrow::Cow<'a, str>,
}

/// Ad categories are used in creative separation and for compliance in certain programs.
///
/// ```text
/// <xs:element name="BlockedAdCategories">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:string">
///         <xs:attribute name="authority" type="xs:anyURI">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "BlockedAdCategories", strict(unknown_attribute, unknown_element))]
pub struct BlockedAdCategories<'a> {
    /// A URL for the organizational authority that produced the list being used to identify
    /// ad content.
    #[xml(attr = "authority", default)]
    pub authority: Option<std::borrow::Cow<'a, str>>,

    /// A string that provides a comma separated list of category codes or labels per
    /// authority that identify the ad content.
    #[xml(text)]
    pub codes: std::borrow::Cow<'a, str>,
}
