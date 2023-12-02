fn main() {
    let input: &'static str = "treb7uchet";

    get_calibration_value(input);
}

fn get_calibration_value(text: &str) {
    let mut satu: u32 = 0;
    let mut dua: u32 = 0;

    for c in text.chars() {
        if c.is_ascii_digit() {
            if satu == 0 {
                satu = c.to_digit(10).unwrap()
            } else {
                dua = c.to_digit(10).unwrap()
            }
        }
        // println!("{}", c.is_ascii_digit());
    }

    if dua == 0 {
        dua = satu
    }

    let value: u32 = format!("{satu}{dua}").parse().unwrap();

    println!("{value}");
}

// fn sum_calibration_value(value: u32) {
//     unimplemented!();
// }
