/// The `<TrackingEvents>` element is available for [`Linear`](crate::Linear),
/// [`NonLinear`](crate::NonLinear), and [`Companion`](crate::Companion), elements in both
/// [`InLine`](crate::InLine) and [`Wrapper`](crate::Wrapper) formats.
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
pub struct TrackingEvents<'a> {
    /// The container for zero or more [`<Tracking>`](Tracking) elements.
    #[xml(child = "Tracking", default)]
    pub trackings: Vec<Tracking<'a>>,
}

/// Each `<Tracking>` element is used to define a single event to be tracked.
///
/// ```text
/// <xs:element name="Tracking">
///   <xs:complexType>
///     <xs:simpleContent>
///       <xs:extension base="xs:anyURI">
///         <xs:attribute name="event" use="required">
///         </xs:attribute>
///         <xs:attribute name="offset" use="optional">
///       </xs:extension>
///     </xs:simpleContent>
///   </xs:complexType>
/// </xs:element>
/// ```
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, PartialEq, Clone, Debug)]
#[xml(tag = "Tracking", strict(unknown_attribute, unknown_element))]
pub struct Tracking<'a> {
    /// A string that defines the event being tracked. Accepted values are listed in section
    /// 3.14.1 and differ for [`<Linear>`](crate::Linear), [`<NonLinear>`](crate::NonLinear), and
    /// [`<Companion>`](crate::Companion).
    #[xml(attr = "event")]
    pub event: TrackingEvent,
    /// Only available when [`<Linear>`](crate::Linear) is the parent. Accepts values of time in
    /// the format `HH:MM:SS` or as a percentage in the format `n%`. When the progress of the
    /// Linear creative has matched the value specified, the included URI is triggered. If the
    /// duration is not known when the offset is set to a percentage value, the progress
    /// event may be ignored.
    #[xml(attr = "offset", default)]
    pub offset: Option<Offset>,

    /// A URI to the tracking resource for the event specified using the event attribute.
    #[xml(text, cdata, default)]
    pub uri: std::borrow::Cow<'a, str>,
}

/// TrackingEvent represents tracking's event.
///
/// ```text
/// <xs:simpleType>
///   <xs:restriction base="xs:token">
///     <xs:enumeration value="mute" />
///     <xs:enumeration value="unmute" />
///     <xs:enumeration value="pause" />
///     <xs:enumeration value="resume" />
///     <xs:enumeration value="rewind" />
///     <xs:enumeration value="skip" />
///     <xs:enumeration value="playerExpand" />
///     <xs:enumeration value="playerCollapse" />
///     <xs:enumeration value="loaded" />
///     <xs:enumeration value="start" />
///     <xs:enumeration value="firstQuartile" />
///     <xs:enumeration value="midpoint" />
///     <xs:enumeration value="thirdQuartile" />
///     <xs:enumeration value="complete" />
///     <xs:enumeration value="progress" />
///     <xs:enumeration value="closeLinear" />
///     <xs:enumeration value="creativeView" />
///     <xs:enumeration value="acceptInvitation" />
///     <xs:enumeration value="adExpand" />
///     <xs:enumeration value="adCollapse" />
///     <xs:enumeration value="minimize" />
///     <xs:enumeration value="close" />
///     <xs:enumeration value="overlayViewDuration" />
///     <xs:enumeration value="otherAdInteraction" />
///     <xs:enumeration value="interactiveStart" />
///   </xs:restriction>
/// </xs:simpleType>
/// ```
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TrackingEvent {
    /// The user activated the mute control and muted the creative.
    Mute,
    /// The user activated the mute control and unmuted the creative.
    Unmute,
    /// The user clicked the pause control and stopped the creative.
    Pause,
    /// The user activated the resume control after the creative had been stopped
    /// or paused.
    Resume,
    /// The user activated the rewind control to access a previous point in the
    /// creative timeline.
    Rewind,
    /// The user activated a skip control to skip the creative.
    Skip,
    /// The user activated a control to extend the player to a larger size. This
    /// event replaces the fullscreen event per the 2014 Digital Video In-Stream Ad Metric
    /// Definitions.
    PlayerExpand,
    /// The user activated a control to reduce player to a smaller size. This
    /// event replaces the exitFullscreen event per the 2014 Digital Video In-Stream Ad
    /// Metric Definitions.
    PlayerCollapse,
    /// This event should be used to indicate when the player considers that it has loaded and
    /// buffered the creative’s media and assets either fully or to the extent that it is ready to
    /// play the media.
    Loaded,
    /// This event is used to indicate that an individual creative within the ad was loaded and
    /// playback began. As with creativeView, this event is another way of tracking creative
    /// playback. Macros defined to describe auto-play and muted states.
    Start,
    /// The creative played continuously for at least 25% of the total duration at normal speed.
    FirstQuartile,
    /// The creative played continuously for at least 50% of the total duration at normal speed.
    MidPoint,
    /// The creative played continuously for at least 75% of the duration at normal speed.
    ThirdQuartile,
    /// The creative was played to the end at normal speed so that 100% of the creative was played.
    Complete,
    /// The creative played for a duration at normal speed that is equal to or greater than the
    /// value provided in an additional `offset` attribute for the [`<Tracking>`](Tracking) element
    /// under Linear ads.
    Progress,
    /// The viewer has chosen to close the linear ad unit
    CloseLinear,
    // NonLinear Ad Metrics:
    /// Not to be confused with an impression, this event indicates that an individual creative
    /// portion of the ad was viewed.
    CreativeView,
    /// The user clicked or otherwise activated a control used to pause streaming content.
    AcceptInvitation,
    /// The user activated a control to expand the creative.
    AdExpand,
    /// The user activated a control to reduce the creative to its original dimensions.
    AdCollapse,
    /// The user clicked or otherwise activated a control used to minimize the ad to a size smaller
    /// than a collapsed ad but without fully dispatching the ad from the player environment.
    Minimize,
    /// The user clicked or otherwise activated a control for removing the ad, which fully
    /// dispatches the ad from the player environment in a manner that does not allow the user to
    /// re-display the ad.
    Close,
    /// The time that the initial ad is displayed.
    OverlayViewDuration,
    /// An optional metric that can capture all other user interactions under one metric such a s
    /// hover-overs, or custom clicks.
    OtherAdInteraction,
    /// With VAST 4, video playback and interactive creative playback now happens in parallel.
    InteractiveStart,
}

impl std::str::FromStr for TrackingEvent {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "mute" => Self::Mute,
            "unmute" => Self::Unmute,
            "pause" => Self::Pause,
            "resume" => Self::Resume,
            "rewind" => Self::Rewind,
            "skip" => Self::Skip,
            "playerExpand" => Self::PlayerExpand,
            "playerCollapse" => Self::PlayerCollapse,
            "loaded" => Self::Loaded,
            "start" => Self::Start,
            "firstQuartile" => Self::FirstQuartile,
            "midpoint" => Self::MidPoint,
            "thirdQuartile" => Self::ThirdQuartile,
            "complete" => Self::Complete,
            "progress" => Self::Progress,
            "closeLinear" => Self::CloseLinear,
            "creativeView" => Self::CreativeView,
            "acceptInvitation" => Self::AcceptInvitation,
            "adExpand" => Self::AdExpand,
            "adCollapse" => Self::AdCollapse,
            "minimize" => Self::Minimize,
            "close" => Self::Close,
            "overlayViewDuration" => Self::OverlayViewDuration,
            "otherAdInteraction" => Self::OtherAdInteraction,
            "interactiveStart" => Self::InteractiveStart,
            _ => {
                return Err(crate::VastParseError::new(format!(
                    "tracking event parsing error: '{s}'",
                )));
            }
        })
    }
}

impl std::fmt::Display for TrackingEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mute => write!(f, "mute"),
            Self::Unmute => write!(f, "unmute"),
            Self::Pause => write!(f, "pause"),
            Self::Resume => write!(f, "resume"),
            Self::Rewind => write!(f, "rewind"),
            Self::Skip => write!(f, "skip"),
            Self::PlayerExpand => write!(f, "playerExpand"),
            Self::PlayerCollapse => write!(f, "playerCollapse"),
            Self::Loaded => write!(f, "loaded"),
            Self::Start => write!(f, "start"),
            Self::FirstQuartile => write!(f, "firstQuartile"),
            Self::MidPoint => write!(f, "midpoint"),
            Self::ThirdQuartile => write!(f, "thirdQuartile"),
            Self::Complete => write!(f, "complete"),
            Self::Progress => write!(f, "progress"),
            Self::CloseLinear => write!(f, "closeLinear"),
            Self::CreativeView => write!(f, "creativeView"),
            Self::AcceptInvitation => write!(f, "acceptInvitation"),
            Self::AdExpand => write!(f, "adExpand"),
            Self::AdCollapse => write!(f, "adCollapse"),
            Self::Minimize => write!(f, "minimize"),
            Self::Close => write!(f, "close"),
            Self::OverlayViewDuration => write!(f, "overlayViewDuration"),
            Self::OtherAdInteraction => write!(f, "otherAdInteraction"),
            Self::InteractiveStart => write!(f, "interactiveStart"),
        }
    }
}

/// Offset represents a duration value or percentage value.
#[derive(PartialEq, Clone, Debug)]
pub enum Offset {
    Duration(crate::Duration),
    Percentage(i32),
}

impl std::str::FromStr for Offset {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimed = s.trim();
        if let Some((head, "")) = trimed.split_once('%') {
            let offset = head.parse::<i32>().map_err(|_| {
                crate::VastParseError::new(format!("offset value parsing error: '{s}'"))
            })?;
            return Ok(Self::Percentage(offset));
        }

        Ok(Self::Duration(s.parse::<crate::Duration>()?))
    }
}

impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Offset::Duration(dur) => write!(f, "{dur}"),
            Offset::Percentage(per) => write!(f, "{per}%"),
        }
    }
}

crate::declare_test!(
    test_tracking_events,
    TrackingEvents,
    "<TrackingEvents>\
        <Tracking event=\"mute\"><![CDATA[https://hoge.com/fuga?mute]]></Tracking>\
        <Tracking event=\"progress\" offset=\"00:00:10\"><![CDATA[https://hoge.com/fuga?offset=10]]></Tracking>\
        <Tracking event=\"progress\" offset=\"20%\"><![CDATA[https://hoge.com/fuga?offset=20-per]]></Tracking>\
    </TrackingEvents>",
    TrackingEvents {
        trackings: vec![
            Tracking {
                event: TrackingEvent::Mute,
                uri: "https://hoge.com/fuga?mute".into(),
                offset: Default::default(),
            },
            Tracking {
                event: TrackingEvent::Progress,
                offset: Some(crate::Offset::Duration(crate::Duration::new(00, 00, 10, 000))),
                uri: "https://hoge.com/fuga?offset=10".into(),
            },
            Tracking {
                event: TrackingEvent::Progress,
                offset: Some(crate::Offset::Percentage(20)),
                uri: "https://hoge.com/fuga?offset=20-per".into(),
            }
        ]
    }
);
