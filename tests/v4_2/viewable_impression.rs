crate::declare_test_v4_2!(
    "Viewable_Impression-test",
    Vast {
        version: Cow::Borrowed("4.2"),
        ads: vec![Ad {
            id: Some(Cow::Borrowed("20010")),
            sequence: Some(1),
            wrapper: Some(Wrapper {
                follow_additional_wrappers: Some(false),
                allow_multiple_ads: Some(true),
                fallback_on_no_ad: Some(false),
                impressions: vec![Impression {
                    id: Some(Cow::Borrowed("Impression-ID")),
                    uri: Cow::Borrowed("https://example.com/track/impression"),
                },],
                vast_ad_tag_uri: Cow::Borrowed(
                    "https://raw.githubusercontent.com/InteractiveAdvertisingBureau/VAST_Samples/master/VAST%204.0%20Samples/Inline_Companion_Tag-test.xml"
                ),
                ad_system: AdSystem {
                    version: Some(Cow::Borrowed("4.0")),
                    name: Cow::Borrowed("iabtechlab"),
                },
                errors: vec![Cow::Borrowed("https://example.com/error"),],
                viewable_impression: Some(ViewableImpression {
                    id: Some(Cow::Borrowed("1543")),
                    viewables: vec![Cow::Borrowed(
                        "https://search.iabtechlab.com/error?errcode=102&imprid=s5-ea2f7f298e28c0c98374491aec3dfeb1&ts=1243"
                    ),],
                    not_viewables: vec![Cow::Borrowed(
                        "https://search.iabtechlab.com/error?errcode=102&imprid=s5-ea2f7f298e28c0c98374491aec3dfeb1&ts=1243"
                    ),],
                    view_undetermineds: vec![Cow::Borrowed(
                        "https://search.iabtechlab.com/error?errcode=102&imprid=s5-ea2f7f298e28c0c98374491aec3dfeb1&ts=1243"
                    ),],
                }),
                creatives: Some(Creatives {
                    creatives: vec![Creative {
                        id: Some(Cow::Borrowed("5480")),
                        ad_id: Some(Cow::Borrowed("2447226")),
                        sequence: Some(1),
                        linear: Some(Linear {
                            tracking_events: Some(TrackingEvents {
                                trackings: vec![
                                    Tracking {
                                        event: TrackingEvent::Start,
                                        offset: Default::default(),
                                        uri: Cow::Borrowed("http://example.com/tracking/start"),
                                    },
                                    Tracking {
                                        event: TrackingEvent::Progress,
                                        offset: Some(Offset::Duration(Duration::new(
                                            00, 00, 10, 0
                                        ))),
                                        uri: Cow::Borrowed(
                                            "http://example.com/tracking/progress-10"
                                        ),
                                    },
                                ]
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },],
                }),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
);
