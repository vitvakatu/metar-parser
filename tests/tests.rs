use parser::prelude::*;

#[test]
fn primary() {
    let input =
        "METAR EHLE 280925Z AUTO 21009G19KT 060V130 5000 -RA FEW007 BKN014CB BKN017 02/M01 Q1001";
    let parser = Parser::new(input);
    assert!(parser.parse().is_ok());
    let expected = Report {
        origin: input,
        kind: ReportKind::Metar,
        station: Station {
            icao_code: "EHLE",
            name: "Lelystad Airport".to_owned(),
            country: "Germany".to_owned(),
        },
        time: Time {
            day: 28,
            hour: 9,
            minute: 25,
        },
        is_correction: false,
        is_auto: true,
        wind: Wind {
            direction: 210,
            speed: Knots(9),
            gust: Some(Knots(19)),
            variable: Some(60..=130),
        },
        visibility: 5000,
        percipitation: Some(Percipitation::Rain {
            intensity: Some(Intensity::Light),
        }),
        clouds: vec![
            CloudLayer {
                ceiling: 700,
                cover: Cover::Few,
                ..Default::default()
            },
            CloudLayer {
                ceiling: 1400,
                cover: Cover::Broken,
                significant: Some(CloudSignificant::Cumulonimbus),
            },
            CloudLayer {
                ceiling: 1700,
                cover: Cover::Broken,
                ..Default::default()
            },
        ],
        temperature: 2,
        dew_point: -1,
        pressure: Hectopascal(1001),
    };
    assert_eq!(parser.parse().unwrap(), expected);
}
