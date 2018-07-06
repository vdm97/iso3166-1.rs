// ISC License (ISC)
//
// Copyright (c) 2016, Zeyla Hellyer <zeylahellyer@gmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
// RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
// CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// What is ISO 3166-1?
//
// | ISO 3166-1 is part of the ISO 3166 standard published by the International
// | Organization for Standardization (ISO), and defines codes for the names of
// | countries, dependent territories, and special areas of geographical
// | interest.
// |
// | - [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-1)
//
// Originally by zeyla on GitHub.

use CountryCode;

/// Returns a `Vec` of all `CountryCode`s defined by ISO 3166-1.
///
/// # Examples
///
/// ```rust
/// let countries = iso3166_1::all();
/// ```
// A function that returns a really big Vector of all the country codes
// designated by ISO 3166-1.
//
// These can be automatically updated by running 'make update'.
//
// The 'Begin' and 'End' comments are essential as it tells the update script
// where to begin and end inserting the country codes.
//
// Source:
// https://en.wikipedia.org/wiki/ISO_3166-1#Officially_assigned_code_elements
pub fn all() -> Vec<CountryCode> {
    let mut codes: Vec<CountryCode> = vec![];

    // Begin
    codes.push(CountryCode {
        alpha2: String::from("AF"),
        alpha3: String::from("AFG"),
        name: String::from("Afghanistan"),
        num: String::from("004"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AL"),
        alpha3: String::from("ALB"),
        name: String::from("Albania"),
        num: String::from("008"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AQ"),
        alpha3: String::from("ATA"),
        name: String::from("Antarctica"),
        num: String::from("010"),
    });
    codes.push(CountryCode {
        alpha2: String::from("DZ"),
        alpha3: String::from("DZA"),
        name: String::from("Algeria"),
        num: String::from("012"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AS"),
        alpha3: String::from("ASM"),
        name: String::from("American Samoa"),
        num: String::from("016"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AD"),
        alpha3: String::from("AND"),
        name: String::from("Andorra"),
        num: String::from("020"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AO"),
        alpha3: String::from("AGO"),
        name: String::from("Angola"),
        num: String::from("024"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AG"),
        alpha3: String::from("ATG"),
        name: String::from("Antigua and Barbuda"),
        num: String::from("028"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AZ"),
        alpha3: String::from("AZE"),
        name: String::from("Azerbaijan"),
        num: String::from("031"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AR"),
        alpha3: String::from("ARG"),
        name: String::from("Argentina"),
        num: String::from("032"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AU"),
        alpha3: String::from("AUS"),
        name: String::from("Australia"),
        num: String::from("036"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AT"),
        alpha3: String::from("AUT"),
        name: String::from("Austria"),
        num: String::from("040"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BS"),
        alpha3: String::from("BHS"),
        name: String::from("Bahamas"),
        num: String::from("044"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BH"),
        alpha3: String::from("BHR"),
        name: String::from("Bahrain"),
        num: String::from("048"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BD"),
        alpha3: String::from("BGD"),
        name: String::from("Bangladesh"),
        num: String::from("050"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AM"),
        alpha3: String::from("ARM"),
        name: String::from("Armenia"),
        num: String::from("051"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BB"),
        alpha3: String::from("BRB"),
        name: String::from("Barbados"),
        num: String::from("052"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BE"),
        alpha3: String::from("BEL"),
        name: String::from("Belgium"),
        num: String::from("056"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BM"),
        alpha3: String::from("BMU"),
        name: String::from("Bermuda"),
        num: String::from("060"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BT"),
        alpha3: String::from("BTN"),
        name: String::from("Bhutan"),
        num: String::from("064"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BO"),
        alpha3: String::from("BOL"),
        name: String::from("Bolivia (Plurinational State of)"),
        num: String::from("068"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BA"),
        alpha3: String::from("BIH"),
        name: String::from("Bosnia and Herzegovina"),
        num: String::from("070"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BW"),
        alpha3: String::from("BWA"),
        name: String::from("Botswana"),
        num: String::from("072"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BV"),
        alpha3: String::from("BVT"),
        name: String::from("Bouvet Island"),
        num: String::from("074"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BR"),
        alpha3: String::from("BRA"),
        name: String::from("Brazil"),
        num: String::from("076"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BZ"),
        alpha3: String::from("BLZ"),
        name: String::from("Belize"),
        num: String::from("084"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IO"),
        alpha3: String::from("IOT"),
        name: String::from("British Indian Ocean Territory"),
        num: String::from("086"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SB"),
        alpha3: String::from("SLB"),
        name: String::from("Solomon Islands"),
        num: String::from("090"),
    });
    codes.push(CountryCode {
        alpha2: String::from("VG"),
        alpha3: String::from("VGB"),
        name: String::from("Virgin Islands (British)"),
        num: String::from("092"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BN"),
        alpha3: String::from("BRN"),
        name: String::from("Brunei Darussalam"),
        num: String::from("096"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BG"),
        alpha3: String::from("BGR"),
        name: String::from("Bulgaria"),
        num: String::from("100"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MM"),
        alpha3: String::from("MMR"),
        name: String::from("Myanmar"),
        num: String::from("104"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BI"),
        alpha3: String::from("BDI"),
        name: String::from("Burundi"),
        num: String::from("108"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BY"),
        alpha3: String::from("BLR"),
        name: String::from("Belarus"),
        num: String::from("112"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KH"),
        alpha3: String::from("KHM"),
        name: String::from("Cambodia"),
        num: String::from("116"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CM"),
        alpha3: String::from("CMR"),
        name: String::from("Cameroon"),
        num: String::from("120"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CA"),
        alpha3: String::from("CAN"),
        name: String::from("Canada"),
        num: String::from("124"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CV"),
        alpha3: String::from("CPV"),
        name: String::from("Cabo Verde"),
        num: String::from("132"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KY"),
        alpha3: String::from("CYM"),
        name: String::from("Cayman Islands"),
        num: String::from("136"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CF"),
        alpha3: String::from("CAF"),
        name: String::from("Central African Republic"),
        num: String::from("140"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LK"),
        alpha3: String::from("LKA"),
        name: String::from("Sri Lanka"),
        num: String::from("144"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TD"),
        alpha3: String::from("TCD"),
        name: String::from("Chad"),
        num: String::from("148"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CL"),
        alpha3: String::from("CHL"),
        name: String::from("Chile"),
        num: String::from("152"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CN"),
        alpha3: String::from("CHN"),
        name: String::from("China"),
        num: String::from("156"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TW"),
        alpha3: String::from("TWN"),
        name: String::from("Taiwan, Province of China[a]"),
        num: String::from("158"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CX"),
        alpha3: String::from("CXR"),
        name: String::from("Christmas Island"),
        num: String::from("162"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CC"),
        alpha3: String::from("CCK"),
        name: String::from("Cocos (Keeling) Islands"),
        num: String::from("166"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CO"),
        alpha3: String::from("COL"),
        name: String::from("Colombia"),
        num: String::from("170"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KM"),
        alpha3: String::from("COM"),
        name: String::from("Comoros"),
        num: String::from("174"),
    });
    codes.push(CountryCode {
        alpha2: String::from("YT"),
        alpha3: String::from("MYT"),
        name: String::from("Mayotte"),
        num: String::from("175"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CG"),
        alpha3: String::from("COG"),
        name: String::from("Congo"),
        num: String::from("178"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CD"),
        alpha3: String::from("COD"),
        name: String::from("Congo (Democratic Republic of the)"),
        num: String::from("180"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CK"),
        alpha3: String::from("COK"),
        name: String::from("Cook Islands"),
        num: String::from("184"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CR"),
        alpha3: String::from("CRI"),
        name: String::from("Costa Rica"),
        num: String::from("188"),
    });
    codes.push(CountryCode {
        alpha2: String::from("HR"),
        alpha3: String::from("HRV"),
        name: String::from("Croatia"),
        num: String::from("191"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CU"),
        alpha3: String::from("CUB"),
        name: String::from("Cuba"),
        num: String::from("192"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CY"),
        alpha3: String::from("CYP"),
        name: String::from("Cyprus"),
        num: String::from("196"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CZ"),
        alpha3: String::from("CZE"),
        name: String::from("Czech Republic"),
        num: String::from("203"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BJ"),
        alpha3: String::from("BEN"),
        name: String::from("Benin"),
        num: String::from("204"),
    });
    codes.push(CountryCode {
        alpha2: String::from("DK"),
        alpha3: String::from("DNK"),
        name: String::from("Denmark"),
        num: String::from("208"),
    });
    codes.push(CountryCode {
        alpha2: String::from("DM"),
        alpha3: String::from("DMA"),
        name: String::from("Dominica"),
        num: String::from("212"),
    });
    codes.push(CountryCode {
        alpha2: String::from("DO"),
        alpha3: String::from("DOM"),
        name: String::from("Dominican Republic"),
        num: String::from("214"),
    });
    codes.push(CountryCode {
        alpha2: String::from("EC"),
        alpha3: String::from("ECU"),
        name: String::from("Ecuador"),
        num: String::from("218"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SV"),
        alpha3: String::from("SLV"),
        name: String::from("El Salvador"),
        num: String::from("222"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GQ"),
        alpha3: String::from("GNQ"),
        name: String::from("Equatorial Guinea"),
        num: String::from("226"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ET"),
        alpha3: String::from("ETH"),
        name: String::from("Ethiopia"),
        num: String::from("231"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ER"),
        alpha3: String::from("ERI"),
        name: String::from("Eritrea"),
        num: String::from("232"),
    });
    codes.push(CountryCode {
        alpha2: String::from("EE"),
        alpha3: String::from("EST"),
        name: String::from("Estonia"),
        num: String::from("233"),
    });
    codes.push(CountryCode {
        alpha2: String::from("FO"),
        alpha3: String::from("FRO"),
        name: String::from("Faroe Islands"),
        num: String::from("234"),
    });
    codes.push(CountryCode {
        alpha2: String::from("FK"),
        alpha3: String::from("FLK"),
        name: String::from("Falkland Islands"),
        num: String::from("238"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GS"),
        alpha3: String::from("SGS"),
        name: String::from("South Georgia and the South Sandwich Islands"),
        num: String::from("239"),
    });
    codes.push(CountryCode {
        alpha2: String::from("FJ"),
        alpha3: String::from("FJI"),
        name: String::from("Fiji"),
        num: String::from("242"),
    });
    codes.push(CountryCode {
        alpha2: String::from("FI"),
        alpha3: String::from("FIN"),
        name: String::from("Finland"),
        num: String::from("246"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AX"),
        alpha3: String::from("ALA"),
        name: String::from("Åland Islands"),
        num: String::from("248"),
    });
    codes.push(CountryCode {
        alpha2: String::from("FR"),
        alpha3: String::from("FRA"),
        name: String::from("France"),
        num: String::from("250"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GF"),
        alpha3: String::from("GUF"),
        name: String::from("French Guiana"),
        num: String::from("254"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PF"),
        alpha3: String::from("PYF"),
        name: String::from("French Polynesia"),
        num: String::from("258"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TF"),
        alpha3: String::from("ATF"),
        name: String::from("French Southern Territories"),
        num: String::from("260"),
    });
    codes.push(CountryCode {
        alpha2: String::from("DJ"),
        alpha3: String::from("DJI"),
        name: String::from("Djibouti"),
        num: String::from("262"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GA"),
        alpha3: String::from("GAB"),
        name: String::from("Gabon"),
        num: String::from("266"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GE"),
        alpha3: String::from("GEO"),
        name: String::from("Georgia"),
        num: String::from("268"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GM"),
        alpha3: String::from("GMB"),
        name: String::from("Gambia"),
        num: String::from("270"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PS"),
        alpha3: String::from("PSE"),
        name: String::from("Palestine, State of"),
        num: String::from("275"),
    });
    codes.push(CountryCode {
        alpha2: String::from("DE"),
        alpha3: String::from("DEU"),
        name: String::from("Germany"),
        num: String::from("276"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GH"),
        alpha3: String::from("GHA"),
        name: String::from("Ghana"),
        num: String::from("288"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GI"),
        alpha3: String::from("GIB"),
        name: String::from("Gibraltar"),
        num: String::from("292"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KI"),
        alpha3: String::from("KIR"),
        name: String::from("Kiribati"),
        num: String::from("296"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GR"),
        alpha3: String::from("GRC"),
        name: String::from("Greece"),
        num: String::from("300"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GL"),
        alpha3: String::from("GRL"),
        name: String::from("Greenland"),
        num: String::from("304"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GD"),
        alpha3: String::from("GRD"),
        name: String::from("Grenada"),
        num: String::from("308"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GP"),
        alpha3: String::from("GLP"),
        name: String::from("Guadeloupe"),
        num: String::from("312"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GU"),
        alpha3: String::from("GUM"),
        name: String::from("Guam"),
        num: String::from("316"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GT"),
        alpha3: String::from("GTM"),
        name: String::from("Guatemala"),
        num: String::from("320"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GN"),
        alpha3: String::from("GIN"),
        name: String::from("Guinea"),
        num: String::from("324"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GY"),
        alpha3: String::from("GUY"),
        name: String::from("Guyana"),
        num: String::from("328"),
    });
    codes.push(CountryCode {
        alpha2: String::from("HT"),
        alpha3: String::from("HTI"),
        name: String::from("Haiti"),
        num: String::from("332"),
    });
    codes.push(CountryCode {
        alpha2: String::from("HM"),
        alpha3: String::from("HMD"),
        name: String::from("Heard Island and McDonald Islands"),
        num: String::from("334"),
    });
    codes.push(CountryCode {
        alpha2: String::from("VA"),
        alpha3: String::from("VAT"),
        name: String::from("Holy See"),
        num: String::from("336"),
    });
    codes.push(CountryCode {
        alpha2: String::from("HN"),
        alpha3: String::from("HND"),
        name: String::from("Honduras"),
        num: String::from("340"),
    });
    codes.push(CountryCode {
        alpha2: String::from("HK"),
        alpha3: String::from("HKG"),
        name: String::from("Hong Kong"),
        num: String::from("344"),
    });
    codes.push(CountryCode {
        alpha2: String::from("HU"),
        alpha3: String::from("HUN"),
        name: String::from("Hungary"),
        num: String::from("348"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IS"),
        alpha3: String::from("ISL"),
        name: String::from("Iceland"),
        num: String::from("352"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IN"),
        alpha3: String::from("IND"),
        name: String::from("India"),
        num: String::from("356"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ID"),
        alpha3: String::from("IDN"),
        name: String::from("Indonesia"),
        num: String::from("360"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IR"),
        alpha3: String::from("IRN"),
        name: String::from("Iran (Islamic Republic of)"),
        num: String::from("364"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IQ"),
        alpha3: String::from("IRQ"),
        name: String::from("Iraq"),
        num: String::from("368"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IE"),
        alpha3: String::from("IRL"),
        name: String::from("Ireland"),
        num: String::from("372"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IL"),
        alpha3: String::from("ISR"),
        name: String::from("Israel"),
        num: String::from("376"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IT"),
        alpha3: String::from("ITA"),
        name: String::from("Italy"),
        num: String::from("380"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CI"),
        alpha3: String::from("CIV"),
        name: String::from("Côte d'Ivoire"),
        num: String::from("384"),
    });
    codes.push(CountryCode {
        alpha2: String::from("JM"),
        alpha3: String::from("JAM"),
        name: String::from("Jamaica"),
        num: String::from("388"),
    });
    codes.push(CountryCode {
        alpha2: String::from("JP"),
        alpha3: String::from("JPN"),
        name: String::from("Japan"),
        num: String::from("392"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KZ"),
        alpha3: String::from("KAZ"),
        name: String::from("Kazakhstan"),
        num: String::from("398"),
    });
    codes.push(CountryCode {
        alpha2: String::from("JO"),
        alpha3: String::from("JOR"),
        name: String::from("Jordan"),
        num: String::from("400"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KE"),
        alpha3: String::from("KEN"),
        name: String::from("Kenya"),
        num: String::from("404"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KP"),
        alpha3: String::from("PRK"),
        name: String::from("Korea (Democratic People's Republic of)"),
        num: String::from("408"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KR"),
        alpha3: String::from("KOR"),
        name: String::from("Korea (Republic of)"),
        num: String::from("410"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KW"),
        alpha3: String::from("KWT"),
        name: String::from("Kuwait"),
        num: String::from("414"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KG"),
        alpha3: String::from("KGZ"),
        name: String::from("Kyrgyzstan"),
        num: String::from("417"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LA"),
        alpha3: String::from("LAO"),
        name: String::from("Lao People's Democratic Republic"),
        num: String::from("418"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LB"),
        alpha3: String::from("LBN"),
        name: String::from("Lebanon"),
        num: String::from("422"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LS"),
        alpha3: String::from("LSO"),
        name: String::from("Lesotho"),
        num: String::from("426"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LV"),
        alpha3: String::from("LVA"),
        name: String::from("Latvia"),
        num: String::from("428"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LR"),
        alpha3: String::from("LBR"),
        name: String::from("Liberia"),
        num: String::from("430"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LY"),
        alpha3: String::from("LBY"),
        name: String::from("Libya"),
        num: String::from("434"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LI"),
        alpha3: String::from("LIE"),
        name: String::from("Liechtenstein"),
        num: String::from("438"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LT"),
        alpha3: String::from("LTU"),
        name: String::from("Lithuania"),
        num: String::from("440"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LU"),
        alpha3: String::from("LUX"),
        name: String::from("Luxembourg"),
        num: String::from("442"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MO"),
        alpha3: String::from("MAC"),
        name: String::from("Macao"),
        num: String::from("446"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MG"),
        alpha3: String::from("MDG"),
        name: String::from("Madagascar"),
        num: String::from("450"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MW"),
        alpha3: String::from("MWI"),
        name: String::from("Malawi"),
        num: String::from("454"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MY"),
        alpha3: String::from("MYS"),
        name: String::from("Malaysia"),
        num: String::from("458"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MV"),
        alpha3: String::from("MDV"),
        name: String::from("Maldives"),
        num: String::from("462"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ML"),
        alpha3: String::from("MLI"),
        name: String::from("Mali"),
        num: String::from("466"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MT"),
        alpha3: String::from("MLT"),
        name: String::from("Malta"),
        num: String::from("470"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MQ"),
        alpha3: String::from("MTQ"),
        name: String::from("Martinique"),
        num: String::from("474"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MR"),
        alpha3: String::from("MRT"),
        name: String::from("Mauritania"),
        num: String::from("478"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MU"),
        alpha3: String::from("MUS"),
        name: String::from("Mauritius"),
        num: String::from("480"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MX"),
        alpha3: String::from("MEX"),
        name: String::from("Mexico"),
        num: String::from("484"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MC"),
        alpha3: String::from("MCO"),
        name: String::from("Monaco"),
        num: String::from("492"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MN"),
        alpha3: String::from("MNG"),
        name: String::from("Mongolia"),
        num: String::from("496"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MD"),
        alpha3: String::from("MDA"),
        name: String::from("Moldova (Republic of)"),
        num: String::from("498"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ME"),
        alpha3: String::from("MNE"),
        name: String::from("Montenegro"),
        num: String::from("499"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MS"),
        alpha3: String::from("MSR"),
        name: String::from("Montserrat"),
        num: String::from("500"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MA"),
        alpha3: String::from("MAR"),
        name: String::from("Morocco"),
        num: String::from("504"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MZ"),
        alpha3: String::from("MOZ"),
        name: String::from("Mozambique"),
        num: String::from("508"),
    });
    codes.push(CountryCode {
        alpha2: String::from("OM"),
        alpha3: String::from("OMN"),
        name: String::from("Oman"),
        num: String::from("512"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NA"),
        alpha3: String::from("NAM"),
        name: String::from("Namibia"),
        num: String::from("516"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NR"),
        alpha3: String::from("NRU"),
        name: String::from("Nauru"),
        num: String::from("520"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NP"),
        alpha3: String::from("NPL"),
        name: String::from("Nepal"),
        num: String::from("524"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NL"),
        alpha3: String::from("NLD"),
        name: String::from("Netherlands"),
        num: String::from("528"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CW"),
        alpha3: String::from("CUW"),
        name: String::from("Curaçao"),
        num: String::from("531"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AW"),
        alpha3: String::from("ABW"),
        name: String::from("Aruba"),
        num: String::from("533"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SX"),
        alpha3: String::from("SXM"),
        name: String::from("Sint Maarten (Dutch part)"),
        num: String::from("534"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BQ"),
        alpha3: String::from("BES"),
        name: String::from("Bonaire, Sint Eustatius and Saba"),
        num: String::from("535"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NC"),
        alpha3: String::from("NCL"),
        name: String::from("New Caledonia"),
        num: String::from("540"),
    });
    codes.push(CountryCode {
        alpha2: String::from("VU"),
        alpha3: String::from("VUT"),
        name: String::from("Vanuatu"),
        num: String::from("548"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NZ"),
        alpha3: String::from("NZL"),
        name: String::from("New Zealand"),
        num: String::from("554"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NI"),
        alpha3: String::from("NIC"),
        name: String::from("Nicaragua"),
        num: String::from("558"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NE"),
        alpha3: String::from("NER"),
        name: String::from("Niger"),
        num: String::from("562"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NG"),
        alpha3: String::from("NGA"),
        name: String::from("Nigeria"),
        num: String::from("566"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NU"),
        alpha3: String::from("NIU"),
        name: String::from("Niue"),
        num: String::from("570"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NF"),
        alpha3: String::from("NFK"),
        name: String::from("Norfolk Island"),
        num: String::from("574"),
    });
    codes.push(CountryCode {
        alpha2: String::from("NO"),
        alpha3: String::from("NOR"),
        name: String::from("Norway"),
        num: String::from("578"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MP"),
        alpha3: String::from("MNP"),
        name: String::from("Northern Mariana Islands"),
        num: String::from("580"),
    });
    codes.push(CountryCode {
        alpha2: String::from("UM"),
        alpha3: String::from("UMI"),
        name: String::from("United States Minor Outlying Islands"),
        num: String::from("581"),
    });
    codes.push(CountryCode {
        alpha2: String::from("FM"),
        alpha3: String::from("FSM"),
        name: String::from("Micronesia (Federated States of)"),
        num: String::from("583"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MH"),
        alpha3: String::from("MHL"),
        name: String::from("Marshall Islands"),
        num: String::from("584"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PW"),
        alpha3: String::from("PLW"),
        name: String::from("Palau"),
        num: String::from("585"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PK"),
        alpha3: String::from("PAK"),
        name: String::from("Pakistan"),
        num: String::from("586"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PA"),
        alpha3: String::from("PAN"),
        name: String::from("Panama"),
        num: String::from("591"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PG"),
        alpha3: String::from("PNG"),
        name: String::from("Papua New Guinea"),
        num: String::from("598"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PY"),
        alpha3: String::from("PRY"),
        name: String::from("Paraguay"),
        num: String::from("600"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PE"),
        alpha3: String::from("PER"),
        name: String::from("Peru"),
        num: String::from("604"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PH"),
        alpha3: String::from("PHL"),
        name: String::from("Philippines"),
        num: String::from("608"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PN"),
        alpha3: String::from("PCN"),
        name: String::from("Pitcairn"),
        num: String::from("612"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PL"),
        alpha3: String::from("POL"),
        name: String::from("Poland"),
        num: String::from("616"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PT"),
        alpha3: String::from("PRT"),
        name: String::from("Portugal"),
        num: String::from("620"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GW"),
        alpha3: String::from("GNB"),
        name: String::from("Guinea-Bissau"),
        num: String::from("624"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TL"),
        alpha3: String::from("TLS"),
        name: String::from("Timor-Leste"),
        num: String::from("626"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PR"),
        alpha3: String::from("PRI"),
        name: String::from("Puerto Rico"),
        num: String::from("630"),
    });
    codes.push(CountryCode {
        alpha2: String::from("QA"),
        alpha3: String::from("QAT"),
        name: String::from("Qatar"),
        num: String::from("634"),
    });
    codes.push(CountryCode {
        alpha2: String::from("RE"),
        alpha3: String::from("REU"),
        name: String::from("Réunion"),
        num: String::from("638"),
    });
    codes.push(CountryCode {
        alpha2: String::from("RO"),
        alpha3: String::from("ROU"),
        name: String::from("Romania"),
        num: String::from("642"),
    });
    codes.push(CountryCode {
        alpha2: String::from("RU"),
        alpha3: String::from("RUS"),
        name: String::from("Russian Federation"),
        num: String::from("643"),
    });
    codes.push(CountryCode {
        alpha2: String::from("RW"),
        alpha3: String::from("RWA"),
        name: String::from("Rwanda"),
        num: String::from("646"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BL"),
        alpha3: String::from("BLM"),
        name: String::from("Saint Barthélemy"),
        num: String::from("652"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SH"),
        alpha3: String::from("SHN"),
        name: String::from("Saint Helena, Ascension and Tristan da Cunha"),
        num: String::from("654"),
    });
    codes.push(CountryCode {
        alpha2: String::from("KN"),
        alpha3: String::from("KNA"),
        name: String::from("Saint Kitts and Nevis"),
        num: String::from("659"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AI"),
        alpha3: String::from("AIA"),
        name: String::from("Anguilla"),
        num: String::from("660"),
    });
    codes.push(CountryCode {
        alpha2: String::from("LC"),
        alpha3: String::from("LCA"),
        name: String::from("Saint Lucia"),
        num: String::from("662"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MF"),
        alpha3: String::from("MAF"),
        name: String::from("Saint Martin (French part)"),
        num: String::from("663"),
    });
    codes.push(CountryCode {
        alpha2: String::from("PM"),
        alpha3: String::from("SPM"),
        name: String::from("Saint Pierre and Miquelon"),
        num: String::from("666"),
    });
    codes.push(CountryCode {
        alpha2: String::from("VC"),
        alpha3: String::from("VCT"),
        name: String::from("Saint Vincent and the Grenadines"),
        num: String::from("670"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SM"),
        alpha3: String::from("SMR"),
        name: String::from("San Marino"),
        num: String::from("674"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ST"),
        alpha3: String::from("STP"),
        name: String::from("Sao Tome and Principe"),
        num: String::from("678"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SA"),
        alpha3: String::from("SAU"),
        name: String::from("Saudi Arabia"),
        num: String::from("682"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SN"),
        alpha3: String::from("SEN"),
        name: String::from("Senegal"),
        num: String::from("686"),
    });
    codes.push(CountryCode {
        alpha2: String::from("RS"),
        alpha3: String::from("SRB"),
        name: String::from("Serbia"),
        num: String::from("688"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SC"),
        alpha3: String::from("SYC"),
        name: String::from("Seychelles"),
        num: String::from("690"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SL"),
        alpha3: String::from("SLE"),
        name: String::from("Sierra Leone"),
        num: String::from("694"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SG"),
        alpha3: String::from("SGP"),
        name: String::from("Singapore"),
        num: String::from("702"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SK"),
        alpha3: String::from("SVK"),
        name: String::from("Slovakia"),
        num: String::from("703"),
    });
    codes.push(CountryCode {
        alpha2: String::from("VN"),
        alpha3: String::from("VNM"),
        name: String::from("Viet Nam"),
        num: String::from("704"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SI"),
        alpha3: String::from("SVN"),
        name: String::from("Slovenia"),
        num: String::from("705"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SO"),
        alpha3: String::from("SOM"),
        name: String::from("Somalia"),
        num: String::from("706"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ZA"),
        alpha3: String::from("ZAF"),
        name: String::from("South Africa"),
        num: String::from("710"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ZW"),
        alpha3: String::from("ZWE"),
        name: String::from("Zimbabwe"),
        num: String::from("716"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ES"),
        alpha3: String::from("ESP"),
        name: String::from("Spain"),
        num: String::from("724"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SS"),
        alpha3: String::from("SSD"),
        name: String::from("South Sudan"),
        num: String::from("728"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SD"),
        alpha3: String::from("SDN"),
        name: String::from("Sudan"),
        num: String::from("729"),
    });
    codes.push(CountryCode {
        alpha2: String::from("EH"),
        alpha3: String::from("ESH"),
        name: String::from("Western Sahara"),
        num: String::from("732"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SR"),
        alpha3: String::from("SUR"),
        name: String::from("Suriname"),
        num: String::from("740"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SJ"),
        alpha3: String::from("SJM"),
        name: String::from("Svalbard and Jan Mayen"),
        num: String::from("744"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SZ"),
        alpha3: String::from("SWZ"),
        name: String::from("Swaziland"),
        num: String::from("748"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SE"),
        alpha3: String::from("SWE"),
        name: String::from("Sweden"),
        num: String::from("752"),
    });
    codes.push(CountryCode {
        alpha2: String::from("CH"),
        alpha3: String::from("CHE"),
        name: String::from("Switzerland"),
        num: String::from("756"),
    });
    codes.push(CountryCode {
        alpha2: String::from("SY"),
        alpha3: String::from("SYR"),
        name: String::from("Syrian Arab Republic"),
        num: String::from("760"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TJ"),
        alpha3: String::from("TJK"),
        name: String::from("Tajikistan"),
        num: String::from("762"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TH"),
        alpha3: String::from("THA"),
        name: String::from("Thailand"),
        num: String::from("764"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TG"),
        alpha3: String::from("TGO"),
        name: String::from("Togo"),
        num: String::from("768"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TK"),
        alpha3: String::from("TKL"),
        name: String::from("Tokelau"),
        num: String::from("772"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TO"),
        alpha3: String::from("TON"),
        name: String::from("Tonga"),
        num: String::from("776"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TT"),
        alpha3: String::from("TTO"),
        name: String::from("Trinidad and Tobago"),
        num: String::from("780"),
    });
    codes.push(CountryCode {
        alpha2: String::from("AE"),
        alpha3: String::from("ARE"),
        name: String::from("United Arab Emirates"),
        num: String::from("784"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TN"),
        alpha3: String::from("TUN"),
        name: String::from("Tunisia"),
        num: String::from("788"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TR"),
        alpha3: String::from("TUR"),
        name: String::from("Turkey"),
        num: String::from("792"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TM"),
        alpha3: String::from("TKM"),
        name: String::from("Turkmenistan"),
        num: String::from("795"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TC"),
        alpha3: String::from("TCA"),
        name: String::from("Turks and Caicos Islands"),
        num: String::from("796"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TV"),
        alpha3: String::from("TUV"),
        name: String::from("Tuvalu"),
        num: String::from("798"),
    });
    codes.push(CountryCode {
        alpha2: String::from("UG"),
        alpha3: String::from("UGA"),
        name: String::from("Uganda"),
        num: String::from("800"),
    });
    codes.push(CountryCode {
        alpha2: String::from("UA"),
        alpha3: String::from("UKR"),
        name: String::from("Ukraine"),
        num: String::from("804"),
    });
    codes.push(CountryCode {
        alpha2: String::from("MK"),
        alpha3: String::from("MKD"),
        name: String::from("Macedonia (the former Yugoslav Republic of)"),
        num: String::from("807"),
    });
    codes.push(CountryCode {
        alpha2: String::from("EG"),
        alpha3: String::from("EGY"),
        name: String::from("Egypt"),
        num: String::from("818"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GB"),
        alpha3: String::from("GBR"),
        name: String::from("United Kingdom of Great Britain and Northern Ireland"),
        num: String::from("826"),
    });
    codes.push(CountryCode {
        alpha2: String::from("GG"),
        alpha3: String::from("GGY"),
        name: String::from("Guernsey"),
        num: String::from("831"),
    });
    codes.push(CountryCode {
        alpha2: String::from("JE"),
        alpha3: String::from("JEY"),
        name: String::from("Jersey"),
        num: String::from("832"),
    });
    codes.push(CountryCode {
        alpha2: String::from("IM"),
        alpha3: String::from("IMN"),
        name: String::from("Isle of Man"),
        num: String::from("833"),
    });
    codes.push(CountryCode {
        alpha2: String::from("TZ"),
        alpha3: String::from("TZA"),
        name: String::from("Tanzania, United Republic of"),
        num: String::from("834"),
    });
    codes.push(CountryCode {
        alpha2: String::from("US"),
        alpha3: String::from("USA"),
        name: String::from("United States of America"),
        num: String::from("840"),
    });
    codes.push(CountryCode {
        alpha2: String::from("VI"),
        alpha3: String::from("VIR"),
        name: String::from("Virgin Islands (U.S.)"),
        num: String::from("850"),
    });
    codes.push(CountryCode {
        alpha2: String::from("BF"),
        alpha3: String::from("BFA"),
        name: String::from("Burkina Faso"),
        num: String::from("854"),
    });
    codes.push(CountryCode {
        alpha2: String::from("UY"),
        alpha3: String::from("URY"),
        name: String::from("Uruguay"),
        num: String::from("858"),
    });
    codes.push(CountryCode {
        alpha2: String::from("UZ"),
        alpha3: String::from("UZB"),
        name: String::from("Uzbekistan"),
        num: String::from("860"),
    });
    codes.push(CountryCode {
        alpha2: String::from("VE"),
        alpha3: String::from("VEN"),
        name: String::from("Venezuela (Bolivarian Republic of)"),
        num: String::from("862"),
    });
    codes.push(CountryCode {
        alpha2: String::from("WF"),
        alpha3: String::from("WLF"),
        name: String::from("Wallis and Futuna"),
        num: String::from("876"),
    });
    codes.push(CountryCode {
        alpha2: String::from("WS"),
        alpha3: String::from("WSM"),
        name: String::from("Samoa"),
        num: String::from("882"),
    });
    codes.push(CountryCode {
        alpha2: String::from("YE"),
        alpha3: String::from("YEM"),
        name: String::from("Yemen"),
        num: String::from("887"),
    });
    codes.push(CountryCode {
        alpha2: String::from("ZM"),
        alpha3: String::from("ZMB"),
        name: String::from("Zambia"),
        num: String::from("894"),
    });
    // End

    codes
}
