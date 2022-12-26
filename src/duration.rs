/// Duration represents a span of time. The value format is `HH:MM:SS.mmm` or `HH:MM:SS`.
#[derive(Default, PartialEq, Clone, Debug)]
pub struct Duration(std::time::Duration);

impl Duration {
    const NANOS_PER_MILLI: u32 = 1_000_000;
    const SECS_PER_HOURS: u64 = Self::SECS_PER_MINUTE * 60;
    const SECS_PER_MINUTE: u64 = 60;

    pub fn new(hours: u64, minutes: u64, secs: u64, milli_secs: u32) -> Self {
        let secs = hours * Self::SECS_PER_HOURS + minutes * Self::SECS_PER_MINUTE + secs;
        let nanos = milli_secs * Self::NANOS_PER_MILLI;
        Self(std::time::Duration::new(secs, nanos))
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self(value)
    }
}

impl From<Duration> for std::time::Duration {
    fn from(value: Duration) -> Self {
        value.0
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.0.as_secs();
        let millis = self.0.subsec_millis();

        let hh = secs / Self::SECS_PER_HOURS;
        let mm = (secs % Self::SECS_PER_HOURS) / Self::SECS_PER_MINUTE;
        let ss = secs % Self::SECS_PER_MINUTE;

        write!(f, "{hh:02}:{mm:02}:{ss:02}")?;
        if millis > 0 {
            write!(f, ".{millis:03}")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Duration {
    type Err = crate::VastParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        macro_rules! error {
            () => {
                crate::VastParseError::new(format!("duration parse error: '{s}'"))
            };
        }
        macro_rules! parse {
            ($var:ident, $ty:ty) => {
                $var.parse::<$ty>().map_err(|_| error!())
            };
        }

        let mut iter = s.trim().split(':');

        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(hh), Some(mm), Some(ss_ms), None) => {
                let hh = parse!(hh, u64)?;
                let mm = parse!(mm, u64)?;
                Ok(match ss_ms.split_once('.') {
                    Some((ss, ms)) => Duration::new(hh, mm, parse!(ss, u64)?, parse!(ms, u32)?),
                    None => Duration::new(hh, mm, parse!(ss_ms, u64)?, 0),
                })
            }
            _ => Err(error!()),
        }
    }
}

impl hard_xml::XmlWrite for Duration {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut hard_xml::XmlWriter<W>,
    ) -> hard_xml::XmlResult<()> {
        if !self.0.is_zero() {
            writer.write_element_start("Duration")?;
            writer.write_element_end_open()?;
            write!(writer.inner, "{self}")?;
            writer.write_element_end_close("Duration")?;
        }
        Ok(())
    }
}

impl<'a> hard_xml::XmlRead<'a> for Duration {
    fn from_reader(reader: &mut hard_xml::XmlReader<'a>) -> hard_xml::XmlResult<Self> {
        reader.read_till_element_start("Duration")?;
        let text = reader.read_text("Duration")?;
        <Duration as std::str::FromStr>::from_str(text.as_ref())
            .map_err(|e| hard_xml::XmlError::FromStr(e.into()))
    }
}

#[cfg(test)]
#[test]
fn test_duration_parse_and_format() {
    use std::str::FromStr;

    macro_rules! parse_and_format {
        ($str:expr, $dur:expr) => {
            let got = Duration::from_str($str).unwrap();
            assert_eq!(got, $dur);
            assert_eq!(format!("{got}"), $str);
        };
    }

    parse_and_format!("11:11:11.111", Duration::new(11, 11, 11, 111));
    parse_and_format!("12:34:56", Duration::new(12, 34, 56, 0));
    assert!(Duration::from_str("").is_err());
    assert!(Duration::from_str("12:34:56:78").is_err());
}

crate::declare_test!(
    test_duration,
    Duration,
    r#"<Duration>11:11:11.111</Duration>"#,
    Duration::new(11, 11, 11, 111)
);

// crate::declare_test!(test_duration_empty, Duration, "", Duration::new(11, 11, 11, 111));

#[cfg(test)]
#[derive(hard_xml::XmlWrite, hard_xml::XmlRead, PartialEq, Debug)]
#[xml(tag = "Xml", strict(unknown_attribute, unknown_element))]
struct Xml {
    #[xml(child = "Duration", default)]
    dur: Duration,
}

crate::declare_test!(
    test_duration_property,
    Xml,
    "<Xml><Duration>12:34:56.999</Duration></Xml>",
    Xml { dur: Duration::new(12, 34, 56, 999) }
);

crate::declare_test!(
    test_duration_property_empty,
    Xml,
    "<Xml></Xml>",
    Xml { dur: Default::default() }
);
