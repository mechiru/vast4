/// The AdVerification element is used to initiate a controlled container where code can be executed
/// for collecting data to verify ad playback details.
///
/// ```text
/// <xs:complexType name="AdVerifications" >
///   <xs:sequence>
///     <xs:element name="Verification" minOccurs="0" maxOccurs="unbounded" type="vast:Verification_type" />
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "AdVerifications", strict(unknown_attribute, unknown_element))]
pub struct AdVerifications<'a> {
    /// The container for zero or more [`<Verification>`](Verification) elements.
    #[xml(child = "Verification", default)]
    pub verifications: Vec<Verification<'a>>,
}

/// The `<Verification>` element contains the executable and bootstrapping data required to
/// run the measurement code for a single verification vendor.
///
/// ```text
/// <xs:complexType name="Verification">
///   <xs:attribute name="vendor" type="xs:string" use="optional">
///   <xs:sequence>
///     <xs:element name="ExecutableResource" minOccurs="0" maxOccurs="unbounded">
///     <xs:element name="JavaScriptResource" minOccurs="0" maxOccurs="unbounded">
///     <xs:element name="TrackingEvents" minOccurs="0" maxOccurs="1" type="vast:TrackingEvents_Verification_type" />
///     <xs:element name="VerificationParameters" minOccurs="0" maxOccurs="1" type="xs:string">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Verification", strict(unknown_attribute, unknown_element))]
pub struct Verification<'a> {
    // https://github.com/InteractiveAdvertisingBureau/vast/blob/f28dbb4768744062fcb638a1859364cdafb3a449/vast_4.2.xsd#L1136
    /// An identifier for the verification vendor. The recommended format is `[domain]-[useCase]`,
    /// to avoid name collisions. For example, "company.com-omid".
    #[xml(attr = "vendor", default)]
    pub vendor: Option<std::borrow::Cow<'a, str>>,

    /// The container for zero or more [`<ExecutableResource>`](ExecutableResource) elements.
    #[xml(child = "ExecutableResource", default)]
    pub executable_resources: Vec<ExecutableResource<'a>>,
    /// The container for zero or more [`<JavaScriptResource>`](JavaScriptResource) elements.
    #[xml(child = "JavaScriptResource", default)]
    pub javascript_resources: Vec<JavaScriptResource<'a>>,
    /// The container for zero or one [`<TrackingEvents>`](VerificationTrackingEvents) element.
    #[xml(child = "TrackingEvents", default)]
    pub tracking_events: Option<VerificationTrackingEvents<'a>>,
    /// The container for zero or one `<VerificationParameters>` element.
    #[xml(flatten_text = "VerificationParameters", cdata, default)]
    pub verification_parameters: Option<std::borrow::Cow<'a, str>>,
}

/// A container for the URI to the JavaScript file used to collect verification data.
///
/// ```text
/// <xs:element name="JavaScriptResource">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="apiFramework" type="xs:string" use="optional" />
///         <xs:attribute name="browserOptional" type="xs:boolean" use="optional" />
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "JavaScriptResource", strict(unknown_attribute, unknown_element))]
pub struct JavaScriptResource<'a> {
    // https://github.com/InteractiveAdvertisingBureau/vast/blob/f28dbb4768744062fcb638a1859364cdafb3a449/vast_4.2.xsd#L1123
    /// The name of the API framework used to execute the AdVerification code.
    #[xml(attr = "apiFramework", default)]
    pub api_framework: Option<std::borrow::Cow<'a, str>>,
    // https://github.com/InteractiveAdvertisingBureau/vast/blob/f28dbb4768744062fcb638a1859364cdafb3a449/vast_4.2.xsd#L1124
    /// Boolean value. If true, this resource is optimized and able to execute in an
    /// environment without DOM and other browser built-ins (e.g. iOS' JavaScriptCore)
    #[xml(attr = "browserOptional", default)]
    pub browser_optional: Option<bool>,

    /// A CDATA-wrapped URI to the JavaScript used to collect data.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// A reference to a non-JavaScript or custom-integration resource intended for collecting
/// verification data via the listed apiFramework.
///
/// ```text
/// <xs:element name="ExecutableResource">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="apiFramework" type="xs:string" use="optional" />
///         <xs:attribute name="type" type="xs:string" use="optional" />
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "ExecutableResource", strict(unknown_attribute, unknown_element))]
pub struct ExecutableResource<'a> {
    /// The name of the API framework used to execute the AdVerification code.
    #[xml(attr = "apiFramework")]
    pub api_framework: std::borrow::Cow<'a, str>,
    /// The type of executable resource provided. The exact value used should be agreed
    /// upon by verification integrators and vendors who are implementing verification in
    /// a custom environment.
    #[xml(attr = "type", default)]
    pub resource_type: Option<std::borrow::Cow<'a, str>>,

    /// A CDATA-wrapped reference to the resource. This may be a URI, but
    /// depending on the execution environment can be any value which enables the
    /// player to load the required verification code.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// The verification vendor may provide URIs for tracking events relating to the execution of
/// their code during the ad session.
///
/// ```text
/// <xs:complexType name="TrackingEvents" >
///   <xs:sequence>
///     <xs:element name="Tracking" minOccurs="0" maxOccurs="unbounded">
///   </xs:sequence>
/// </xs:complexType>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "TrackingEvents", strict(unknown_attribute, unknown_element))]
pub struct VerificationTrackingEvents<'a> {
    /// The container for zero or more [`<Tracking>`](VerificationTracking) elements.
    #[xml(child = "Tracking", default)]
    pub trackings: Vec<VerificationTracking<'a>>,
}

/// Each `<Tracking>` element is used to define a single event to be tracked by the verification
/// vendor.
///
/// ```text
/// <xs:element name="Tracking">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="event" type="xs:string" use="required">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, Default, PartialEq, Clone, Debug)]
#[xml(tag = "Tracking", strict(unknown_attribute, unknown_element))]
pub struct VerificationTracking<'a> {
    /// A string that defines the event being tracked. Accepted values are listed in section
    /// 3.17.3.
    #[xml(attr = "event")]
    pub event: std::borrow::Cow<'a, str>,

    /// A URI to the tracking resource for the event specified using the event attribute.
    #[xml(text, cdata)]
    pub uri: std::borrow::Cow<'a, str>,
}
