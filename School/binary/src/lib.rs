pub trait ToBinary {
    /// This will convert a input string to binary
    fn to_binary(&self) -> String;
}
pub trait BinaryToString {
    /// Reads binary and returns a string
    fn binary_to_string(&self) -> String;
}

impl ToBinary for String {
    fn to_binary(self: &String) -> String {
        let mut ret = String::from("");
        let chars = self.chars().into_iter();

        for char in chars {
            ret.push_str(&*(" ".to_owned() + &*convert(char)));
        }
        ret
    }
}

impl BinaryToString for String {
    fn binary_to_string(&self) -> String {
        let num = String::from(self);
        let mut dec_value: i32 = 0;

        let mut base: i32 = 1;

        let len: i32 = num.len() as i32;
        let mut i: i32 = len - 1;

        while i >= 0 {
            if num.chars().nth(i as usize).unwrap() == '1' { dec_value += base; }
            base = base * 2;
            i = i - 1;
        }

        let ret = std::char::from_digit((dec_value - 87) as u32, 36);
        ret.unwrap().to_string()
    }
}

fn convert(s: char) -> String {
    let mut val: i32 = s.to_digit(36).unwrap() as i32 + 87;

    let mut bin: String = String::from("");
    while val > 0 {
        if val % 2 == 1 {
            bin.push_str("1");
        } else {
            bin.push_str("0");
        }
        val = val / 2;
    }
    bin = bin.chars().rev().collect();

    bin.to_string()
}