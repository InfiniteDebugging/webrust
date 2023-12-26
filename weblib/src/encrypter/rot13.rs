pub struct Rot13(pub String);

impl super::Encryptable for Rot13 {
    fn encrypt(&self) -> String {
        let mut new_string = String::new();
        // This was failing because one cannot index into strings.
        // let len = self.0.len();
        // for i in 0..len {
        //     let _ch = self.0[i];
        for _ch in self.0.chars() {
            if (_ch >= 'a' && _ch < 'n') || ( _ch >= 'A' &&  _ch < 'N') {
                new_string.push((_ch as u8 + 13) as char);
            }
            if (_ch >= 'n' && _ch < 'z') || ( _ch >= 'N' &&  _ch < 'Z') {
                new_string.push((_ch as u8 - 13) as char);
            }
        }
        new_string
    }
}
