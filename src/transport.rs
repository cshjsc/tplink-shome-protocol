pub fn encrypt(data: &[u8]) -> Vec<u8> {
    let mut encryption_key = 0xAB;
    data.iter()
        .map(|e| {
            let res = e ^ encryption_key;
            encryption_key = res;
            res
        })
        .collect()
}

pub fn decrypt(encrypted: &[u8]) -> Vec<u8> {
    let mut encryption_key = 0xAB;
    encrypted
        .iter()
        .map(|e| {
            let res = e ^ encryption_key;
            encryption_key = *e;
            //println!("encryption key : {encryption_key}");
            res
        })
        .collect()
}
