use serde::Deserialize;
use serde::Serialize;
use serde_json;
use anyhow::Result;
use tera::{Tera, Context};
use crate::gram::Hexagram;

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

pub const ORACLE_GUA_TEMPLATE: &str = include_str!("../assets/templates/oracle_gua.md");
pub const ORACLE_YAO_TEMPLATE: &str = include_str!("../assets/templates/oracle_yao.md");

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Case {
    pub Q: String,
    pub A: Vec<String>,
}

#[derive(Default, Clone, Deserialize)]
pub struct Yao {
    pub yaoci: String,
    pub xiaoxiang: String,
    #[serde(rename = "yaoci-explain")]
    pub yaoci_explain: Vec<String>,
    pub yaozhan: Vec<String>,
    pub cases: Vec<Case>,
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

pub fn get_gua_oracle(order: u8) -> Result<HexagramOracle> {
    let hexagram_oracle: HexagramOracle = serde_json::from_slice(HEXAGRAM_DATA[order as usize - 1])?;
    Ok(hexagram_oracle)
}

pub fn get_gua_oracle_md(order: u8) -> Result<String> {
    let hexagram = Hexagram::from_order(order);
    let ho = get_gua_oracle(order)?;

    let mut tmpl = Tera::default();
    tmpl.add_raw_template("OracleGua", ORACLE_GUA_TEMPLATE)?;

    let mut ctx = Context::new();
    ctx.insert("unicode", &hexagram.unicode.to_string());
    ctx.insert("long_name", &hexagram.long_name);
    ctx.insert("order", &hexagram.order.to_string());
    ctx.insert("summary", &ho.summary);
    ctx.insert("guaci", &ho.guaci);
    ctx.insert("guaci_explain", &ho.guaci_explain);
    ctx.insert("tuan", &ho.tuan);
    ctx.insert("tuan_explain", &ho.tuan_explain);
    ctx.insert("daxiang", &ho.daxiang);
    ctx.insert("daxiang_explain", &ho.daxiang_explain);
    ctx.insert("guazhan", &ho.guazhan);

    Ok(tmpl.render("OracleGua", &ctx)?)
}

pub fn get_yao_oracle_md(order: u8, yao: u8) -> Result<String> {
    let ho = get_gua_oracle(order)?;
    let yao = &ho.yaos[yao as usize - 1];

    let mut tmpl = Tera::default();
    tmpl.add_raw_template("OracleYao", ORACLE_YAO_TEMPLATE)?;

    let mut ctx = Context::new();
    ctx.insert("yaoci", &yao.yaoci);
    ctx.insert("xiaoxiang", &yao.xiaoxiang);
    ctx.insert("yaoci_explain", &yao.yaoci_explain);
    ctx.insert("yaozhan", &yao.yaozhan);
    ctx.insert("cases", &yao.cases);

    Ok(tmpl.render("OracleYao", &ctx)?)
}

pub fn get_yao_oracle(order: u8, yao: u8) -> Result<Yao> {
    let hexagram_oracle: HexagramOracle = serde_json::from_slice(HEXAGRAM_DATA[order as usize - 1])?;
    let yao_oracle: Yao = hexagram_oracle.yaos[yao as usize - 1].clone();
    Ok(yao_oracle)
}