/// Linear Ads are the video or audio formatted ads that play linearly within the streaming
/// content, meaning before the streaming content, during a break, or after the streaming
/// content.
///
/// ## Linear base type:
/// ```text
/// <xs:complexType name="Linear_Base_type">
///   <xs:attribute name="skipoffset" use="optional">
///   <xs:sequence>
///     <xs:element name="Icons" minOccurs="0" maxOccurs="1">
///     <xs:element name="TrackingEvents" minOccurs="0" maxOccurs="1" type="vast:TrackingEvents_type" />
///   </xs:sequence>
/// </xs:complexType>
/// ```
///
/// ## Linear in [`InLine`](crate::InLine):
/// ```text
/// <xs:complexType name="Linear">
///   <xs:complexContent>
///     <xs:extension base="vast:Linear_Base_type">
///       <xs:sequence>
///         <xs:element name="AdParameters" minOccurs="0" maxOccurs="1" type="vast:AdParameters_type">
///         <xs:element name="Duration" minOccurs="1" maxOccurs="1" type="xs:time">
///         <xs:element name="MediaFiles" minOccurs="1" maxOccurs="1">
///         <xs:element name="VideoClicks" minOccurs="0" maxOccurs="1" type="vast:VideoClicks_type" />
///       </xs:sequence>
///     </xs:extension>
///   </xs:complexContent>
/// </xs:complexType>
/// ```
///
/// ## Linear in [`Wrapper`](crate::Wrapper):
/// ```text
/// <xs:complexType name="Linear">
///   <xs:complexContent>
///     <xs:extension base="vast:Linear_Base_type">
///       <xs:sequence>
///         <xs:element name="VideoClicks" minOccurs="0" maxOccurs="1" type="vast:VideoClicks_type" />
///       </xs:sequence>
///     </xs:extension>
///   </xs:complexContent>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Linear", strict(unknown_attribute, unknown_element))]
pub struct Linear<'a> {
    /// The time at which the ad becomes skippable, if absent, the ad is not skippable.
    #[xml(attr = "skipoffset", default)]
    pub skipoffset: Option<crate::Duration>,

    /// The container for zero or one [`<Icons>`](crate::Icons) element.
    #[xml(child = "Icons", default)]
    pub icons: Option<crate::Icons<'a>>,
    /// The container for zero or one [`<TrackingEvents>`](crate::TrackingEvents) element.
    #[xml(child = "TrackingEvents", default)]
    pub tracking_events: Option<crate::TrackingEvents<'a>>,

    /// InLine: The container for zero or one [`<AdParameters>`](AdParameters) element.
    /// Wrapper: No use this field.
    #[xml(child = "AdParameters", default)]
    pub ad_parameters: Option<AdParameters<'a>>,
    /// InLine: The ad server uses the [`<Duration>`](crate::Duration) element to denote the
    /// intended playback duration for the video or audio component of the ad. **Required this
    /// field inside [`InLine`](crate::InLine) element.**
    /// Wrapper: No use this field.
    #[xml(child = "Duration", default)]
    pub duration: Option<crate::Duration>,
    /// InLine: The container for zero or one [`<MediaFiles>`](crate::MediaFiles) element.
    /// **Required this field inside [`InLine`](crate::InLine) element.**
    /// Wrapper: No use this field.
    #[xml(child = "MediaFiles", default)]
    pub media_files: Option<crate::MediaFiles<'a>>,

    /// The container for zero or one [`<VideoClicks>`](crate::VideoClicks) element.
    #[xml(child = "VideoClicks", default)]
    pub video_clicks: Option<crate::VideoClicks<'a>>,
}

/// The `<AdParameters>` element is currently the only way to pass information from the
/// VAST response into the VPAID object.
///
/// ```text
/// <xs:complexType name="AdParameters">
///   <xs:simpleContent>
///     <xs:extension base="xs:string">
///       <xs:attribute name="xmlEncoded" type="xs:boolean" use="optional">
///     </xs:extension>
///   </xs:simpleContent>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "AdParameters", strict(unknown_attribute, unknown_element))]
pub struct AdParameters<'a> {
    /// Identifies whether the ad parameters are xml-encoded.
    #[xml(attr = "xmlEncoded", default)]
    pub xml_encoded: Option<bool>,

    /// Metadata for the ad.
    #[xml(text)]
    pub metadata: std::borrow::Cow<'a, str>,
}
