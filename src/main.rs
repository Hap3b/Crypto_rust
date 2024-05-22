use std::collections::HashMap;

fn main() {
    let msg = "vcfgrwqwfsbhfsntowbsobgfsbhfsnqvsnjcigsghqsoixcifrviwtshseicwbsgojsnjcigdogeisjcigoihfsgofhwgobgjcigbsrsjsnqwfqizsfrobgzsgfisgzsgxcifgcijfopzsgeiojsqzsggwubsgrsjchfsdfctsggwcbdofzseiszsghhcbashwsf";
    let key = find_key(msg);
    let display = dec(key, &msg);
    println!("{}", display);
}

fn dec(key: u8, msg: &str) -> String {
    msg.chars().map(|c| {
        match c {
            'a'..='z' => (((c as u8 - b'a' + key)%26)+b'a') as char,
            'A'..='Z' => (((c as u8 - b'a' + key)%26)+b'a') as char,
            _         => c
        }
    }).collect()
}

fn find_key(msg: &str) -> u8 {
    let mut occurences = HashMap::new();

    for letter in msg.chars() {
        let count = occurences.entry(letter).or_insert(0);
        *count   += 1;
    }
    println!("{:?}", occurences);

    let mut max = 0;
    let mut key = 0;

    for (letter, value) in occurences {
        if value > max {
            max = value;
            key = (letter as u8 - b'e')%26;
        };
    }

    println!("{}", key);
    key = 26 - key;
    key

}
