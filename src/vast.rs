/// IAB VAST (Video Ad Serving Template).
///
/// ```text
/// <xs:element name="VAST" >
///   <xs:complexType>
///     <xs:attribute name="version" type="xs:string" use="required">
///     <xs:choice>
///       <xs:element name="Ad" minOccurs="0" maxOccurs="unbounded" >
///       <xs:element name="Error" minOccurs="0" maxOccurs="unbounded" type="xs:anyURI" >
///     </xs:choice>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "VAST", strict(unknown_element))]
pub struct Vast<'a> {
    /// A float (number with decimal) to indicate the VAST version being used.
    #[xml(attr = "version")]
    pub version: std::borrow::Cow<'a, str>,

    /// Top-level element, wraps each ad in the response or ad unit in an ad pod. This MUST be
    /// present unless an `Error` element is present.
    #[xml(child = "Ad", default)]
    pub ads: Vec<crate::Ad<'a>>,
    /// Used when there is no ad response. When the ad server does not or cannot return an
    /// [Ad](crate::Ad). If included the video player must send a request to the URI provided
    /// (Sec 3.2.1).
    #[xml(flatten_text = "Error", cdata, default)]
    pub errors: Vec<std::borrow::Cow<'a, str>>,
}

crate::declare_test!(
    test_vast_errors,
    Vast,
    r#"<VAST version="4.2"><Error><![CDATA[hoge]]></Error><Error><![CDATA[fuga]]></Error></VAST>"#,
    Vast {
        version: "4.2".into(),
        errors: vec!["hoge".into(), "fuga".into()],
        ..Default::default()
    }
);
