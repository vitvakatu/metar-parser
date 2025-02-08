use parser::prelude::*;

#[test]
fn primary() {
    let input =
        "METAR EHLE 280925Z AUTO 21009G19KT 060V130 5000 -RA FEW007 BKN014CB BKN017 02/M01 Q1001";
    let parser = Parser::new(input);
    assert!(parser.parse().is_ok());
    let expected = Report {
        origin: input,
        kind: Annotated::with_range(ReportKind::Metar, input, 0..4),
        station: Annotated::with_range(
            Station {
                icao_code: "EHLE",
                name: "Lelystad Airport".to_owned(),
                country: "Netherlands".to_owned(),
            },
            input,
            0..4,
        ),
        time: Annotated::with_range(
            Time {
                day: 28,
                hour: 9,
                minute: 25,
            },
            input,
            4..10,
        ),
        wind: Annotated::with_range(
            Wind {
                direction: 210,
                speed: Knots(9),
                gust: Some(Knots(19)),
                variable: Some(60..=130),
            },
            input,
            10..20,
        ),
        visibility: Annotated::with_range(Meters(5000), input, 20..25),
        percipitation: Some(Annotated::with_range(
            Percipitation::Rain {
                intensity: Some(Intensity::Light),
            },
            input,
            25..30,
        )),
        clouds: vec![
            Annotated::with_range(
                CloudLayer {
                    ceiling: 700,
                    cover: Cover::Few,
                    ..Default::default()
                },
                input,
                30..35,
            ),
            Annotated::with_range(
                CloudLayer {
                    ceiling: 1400,
                    cover: Cover::Broken,
                    significant: Some(CloudSignificant::Cumulonimbus),
                },
                input,
                35..40,
            ),
            Annotated::with_range(
                CloudLayer {
                    ceiling: 1700,
                    cover: Cover::Broken,
                    ..Default::default()
                },
                input,
                40..45,
            ),
        ],
        temperature: Annotated::with_range(2, input, 45..47),
        dew_point: Annotated::with_range(-1, input, 47..49),
        pressure: Annotated::with_range(Hectopascal(1001), input, 49..54),
    };
    assert_eq!(parser.parse().unwrap(), expected);
}
