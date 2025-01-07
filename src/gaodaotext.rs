use serde::Deserialize;
use serde_json;
use anyhow::Result;

const HEXAGRAM_DATA: [&[u8]; 64] = [
    include_bytes!("../assets/json/gd01.json"),
    include_bytes!("../assets/json/gd02.json"),
    include_bytes!("../assets/json/gd03.json"),
    include_bytes!("../assets/json/gd04.json"),
    include_bytes!("../assets/json/gd05.json"),
    include_bytes!("../assets/json/gd06.json"),
    include_bytes!("../assets/json/gd07.json"),
    include_bytes!("../assets/json/gd08.json"),
    include_bytes!("../assets/json/gd09.json"),
    include_bytes!("../assets/json/gd10.json"),
    include_bytes!("../assets/json/gd11.json"),
    include_bytes!("../assets/json/gd12.json"),
    include_bytes!("../assets/json/gd13.json"),
    include_bytes!("../assets/json/gd14.json"),
    include_bytes!("../assets/json/gd15.json"),
    include_bytes!("../assets/json/gd16.json"),
    include_bytes!("../assets/json/gd17.json"),
    include_bytes!("../assets/json/gd18.json"),
    include_bytes!("../assets/json/gd19.json"),
    include_bytes!("../assets/json/gd20.json"),
    include_bytes!("../assets/json/gd21.json"),
    include_bytes!("../assets/json/gd22.json"),
    include_bytes!("../assets/json/gd23.json"),
    include_bytes!("../assets/json/gd24.json"),
    include_bytes!("../assets/json/gd25.json"),
    include_bytes!("../assets/json/gd26.json"),
    include_bytes!("../assets/json/gd27.json"),
    include_bytes!("../assets/json/gd28.json"),
    include_bytes!("../assets/json/gd29.json"),
    include_bytes!("../assets/json/gd30.json"),
    include_bytes!("../assets/json/gd31.json"),
    include_bytes!("../assets/json/gd32.json"),
    include_bytes!("../assets/json/gd33.json"),
    include_bytes!("../assets/json/gd34.json"),
    include_bytes!("../assets/json/gd35.json"),
    include_bytes!("../assets/json/gd36.json"),
    include_bytes!("../assets/json/gd37.json"),
    include_bytes!("../assets/json/gd38.json"),
    include_bytes!("../assets/json/gd39.json"),
    include_bytes!("../assets/json/gd40.json"),
    include_bytes!("../assets/json/gd41.json"),
    include_bytes!("../assets/json/gd42.json"),
    include_bytes!("../assets/json/gd43.json"),
    include_bytes!("../assets/json/gd44.json"),
    include_bytes!("../assets/json/gd45.json"),
    include_bytes!("../assets/json/gd46.json"),
    include_bytes!("../assets/json/gd47.json"),
    include_bytes!("../assets/json/gd48.json"),
    include_bytes!("../assets/json/gd49.json"),
    include_bytes!("../assets/json/gd50.json"),
    include_bytes!("../assets/json/gd51.json"),
    include_bytes!("../assets/json/gd52.json"),
    include_bytes!("../assets/json/gd53.json"),
    include_bytes!("../assets/json/gd54.json"),
    include_bytes!("../assets/json/gd55.json"),
    include_bytes!("../assets/json/gd56.json"),
    include_bytes!("../assets/json/gd57.json"),
    include_bytes!("../assets/json/gd58.json"),
    include_bytes!("../assets/json/gd59.json"),
    include_bytes!("../assets/json/gd60.json"),
    include_bytes!("../assets/json/gd61.json"),
    include_bytes!("../assets/json/gd62.json"),
    include_bytes!("../assets/json/gd63.json"),
    include_bytes!("../assets/json/gd64.json"),
];

#[derive(Default, Deserialize)]
struct Case {
    Q: String,
    A: Vec<String>,
}

#[derive(Default, Deserialize)]
struct Yao {
    yaoci: String,
    xiaoxiang: String,
    #[serde(rename = "yaoci-explain")]
    yaoci_explain: Vec<String>,
    yaozhan: Vec<String>,
    cases: Vec<Case>,
}

#[derive(Default, Deserialize)]
pub struct HexagramOracle {
    pub order: String,
    pub name: String,
    pub summary: String,
    pub guaci: String,
    #[serde(rename = "guaci-explain")]
    pub guaci_explain: Vec<String>,
    pub tuan: String,
    #[serde(rename = "tuan-explain")]
    pub tuan_explain: Vec<String>,
    pub daxiang: String,
    #[serde(rename = "daxiang-explain")]
    pub daxiang_explain: Vec<String>,
    pub guazhan: Vec<String>,
    pub yaos: Vec<Yao>,
}

pub fn get_oracle(order: usize) -> Result<HexagramOracle> {
    let hexagram_oracle: HexagramOracle = serde_json::from_slice(HEXAGRAM_DATA[order - 1])?;
    Ok(hexagram_oracle)
}