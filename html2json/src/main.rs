use anyhow::Result;
use clap::Parser;
use scraper::{Html, Selector, ElementRef};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Input HTML file
    #[arg(short, long)]
    inputdir: Option<PathBuf>,

    /// Output JSON file
    #[arg(short, long)]
    outputdir: Option<PathBuf>,
}

#[derive(Debug)]
enum Step {
    Pre,
    Title,
    Summary,
    Guaci,
    GuaciExplain,
    Tuan,
    TuanExplain,
    Daxiang,
    DaxiangExplain,
    Guazhan,
    Yaoci,
    Yaoci_Xiaoxiang,
    Yaoci_Explain,
    Yaoci_Zhan,
    Yaoci_CaseQ,
    Yaoci_CaseA,
}

#[derive(Default, Serialize)]
struct Case {
    Q: String,
    A: Vec<String>,
}

#[derive(Default, Serialize)]
struct Yao {
    yaoci: String,
    xiaoxiang: String,
    #[serde(rename = "yaoci-explain")]
    yaoci_explain: Vec<String>,
    yaozhan: Vec<String>,
    cases: Vec<Case>,
}

#[derive(Serialize, Default)]
struct Hexagram {
    order: String,
    name: String,
    summary: String,
    guaci: String,
    #[serde(rename = "guaci-explain")]
    guaci_explain: Vec<String>,
    tuan: String,
    #[serde(rename = "tuan-explain")]
    tuan_explain: Vec<String>,
    daxiang: String,
    #[serde(rename = "daxiang-explain")]
    daxiang_explain: Vec<String>,
    guazhan: Vec<String>,
    glyphs: Vec<String>,

    yaos: Vec<Yao>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let cwd = PathBuf::from(".");
    let inputdir = args.inputdir.unwrap_or(cwd.clone());
    let outputdir = args.outputdir.unwrap_or(cwd);

    if !outputdir.exists() {
        fs::create_dir_all(outputdir.clone())?;
    }

    let mut files = glob::glob(inputdir.join("Section*.html").to_str().unwrap())?;
    while let Some(file) = files.next() {
        let path = file.unwrap();
        if path.is_file() {
            println!("Processing: {:?}", path);
            convert_one(&inputdir, &path, &outputdir)?;
        }
    }

    Ok(())
}
fn convert_one(inputdir: &PathBuf, input: &PathBuf, outputdir: &PathBuf) -> Result<()> {
    // Read HTML file
    let html_content = fs::read_to_string(input)?;
    let document = Html::parse_document(&html_content);
    
    // Build hexagram structure
    let mut hexagram = Hexagram::default();

    parse_hexagram(&document, &mut hexagram);
    extract_glyphs(inputdir, outputdir, &document, &mut hexagram);

    // Write to JSON file
    let json = serde_json::to_string_pretty(&hexagram)?;

    let output = outputdir.join(format!("gd{}.json", hexagram.order));
    fs::write(output, json)?;

    Ok(())
}

fn parse_hexagram(document: &Html, hexagram: &mut Hexagram) {
    // Implementation
    let p_selector = Selector::parse("p").unwrap();

    #[inline]
    fn _text(part: &ElementRef) -> String {
        let mut text = part.text().collect::<String>().trim().to_string();
        let re = regex::Regex::new(r"\s*［\d+］\s*").unwrap();
        text = re.replace_all(&text, "").to_string();
        text = text.replace("【占】　　", "");
        text = text.replace("【占】　", "");
        text = text.replace("○　", "");
        text = text.replace("\n ", "◻️"); // ⬛  ⬜  ⚪ ⚫ ♦ ♦ ♦
        text
    }

    let mut step = Step::Pre;
    let mut part_iter = document.select(&p_selector).peekable();
    while let Some(part) = part_iter.next() {
        let class = part.attr("class").unwrap();

        match step {
            Step::Pre => {
                if class == "title2" {
                    let title = _text(&part);
                    // Parse order and name from title (e.g., "08　水地比")
                    let parts: Vec<&str> = title.split('　').collect();
                    hexagram.order = parts[0].trim().to_string();
                    hexagram.name  = parts[1].trim().to_string();

                    step = Step::Summary;
                }
            }
            Step::Summary => {
                if class == "normaltext11" {
                    hexagram.summary = _text(&part);
                    step = Step::Guaci;
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            Step::Guaci => {
                if class == "title3" {
                    hexagram.guaci = _text(&part);
                    step = Step::GuaciExplain;
                } else if class == "normaltext" {
                    // ignore
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            Step::GuaciExplain => {
                if class == "normaltext" {
                    hexagram.guaci_explain.push(_text(&part));
                } else if class == "normaltext11" {
                    hexagram.tuan = _text(&part);
                    step = Step::TuanExplain;
                } else if class == "picture" || class == "tz" {
                    //println!("IGNORE picture: {:?}", part);
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            Step::TuanExplain => {
                if class == "normaltext" || class == "normaltext8" || class == "normaltext9" || class == "normaltext12" {
                    hexagram.tuan_explain.push(_text(&part));
                } else if class == "normaltext11" {
                    hexagram.daxiang = _text(&part);
                    step = Step::DaxiangExplain;
                } else if class == "picture" || class == "tz" {
                    // println!("IGNORE picture: {:?}", part);
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            Step::DaxiangExplain => {
                if class == "normaltext" {
                    hexagram.daxiang_explain.push(_text(&part));
                } else if class == "normaltext8" {
                    hexagram.guazhan.push(_text(&part));
                    step = Step::Guazhan;
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            Step::Guazhan => {
                if class == "normaltext8" {
                    hexagram.guazhan.push(_text(&part));
                } else if class == "title3" {
                    hexagram.yaos.push(Yao::default());
                    hexagram.yaos.last_mut().unwrap().yaoci = _text(&part);

                    step = Step::Yaoci_Xiaoxiang;
                }
            }
            Step::Yaoci_Xiaoxiang => {
                if class == "normaltext11" {
                    hexagram.yaos.last_mut().unwrap().xiaoxiang = _text(&part);
                    step = Step::Yaoci_Explain;
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            Step::Yaoci_Explain => {
                if class == "normaltext" {
                    hexagram.yaos.last_mut().unwrap().yaoci_explain.push(_text(&part));
                } else if class == "normaltext8" {
                    hexagram.yaos.last_mut().unwrap().yaozhan.push(_text(&part));
                    step = Step::Yaoci_Zhan;
                } else if class == "picture" || class == "tz" {
                    // println!("IGNORE picture: {:?}", part);
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            Step::Yaoci_Zhan => {
                if class == "normaltext8" {
                    hexagram.yaos.last_mut().unwrap().yaozhan.push(_text(&part));
                } else if class == "normaltext" {
                    let text = _text(&part);
                    if !text.is_empty() {
                        hexagram.yaos.last_mut().unwrap().yaoci_explain.push(text);
                    }
                } else if class == "normaltext9" {
                    hexagram.yaos.last_mut().unwrap().cases.push(Case { Q: _text(&part), A: vec![] });
                    step = Step::Yaoci_CaseA;
                } else if class == "title3" {
                    hexagram.yaos.push(Yao::default());
                    hexagram.yaos.last_mut().unwrap().yaoci = _text(&part);

                    step = Step::Yaoci_Xiaoxiang;
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            Step::Yaoci_CaseA => {
                if class == "normaltext12" {
                    hexagram.yaos.last_mut().unwrap().cases.last_mut().unwrap().A.push(_text(&part));
                } else if class == "normaltext" {
                    let text = _text(&part);
                    if !text.is_empty() {
                        hexagram.yaos.last_mut().unwrap().yaoci_explain.push(text);
                    }
                } else if class == "normaltext9" {
                    hexagram.yaos.last_mut().unwrap().cases.push(Case { Q: _text(&part), A: vec![] });
                    step = Step::Yaoci_CaseA;
                } else if class == "title3" {
                    hexagram.yaos.push(Yao::default());
                    hexagram.yaos.last_mut().unwrap().yaoci = _text(&part);

                    step = Step::Yaoci_Xiaoxiang;
                } else {
                    panic!("Unexpected step: {}", class);
                }
            }
            _ => {
                panic!("Unexpected step: {}", class);
            }
        }
    }
}

fn extract_glyphs(input_path: &PathBuf, output_path: &PathBuf, document: &Html, hexagram: &mut Hexagram) {
    // Implementation
    let p_selector = Selector::parse("p").unwrap();
    let mut part_iter = document.select(&p_selector).peekable();
    while let Some(part) = part_iter.next() {
        let class = part.attr("class").unwrap();
        if class == "picture" {
            let img = part.select(&Selector::parse("img").unwrap()).next().unwrap();
            let src = img.attr("src").unwrap();

            if let Some(nextp) = part_iter.next() {
                if nextp.attr("class").unwrap() == "tz" {
                    let mut text = nextp.text().collect::<String>().trim().to_string();
                    if text.contains("文") || text.contains("书") || text.contains("体") {
                        text = text.replace("▲　", "");
                        let newfile = format!("gd{}_{}.jpg", hexagram.order, text);
                        let glyph_path = PathBuf::from(input_path).join(&src);
                        let newpath = PathBuf::from(output_path).join(&newfile);
                        println!("COPY: {:?} -> {:?}", glyph_path, newpath);
                        if !newpath.exists() {
                            std::fs::copy(&glyph_path, &newpath).unwrap();
                        }
                        hexagram.glyphs.push(newfile);
                    } else {
                        println!("IGNORE: {:?}", text);
                    }

                }
            }
        }
    }
}
