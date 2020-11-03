#[derive(Debug)]
pub struct RSAKey {
    n: usize,
    p: usize,
    s: Option<usize>,
}

impl RSAKey {
    pub fn public_key(n: usize, p: usize) -> Self {
        Self { n, p, s: None }
    }
    pub fn private_key(n: usize, p: usize, s: usize) -> Self {
        Self { n, p, s: Some(s) }
    }
    pub fn key_gen(p: usize, q: usize) -> Self {
        let n = p * q;
        let phi = (p - 1) * (q - 1);
        let e = 1025;
        let d = inv_mod(e, phi); //e^{-1} mod phi
        Self {
            n,
            p: e,
            s: Some(d),
        }
    }
    pub fn encrypt_secret(&self, m: usize) -> usize {
        0
    }
    pub fn encrypt_public(&self, m: usize) -> usize {
        0
    }
    pub fn decrypt_secret(&self, c: usize) -> usize {
        // c^s mod n
        pow_mod(c, self.s.expect("Cannot decrypt w/o secret key"), self.n)
    }
}

fn pow_mod(base: usize, exponent: usize, modulus: usize) -> usize {
    let mut total = base;
    for i in 1..exponent {
        total *= base;
        total %= modulus;
    }
    total
}
fn inv_mod(num: usize, modulus: usize) -> usize {
    for i in 0..modulus {
        if (i * num) % modulus == 1 {
            return i;
        }
    }
    panic!("No inverse found")
}
