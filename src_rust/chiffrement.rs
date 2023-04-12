fn substitution (binaire: u128, mot: &str) -> u128 {
    let liste_sub = [12, 5, 6, 11, 9, 0, 10, 13, 3, 14, 15, 8, 4, 7, 1, 2];
    let mut result = 0;
    if mot == "key" {
        result = result | liste_sub[binaire as usize]
    }
    else {
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
        result = (w5 << 20) | (w4 << 16) | (w3 << 12) | (w2 << 8) | (w1 << 4) | (w0);
    }
    result
}

fn permutation(num: u128) -> u128 {
    let mut nvl_num = num & 0b1;
    nvl_num |= (num >> 3) & 0b10;
    nvl_num |= (num >> 6) & 0b100;
    nvl_num |= (num >> 9) & 0b1000;
    nvl_num |= (num >> 12) & 0b10000;
    nvl_num |= (num >> 15) & 0b100000;
    nvl_num |= (num << 5) & 0b1000000;
    nvl_num |= (num << 2) & 0b10000000;
    nvl_num |= (num >> 1) & 0b100000000;
    nvl_num |= (num >> 4) & 0b1000000000;
    nvl_num |= (num >> 7) & 0b10000000000;
    nvl_num |= (num >> 10) & 0b100000000000;
    nvl_num |= (num << 10) & 0b1000000000000;
    nvl_num |= (num << 7) & 0b10000000000000;
    nvl_num |= (num << 4) & 0b100000000000000;
    nvl_num |= (num << 1) & 0b1000000000000000;
    nvl_num |= (num >> 2) & 0b10000000000000000;
    nvl_num |= (num >> 5) & 0b100000000000000000;
    nvl_num |= (num << 15) & 0b1000000000000000000;
    nvl_num |= (num << 12) & 0b10000000000000000000;
    nvl_num |= (num << 9) & 0b100000000000000000000;
    nvl_num |= (num << 6) & 0b1000000000000000000000;
    nvl_num |= (num << 3) & 0b10000000000000000000000;
    nvl_num |= (num) & 0b100000000000000000000000;
    nvl_num
}

pub fn generate_key(key : u128) -> [u128;11] {
    let mut list_key : [u128; 11] = [0;11];
    let mut tour : u32 = 1;
    let mut keybin: u128 = (key << 56) as u128;
    let mask_key = 0x0000000000FFFFFF;
    let mut y = (keybin >> 16) & mask_key;
    list_key[0] = y;
    let mask = (1 << 19) - 1;
    let mut last_19_bits : u128;
    let mut rotate_bits : u128;
    let mut x :u128;
    let mask_sub = 0xF;
    let mut first : u128;
    let mut substitue : u128;
    let mask_xor = 0b00000000000000000000000000000000000000000000000000000000000011111;
    let mut key_xor: u32;
    for i in 0..10 {
        y = keybin >> 19;
        last_19_bits = keybin & mask;
        rotate_bits = last_19_bits << (80 - 19);
        keybin = rotate_bits | y;
        x = keybin >> 76;
        first = x & mask_sub;
        keybin = keybin & 0x0FFFFFFFFFFFFFFFFFFF;
        substitue = substitution(first, "key");
        substitue = substitue << 76;
        keybin = substitue | keybin;
        key_xor = ((keybin >> 15) & mask_xor) as u32;
        keybin = keybin & 0b11111111111111111111111111111111111111111111111111111111111100000111111111111111;
        key_xor = key_xor ^ tour;
        keybin = keybin | (key_xor << 15) as u128;
        tour += 1;
        list_key[i+1] = (keybin >> 16) & mask_key;
    }
    list_key
}

pub fn chiffrement(message: &str, list_key: [u128;11]) -> u128{
    let mut etat = u128::from_str_radix(message, 16).unwrap();
    for i in 0..10 {
        let key = list_key[i];
        etat = etat ^ key as u128;
        etat = substitution(etat, "chiffre");
        etat = permutation(etat);
    }
    etat ^ list_key[10]
}