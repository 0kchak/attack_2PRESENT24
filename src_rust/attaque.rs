use crate::chiffrement::{generate_key, chiffrement};
use crate::dechiffrement::{dechiffrement};

fn encrypt(m : &str, k: u128) -> (u128,u128) {
    (chiffrement(m, generate_key(k)), k)
}

fn decrypt(c : &str, k: u128) -> (u128,u128) {
    (dechiffrement(c, generate_key(k)), k)
}

fn verif(m : &str, k: (u128,u128)) -> (u128, u128, (u128, u128)) {
    let c = chiffrement(m, generate_key(k.0));
    let chex = format!("{:x}",c);
    let cf = chiffrement(&chex, generate_key(k.1));
    (c, cf, k)
}
/* 
fn tri_rapide(liste: Vec<(u128,u128)>) -> Vec<(u128,u128)> {
    if liste.len() <= 1 {
        liste
    }
    else {
        let pointeur = liste[0].0;
        let mut petit: Vec<(u128,u128)> = Vec::new();
        let mut grand: Vec<(u128,u128)> = Vec::new();
        let mut egal: Vec<(u128,u128)> = Vec::new();
        for elt in liste {
            if elt.0 < pointeur {
                petit.push(elt);
            }
            else if elt.0 == pointeur{
                egal.push(elt)
            }
            else {
                grand.push(elt)
            }
        }
        egal.extend(tri_rapide(grand));
        let mut triee = tri_rapide(petit);
        triee.extend(egal);
        triee
    }
}*/

fn tri_rapide(liste: & mut Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    if liste.len() <= 1 {
        return liste.to_vec()
    }

    let pointeur = liste[0].0;
    let mut petit: Vec<(u128, u128)> = Vec::new();
    let mut grand: Vec<(u128, u128)> = Vec::new();
    let mut egal: Vec<(u128, u128)> = Vec::new();

    for elt in liste.iter() {
        if elt.0 < pointeur {
            petit.push(*elt);
        } else if elt.0 == pointeur {
            egal.push(*elt);
        } else {
            grand.push(*elt);
        }
    }

    tri_rapide(&mut petit);
    tri_rapide(&mut grand);

    let mut i = 0;
    for elt in petit.iter() {
        liste[i] = *elt;
        i += 1;
    }
    for elt in egal.iter() {
        liste[i] = *elt;
        i += 1;
    }
    for elt in grand.iter() {
        liste[i] = *elt;
        i += 1;
    }
    liste.to_vec()
}

fn find_common_tuples(liste1_triee: Vec<(u128, u128)>, liste2_triee: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    // Parcours les deux listes simultanément en maintenant deux indices
    let (mut i, mut j) = (0, 0);
    let mut cle = vec![];
    while i < liste1_triee.len() && j < liste2_triee.len() {
        if liste1_triee[i].0 < liste2_triee[j].0 {
            i += 1;
        } else if liste1_triee[i].0 > liste2_triee[j].0 {
            j += 1;
        } else {
            let mut k = i;
            while k < liste1_triee.len() {
                if liste1_triee[k].0 == liste2_triee[j].0 {
                    cle.push((liste1_triee[k].1,liste2_triee[j].1));
                    k += 1;
                }
                else {
                    break;
                }
            }
            j += 1;
            k = j;
            while k < liste2_triee.len() {
                if liste1_triee[i].0 == liste2_triee[k].0 {
                    cle.push((liste1_triee[i].1,liste2_triee[k].1));
                    k += 1;
                }
                else {
                    break;
                }
            }
            i += 1;
        }
    }

    cle
}

pub fn attaque() {
    let mut liste_m1: Vec<(u128, u128)> = vec![(0,0);16777216];
    let mut liste_c1: Vec<(u128, u128)> = vec![(0,0);16777216];
    let mut cle_possible: Vec<(u128, u128)> = Vec::new();

    let clair1 = "5378ee";
    let chiffre1 = "d0dd2b";
    let clair2 = "f746a9";
    let chiffre2 = "c82611";

    let cleatester: Vec<u128> = (0..=0xFFFFFF).collect();
    
    for i in 0..cleatester.len() {
        liste_m1[i] = encrypt(clair1, cleatester[i]);
        liste_c1[i] = decrypt(chiffre1, cleatester[i]);
    }
    liste_m1 = tri_rapide(&mut liste_m1);
    liste_c1 = tri_rapide(&mut liste_c1);
    let liste_key = find_common_tuples(liste_m1, liste_c1);

    let mut liste_m2: Vec<(u128, u128, (u128, u128))> = Vec::new();
    for key in liste_key {
        liste_m2.push(verif(clair2, key));
    }
    for elt in liste_m2 {
        if format!("{:x}", elt.1) == chiffre2 {
            cle_possible.push(elt.2);
            println!("- Couple de clé(k1,k2) obtenu : k1 = {} et k2 = {}", format!("{:x}", elt.2.0), format!("{:x}", elt.2.1));
        }
    }
    println!("Fin de l'attaque.");

}
