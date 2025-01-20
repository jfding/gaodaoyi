use std::fmt::Display;

const TRIGRAM_NAMES: [(&str, &str, &str); 8] = [
    ("乾", "Qian", "Heaven"),
    ("兌", "Dui", "Lake"),
    ("離", "Li", "Fire"),
    ("震", "Zhen", "Thunder"),
    ("巽", "Xun", "Wind"),
    ("坎", "Kan", "Water"),
    ("艮", "Gen", "Mountain"),
    ("坤", "Kun", "Earth"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrigramName {
    Qian = '☰' as isize,  // 乾 Heaven
    Dui = '☱' as isize,   // 兌 Lake
    Li = '☲' as isize,    // 離 Fire
    Zhen = '☳' as isize,  // 震 Thunder
    Xun = '☴' as isize,   // 巽 Wind
    Kan = '☵' as isize,   // 坎 Water
    Gen = '☶' as isize,   // 艮 Mountain
    Kun = '☷' as isize,   // 坤 Earth
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Trigram {
    pub name: TrigramName,
    pub unicode: char,
    pub order: u8,
    pub cn_name: String,
    pub py_name: String,
    pub meaning: String,
}
impl Default for Trigram {
    fn default() -> Self {
        Trigram::from_name(TrigramName::Qian)
    }
}
impl Trigram {
    pub fn from_name(name: TrigramName) -> Self {
        let order = name as u32 - TrigramName::Qian as u32 + 1;
        Trigram {
            name,
            unicode: char::from_u32(name as u32).unwrap(),
            order: order as u8,
            cn_name: TRIGRAM_NAMES[order as usize - 1].0.to_string(),
            py_name: TRIGRAM_NAMES[order as usize - 1].1.to_string(),
            meaning: TRIGRAM_NAMES[order as usize - 1].2.to_string(),
        }
    }
    pub fn from_order(order: u8) -> Trigram {
        let name = match order {
            1 => TrigramName::Qian,
            2 => TrigramName::Dui,
            3 => TrigramName::Li,
            4 => TrigramName::Zhen,
            5 => TrigramName::Xun,
            6 => TrigramName::Kan,
            7 => TrigramName::Gen,
            8 => TrigramName::Kun,
            _ => panic!("Invalid trigram number")
        };
        Trigram::from_name(name)
    }

    // get the trigram after flipping the specified yao
    pub fn get_change(&self, yao: u8) -> Self {
        if yao > 3 {
            panic!("Invalid yao number");
        }
        let yao_index :usize = yao as usize - 1;

        let yao_binary = format!("{:03b}", self.order-1);
        let mut yao_binary_vec = yao_binary.chars().collect::<Vec<char>>();
        yao_binary_vec[yao_index] = if yao_binary_vec[yao_index] == '0' { '1' } else { '0' };
        let yao_binary = yao_binary_vec.iter().collect::<String>();
        let yao_number = u8::from_str_radix(&yao_binary, 2).unwrap() + 1;

        Trigram::from_order(yao_number)
    }
}

#[derive(Debug, Clone, Copy,PartialEq, Eq)]
pub enum HexagramName {
    // 1-8
    Qian = '䷀' as isize,  // 乾 Heaven
    Kun = '䷁' as isize,   // 坤 Earth
    Zhun = '䷂' as isize,   // 屯 Difficulty at the Beginning
    Meng = '䷃' as isize,   // 蒙 Youthful Folly
    Xu = '䷄' as isize,     // 需 Waiting
    Song = '䷅' as isize,   // 訟 Conflict
    Shi = '䷆' as isize,    // 師 The Army
    Bi = '䷇' as isize,     // 比 Holding Together

    // 9-16  
    XiaoXu = '䷈' as isize, // 小畜 The Taming Power of the Small
    Lu = '䷉' as isize,     // 履 Treading
    Tai = '䷊' as isize,    // 泰 Peace
    Pi = '䷋' as isize,     // 否 Standstill
    TongRen = '䷌' as isize,// 同人 Fellowship
    DaYou = '䷍' as isize,  // 大有 Possession in Great Measure
    Qian2 = '䷎' as isize,  // 謙 Modesty
    Yu = '䷏' as isize,     // 豫 Enthusiasm

    // 17-24
    Sui = '䷐' as isize,    // 隨 Following
    Gu = '䷑' as isize,     // 蠱 Work on the Decayed
    Lin = '䷒' as isize,    // 臨 Approach
    Guan = '䷓' as isize,   // 觀 Contemplation
    ShiHe = '䷔' as isize,  // 噬嗑 Biting Through
    Bi2 = '䷕' as isize,    // 賁 Grace
    Bo = '䷖' as isize,     // 剝 Splitting Apart
    Fu = '䷗' as isize,     // 復 Return

    // 25-32
    WuWang = '䷘' as isize, // 無妄 Innocence
    DaXu = '䷙' as isize,   // 大畜 The Taming Power of the Great
    Yi = '䷚' as isize,     // 頤 The Corners of the Mouth
    DaGuo = '䷛' as isize,  // 大過 Preponderance of the Great
    Kan = '䷜' as isize,   // 坎 Water
    Li = '䷝' as isize,    // 離 Fire
    Xian= '䷞' as isize,// 咸 Influence
    Heng = '䷟' as isize,   // 恆 Duration

    // 33-40
    Dun = '䷠' as isize,    // 遯 Retreat
    DaZhuang = '䷡' as isize,// 大壯 The Power of the Great
    Jin = '䷢' as isize,    // 晉 Progress
    MingYi = '䷣' as isize, // 明夷 Darkening of the Light
    JiaRen = '䷤' as isize, // 家人 The Family
    Kui = '䷥' as isize,    // 睽 Opposition
    Jian = '䷦' as isize,   // 蹇 Obstruction
    Jie = '䷧' as isize,    // 解 Deliverance

    // 41-48
    Sun = '䷨' as isize,    // 損 Decrease
    Yi2 = '䷩' as isize,    // 益 Increase
    Guai = '䷪' as isize,   // 夬 Breakthrough
    Gou = '䷫' as isize,    // 姤 Coming to Meet
    Cui = '䷬' as isize,    // 萃 Gathering Together
    Sheng = '䷭' as isize,  // 升 Pushing Upward
    Kun2 = '䷮' as isize,   // 困 Oppression
    Jing = '䷯' as isize,   // 井 The Well

    // 49-56
    Ge = '䷰' as isize,     // 革 Revolution
    Ding = '䷱' as isize,   // 鼎 The Cauldron
    Zhen = '䷲' as isize,  // 震 Thunder
    Gen = '䷳' as isize,   // 艮 Mountain
    Jian2 = '䷴' as isize, // 渐 Progress
    GuiMei = '䷵' as isize,// 归妹 The Marrying Maiden
    Feng = '䷶' as isize,   // 豐 Abundance
    LvGua = '䷷' as isize,  // 旅 The Wanderer

    // 57-64
    Xun = '䷸' as isize,   // 巽 Wind/Gentle
    Dui = '䷹' as isize,   // 兌 Lake/Joyous
    Huan = '䷺' as isize,  // 涣 Dispersion
    Jie2 = '䷻' as isize,   // 節 Limitation/Restraint
    ZhongFu = '䷼' as isize,// 中孚 Inner Truth/Central Truth
    XiaoGuo = '䷽' as isize,// 小過 Small Exceeding/Small Preponderance
    JiJi = '䷾' as isize,   // 既濟 After Completion/Already Fulfilled
    WeiJi = '䷿' as isize,  // 未濟 Before Completion/Not Yet Fulfilled
}
impl Display for HexagramName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from_u32(*self as u32).unwrap())
    }
}

const HEXAGRAM_NAMES: [(&str, &str, &str); 64] = [
    ("乾", "乾為天", "Heaven"),
    ("坤", "坤為地", "Earth"),
    ("屯", "水雷屯", "Difficulty at the Beginning"),
    ("蒙", "山水蒙", "Youthful Folly"),
    ("需", "水天需", "Waiting"),
    ("訟", "天水訟", "Conflict"),
    ("師", "地水師", "The Army"),
    ("比", "水地比", "Holding Together"),

    // 9-16  
    ("小畜", "風天小畜", "The Taming Power of the Small"),
    ("履", "天澤履", "Treading"),
    ("泰", "地天泰", "Peace"),
    ("否", "天地否", "Standstill"),
    ("同人", "天火同人", "Fellowship"),
    ("大有", "火天大有", "Possession in Great Measure"),
    ("謙", "地山謙", "Modesty"),
    ("豫", "雷地豫", "Enthusiasm"),

    // 17-24
    ("隨", "泽雷隨", "Following"),
    ("蠱", "山風蠱", "Work on the Decayed"),
    ("臨", "地澤臨", "Approach"),
    ("觀", "風地觀", "Contemplation"),
    ("噬嗑", "火雷噬嗑", "Biting Through"),
    ("賁", "山火賁", "Grace"),
    ("剝", "山地剝", "Splitting Apart"),
    ("復", "地雷復", "Return"),

    // 25-32
    ("無妄", "天雷無妄", "Innocence"),
    ("大畜", "山天大畜", "The Taming Power of the Great"),
    ("頤", "山雷頤", "The Corners of the Mouth"),
    ("大過", "澤風大過", "Preponderance of the Great"),
    ("坎", "坎為水", "Water"),
    ("離", "離為火", "Fire"),
    ("咸", "澤山咸", "Influence"),
    ("恆", "雷風恆", "Duration"),

    // 33-40
    ("遯", "天山遯", "Retreat"),
    ("大壯", "雷天大壯", "The Power of the Great"),
    ("晉", "水火晉", "Progress"),
    ("明夷", "地火明夷", "Darkening of the Light"),
    ("家人", "風火家人", "The Family"),
    ("睽", "火澤睽", "Opposition"),
    ("蹇", "水山蹇", "Obstruction"),
    ("解", "雷水解", "Deliverance"),

    // 41-48
    ("損", "山澤損", "Decrease"),
    ("益", "風雷益", "Increase"),
    ("夬", "澤天夬", "Breakthrough"),
    ("姤", "天風姤", "Coming to Meet"),
    ("萃", "澤地萃", "Gathering Together"),
    ("升", "地風升", "Pushing Upward"),
    ("困", "澤水困", "Oppression"),
    ("井", "水風井", "The Well"),

    // 49-56
    ("革", "泽火革", "Revolution"),
    ("鼎", "火風鼎", "The Cauldron"),
    ("震", "震為雷", "Thunder"),
    ("艮", "艮為山", "Mountain"),
    ("渐", "風山渐", "Progress"),
    ("归妹", "雷澤归妹", "The Marrying Maiden"),
    ("豐", "雷火豐", "Abundance"),
    ("旅", "火山旅", "The Wanderer"),

    // 57-64
    ("巽", "巽為風", "Wind/Gentle"),
    ("兌", "兑為澤", "Lake/Joyous"),
    ("涣", "風水涣", "Dispersion"),
    ("節", "水澤节", "Limitation/Restraint"),
    ("中孚", "風澤中孚", "Inner Truth/Central Truth"),
    ("小過", "雷山小過", "Small Exceeding/Small Preponderance"),
    ("既濟", "水火既濟", "After Completion/Already Fulfilled"),
    ("未濟", "火水未濟", "Before Completion/Not Yet Fulfilled"),
];

const HEXAGRAM_UPDOWN: [(TrigramName, TrigramName); 64] = [
    (TrigramName::Qian, TrigramName::Qian),
    (TrigramName::Kun, TrigramName::Kun),
    (TrigramName::Kan, TrigramName::Zhen),
    (TrigramName::Gen, TrigramName::Kan),
    (TrigramName::Kan, TrigramName::Qian),
    (TrigramName::Qian, TrigramName::Kan),
    (TrigramName::Kun, TrigramName::Kan),
    (TrigramName::Kan, TrigramName::Kun),
    (TrigramName::Xun, TrigramName::Qian),
    (TrigramName::Qian, TrigramName::Dui),
    (TrigramName::Kun, TrigramName::Qian),
    (TrigramName::Qian, TrigramName::Kun),
    (TrigramName::Qian, TrigramName::Li),
    (TrigramName::Li, TrigramName::Qian),
    (TrigramName::Kun, TrigramName::Gen),
    (TrigramName::Zhen, TrigramName::Kun),
    (TrigramName::Dui, TrigramName::Zhen),
    (TrigramName::Gen, TrigramName::Xun),
    (TrigramName::Kun, TrigramName::Dui),
    (TrigramName::Xun, TrigramName::Kun),
    (TrigramName::Li, TrigramName::Zhen),
    (TrigramName::Gen, TrigramName::Li),
    (TrigramName::Gen, TrigramName::Kun),
    (TrigramName::Kun, TrigramName::Zhen),
    (TrigramName::Qian, TrigramName::Zhen),
    (TrigramName::Gen, TrigramName::Qian),
    (TrigramName::Gen, TrigramName::Zhen),
    (TrigramName::Dui, TrigramName::Xun),
    (TrigramName::Kan, TrigramName::Kan),
    (TrigramName::Li, TrigramName::Li),
    (TrigramName::Dui, TrigramName::Gen),
    (TrigramName::Zhen, TrigramName::Xun),
    (TrigramName::Qian, TrigramName::Gen),
    (TrigramName::Zhen, TrigramName::Qian),
    (TrigramName::Li, TrigramName::Kun),
    (TrigramName::Kun, TrigramName::Li),
    (TrigramName::Xun, TrigramName::Li),
    (TrigramName::Li, TrigramName::Dui),
    (TrigramName::Kan, TrigramName::Gen),
    (TrigramName::Zhen, TrigramName::Kan),
    (TrigramName::Gen, TrigramName::Dui),
    (TrigramName::Xun, TrigramName::Zhen),
    (TrigramName::Dui, TrigramName::Qian),
    (TrigramName::Qian, TrigramName::Xun),
    (TrigramName::Dui, TrigramName::Kun),
    (TrigramName::Kun, TrigramName::Xun),
    (TrigramName::Dui, TrigramName::Kan),
    (TrigramName::Kan, TrigramName::Xun),
    (TrigramName::Dui, TrigramName::Li),
    (TrigramName::Li, TrigramName::Xun),
    (TrigramName::Zhen, TrigramName::Zhen),
    (TrigramName::Gen, TrigramName::Gen),
    (TrigramName::Xun, TrigramName::Gen),
    (TrigramName::Zhen, TrigramName::Dui),
    (TrigramName::Zhen, TrigramName::Li),
    (TrigramName::Li, TrigramName::Gen),
    (TrigramName::Xun, TrigramName::Xun),
    (TrigramName::Dui, TrigramName::Dui),
    (TrigramName::Xun, TrigramName::Kan),
    (TrigramName::Kan, TrigramName::Dui),
    (TrigramName::Xun, TrigramName::Dui),
    (TrigramName::Zhen, TrigramName::Gen),
    (TrigramName::Kan, TrigramName::Li),
    (TrigramName::Li, TrigramName::Kan),
];

/* used to be used to generate hexagrams from up and down trigrams
pub fn list_hexagrams() {
    for order in 1..=64 {
        let hexagram = Hexagram::from_order(order);
        for up in 1..=8 {
            for down in 1..=8 {
                let up_trigram = Trigram::from_order(up);
                let down_trigram = Trigram::from_order(down);
                let (f_hexagram, name, order) = get_hexagram(up_trigram.name, down_trigram.name);
                if f_hexagram == hexagram.name {
                    println!("(Trigram::{}, Trigram::{}),", up_trigram.py_name, down_trigram.py_name);
                    break;
                }
            }
        }
    }
}
*/

#[allow(dead_code)]
#[derive(Debug)]
pub struct Hexagram {
    pub name: HexagramName,
    pub unicode: char,
    pub order: u8,
    pub cn_name: String,
    pub long_name: String,
    pub meaning: String,
    pub up: Trigram,
    pub down: Trigram,
}
impl Hexagram {
    pub fn from_name(name: HexagramName) -> Self {
        let order = name as u32 - HexagramName::Qian as u32 + 1;
        Hexagram {
            name,
            unicode: char::from_u32(name as u32).unwrap(),
            order: order as u8,
            cn_name: HEXAGRAM_NAMES[order as usize - 1].0.to_string(),
            long_name: HEXAGRAM_NAMES[order as usize - 1].1.to_string(),
            meaning: HEXAGRAM_NAMES[order as usize - 1].2.to_string(),
            up: Trigram::from_name(HEXAGRAM_UPDOWN[order as usize - 1].0),
            down: Trigram::from_name(HEXAGRAM_UPDOWN[order as usize - 1].1),
        }
    }
    pub fn from_order(order: u8) -> Self {
        let name = match order {
            1 => HexagramName::Qian,
            2 => HexagramName::Kun,
            3 => HexagramName::Zhun,
            4 => HexagramName::Meng,
            5 => HexagramName::Xu,
            6 => HexagramName::Song,
            7 => HexagramName::Shi,
            8 => HexagramName::Bi,
            9 => HexagramName::XiaoXu,
            10 => HexagramName::Lu,
            11 => HexagramName::Tai,
            12 => HexagramName::Pi,
            13 => HexagramName::TongRen,
            14 => HexagramName::DaYou,
            15 => HexagramName::Qian2,
            16 => HexagramName::Yu, 
            17 => HexagramName::Sui,
            18 => HexagramName::Gu,
            19 => HexagramName::Lin,
            20 => HexagramName::Guan,   
            21 => HexagramName::ShiHe,
            22 => HexagramName::Bi2,
            23 => HexagramName::Bo,
            24 => HexagramName::Fu, 
            25 => HexagramName::WuWang,
            26 => HexagramName::DaXu,
            27 => HexagramName::Yi,
            28 => HexagramName::DaGuo,
            29 => HexagramName::Kan,
            30 => HexagramName::Li, 
            31 => HexagramName::Xian,
            32 => HexagramName::Heng,
            33 => HexagramName::Dun,
            34 => HexagramName::DaZhuang,
            35 => HexagramName::Jin,    
            36 => HexagramName::MingYi,
            37 => HexagramName::JiaRen,
            38 => HexagramName::Kui,
            39 => HexagramName::Jian,
            40 => HexagramName::Jie,    
            41 => HexagramName::Sun,
            42 => HexagramName::Yi2,
            43 => HexagramName::Guai,
            44 => HexagramName::Gou,
            45 => HexagramName::Cui,
            46 => HexagramName::Sheng,
            47 => HexagramName::Kun2,
            48 => HexagramName::Jing,   
            49 => HexagramName::Ge,
            50 => HexagramName::Ding,
            51 => HexagramName::Zhen,
            52 => HexagramName::Gen,
            53 => HexagramName::Jian2,
            54 => HexagramName::GuiMei,
            55 => HexagramName::Feng,
            56 => HexagramName::LvGua,
            57 => HexagramName::Xun,
            58 => HexagramName::Dui,
            59 => HexagramName::Huan,
            60 => HexagramName::Jie2,
            61 => HexagramName::ZhongFu,
            62 => HexagramName::XiaoGuo,
            63 => HexagramName::JiJi,
            64 => HexagramName::WeiJi,  
            _ => panic!("Invalid hexagram number")
        };
        Hexagram::from_name(name)
    }

    pub fn from_up_down(up: Trigram, down: Trigram) -> Self {
        let order = HEXAGRAM_UPDOWN.iter().position(|(u, d)| *u == up.name && *d == down.name).unwrap();
        Hexagram::from_order(order as u8 + 1)
    }   

    // get the hexagram after flipping the specified yao
    pub fn get_change(&self, yao: u8) -> Self {
        if yao <= 3 {
            Hexagram::from_up_down(self.up.clone(), self.down.get_change(yao))
        } else if yao <= 6 {
            Hexagram::from_up_down(self.up.get_change(yao-3), self.down.clone())
        } else {
            panic!("Invalid yao number");
        }
    }

    // list all hexagrams
    pub fn list_all() {
        for order in 1..=64 {
            let hexagram = Hexagram::from_order(order);
            println!("({}{} / {}{}) => [{:02}] {} {}",
                hexagram.up.unicode,
                hexagram.up.cn_name,
                hexagram.down.unicode,
                hexagram.down.cn_name,
                hexagram.order,
                hexagram.unicode,
                hexagram.long_name);
        }
    }

}
