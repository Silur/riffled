extern crate num;
use self::num::bigint::BigUint;

struct Shuffler {
    p: BigUint,
    q: BigUint,
    g: BigUint,
    commitment: Vec<u8>,
    proof: Vec<u8>
}

impl Shuffler {
    pub fn new() -> Shuffler {
        // 2048-bit MODP Group with 256-bit Prime Order Subgroup
        // taken from RFC5114
        Shuffler {
            p: BigUint::parse_bytes(b"87A8E61DB4B6663CFFBBD19C65195999\
            8CEEF608660DD0F25D2CEED4435E3B00E00DF8F1D61957D4FAF7DF45\
            61B2AA3016C3D91134096FAA3BF4296D830E9A7C209E0C6497517ABD\
            5A8A9D306BCF67ED91F9E6725B4758C022E0B1EF4275BF7B6C5BFC11\
            D45F9088B941F54EB1E59BB8BC39A0BF12307F5C4FDB70C581B23F76\
            B63ACAE1CAA6B7902D52526735488A0EF13C6D9A51BFA4AB3AD83477\
            96524D8EF6A167B5A41825D967E144E5140564251CCACB83E6B486F6\
            B3CA3F7971506026C0B857F689962856DED4010ABD0BE621C3A3960A\
            54E710C375F26375D7014103A4B54330C198AF126116D2276E11715F\
            693877FAD7EF09CADB094AE91E1A1597", 16).unwrap(),
            q: BigUint::parse_bytes(b"8CF83642A709A097B447997640129DA2\
            99B1A47D1EB3750BA308B0FE64F5FBD3", 16).unwrap(),
            g: BigUint::parse_bytes(b"3FB32C9B73134D0B2E77506660EDBD48\
            4CA7B18F21EF205407F4793A1A0BA12510DBC15077BE463FFF4FED4A\
            AC0BB555BE3A6C1B0C6B47B1BC3773BF7E8C6F62901228F8C28CBB18\
            A55AE31341000A650196F931C77A57F2DDF463E5E9EC144B777DE62A\
            AAB8A8628AC376D282D6ED3864E67982428EBC831D14348F6F2F9193\
            B5045AF2767164E1DFC967C1FB3F2E55A4BD1BFFE83B9C80D052B985\
            D182EA0ADB2A3B7313D3FE14C8484B1E052588B9B7D2BBD2DF016199\
            ECD06E1557CD0915B3353BBB64E0EC377FD028370DF92B52C7891428\
            CDC67EB6184B523D1DB246C32F63078490F00EF8D647D148D4795451\
            5E2327CFEF98C582664B4C0F6CC41659", 16).unwrap(),
            commitment: Vec::new(),
            proof: Vec::new()
        }
    }

    fn ilmpp_prove(&self, a: Vec<u8>, b: Vec<u8>) {
        unimplemented!();
    }

    fn ilmpp_verify(&self, a: Vec<u8>, b: Vec<u8>) {
        unimplemented!();
    }

    fn knuth_permute(data: Vec<u8>) {
        unimplemented!();
    }

    fn shuffle_prove(&self, data: Vec<u8>) {
        unimplemented!();
    }

    fn shuffle_verify(&self, data: Vec<u8>) {
        unimplemented!();
    }
}
