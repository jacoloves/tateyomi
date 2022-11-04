use std::char;

pub fn han2zen(c: char) -> char {
    match c {
        '0'..='9' => char_from_u32(c as u32 + 0xFF10 - 0x30, c),
        'A'..='Z' | 'a'..='z' => char_from_u32(c as u32 + 0xFF21 - 0x41, c),
        '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => char_from_u32(c as u32 + 0xFF01 - 0x21, c),
        _ => c,
    }
}

pub fn char_from_u32(i: u32, def: char) -> char {
    char::from_u32(i).unwrap_or(def)
}

fn main() {
    let _haiku: &'static str = "書いてみたり
けしたり果ては
けしの花
- 立花北枝";

    let _otameshi: &'static str = "Hello,World!
これはRustで書いた縦読み変換ツールです。
しっかり縦読みになってますか？";

    let today_memo: &'static str = "今日のエルダー面談なんだけど、
中止にさせてもらってもいいかな？
今日いきなりで申し訳ない。";

    let ota = today_memo;

    let mut haiku_array = ota.split('\n').fold(Vec::new(), |mut s, i| {
        let mut tmp_str: String = "".to_string();
        for c in i.chars() {
            tmp_str += &han2zen(c).to_string();
        }
        s.push(tmp_str);
        s
    });

    let mut max_num = 0;

    for i in 0..haiku_array.len() {
        if max_num < haiku_array[i].len() {
            max_num = haiku_array[i].len();
        }
    }

    for i in 0..haiku_array.len() {
        if max_num > haiku_array[i].len() {
            for _ in 1..=(max_num - haiku_array[i].len()) {
                haiku_array[i].push('　');
            }
        }
    }

    for i in 0..max_num / 3 {
        let mut v_str: String = "".to_string();
        for j in (0..haiku_array.len()).rev() {
            if haiku_array[j].chars().nth(i).unwrap() == 'ー' {
                v_str += "｜";
            } else if haiku_array[j].chars().nth(i).unwrap() == ' ' {
                v_str += "　";
            } else {
                v_str += &han2zen(haiku_array[j].chars().nth(i).unwrap()).to_string();
            }
        }
        println!("{}", v_str);
    }
}
