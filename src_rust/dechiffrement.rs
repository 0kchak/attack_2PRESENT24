fn substitution_inverse (binaire: u128) -> u128 {
    let liste_sub = [5, 14, 15, 8, 12, 1, 2, 13, 11, 4, 6, 3, 0, 7, 9, 10];
    let mut w5 = (binaire >> 4*5) & 0xF;
    w5 = liste_sub[w5 as usize];
    let mut w4 = (binaire >> 4*4) & 0xF;
    w4 = liste_sub[w4 as usize];
    let mut w3 = (binaire >> 4*3) & 0xF;
    w3 = liste_sub[w3 as usize];
    let mut w2 = (binaire >> 4*2) & 0xF;
    w2 = liste_sub[w2 as usize];
    let mut w1 = (binaire >> 4*1) & 0xF;
    w1 = liste_sub[w1 as usize];
    let mut w0 = (binaire >> 4*0) & 0xF;
    w0 = liste_sub[w0 as usize];
    let result = (w5 << 20) | (w4 << 16) | (w3 << 12) | (w2 << 8) | (w1 << 4) | (w0);
    result
}

fn permutation_inverse(num: u128) -> u128 {
    let mut nvl_num = num & 0b1;
    nvl_num |= (num >> 5) & 0b10;
    nvl_num |= (num >> 10) & 0b100;
    nvl_num |= (num >> 15) & 0b1000;
    nvl_num |= (num << 3) & 0b10000;
    nvl_num |= (num >> 2) & 0b100000;
    nvl_num |= (num >> 7) & 0b1000000;
    nvl_num |= (num >> 12) & 0b10000000;
    nvl_num |= (num << 6) & 0b100000000;
    nvl_num |= (num << 1) & 0b1000000000;
    nvl_num |= (num >> 4) & 0b10000000000;
    nvl_num |= (num >> 9) & 0b100000000000;
    nvl_num |= (num << 9) & 0b1000000000000;
    nvl_num |= (num << 4) & 0b10000000000000;
    nvl_num |= (num >> 1) & 0b100000000000000;
    nvl_num |= (num >> 6) & 0b1000000000000000;
    nvl_num |= (num << 12) & 0b10000000000000000;
    nvl_num |= (num << 7) & 0b100000000000000000;
    nvl_num |= (num << 2) & 0b1000000000000000000;
    nvl_num |= (num >> 3) & 0b10000000000000000000;
    nvl_num |= (num << 15) & 0b100000000000000000000;
    nvl_num |= (num << 10) & 0b1000000000000000000000;
    nvl_num |= (num << 5) & 0b10000000000000000000000;
    nvl_num |= (num) & 0b100000000000000000000000;
    nvl_num
}

pub fn dechiffrement(chiffre : &str, list_key: [u128;11]) -> u128 {
    let mut etat = u128::from_str_radix(chiffre, 16).unwrap();
    etat = etat ^ list_key[10];
    for i in (1..=10).rev() {
        etat = permutation_inverse(etat);
        etat = substitution_inverse(etat);
        etat = etat ^ list_key[i-1];
    }
    etat
}