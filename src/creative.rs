/// A container for one or more [`<Creative>`](Creative) element used to provide creative files for
/// the ad.
///
/// ```text
/// <xs:element name="Creatives">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="Creative" minOccurs="1" maxOccurs="unbounded" type="vast:Creative" />
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Creatives", strict(unknown_attribute, unknown_element))]
pub struct Creatives<'a> {
    /// The container for one or more [`<Creative>`](Creative) elements.
    #[xml(child = "Creative")]
    pub creatives: Vec<Creative<'a>>,
}

/// Each `<Creative>` element contains nested elements that describe the type of ad being
/// served using nested sub-elements.
///
/// ## Creative in [`InLine`](crate::InLine):
/// ```text
/// <xs:complexType name="Creative">
///   <!-- Creative_Base_type -->
///   <xs:attribute name="sequence" type="xs:integer" use="optional">
///   <xs:attribute name="apiFramework" type="xs:string" use="optional">
///   <xs:attribute name="id" type="xs:string" use="optional">
///   <xs:attribute name="adId" type="xs:string" use="optional">
///
///   <xs:sequence>
///     <xs:element name="CompanionAds" minOccurs="0" maxOccurs="1" type="vast:CompanionAds_Collection_type" />
///     <xs:element name="CreativeExtensions" minOccurs="0" maxOccurs="1" type="vast:CreativeExtensions_type" />
///     <xs:element name="Linear" minOccurs="0" maxOccurs="1" type="vast:Linear_Inline_type" />
///     <xs:element name="NonLinearAds" minOccurs="0" maxOccurs="1">
///     <xs:element name="UniversalAdId" minOccurs="1" maxOccurs="unbounded">
///   </xs:sequence>
/// </xs:complexType>
/// ```
///
/// ## Creative in [`Wrapper`](crate::Wrapper):
/// ```text
/// <xs:complexType name="Creative">
///   <!-- Creative_Base_type -->
///   <xs:attribute name="sequence" type="xs:integer" use="optional">
///   <xs:attribute name="apiFramework" type="xs:string" use="optional">
///   <xs:attribute name="id" type="xs:string" use="optional">
///   <xs:attribute name="adId" type="xs:string" use="optional">
///
///   <xs:sequence>
///     <xs:element name="CompanionAds" minOccurs="0" maxOccurs="1" type="vast:CompanionAds_Collection_type" />
///     <xs:element name="Linear" minOccurs="0" maxOccurs="1" type="vast:Linear_Wrapper_type" />
///     <xs:element name="NonLinearAds" minOccurs="0" maxOccurs="1">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Creative", strict(unknown_attribute, unknown_element))]
pub struct Creative<'a> {
    /// A number representing the numerical order in which each sequenced creative within
    /// an ad should play.
    #[xml(attr = "sequence", default)]
    pub sequence: Option<i32>,
    /// A string that identifies an API that is needed to execute the creative.
    #[xml(attr = "apiFramework", default)]
    pub api_framework: Option<std::borrow::Cow<'a, str>>,
    /// A string used to identify the ad server that provides the creative.
    #[xml(attr = "id", default)]
    pub id: Option<std::borrow::Cow<'a, str>>,
    /// Used to provide the ad server’s unique identifier for the creative. In VAST 4, the
    /// [UniversalAdId] element was introduced to provide a unique identifier for the creative
    /// that is maintained across systems. Please see section 3.7.1 for details on the
    /// [UniversalAdId].
    #[xml(attr = "adId", default)]
    pub ad_id: Option<std::borrow::Cow<'a, str>>,

    /// The container for zero or one [`<CompanionAds>`](crate::CompanionAds) element.
    #[xml(child = "CompanionAds", default)]
    pub companion_ads: Option<crate::CompanionAds<'a>>,
    /// InLine: The container for zero or one [`<CreativeExtensions>`](CreativeExtensions) element.
    /// Wrapper: No use this field.
    #[xml(child = "CreativeExtensions", default)]
    pub creative_extensions: Option<CreativeExtensions>,
    /// The container for zero or one [`<Linear>`](crate::Linear) element.
    #[xml(child = "Linear", default)]
    pub linear: Option<crate::Linear<'a>>,
    /// The container for zero or one [`<NonLinearAds>`](crate::NonLinearAds) element.
    #[xml(child = "NonLinearAds", default)]
    pub non_linear_ads: Option<crate::NonLinearAds<'a>>,
    /// InLine: The container for one or more [`<UniversalAdId>`](UniversalAdId) elements.
    /// Wrapper: No use this field.
    #[xml(child = "UniversalAdId", default)]
    pub universal_ad_id: Vec<UniversalAdId<'a>>,
}

/// A required element for the purpose of tracking ad creative, the `<UniversalAdId>` is used to
/// provide a unique creative identifier that is maintained across systems.
///
/// ```text
/// <xs:element name="UniversalAdId">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:string">
///         <xs:attribute name="idRegistry" type="xs:string" use="required" />
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "UniversalAdId", strict(unknown_attribute, unknown_element))]
pub struct UniversalAdId<'a> {
    /// A string used to identify the URL for the registry website where the unique
    /// creative ID is cataloged. Default value is "unknown".
    #[xml(attr = "idRegistry", default = "unknown")]
    pub id_registry: std::borrow::Cow<'a, str>,

    /// A string identifying the unique creative identifier. Default value is "unknown".
    #[xml(text, default = "unknown")]
    pub id: std::borrow::Cow<'a, str>,
}

/// This extension can be used to load an executable creative with or without using the
/// [`<MediaFile>`](crate::MediaFile).
///
/// ```text
/// <xs:complexType name="CreativeExtensions">
///   <xs:sequence>
///     <xs:element name="CreativeExtension" minOccurs="0" maxOccurs="unbounded">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "CreativeExtensions", strict(unknown_attribute, unknown_element))]
pub struct CreativeExtensions {
    /// The container for zero or more [`<CreativeExtension>`](CreativeExtension) elements.
    #[xml(child = "CreativeExtension", default)]
    pub creative_extensions: Vec<CreativeExtension>,
}

// TODO: custom xml object
/// Any valid XML may be included in the Extensions node.
///
/// ```text
/// <xs:element name="CreativeExtension" minOccurs="0" maxOccurs="unbounded">
///   <xs:complexType>
///     <xs:attribute name="type" type="xs:string" use="optional">
///     <xs:anyAttribute namespace="##any" processContents="skip" />
///     <xs:sequence>
///       <xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="skip" />
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Default, PartialEq, Clone, Debug)]
pub struct CreativeExtension {
    /// The MIME type of any code that might be included in the extension.
    pub mime_type: Option<String>,

    /// Custom XML object.
    pub xml: String,
}

impl CreativeExtension {
    pub fn from_struct<T: hard_xml::XmlWrite>(
        mime_type: &str,
        value: &T,
    ) -> hard_xml::XmlResult<Self> {
        Ok(Self {
            mime_type: Some(mime_type.to_owned()),
            xml: hard_xml::XmlWrite::to_string(value)?,
        })
    }

    pub fn as_struct<'a, 'b, T>(&'a self) -> hard_xml::XmlResult<T>
    where
        T: hard_xml::XmlRead<'b>,
        'a: 'b,
    {
        <T as hard_xml::XmlRead<'b>>::from_str(&self.xml)
    }
}

impl hard_xml::XmlWrite for CreativeExtension {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut hard_xml::XmlWriter<W>,
    ) -> hard_xml::XmlResult<()> {
        writer.write_element_start("CreativeExtension")?;
        if let Some(ref attr) = self.mime_type {
            writer.write_attribute("type", attr)?;
        }
        writer.write_element_end_open()?;
        write!(writer.inner, "{}", self.xml)?;
        writer.write_element_end_close("CreativeExtension")?;
        Ok(())
    }
}

impl<'a> hard_xml::XmlRead<'a> for CreativeExtension {
    fn from_reader(reader: &mut hard_xml::XmlReader<'a>) -> hard_xml::XmlResult<Self> {
        use hard_xml::xmlparser::{ElementEnd, Token};

        reader.read_till_element_start("CreativeExtension")?;

        let mut mime_type = None;
        while let Some(("type", value)) = reader.find_attribute()? {
            mime_type = Some(match value {
                std::borrow::Cow::Borrowed(s) => s.to_owned(),
                std::borrow::Cow::Owned(s) => s,
            });
        }

        if let Some(t) = reader.next() {
            let t = t?;
            if !matches!(t, Token::ElementEnd { end: ElementEnd::Open, .. }) {
                return Err(hard_xml::XmlError::UnexpectedToken { token: format!("{t:?}") });
            }
        }

        let mut depth = 0;
        let mut xml = String::new();

        while let Some(t) = reader.next() {
            match t? {
                Token::Declaration { span, .. }
                | Token::ProcessingInstruction { span, .. }
                | Token::Comment { span, .. }
                | Token::DtdStart { span, .. }
                | Token::EmptyDtd { span, .. }
                | Token::EntityDeclaration { span, .. }
                | Token::DtdEnd { span }
                | Token::ElementStart { span, .. }
                | Token::Cdata { span, .. } => xml.push_str(span.as_str()),
                Token::Attribute { span, .. } => {
                    xml.push(' ');
                    xml.push_str(span.as_str())
                }
                Token::Text { text } => xml.push_str(text.as_str()),
                Token::ElementEnd { end, span } => {
                    match end {
                        ElementEnd::Open => {
                            depth += 1;
                        }
                        ElementEnd::Close(_, name) => {
                            if depth == 0 && name.as_str() == "CreativeExtension" {
                                break;
                            } else {
                                depth -= 1;
                            }
                        }
                        ElementEnd::Empty => {}
                    };
                    xml.push_str(span.as_str())
                }
            }
        }

        Ok(Self { mime_type, xml })
    }
}

crate::declare_test!(
    test_creative_extension,
    CreativeExtension,
    r#"<CreativeExtension type="text/javascript">
         <Ext>
           <Some id="hoge">fuga</Some>
           <Text><![CDATA[some cdata text]]></Text>
         </Ext>
       </CreativeExtension>"#,
    CreativeExtension {
        mime_type: Some("text/javascript".to_owned()),
        xml: r#"
         <Ext>
           <Some id="hoge">fuga</Some>
           <Text><![CDATA[some cdata text]]></Text>
         </Ext>
       "#
        .into()
    }
);

#[cfg(test)]
#[test]
fn test_creative_extension_attribute() {
    let xml = r#"<CreativeExtension type="text/javascript" attr="value1">
         <Ext>
           <Some id="hoge">fuga</Some>
           <Text><![CDATA[some cdata text]]></Text>
         </Ext>
       </CreativeExtension>"#;
    let ext = crate::from_str::<CreativeExtension>(xml).unwrap();
    assert_eq!(
        ext,
        CreativeExtension {
            mime_type: Some("text/javascript".to_owned()),
            xml: r#"
         <Ext>
           <Some id="hoge">fuga</Some>
           <Text><![CDATA[some cdata text]]></Text>
         </Ext>
       "#
            .into()
        }
    );

    assert_eq!(
        crate::to_string(&ext).unwrap(),
        r#"<CreativeExtension type="text/javascript">
         <Ext>
           <Some id="hoge">fuga</Some>
           <Text><![CDATA[some cdata text]]></Text>
         </Ext>
       </CreativeExtension>"#
    );
}
