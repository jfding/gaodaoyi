use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trigram {
    Qian = '☰' as isize,  // 乾 Heaven ䷀
    Dui = '☱' as isize,   // 兌 Lake ䷹
    Li = '☲' as isize,    // 離 Fire ䷝
    Zhen = '☳' as isize,  // 震 Thunder ䷲
    Xun = '☴' as isize,   // 巽 Wind ䷸
    Kan = '☵' as isize,   // 坎 Water ䷜
    Gen = '☶' as isize,   // 艮 Mountain ䷳
    Kun = '☷' as isize,   // 坤 Earth ䷁
}
impl Default for Trigram {
    fn default() -> Self {
        Trigram::Qian
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hexagram {
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
impl Display for Hexagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from_u32(self.clone() as u32).unwrap())
    }
}

pub fn get_hexagram(up: Trigram, down: Trigram) -> (Hexagram, String, usize) {
    let (hexagram, name) = match (up.clone(), down.clone()) {
        (Trigram::Qian, Trigram::Qian) => (Hexagram::Qian, "乾 Heaven"),
        (Trigram::Qian, Trigram::Dui) => (Hexagram::Lu, "履 Treading"),
        (Trigram::Qian, Trigram::Li) => (Hexagram::TongRen, "同人 Fellowship"),
        (Trigram::Qian, Trigram::Zhen) => (Hexagram::WuWang, "無妄 Innocence"),
        (Trigram::Qian, Trigram::Xun) => (Hexagram::Gou, "姤 Coming to Meet"),
        (Trigram::Qian, Trigram::Kan) => (Hexagram::Song, "訟 Conflict"),
        (Trigram::Qian, Trigram::Gen) => (Hexagram::Dun, "遯 Retreat"),
        (Trigram::Qian, Trigram::Kun) => (Hexagram::Pi, "否 Standstill"),

        (Trigram::Kun, Trigram::Qian) => (Hexagram::Tai, "泰 Peace"),
        (Trigram::Kun, Trigram::Dui) => (Hexagram::Lin, "臨 Approach"),
        (Trigram::Kun, Trigram::Li) => (Hexagram::MingYi, "明夷 Darkening of the Light"),
        (Trigram::Kun, Trigram::Zhen) => (Hexagram::Fu, "復 Return"),
        (Trigram::Kun, Trigram::Xun) => (Hexagram::Sheng, "升 Pushing Upward"),
        (Trigram::Kun, Trigram::Kan) => (Hexagram::Shi, "師 The Army"),
        (Trigram::Kun, Trigram::Gen) => (Hexagram::Qian2, "謙 Modesty"),
        (Trigram::Kun, Trigram::Kun) => (Hexagram::Kun, "坤 Earth"),

        (Trigram::Dui, Trigram::Qian) => (Hexagram::Guai, "夬 Breakthrough"),
        (Trigram::Dui, Trigram::Dui) => (Hexagram::Dui, "兌 Lake/Joyous"),
        (Trigram::Dui, Trigram::Li) => (Hexagram::Ge, "革 Revolution"),
        (Trigram::Dui, Trigram::Zhen) => (Hexagram::Sui, "隨 Following"),
        (Trigram::Dui, Trigram::Xun) => (Hexagram::DaGuo, "大過 Preponderance of the Great"),
        (Trigram::Dui, Trigram::Kan) => (Hexagram::Kun2, "困 Oppression"),
        (Trigram::Dui, Trigram::Gen) => (Hexagram::Xian, "咸 Influence"),
        (Trigram::Dui, Trigram::Kun) => (Hexagram::Cui, "萃 Gathering Together"),
        
        (Trigram::Li, Trigram::Qian) => (Hexagram::DaYou, "大有 Possession in Great Measure"),
        (Trigram::Li, Trigram::Dui) => (Hexagram::Kui, "睽 Opposition"),
        (Trigram::Li, Trigram::Li) => (Hexagram::Li, "離 Fire"),
        (Trigram::Li, Trigram::Zhen) => (Hexagram::ShiHe, "噬嗑 Biting Through"),
        (Trigram::Li, Trigram::Xun) => (Hexagram::Ding, "鼎 The Cauldron"),
        (Trigram::Li, Trigram::Kan) => (Hexagram::WeiJi, "未濟 Before Completion/Not Yet Fulfilled"),
        (Trigram::Li, Trigram::Gen) => (Hexagram::LvGua, "旅 The Wanderer"),
        (Trigram::Li, Trigram::Kun) => (Hexagram::Jin, "晉 Progress"),

        (Trigram::Zhen, Trigram::Qian) => (Hexagram::DaZhuang, "大壯 The Power of the Great"),
        (Trigram::Zhen, Trigram::Dui) => (Hexagram::GuiMei, "归妹 The Marrying Maiden"),
        (Trigram::Zhen, Trigram::Li) => (Hexagram::Feng, "豐 Abundance"),
        (Trigram::Zhen, Trigram::Zhen) => (Hexagram::Zhen, "震 Thunder"),
        (Trigram::Zhen, Trigram::Xun) => (Hexagram::Heng, "恆 Duration"),
        (Trigram::Zhen, Trigram::Kan) => (Hexagram::Jie, "解 Deliverance"),
        (Trigram::Zhen, Trigram::Gen) => (Hexagram::XiaoGuo, "小過 Small Exceeding/Small Preponderance"),
        (Trigram::Zhen, Trigram::Kun) => (Hexagram::Yu, "豫 Enthusiasm"),

        (Trigram::Xun, Trigram::Qian) => (Hexagram::XiaoXu, "小畜 The Taming Power of the Small"),
        (Trigram::Xun, Trigram::Dui) => (Hexagram::ZhongFu, "中孚 Inner Truth/Central Truth"),
        (Trigram::Xun, Trigram::Li) => (Hexagram::JiaRen, "家人 The Family"),
        (Trigram::Xun, Trigram::Zhen) => (Hexagram::Yi2, "益 Increase"),
        (Trigram::Xun, Trigram::Xun) => (Hexagram::Xun, "巽 Wind/Gentle"),
        (Trigram::Xun, Trigram::Kan) => (Hexagram::Huan, "涣 Dispersion"),
        (Trigram::Xun, Trigram::Gen) => (Hexagram::Jian2, "渐 Progress"),
        (Trigram::Xun, Trigram::Kun) => (Hexagram::Guan, "觀 Contemplation"),

        (Trigram::Kan, Trigram::Qian) => (Hexagram::Xu, "需 Waiting"),
        (Trigram::Kan, Trigram::Dui) => (Hexagram::Jie2, "節 Limitation/Restraint"),
        (Trigram::Kan, Trigram::Li) => (Hexagram::JiJi, "既濟 After Completion/Already Fulfilled"),
        (Trigram::Kan, Trigram::Zhen) => (Hexagram::Zhun, "屯 Difficulty at the Beginning"),
        (Trigram::Kan, Trigram::Xun) => (Hexagram::Jing, "井 The Well"),
        (Trigram::Kan, Trigram::Kan) => (Hexagram::Kan, "坎 Water"),
        (Trigram::Kan, Trigram::Gen) => (Hexagram::Jian, "蹇 Obstruction"),
        (Trigram::Kan, Trigram::Kun) => (Hexagram::Bi, "比 Holding Together"),

        (Trigram::Gen, Trigram::Qian) => (Hexagram::DaXu, "大畜 The Taming Power of the Great"),
        (Trigram::Gen, Trigram::Dui) => (Hexagram::Sun, "損 Decrease"),
        (Trigram::Gen, Trigram::Li) => (Hexagram::Bi2, "賁 Grace"),
        (Trigram::Gen, Trigram::Zhen) => (Hexagram::Yi, "頤 The Corners of the Mouth"),
        (Trigram::Gen, Trigram::Xun) => (Hexagram::Gu, "蠱 Work on the Decayed"),
        (Trigram::Gen, Trigram::Kan) => (Hexagram::Meng, "蒙 Youthful Folly"),
        (Trigram::Gen, Trigram::Gen) => (Hexagram::Gen, "艮 Mountain"),
        (Trigram::Gen, Trigram::Kun) => (Hexagram::Bo, "剝 Splitting Apart"),

        // Default case for any unmatched combination
        _ => {
            panic!("Invalid hexagram combination: {:?} {:?}", up, down);
        }
    };

    return (hexagram.clone(), name.to_string(), hexagram as usize - Hexagram::Qian as usize + 1);
}

pub fn get_trigram(number: u8) -> Trigram {
    match number {
        1 => Trigram::Qian,
        2 => Trigram::Dui,
        3 => Trigram::Li,
        4 => Trigram::Zhen,
        5 => Trigram::Xun,
        6 => Trigram::Kan,
        7 => Trigram::Gen,
        8 => Trigram::Kun,
        _ => panic!("Invalid trigram number")
    }
}

fn list_hexagrams() {
    for hexagram in [
    Hexagram::Qian,  // 乾 Heaven
    Hexagram::Kun,   // 坤 Earth
    Hexagram::Zhun,   // 屯 Difficulty at the Beginning
    Hexagram::Meng,   // 蒙 Youthful Folly
    Hexagram::Xu,     // 需 Waiting
    Hexagram::Song,   // 訟 Conflict
    Hexagram::Shi,    // 師 The Army
    Hexagram::Bi,     // 比 Holding Together

    // 9-16  
    Hexagram::XiaoXu, // 小畜 The Taming Power of the Small
    Hexagram::Lu,     // 履 Treading
    Hexagram::Tai,    // 泰 Peace
    Hexagram::Pi,     // 否 Standstill
    Hexagram::TongRen,// 同人 Fellowship
    Hexagram::DaYou,  // 大有 Possession in Great Measure
    Hexagram::Qian2,  // 謙 Modesty
    Hexagram::Yu,     // 豫 Enthusiasm

    // 17-24
    Hexagram::Sui,    // 隨 Following
    Hexagram::Gu,     // 蠱 Work on the Decayed
    Hexagram::Lin,    // 臨 Approach
    Hexagram::Guan,   // 觀 Contemplation
    Hexagram::ShiHe,  // 噬嗑 Biting Through
    Hexagram::Bi2,    // 賁 Grace
    Hexagram::Bo,     // 剝 Splitting Apart
    Hexagram::Fu,     // 復 Return

    // 25-32
    Hexagram::WuWang, // 無妄 Innocence
    Hexagram::DaXu,   // 大畜 The Taming Power of the Great
    Hexagram::Yi,     // 頤 The Corners of the Mouth
    Hexagram::DaGuo,  // 大過 Preponderance of the Great
    Hexagram::Kan,   // 坎 Water
    Hexagram::Li,    // 離 Fire
    Hexagram::Xian,// 咸 Influence
    Hexagram::Heng,   // 恆 Duration

    // 33-40
    Hexagram::Dun,    // 遯 Retreat
    Hexagram::DaZhuang,// 大壯 The Power of the Great
    Hexagram::Jin,    // 晉 Progress
    Hexagram::MingYi, // 明夷 Darkening of the Light
    Hexagram::JiaRen, // 家人 The Family
    Hexagram::Kui,    // 睽 Opposition
    Hexagram::Jian,   // 蹇 Obstruction
    Hexagram::Jie,    // 解 Deliverance

    // 41-48
    Hexagram::Sun,    // 損 Decrease
    Hexagram::Yi2,    // 益 Increase
    Hexagram::Guai,   // 夬 Breakthrough
    Hexagram::Gou,    // 姤 Coming to Meet
    Hexagram::Cui,    // 萃 Gathering Together
    Hexagram::Sheng,  // 升 Pushing Upward
    Hexagram::Kun2,   // 困 Oppression
    Hexagram::Jing,   // 井 The Well

    // 49-56
    Hexagram::Ge,     // 革 Revolution
    Hexagram::Ding,   // 鼎 The Cauldron
    Hexagram::Zhen,  // 震 Thunder
    Hexagram::Gen,   // 艮 Mountain
    Hexagram::Jian2, // 渐 Progress
    Hexagram::GuiMei,// 归妹 The Marrying Maiden
    Hexagram::Feng,   // 豐 Abundance
    Hexagram::LvGua,  // 旅 The Wanderer

    // 57-64
    Hexagram::Xun,   // 巽 Wind/Gentle
    Hexagram::Dui,   // 兌 Lake/Joyous
    Hexagram::Huan,  // 涣 Dispersion
    Hexagram::Jie2,   // 節 Limitation/Restraint
    Hexagram::ZhongFu,// 中孚 Inner Truth/Central Truth
    Hexagram::XiaoGuo,// 小過 Small Exceeding/Small Preponderance
    Hexagram::JiJi,   // 既濟 After Completion/Already Fulfilled
    Hexagram::WeiJi,  // 未濟 Before Completion/Not Yet Fulfilled

    ].iter() {
        println!("{:?} = {}", hexagram, hexagram.clone() as u32);
        println!("As number: {}", hexagram.clone() as isize);
        println!("From number back to char: {}", char::from_u32(hexagram.clone() as u32).unwrap());
    }
}