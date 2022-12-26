declare_test_v4_2!(
    "Inline_Non-Linear_Tag-test",
    Vast {
        version: Cow::Borrowed("4.2"),
        ads: vec![Ad {
            id: Some(Cow::Borrowed("20005")),
            sequence: Some(1),
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
                description: Some(Cow::Borrowed("VAST 4.0 sample tag for Non Linear ad (i.e Overlay ad). Change the StaticResources to have a tag with your own content. Change NonLinear tag's parameters accordingly to view desired results.")),
                pricing: Some(Pricing {
                    model: PriceModel::Cpm,
                    currency: Cow::Borrowed("USD"),
                    price: 25.0
                }),
                errors: vec![Cow::Borrowed("https://example.com/error"),],
                extensions: Some(Extensions {
                    extensions: vec![Extension {
                        mime_type: Some("iab-Count".to_owned()),
                        xml: "\n          <total_available>\n            <![CDATA[2]]>\n          </total_available>\n        "
                            .to_owned()
                    }]
                }),
                creatives: Creatives {
                    creatives: vec![Creative {
                        id: Some(Cow::Borrowed("5480")),
                        ad_id: Some(Cow::Borrowed("2447226")),
                        sequence: Some(1),
                        universal_ad_id: vec![UniversalAdId {
                            id_registry: Cow::Borrowed("Ad-ID"),
                            id: Cow::Borrowed("8465")
                        }],
                        non_linear_ads: Some(
                            NonLinearAds { non_linears: vec![
                                NonLinear {
                                    width: Some(350),
                                    height: Some(350),
                                    static_resources: vec![
                                        StaticResource {
                                            creative_type: Cow::Borrowed("image/png"),
                                            uri: Cow::Borrowed("https://mms.businesswire.com/media/20150623005446/en/473787/21/iab_tech_lab.jpg")
                                        },
                                    ],
                                    non_linear_click_through: Some(Cow::Borrowed("https://iabtechlab.com")),
                                    non_linear_click_trackings: vec![
                                        NonLinearClickTracking { uri: Cow::Borrowed("https://example.com/tracking/clickTracking"), ..Default::default() },
                                    ],
                                    ..Default::default()
                                },
                            ], ..Default::default() }
                        ),
                        ..Default::default()
                    },],
                },
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    }
);
