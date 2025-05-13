pub struct Solution;

impl Solution   {
    pub fn num_decodings(s: String) -> i32 {
        let s: Vec<_> = s.chars().collect();
        let mut count = 0;
        traverse(&s, 0, 1, &mut count);
        count as i32
    }
}

// Time Limit Exceeded version
// https://excalidraw.com/#json=18SkzARuhP4oxVhFqGyN_,0lMgdgeWP4wXR2thy9ofBg
fn traverse(s: &[char], begin: usize, end: usize, count: &mut usize) {
    if end >= s.len() {
        let n = s[begin..end]
            .iter()
            .collect::<String>()
            .parse::<i32>();
        // println!("n={n:?}");
        if n.is_ok_and(|n| n > 0 && n <= 26)
        {
            *count += 1;
        }
        return;
    }
    let n = s[begin..end]
        .iter()
        .collect::<String>()
        .parse::<i32>();
    // println!("n={n:?}");
    if n.is_ok_and(|n| n <= 0 || n > 26)
    {
        return;
    }
    traverse(s, begin, end + 1, count);
    traverse(s, end, end + 1, count);
}
