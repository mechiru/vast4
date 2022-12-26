/// The URI to a static creative file to be used for the ad component identified in the parent
/// element.
///
/// ```text
/// <xs:element name="StaticResource">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="creativeType" type="xs:string" use="required">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "StaticResource", strict(unknown_attribute, unknown_element))]
pub struct StaticResource<'a> {
    /// Identifies the MIME type of the creative provided.
    #[xml(attr = "creativeType")]
    pub creative_type: std::borrow::Cow<'a, str>,

    /// A URI to the static creative file to be used for the ad component identified in the
    /// parent element.
    #[xml(text, cdata, default)]
    pub uri: std::borrow::Cow<'a, str>,
}
