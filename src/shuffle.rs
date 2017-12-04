use sodiumoxide::randombytes::randombytes;
use num::bigint::{BigUint, ToBigUint};
use num::{Zero, One};
use num::Integer;
use std::ops::*;
use std::cmp::Ordering;

struct Shuffler {
    p: BigUint,
    q: BigUint,
    g: BigUint,
    commitment: Vec<u8>,
    proof: Vec<u8>
}

    fn xgcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
        let (zero, one): (BigUint, BigUint) = (Zero::zero(), One::one());
        let (mut u_a, mut v_a, mut u_b, mut v_b) = (one.clone(), zero.clone(), zero.clone(), one.clone());
        let (mut aa, mut bb) = (a.to_biguint().unwrap(), b.to_biguint().unwrap());

        while aa != zero {
            let q = &bb / &aa;

            let new_a = &bb - &q * &aa;
            bb = aa;
            aa = new_a;

            let new_u_a = &u_b - &q * &u_a; 
            u_b = u_a;
            u_a = new_u_a;

            let new_v_a = &v_b - &q * &v_a; 
            v_b = v_a;
            v_a = new_v_a;
        }
        (bb, u_b, v_b)
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


    fn ilmpp_prove(&self, a: Vec<BigUint>, b: Vec<BigUint>, gamma: BigUint) 
        -> Result<(Vec<BigUint>, Vec<BigUint>), &str>{
        if a.len() != b.len() {
            return Err("samples are not the same length")
        }

        let p = &self.p;
        let q = &self.q;
        let g = &self.g;

        let n = a.len()+1;

        let mut theta: Vec<BigUint> = Vec::new();
        for i in 0..n {
            let mut rand = BigUint::from_bytes_be(&randombytes(64));
            rand %= q - (1 as u32);
            rand += (1 as u32);
            theta.push(rand);
        }

        let mut commitment: Vec<BigUint> = Vec::new();
        let mut x = BigUint::from(0u32);
        let mut y = BigUint::from(0u32);
        for i in 0..n {
            
            x = &a[i] * &theta[i];
            x = &b[i] * &theta[i];
            x = g.modpow(&x, p);
            y = g.modpow(&y, p);
            commitment[i] = x * &y;
            commitment[i] %= p;
        }
        
        let mut r: Vec<BigUint> = Vec::with_capacity(n-1);
        let mut num = BigUint::from(1u32);
        let mut den = BigUint::from(1u32);
        let mut z_inv_lq = (Zero::zero(), Zero::zero(), Zero::zero());
        
        for i in n-2..0 {
            den *= &a[i+1];
            num *= &b[i+1];
            z_inv_lq = xgcd(&den, &q);
            r[i] = &num * &z_inv_lq.2;
            r[i] *= &gamma;
            r[i] *= q;
            if n-i-1 % 2 == 1 {
                r[i] = q - &r[i];
            }
            r[i] += &theta[i+1];
        }
        
        Ok((commitment, r))
    }
    
	fn ilmpp_verify(&self, a: Vec<BigUint>, b: Vec<BigUint>,
                    commitment: Vec<BigUint>,
                    proof: Vec<BigUint>,
                    gamma: BigUint) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let n = a.len();
        let mut l: BigUint = Zero::zero();
        let mut r: BigUint = Zero::zero();
        let qsubgamma: BigUint = &self.q - &gamma;
        l = b[0].modpow(&proof[0], &self.p);
        if n-1 % 2 == 1 {
            r = a[0].modpow(&qsubgamma, &self.p);
        } else {
            r = a[0].modpow(&gamma, &self.p);
        }
        r = (&a[0]).mul(&r);
        r = r.rem(&self.p);
        if l.cmp(&r) != Ordering::Equal {
            return false;
        }

        for i in 1..n-1 {
            l = a[i].modpow(&proof[i-1], &self.p);
            r = b[i].modpow(&proof[i], &self.p);
            l *= &r;
            l %= &self.p;
            if l.cmp(&commitment[i]) != Ordering::Equal {
                return false;
            }
        }
        l = a[n-1].modpow(&proof[n-2], &self.p);
        r = b[n-1].modpow(&qsubgamma, &self.p);
        r *= &commitment[n-1];
        r %= &self.p;
        if l.cmp(&r) != Ordering::Equal {
            return false;
        }
        return true;
	}

	fn shuffle_prove(&self, x: Vec<BigUint>, y: Vec<BigUint>, 
                     c: BigUint, d: BigUint, 
                     t: BigUint, gamma: BigUint) 
        -> Result<(Vec<BigUint>, Vec<BigUint>), &str>{
        if x.len() != y.len() {
            return Err("inputs are not the same length");
        }
        let n = x.len();
        let mut phi: Vec<BigUint> = Vec::with_capacity(2*n);
        let mut psi: Vec<BigUint> = Vec::with_capacity(2*n);
        let ct: BigUint = &c * &t;
        let dt: BigUint = &d * &t;

        for i in 0..n {
            phi[i] = &x[i] - &dt;
            phi[i] %= &self.q;
            phi[n+i] = c.clone();
            psi[i] = &y[i] - &ct;
            psi[i] %= &self.q;
            psi[n+i] = d.clone();
        }
        self.ilmpp_prove(psi, phi, gamma)
    }

    fn shuffle_verify(&self, x: Vec<BigUint>, y: Vec<BigUint>,
                      c: BigUint, d: BigUint,
                      t: BigUint,
                      commitment: Vec<BigUint>, proof: Vec<BigUint>,
                      gamma: BigUint) -> bool {
        if x.len() != y.len() { return false; }
        let n = x.len();
        let u: BigUint = c.modpow(&t, &self.p);  
        let w: BigUint = c.modpow(&t, &self.p);
        let mut z_uinv_q = (Zero::zero(), Zero::zero(), Zero::zero());
        let mut z_winv_q = (Zero::zero(), Zero::zero(), Zero::zero());
        z_uinv_q = xgcd(&u, &self.p);
        z_winv_q = xgcd(&w, &self.p);

        let mut phi: Vec<BigUint> = Vec::with_capacity(2*n);
        let mut psi: Vec<BigUint> = Vec::with_capacity(2*n);

        for i in 0..n {
            phi[i] = &x[i] * &z_uinv_q.2;
            phi[i] %= &self.q;
            phi[n+i] = c.clone();
            psi[i] = &y[i] * &z_winv_q.2;
            psi[i] %= &self.q;
            psi[n+i] = d.clone();
        }
        self.ilmpp_verify(psi, phi, commitment, proof, gamma)
    }
}
