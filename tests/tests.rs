use parser::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn test_dynamic_simple() {
    let input = "METAR ULLI 280900Z 10005KT 9999 RA BKN014 10/05 Q1005";
    let parser = Parser::new(input);
    let mut destination = String::new();
    parser.describe(&mut destination).unwrap();
    assert_eq!(
        destination,
        "METAR, ULLI (Pulkovo Airport, Russia), day 28, 09:00 UTC, wind 100° 5KT, visibility 9999 m, rain, broken clouds at 1400ft, temperature 10°C, dew point 5°C, QNH 1005 hpa"
    );
}

#[test]
fn test_simple() {
    let input = "METAR ULLI 280900Z 10005KT 9999 RA BKN014 10/05 Q1005";
    let parser = Parser::new(input);
    assert!(parser.parse().is_ok());
    let expected = Report {
        origin: input,
        kind: Annotated::with_range(ReportKind::Metar, input, 0..5),
        station: Annotated::with_range(
            Station {
                icao_code: "ULLI",
                name: "Pulkovo Airport".to_owned(),
                country: "Russia".to_owned(),
            },
            input,
            6..10,
        ),
        time: Annotated::with_range(
            Time {
                day: 28,
                hour: 9,
                minute: 0,
            },
            input,
            11..18,
        ),
        wind: Annotated::with_range(
            Wind {
                direction: 100,
                speed: Knots(5),
                gust: None,
                variable: None,
            },
            input,
            19..26,
        ),
        visibility: Annotated::with_range(Visibility::Horizontal(Meters(9999)), input, 27..31),
        percipitation: Some(Annotated::with_range(
            Percipitation::Rain { intensity: None },
            input,
            32..34,
        )),
        clouds: vec![Annotated::with_range(
            CloudLayer {
                ceiling: 1400,
                cover: Cover::Broken,
                significant: None,
            },
            input,
            35..41,
        )],
        temperature: Annotated::with_range(
            Temperature {
                value: 10,
                dew_point: 5,
            },
            input,
            42..47,
        ),
        pressure: Annotated::with_range(
            Pressure {
                value: Hectopascal(1005),
            },
            input,
            48..53,
        ),
    };
    assert_eq!(parser.parse().unwrap(), expected);
}

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
        visibility: Annotated::with_range(Visibility::Horizontal(Meters(5000)), input, 20..25),
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
        temperature: Annotated::with_range(
            Temperature {
                value: 2,
                dew_point: -1,
            },
            input,
            45..47,
        ),
        pressure: Annotated::with_range(
            Pressure {
                value: Hectopascal(1001),
            },
            input,
            49..54,
        ),
    };
    assert_eq!(parser.parse().unwrap(), expected);
}
