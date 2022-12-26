/// The ad server may provide URIs for tracking publisher-determined viewability, for both the
/// [`InLine`](crate::InLine) ad and any [`Wrapper`](crate::Wrapper)s, using the
/// `<ViewableImpression>` element.
///
/// ```text
/// <xs:complexType name="ViewableImpression" >
///   <xs:attribute name="id" type="xs:string" use="optional">
///   <xs:sequence>
///     <xs:element name="Viewable" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI">
///     <xs:element name="NotViewable" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI">
///     <xs:element name="ViewUndetermined" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "ViewableImpression", strict(unknown_attribute, unknown_element))]
pub struct ViewableImpression<'a> {
    /// An ad server id for the impression. Impression resources of the same id should be requested
    /// at the same time or as close in time as possible to help prevent discrepancies.
    #[xml(attr = "id")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// A URI that directs the video player to a tracking resource file that the video player
    /// should request at the time that criteria is met for a viewable impression. This can occur
    /// zero to many times.
    #[xml(flatten_text = "Viewable", cdata, default)]
    pub viewables: Vec<std::borrow::Cow<'a, str>>,
    /// A URI that directs the video player to a tracking resource file that the video player
    /// should request if the ad is executed but never meets criteria for a viewable impression.
    #[xml(flatten_text = "NotViewable", cdata, default)]
    pub not_viewables: Vec<std::borrow::Cow<'a, str>>,
    /// A URI that directs the video player to a tracking resource file that the video player
    /// should request if the player cannot determine whether criteria is met for a viewable
    /// impression. This can occur zero to many times.
    #[xml(flatten_text = "ViewUndetermined", cdata, default)]
    pub view_undetermineds: Vec<std::borrow::Cow<'a, str>>,
}
