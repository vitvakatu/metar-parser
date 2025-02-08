use parser::prelude::*;

#[test]
fn primary() {
    let input =
        "METAR EHLE 280925Z AUTO 21009G19KT 060V130 5000 -RA FEW007 BKN014CB BKN017 02/M01 Q1001";
    let parser = Parser::new(input);
    assert!(parser.parse().is_ok());
    let expected = Report {
        origin: input,
        kind: Annotated::new(ReportKind::Metar, input, 0, 4),
        station: Annotated::new(
            Station {
                icao_code: "EHLE",
                name: "Lelystad Airport".to_owned(),
                country: "Germany".to_owned(),
            },
            input,
            0,
            4,
        ),
        time: Annotated::new(
            Time {
                day: 28,
                hour: 9,
                minute: 25,
            },
            input,
            4,
            10,
        ),
        is_correction: false,
        is_auto: true,
        wind: Annotated::new(
            Wind {
                direction: 210,
                speed: Knots(9),
                gust: Some(Knots(19)),
                variable: Some(60..=130),
            },
            input,
            10,
            20,
        ),
        visibility: Annotated::new(Meters(5000), input, 20, 25),
        percipitation: Some(Annotated::new(
            Percipitation::Rain {
                intensity: Some(Intensity::Light),
            },
            input,
            25,
            30,
        )),
        clouds: vec![
            Annotated::new(
                CloudLayer {
                    ceiling: 700,
                    cover: Cover::Few,
                    ..Default::default()
                },
                input,
                30,
                35,
            ),
            Annotated::new(
                CloudLayer {
                    ceiling: 1400,
                    cover: Cover::Broken,
                    significant: Some(CloudSignificant::Cumulonimbus),
                },
                input,
                35,
                40,
            ),
            Annotated::new(
                CloudLayer {
                    ceiling: 1700,
                    cover: Cover::Broken,
                    ..Default::default()
                },
                input,
                40,
                45,
            ),
        ],
        temperature: Annotated::new(2, input, 45, 47),
        dew_point: Annotated::new(-1, input, 47, 49),
        pressure: Annotated::new(Hectopascal(1001), input, 49, 54),
    };
    assert_eq!(parser.parse().unwrap(), expected);
}
