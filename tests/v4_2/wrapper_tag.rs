crate::declare_test_v4_2!(
    "Wrapper_Tag-test",
    Vast {
        version: Cow::Borrowed("4.2"),
        ads: vec![Ad {
            id: Some(Cow::Borrowed("20011")),
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
                    "https://raw.githubusercontent.com/InteractiveAdvertisingBureau/VAST_Samples/master/VAST%204.2%20Samples/Inline_Companion_Tag-test.xml"
                ),
                ad_system: AdSystem {
                    version: Some(Cow::Borrowed("4.0")),
                    name: Cow::Borrowed("iabtechlab"),
                },
                errors: vec![Cow::Borrowed("https://example.com/error"),],
                creatives: Some(Creatives {
                    creatives: vec![Creative {
                        id: Some(Cow::Borrowed("5480")),
                        ad_id: Some(Cow::Borrowed("2447226")),
                        sequence: Some(1),
                        companion_ads: Some(CompanionAds {
                            companions: vec![Companion {
                                width: 100,
                                height: 150,
                                id: Some(Cow::Borrowed("1232")),
                                asset_width: Some(250),
                                asset_height: Some(200),
                                expanded_width: Some(350),
                                expanded_height: Some(250),
                                api_framework: Some(Cow::Borrowed("SIMID")),
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
                    },],
                }),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
);
