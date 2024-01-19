fn calculate_padding(max: usize, current: usize, x_padding: Option<u8>) -> (usize, usize) {
    let total = max - current;
    let extra = x_padding.unwrap_or(0) as usize;
    (total / 2 + extra, total / 2)
}

pub fn center(text: &[&str], x_padding: Option<u8>) -> String {
    let longest = text.iter().max_by_key(|s| s.len()).map_or(0, |s| s.len());

    text.iter().map(|s| {
        let (left_pad, right_pad) = calculate_padding(longest, s.len(), x_padding);
        format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
    }).collect::<Vec<_>>().join("\n")
}

pub fn print_center(text: &[&str], x_padding: Option<u8>) {
    println!("{}", center(text, x_padding));
}

pub fn right(text: &[&str], x_padding: Option<u8>) -> String {
    let longest = text.iter().max_by_key(|s| s.len()).map_or(0, |s| s.len());

    text.iter().map(|s| {
        let (left_pad, _) = calculate_padding(longest, s.len(), x_padding);

        format!("{}{}", " ".repeat(left_pad), s)
    }).collect::<Vec<_>>().join("\n")
}

pub fn print_right(text: &[&str], x_padding: Option<u8>) {
    println!("{}", right(text, x_padding));
}