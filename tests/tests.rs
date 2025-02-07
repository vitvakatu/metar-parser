use parser::Parser;

#[test]
fn primary() {
    let input = "METAR EHLE 280925Z AUTO 21009G19KT 060V130 5000 -RA FEW007 BKN014CB BKN017 02/M01 Q1001 BECMG 6000";
    let parser = Parser::new(input);
    assert!(parser.parse().is_ok());
}
