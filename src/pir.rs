use sodiumoxide::randombytes::randombytes;

pub fn generate_secrets(len: usize) -> (Vec<u8>, Vec<u8>) {
    let m = randombytes(len);
    let s = randombytes(len);
    (m, s)
}

pub fn generate_mask(index: usize, m: Vec<u8>) -> Vec<u8> {
    let mut e: Vec<u8> = vec![0; m.len()];
    let mut ret: Vec<u8> = Vec::with_capacity(m.len());
    
    e[index] = 1;
    for i in 0..m.len() {
        ret[i] = m[i] ^ e[i];
    }
    ret
}

pub fn generate_response(m: Vec<u8>, s: Vec<u8>,
                         plaintext: Vec<u8>,
                         primary: bool) -> Vec<u8> {
    let mut ret = Vec::with_capacity(plaintext.len());
    for i in 0..plaintext.len() { // FIXME
        ret[i] = ret[i] ^ (m[i] ^ plaintext[i]);
    }

    for i in 0..plaintext.len() {
        ret[i] = ret[i] ^ s[i];
    }
    if primary {
        for i in 0..ret.len()-1 {
            ret[i] = ret[i] ^ ret[i+1];
        }
    }
    ret
}

