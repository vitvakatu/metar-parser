use proc_macro::TokenStream;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[proc_macro]
pub fn read_stations(_: TokenStream) -> TokenStream {
    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/stations.txt");
    let input_file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(input_file);

    let mut stations = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts = line.split(';').collect::<Vec<&str>>();
        stations.insert(
            parts[0].to_owned(),
            (parts[1].to_owned(), parts[2].to_owned()),
        );
    }

    let definitions = stations.into_iter().map(|(icao_code, (name, country))| {
        quote::quote! {
            (
                #icao_code,
                Station { icao_code: #icao_code, name: #name.to_owned(), country: #country.to_owned() }
            )
        }
    });

    quote::quote! {
        pub(crate) static KNOWN_STATIONS: std::sync::LazyLock<std::collections::HashMap<&'static str, Station>> = std::sync::LazyLock::new(|| {
            std::collections::HashMap::from([#(#definitions),*])
        });
    }.into()
}
