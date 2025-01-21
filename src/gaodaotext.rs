use serde::{Deserialize, Serialize};
use anyhow::Result;
use tera::{Tera, Context};
use crate::gram::Hexagram;

const HEXAGRAM_GLYPHS_NULL: [(&str, &[u8]); 3] = [
    ("", &[]),
    ("", &[]),
    ("", &[]),
];

const HEXAGRAM_GLYPHS_01: [(&str, &[u8]); 3] = [
    ("甲骨文元", include_bytes!("../assets/images/gd01_甲骨文元.jpg")),
    ("甲骨文利", include_bytes!("../assets/images/gd01_甲骨文利.jpg")),
    ("甲骨文贞", include_bytes!("../assets/images/gd01_甲骨文贞.jpg")),
];
const HEXAGRAM_GLYPHS_02: [(&str, &[u8]); 3] = [
    ("古文“坤”字", include_bytes!("../assets/images/gd02_古文“坤”字.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_03: [(&str, &[u8]); 3] = [
    ("甲骨文屯", include_bytes!("../assets/images/gd03_甲骨文屯.jpg")),
    ("篆体屯", include_bytes!("../assets/images/gd03_篆体屯.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_04: [(&str, &[u8]); 3] = [
    ("甲骨文蒙", include_bytes!("../assets/images/gd04_甲骨文蒙.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_05: [(&str, &[u8]); 3] = [
    ("金文需", include_bytes!("../assets/images/gd05_金文需.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_07: [(&str, &[u8]); 3] = [
    ("篆书师", include_bytes!("../assets/images/gd07_篆书师.jpg")),
    ("金文师", include_bytes!("../assets/images/gd07_金文师.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_08: [(&str, &[u8]); 3] = [
    ("甲骨文比", include_bytes!("../assets/images/gd08_甲骨文比.jpg")),
    ("篆书比", include_bytes!("../assets/images/gd08_篆书比.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_09: [(&str, &[u8]); 3] = [
    ("甲骨文畜", include_bytes!("../assets/images/gd09_甲骨文畜.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_10: [(&str, &[u8]); 3] = [
    ("篆书履", include_bytes!("../assets/images/gd10_篆书履.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_11: [(&str, &[u8]); 3] = [
    ("篆书泰", include_bytes!("../assets/images/gd11_篆书泰.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_12: [(&str, &[u8]); 3] = [
    ("篆书否", include_bytes!("../assets/images/gd12_篆书否.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_15: [(&str, &[u8]); 3] = [
    ("篆书谦", include_bytes!("../assets/images/gd15_篆书谦.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_16: [(&str, &[u8]); 3] = [
    ("甲骨文豫", include_bytes!("../assets/images/gd16_甲骨文豫.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_17: [(&str, &[u8]); 3] = [
    ("篆书随", include_bytes!("../assets/images/gd17_篆书随.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_18: [(&str, &[u8]); 3] = [
    ("甲骨文蛊", include_bytes!("../assets/images/gd18_甲骨文蛊.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_19: [(&str, &[u8]); 3] = [
    ("金文临", include_bytes!("../assets/images/gd19_金文临.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_20: [(&str, &[u8]); 3] = [
    ("甲骨文观", include_bytes!("../assets/images/gd20_甲骨文观.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_21: [(&str, &[u8]); 3] = [
    ("篆书嗑", include_bytes!("../assets/images/gd21_篆书嗑.jpg")),
    ("篆书噬", include_bytes!("../assets/images/gd21_篆书噬.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_22: [(&str, &[u8]); 3] = [
    ("篆书贲", include_bytes!("../assets/images/gd22_篆书贲.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_23: [(&str, &[u8]); 3] = [
    ("篆书剥", include_bytes!("../assets/images/gd23_篆书剥.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_24: [(&str, &[u8]); 3] = [
    ("甲骨文复", include_bytes!("../assets/images/gd24_甲骨文复.jpg")),
    ("篆书复", include_bytes!("../assets/images/gd24_篆书复.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_27: [(&str, &[u8]); 3] = [
    ("篆书颐", include_bytes!("../assets/images/gd27_篆书颐.jpg")),
    ("金文颐", include_bytes!("../assets/images/gd27_金文颐.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_29: [(&str, &[u8]); 3] = [
    ("篆书坎", include_bytes!("../assets/images/gd29_篆书坎.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_30: [(&str, &[u8]); 3] = [
    ("篆书离", include_bytes!("../assets/images/gd30_篆书离.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_31: [(&str, &[u8]); 3] = [
    ("甲骨文咸", include_bytes!("../assets/images/gd31_甲骨文咸.jpg")),
    ("金文咸", include_bytes!("../assets/images/gd31_金文咸.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_32: [(&str, &[u8]); 3] = [
    ("甲骨文恒", include_bytes!("../assets/images/gd32_甲骨文恒.jpg")),
    ("篆书恒", include_bytes!("../assets/images/gd32_篆书恒.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_35: [(&str, &[u8]); 3] = [
    ("金文晋", include_bytes!("../assets/images/gd35_金文晋.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_38: [(&str, &[u8]); 3] = [
    ("金文睽", include_bytes!("../assets/images/gd38_金文睽.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_39: [(&str, &[u8]); 3] = [
    ("篆书蹇", include_bytes!("../assets/images/gd39_篆书蹇.jpg")),
    ("金文蹇", include_bytes!("../assets/images/gd39_金文蹇.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_40: [(&str, &[u8]); 3] = [
    ("甲骨文解", include_bytes!("../assets/images/gd40_甲骨文解.jpg")),
    ("篆书解", include_bytes!("../assets/images/gd40_篆书解.jpg")),
    ("金文解", include_bytes!("../assets/images/gd40_金文解.jpg")),
];
const HEXAGRAM_GLYPHS_42: [(&str, &[u8]); 3] = [
    ("甲骨文益", include_bytes!("../assets/images/gd42_甲骨文益.jpg")),
    ("篆书益", include_bytes!("../assets/images/gd42_篆书益.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_43: [(&str, &[u8]); 3] = [
    ("篆书夬", include_bytes!("../assets/images/gd43_篆书夬.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_45: [(&str, &[u8]); 3] = [
    ("篆书萃", include_bytes!("../assets/images/gd45_篆书萃.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_46: [(&str, &[u8]); 3] = [
    ("篆书升", include_bytes!("../assets/images/gd46_篆书升.jpg")),
    ("金文升", include_bytes!("../assets/images/gd46_金文升.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_47: [(&str, &[u8]); 3] = [
    ("甲骨文困", include_bytes!("../assets/images/gd47_甲骨文困.jpg")),
    ("篆书困", include_bytes!("../assets/images/gd47_篆书困.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_48: [(&str, &[u8]); 3] = [
    ("金文井", include_bytes!("../assets/images/gd48_金文井.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_49: [(&str, &[u8]); 3] = [
    ("金文革", include_bytes!("../assets/images/gd49_金文革.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_50: [(&str, &[u8]); 3] = [
    ("甲骨文鼎", include_bytes!("../assets/images/gd50_甲骨文鼎.jpg")),
    ("金文鼎", include_bytes!("../assets/images/gd50_金文鼎.jpg")),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_55: [(&str, &[u8]); 3] = [
    ("金文丰", include_bytes!("../assets/images/gd55_金文丰.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_56: [(&str, &[u8]); 3] = [
    ("甲骨文旅", include_bytes!("../assets/images/gd56_甲骨文旅.jpg")),
    ("篆书旅", include_bytes!("../assets/images/gd56_篆书旅.jpg")),
    ("金文旅", include_bytes!("../assets/images/gd56_金文旅.jpg")),
];
const HEXAGRAM_GLYPHS_57: [(&str, &[u8]); 3] = [
    ("篆书巽", include_bytes!("../assets/images/gd57_篆书巽.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_58: [(&str, &[u8]); 3] = [
    ("甲骨文兑", include_bytes!("../assets/images/gd58_甲骨文兑.jpg")),
    ("", &[]),
    ("", &[]),
];
const HEXAGRAM_GLYPHS_60: [(&str, &[u8]); 3] = [
    ("金文节", include_bytes!("../assets/images/gd60_金文节.jpg")),
    ("", &[]),
    ("", &[]),
];
const ALL_HEXAGRAM_GLYPHS: [[(&str, &[u8]); 3]; 64] = [
    HEXAGRAM_GLYPHS_01,
    HEXAGRAM_GLYPHS_02,
    HEXAGRAM_GLYPHS_03,
    HEXAGRAM_GLYPHS_04,
    HEXAGRAM_GLYPHS_05,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_07,
    HEXAGRAM_GLYPHS_08,
    HEXAGRAM_GLYPHS_09,
    HEXAGRAM_GLYPHS_10,
    HEXAGRAM_GLYPHS_11,
    HEXAGRAM_GLYPHS_12,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_15,
    HEXAGRAM_GLYPHS_16,
    HEXAGRAM_GLYPHS_17,
    HEXAGRAM_GLYPHS_18,
    HEXAGRAM_GLYPHS_19,
    HEXAGRAM_GLYPHS_20,
    HEXAGRAM_GLYPHS_21,
    HEXAGRAM_GLYPHS_22,
    HEXAGRAM_GLYPHS_23,
    HEXAGRAM_GLYPHS_24,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_27,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_29,
    HEXAGRAM_GLYPHS_30,
    HEXAGRAM_GLYPHS_31,
    HEXAGRAM_GLYPHS_32,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_35,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_38,
    HEXAGRAM_GLYPHS_39,
    HEXAGRAM_GLYPHS_40,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_42,
    HEXAGRAM_GLYPHS_43,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_45,
    HEXAGRAM_GLYPHS_46,
    HEXAGRAM_GLYPHS_47,
    HEXAGRAM_GLYPHS_48,
    HEXAGRAM_GLYPHS_49,
    HEXAGRAM_GLYPHS_50,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_55,
    HEXAGRAM_GLYPHS_56,
    HEXAGRAM_GLYPHS_57,
    HEXAGRAM_GLYPHS_58,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_60,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
    HEXAGRAM_GLYPHS_NULL,
];

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
    #[serde(rename = "Q")]
    pub q: String,
    #[serde(rename = "A")]
    pub a: Vec<String>,
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

#[allow(dead_code)]
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

pub fn get_gua_oracle_json(hexagram: &Hexagram) -> Result<String> {
    let order = hexagram.order;
    Ok(String::from_utf8_lossy(HEXAGRAM_DATA[order as usize - 1]).into_owned())
}

pub fn get_gua_oracle(hexagram: &Hexagram) -> Result<HexagramOracle> {
    let order = hexagram.order;
    let hexagram_oracle: HexagramOracle = serde_json::from_slice(HEXAGRAM_DATA[order as usize - 1])?;
    Ok(hexagram_oracle)
}

pub fn get_gua_glyphs(hexagram: &Hexagram) -> Vec<(&str, &[u8])> {
    ALL_HEXAGRAM_GLYPHS[hexagram.order as usize - 1].to_vec()
}

pub fn get_gua_oracle_md(hexagram: &Hexagram) -> Result<String> {
    let ho = get_gua_oracle(hexagram)?;

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

pub fn get_yao_oracle(hexagram: &Hexagram, yao: u8) -> Result<Yao> {
    let ho = get_gua_oracle(hexagram)?;
    let yao_oracle: Yao = ho.yaos[yao as usize - 1].clone();
    Ok(yao_oracle)
}

pub fn get_yao_oracle_md(hexagram: &Hexagram, yao: u8) -> Result<String> {
    let yao_oracle = get_yao_oracle(hexagram, yao)?;
    let mut tmpl = Tera::default();
    tmpl.add_raw_template("OracleYao", ORACLE_YAO_TEMPLATE)?;

    let mut ctx = Context::new();
    ctx.insert("yaoci", &yao_oracle.yaoci);
    ctx.insert("xiaoxiang", &yao_oracle.xiaoxiang);
    ctx.insert("yaoci_explain", &yao_oracle.yaoci_explain);
    ctx.insert("yaozhan", &yao_oracle.yaozhan);
    ctx.insert("cases", &yao_oracle.cases);

    Ok(tmpl.render("OracleYao", &ctx)?)
}