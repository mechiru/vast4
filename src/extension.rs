/// XML node for custom extensions, as defined by the ad server. When used, a custom element should
/// be nested under `<Extensions>` to help separa te custom XML elements from VAST elements.
///
/// ```text
/// <xs:element name="Extensions" minOccurs="0" maxOccurs="1">
///   <xs:complexType>
///     <xs:sequence>
///       <xs:element name="Extension" minOccurs="0" maxOccurs="unbounded">
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Extensions", strict(unknown_attribute, unknown_element))]
pub struct Extensions {
    /// The container for zero or more [`<Extension>`](Extension) elements.
    #[xml(child = "Extension", default)]
    pub extensions: Vec<Extension>,
}

// TODO: custom xml object
/// One instance of `<Extension>` should be used for each custom extension. The type attribute
/// is a custom value which identifies the extension.
///
/// ```text
/// <xs:element name="Extension" minOccurs="0" maxOccurs="unbounded">
///   <xs:complexType>
///     <xs:attribute name="type" type="xs:string" use="optional">
///     <xs:sequence>
///       <xs:any minOccurs="0" maxOccurs="unbounded" processContents="skip" />
///     </xs:sequence>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Extension {
    /// The MIME type of any code that might be included in the extension.
    pub mime_type: Option<String>,

    /// Custom XML object.
    pub xml: String,
}

impl Extension {
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

impl hard_xml::XmlWrite for Extension {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut hard_xml::XmlWriter<W>,
    ) -> hard_xml::XmlResult<()> {
        writer.write_element_start("Extension")?;
        if let Some(ref attr) = self.mime_type {
            writer.write_attribute("type", attr)?;
        }
        writer.write_element_end_open()?;
        write!(writer.inner, "{}", self.xml)?;
        writer.write_element_end_close("Extension")?;
        Ok(())
    }
}

impl<'a> hard_xml::XmlRead<'a> for Extension {
    fn from_reader(reader: &mut hard_xml::XmlReader<'a>) -> hard_xml::XmlResult<Self> {
        use hard_xml::xmlparser::{ElementEnd, Token};

        reader.read_till_element_start("Extension")?;

        let mut mime_type = None;
        while let Some((name, value)) = reader.find_attribute()? {
            match name {
                "type" => {
                    mime_type = Some(match value {
                        std::borrow::Cow::Borrowed(s) => s.to_owned(),
                        std::borrow::Cow::Owned(s) => s,
                    })
                }
                _ => {
                    return Err(hard_xml::XmlError::UnknownField {
                        name: "Extension".into(),
                        field: "type".into(),
                    });
                }
            };
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
                            if depth == 0 && name.as_str() == "Extension" {
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

#[cfg(test)]
#[test]
fn test_extension_attribute() {
    let xml = r#"<Extension type="application/xml" anyAttribute1="value1">
             <CreativeExtension type="text/javascript">
                 <![CDATA[
                     <Ext>
                         <Some id="hoge">fuga</Some>
                     </Ext>
                 ]]>
             </CreativeExtension>
             <CreativeExtension type="text/javascript">
                 <Ext>
                    <Some id="hoge">fuga</Some>
                 </Ext>
                 <Extension>hoge</Extension>
             </CreativeExtension>
         </Extension>"#;
    assert!(crate::from_str::<Extension>(xml).is_err());
}

crate::declare_test!(
    test_extension,
    Extension,
    r#"<Extension>
             <CreativeExtension type="text/javascript">
                 <![CDATA[
                     <Ext>
                         <Some id="hoge">fuga</Some>
                     </Ext>
                 ]]>
             </CreativeExtension>
             <CreativeExtension type="text/javascript">
                 <Ext>
                    <Some id="hoge">fuga</Some>
                 </Ext>
                 <Extension>hoge</Extension>
             </CreativeExtension>
         </Extension>"#,
    Extension {
        mime_type: None,
        xml: r#"
             <CreativeExtension type="text/javascript">
                 <![CDATA[
                     <Ext>
                         <Some id="hoge">fuga</Some>
                     </Ext>
                 ]]>
             </CreativeExtension>
             <CreativeExtension type="text/javascript">
                 <Ext>
                    <Some id="hoge">fuga</Some>
                 </Ext>
                 <Extension>hoge</Extension>
             </CreativeExtension>
         "#
        .into()
    }
);
