pub struct Rot13(pub String);

impl super::Encryptable for Rot13 {
    fn encrypt(&self) -> String {
        self.0.chars().map(|ch| {
            if (ch >= 'a' && ch < 'n') || ( ch >= 'A' &&  ch < 'N') {
                (ch as u8 + 13) as char
            } else if (ch >= 'n' && ch < 'z') || ( ch >= 'N' &&  ch < 'Z') {
                (ch as u8 - 13) as char
            } else {
                ch
            }

        }).collect()
    }
}
