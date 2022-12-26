crate::declare_test_v4_2!(
    "Inline_Companion_Tag-test",
    Vast {
        version: Cow::Borrowed("4.2"),
        ads: vec![Ad {
            id: Some(Cow::Borrowed("20004")),
            in_line: Some(InLine {
                ad_system: AdSystem {
                    version: Some(Cow::Borrowed("1")),
                    name: Cow::Borrowed("iabtechlab"),
                },
                ad_title: Cow::Borrowed("VAST 4.0 Pilot - Scenario 5"),
                impressions: vec![Impression {
                    id: Some(Cow::Borrowed("Impression-ID")),
                    uri: Cow::Borrowed("https://example.com/track/impression"),
                },],
                ad_serving_id: Cow::Borrowed("a532d16d-4d7f-4440-bd29-2ec0e693fc80"),
                description: Some(Cow::Borrowed(
                    "This is sample companion ad tag with Linear ad tag. This tag while showing video ad on the player, will show a companion ad beside the player where it can be fitted. At most 3 companion ads can be placed. Modify accordingly to see your own content. "
                )),
                pricing: Some(Pricing {
                    model: PriceModel::Cpm,
                    currency: Cow::Borrowed("USD"),
                    price: 25.0
                }),
                errors: vec![Cow::Borrowed("https://example.com/error"),],
                creatives: Creatives {
                    creatives: vec![
                        Creative {
                            id: Some(Cow::Borrowed("5480")),
                            ad_id: Some(Cow::Borrowed("2447226")),
                            sequence: Some(1),
                            universal_ad_id: vec![UniversalAdId {
                                id_registry: Cow::Borrowed("Ad-ID"),
                                id: Cow::Borrowed("8465")
                            }],
                            companion_ads: Some(CompanionAds {
                                companions: vec![Companion {
                                    width: 100,
                                    height: 150,
                                    id: Some(Cow::Borrowed("1232")),
                                    asset_width: Some(250),
                                    asset_height: Some(200),
                                    expanded_width: Some(350),
                                    expanded_height: Some(250),
                                    ad_slot_id: Some(Cow::Borrowed("3214")),
                                    pxratio: Some(1400.0),
                                    static_resources: vec![StaticResource {
                                        creative_type: Cow::Borrowed("image/png"),
                                        uri: Cow::Borrowed(
                                            "https://www.iab.com/wp-content/uploads/2014/09/iab-tech-lab-6-644x290.png"
                                        )
                                    }],
                                    companion_click_through: Some(Cow::Borrowed(
                                        "https://iabtechlab.com"
                                    )),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        Creative {
                            id: Some(Cow::Borrowed("5481")),
                            ad_id: Some(Cow::Borrowed("2447226")),
                            sequence: Some(1),
                            universal_ad_id: vec![UniversalAdId {
                                id_registry: Cow::Borrowed("Ad-ID"),
                                id: Cow::Borrowed("8466")
                            },],
                            linear: Some(Linear {
                                duration: Some(Duration::new(0, 0, 16, 0)),
                                media_files: Some(MediaFiles {
                                    media_files: vec![
                                        MediaFile {
                                            delivery: DeliveryProtocol::Progressive,
                                            mime_type: Cow::Borrowed("video/mp4"),
                                            width: 1280,
                                            height: 720,
                                            codec: Some(Cow::Borrowed("H.264")),
                                            id: Some(Cow::Borrowed("5241")),
                                            bitrate: Some(2000),
                                            min_bitrate: Some(1500),
                                            max_bitrate: Some(2500),
                                            scalable: Some(true),
                                            maintain_aspect_ratio: Some(true),
                                            uri: Cow::Borrowed(
                                                "https://iab-publicfiles.s3.amazonaws.com/vast/VAST-4.0-Short-Intro.mp4"
                                            ),
                                            ..Default::default()
                                        },
                                        MediaFile {
                                            delivery: DeliveryProtocol::Progressive,
                                            mime_type: Cow::Borrowed("video/mp4"),
                                            width: 854,
                                            height: 480,
                                            codec: Some(Cow::Borrowed("H.264")),
                                            id: Some(Cow::Borrowed("5244")),
                                            bitrate: Some(1000),
                                            min_bitrate: Some(700),
                                            max_bitrate: Some(1500),
                                            scalable: Some(true),
                                            maintain_aspect_ratio: Some(true),
                                            uri: Cow::Borrowed(
                                                "https://iab-publicfiles.s3.amazonaws.com/vast/VAST-4.0-Short-Intro-mid-resolution.mp4"
                                            ),
                                            ..Default::default()
                                        },
                                        MediaFile {
                                            delivery: DeliveryProtocol::Progressive,
                                            mime_type: Cow::Borrowed("video/mp4"),
                                            width: 640,
                                            height: 360,
                                            codec: Some(Cow::Borrowed("H.264")),
                                            id: Some(Cow::Borrowed("5246")),
                                            bitrate: Some(600),
                                            min_bitrate: Some(500),
                                            max_bitrate: Some(700),
                                            scalable: Some(true),
                                            maintain_aspect_ratio: Some(true),
                                            uri: Cow::Borrowed(
                                                "https://iab-publicfiles.s3.amazonaws.com/vast/VAST-4.0-Short-Intro-low-resolution.mp4"
                                            ),
                                            ..Default::default()
                                        },
                                    ],
                                    ..Default::default()
                                }),
                                tracking_events: Some(TrackingEvents {
                                    trackings: vec![
                                        Tracking {
                                            event: TrackingEvent::Start,
                                            offset: Default::default(),
                                            uri: Cow::Borrowed(
                                                "https://example.com/tracking/start"
                                            ),
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
                                        Tracking {
                                            event: TrackingEvent::FirstQuartile,
                                            offset: Default::default(),
                                            uri: Cow::Borrowed(
                                                "https://example.com/tracking/firstQuartile"
                                            ),
                                        },
                                        Tracking {
                                            event: TrackingEvent::MidPoint,
                                            offset: Default::default(),
                                            uri: Cow::Borrowed(
                                                "https://example.com/tracking/midpoint"
                                            ),
                                        },
                                        Tracking {
                                            event: TrackingEvent::ThirdQuartile,
                                            offset: Default::default(),
                                            uri: Cow::Borrowed(
                                                "https://example.com/tracking/thirdQuartile"
                                            ),
                                        },
                                        Tracking {
                                            event: TrackingEvent::Complete,
                                            offset: Default::default(),
                                            uri: Cow::Borrowed(
                                                "https://example.com/tracking/complete"
                                            ),
                                        },
                                    ]
                                }),
                                video_clicks: Some(VideoClicks {
                                    click_through: Some(ClickThrough {
                                        id: Some(Cow::Borrowed("blog")),
                                        uri: Cow::Borrowed("https://iabtechlab.com")
                                    }),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                    ]
                },
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
);
