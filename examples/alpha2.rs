extern crate iso3166_1;

fn main() {
    // Retrieve a `CountryCode` given its `alpha2` code. This is an `Option` and
    // should be handled appropriately.
    let country = iso3166_1::alpha("AF");
}
