pub fn unescape_html<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
        .replace("&quot;", "\"")
}

pub fn escape_html<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("  ", "&nbsp;&nbsp;")
        .replace("\"", "&quot;")
}

pub fn typing_effect(v: Vec<String>) -> Vec<String> {
    if v.is_empty() {
        v
    } else {
        let s = v.last().unwrap();
        let mut v = v.iter().map(|i| i.clone() + "_").collect::<Vec<_>>();

        let p = format!("{}", s);
        let q = format!("{}_", s);

        v.push(q.clone());
        v.push(q.clone());
        v.push(q.clone());
        v.push(p.clone());
        v.push(p.clone());
        v.push(p.clone());
        v.push(p.clone());
        v.push(q.clone());
        v.push(q.clone());
        v.push(q.clone());
        v.push(q.clone());
        v.push(p.clone());

        v
    }
}

pub fn typing_process<S: AsRef<str>>(s: S) -> Vec<String> {
    let mut v = Vec::new();
    let mut b = String::new();
    let s = s.as_ref();

    for c in s.chars() {
        for i in hangul_detach(c) {
            let mut t = b.clone();
            t.push(i);
            v.push(t);
        }
        b.push(c);
    }

    v
}

pub fn hangul_detach(c: char) -> Vec<char> {
    let i = c as u32;

    if i < 0xAC00 || i > 0xD7AF {
        vec![c]
    } else {
        let a = (i - 0xAC00) / 21 / 28;
        let b = (i - 0xAC00 - a * 21 * 28) / 28;
        let c = i - 0xAC00 - a * 21 * 28 - b * 28;

        let first = vec![
            0x3131, // ㄱ
            0x3132, // ㄲ
            0x3134, // ㄴ
            0x3137, // ㄷ
            0x3138, // ㄸ
            0x3139, // ㄹ
            0x3141, // ㅁ
            0x3142, // ㅂ
            0x3143, // ㅃ
            0x3145, // ㅅ
            0x3146, // ㅆ
            0x3147, // ㅇ
            0x3148, // ㅈ
            0x3149, // ㅉ
            0x314A, // ㅊ
            0x314B, // ㅋ
            0x314C, // ㅌ
            0x314D, // ㅍ
            0x314E, // ㅎ
        ];

        let middle = vec![
            vec![0],      // ㅏ
            vec![1],      // ㅐ
            vec![2],      // ㅑ
            vec![3],      // ㅒ
            vec![4],      // ㅓ
            vec![5],      // ㅔ
            vec![6],      // ㅕ
            vec![7],      // ㅖ
            vec![8],      // ㅗ
            vec![8, 9],   // ㅘ
            vec![8, 10],  // ㅙ
            vec![8, 11],  // ㅚ
            vec![12],     // ㅛ
            vec![13],     // ㅜ
            vec![13, 14], // ㅝ
            vec![13, 15], // ㅞ
            vec![13, 16], // ㅟ
            vec![17],     // ㅠ
            vec![18],     // ㅡ
            vec![18, 19], // ㅢ
            vec![19],     // ㅣ
        ];

        let last = vec![
            vec![0],      // none
            vec![1],      // ㄱ
            vec![2],      // ㄲ
            vec![1, 3],   // ㄳ
            vec![4],      // ㄴ
            vec![4, 5],   // ㄵ
            vec![4, 6],   // ㄶ
            vec![7],      // ㄷ
            vec![8],      // ㄹ
            vec![8, 9],   // ㄺ
            vec![8, 10],  // ㄻ
            vec![8, 11],  // ㄼ
            vec![8, 12],  // ㄽ
            vec![8, 13],  // ㄾ
            vec![8, 14],  // ㄿ
            vec![8, 15],  // ㅀ
            vec![16],     // ㅁ
            vec![17],     // ㅂ
            vec![17, 18], // ㅄ
            vec![19],     // ㅅ
            vec![20],     // ㅆ
            vec![21],     // ㅇ
            vec![22],     // ㅈ
            vec![23],     // ㅊ
            vec![24],     // ㅋ
            vec![25],     // ㅌ
            vec![26],     // ㅍ
            vec![27],     // ㅎ
        ];

        let build_hangul = |x, y, z| x * 21 * 28 + y * 28 + z + 0xAC00;

        let mut r = vec![std::char::from_u32(first[a as usize]).unwrap()];

        for i in middle[b as usize].iter() {
            r.push(std::char::from_u32(build_hangul(a, *i, 0)).unwrap());
        }

        if c > 0 {
            for i in last[c as usize].iter() {
                r.push(std::char::from_u32(build_hangul(a, b, *i)).unwrap());
            }
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::util::string::{hangul_detach, typing_effect, typing_process};

    #[test]
    fn typing_effect_test() {
        assert_eq!(
            vec![
                "H_", "He_", "Hel_", "Hell_", "Hello_", "Hello_", "Hello_", "Hello_", "Hello",
                "Hello", "Hello", "Hello", "Hello_", "Hello_", "Hello_", "Hello_", "Hello"
            ],
            typing_effect(typing_process("Hello"))
        );
    }

    #[test]
    fn typing_process_test() {
        assert_eq!(
            vec!["H", "He", "Hel", "Hell", "Hello"],
            typing_process("Hello")
        );
        assert_eq!(
            vec!["ㅇ", "아", "안", "안ㄴ", "안녀", "안녕"],
            typing_process("안녕")
        );
        assert_eq!(
            vec![
                "ㄱ",
                "가",
                "가ㄴ",
                "가나",
                "가나ㄷ",
                "가나다",
                "가나다ㄹ",
                "가나다라"
            ],
            typing_process("가나다라")
        );
    }

    #[test]
    fn hangul_detach_test() {
        assert_eq!(vec!['ㅂ', '부', '붸', '뷀', '뷁'], hangul_detach('뷁'));
        assert_eq!(vec!['ㄱ', '그', '긔'], hangul_detach('긔'));
        assert_eq!(vec!['ㅇ', '우', '위'], hangul_detach('위'));
        assert_eq!(vec!['ㄲ', '뀨', '뀰', '뀷'], hangul_detach('뀷'));
    }
}
