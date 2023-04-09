from multiprocessing import Pool, freeze_support
from chiffrement import chiffrement, generate_key_known
from dechiffrement import dechiffrement

liste_m1 = []
liste_c1 = []
liste_key = []
cle_possible = []


"""
Fonction qui chiffre le clair1 avec une clé donnée, créée dans le but
d'être utilisée pour la parallélisation.
"""


def encrypt(m, k):
    return (chiffrement(m, generate_key_known(k)), k)


"""
Fonction qui déchiffre le chiffré1 avec une clé donnée, créée dans le but
d'être utilisée pour la parallélisation.
"""


def decrypt(c, k):
    return (dechiffrement(c, generate_key_known(k)), k)


"""
Fonction qui chiffre le clair2, puis rechiffre le résultat avec
un couple de clé, créée dans le but d'être utilisée pour la parallélisation.
"""


def verif(m, k):
    c = chiffrement(m, generate_key_known(k[0]))
    cf = chiffrement(hex(c)[2:], generate_key_known(k[1]))
    return (c, cf, k)


"""
Pour une liste donnée contenant un tuple, trie la liste en fonction du
premier élément du tuple. La fonction est récursive pour
simplifier le processus.
"""


def tri_rapide(liste):
    if len(liste) <= 1:
        return liste
    else:
        pointeur = liste[0][0]
        petit = []
        grand = []
        egal = []
        for elt in liste:
            if elt[0] < pointeur:
                petit.append(elt)
            elif elt[0] == pointeur:
                egal.append(elt)
            else:
                grand.append(elt)
        return tri_rapide(petit) + egal + tri_rapide(grand)


def find_common_tuples(liste1_triee, liste2_triee):
    # Parcours les deux listes simultanément en maintenant deux indices
    i, j = 0, 0
    cle = []
    while i < len(liste1_triee) and j < len(liste2_triee):
        if liste1_triee[i][0] < liste2_triee[j][0]:
            i += 1
        elif liste1_triee[i][0] > liste2_triee[j][0]:
            j += 1
        else:
            # Les deux éléments sont égaux, donc ajoute toutes les
            # combinaisons possibles
            k = i
            while k < len(liste1_triee) and liste1_triee[k][0] == liste1_triee[i][0]:
                l = j
                while l < len(liste2_triee) and liste2_triee[l][0] == liste2_triee[j][0]:
                    cle.append((liste1_triee[k][1], liste2_triee[l][1]))
                    l += 1
                k += 1
            i = k
            j = l

    return cle


if __name__ == '__main__':
    freeze_support()

    clair1 = "5378ee"
    chiffre1 = "d0dd2b"
    clair2 = "f746a9"
    chiffre2 = "c82611"

    start = input("L’attaque nécessite deux couples clairs-chiffrés, souhaitez vous donner des couples clairs-chiffrés ?\
                    \nSinon le code sera lancé avec des valeurs par défaut. \nValeur par défaut :\
           \n   - (m1,c1) = (5378ee, d0dd2b)\n   - (m2,c2) = (f746a9, c82611)\n yes|no\n")
    while start.lower() != 'no':
        if start.lower() == 'yes':
            clair1 = input("Quel est votre message clair 1?\
               \nEn hexadécimal. Format : 000000. => ")
            while len(clair1) != 6:
                clair1 = input("Le format n'est pas bon. Quel est votre message clair 1?\
                                \nEn hexadécimal. Format: 000000. => ")
            chiffre1 = input("Quel est votre message chiffré 1?\
                        \nEn hexadécimal. Format : 000000. => ")
            while len(chiffre1) != 6:
                chiffre1 = input("Le format n'est pas bon. Quel est votre message chiffré 1?\
                                \nEn hexadécimal. Format: 000000. => ")
            clair2 = input("Quel est votre message clair 2?\
                        \nEn hexadécimal. Format : 000000. => ")
            while len(clair2) != 6:
                clair2 = input("Le format n'est pas bon. Quel est votre message clair 2?\
                                \nEn hexadécimal. Format: 000000. => ")
            chiffre2 = input("Quel est votre message chiffré 2?\
                        \nEn hexadécimal. Format : 000000. => ")
            while len(chiffre2) != 6:
                chiffre2 = input("Le format n'est pas bon. Quel est votre message chiffré 2?\
                                \nEn hexadécimal. Format: 000000. => ")
            start = 'no'
        else:
            while start.lower() != 'no' and start.lower() != 'yes':
                start = input('Seules réponses acceptées: yes | no\n')

    print("L'attaque est lancée [..processing..]")
    """
    Créer la liste_m1 contenant des tuples clair1 chiffré et la clé k, grâce
    au map qui créer une liste avec les return des fonctions encrypt.
    Créer la liste_c1 contenant des tuples chiffre1 déchiffré et la clé k,
    grâce au map qui créer une liste avec les return des fonctions decrypt.
    """
    cleatester = [i for i in range(0xFFFFFF)]
    with Pool() as pool:
        liste_m1 = pool.starmap(encrypt, [(clair1, k) for k in cleatester])
        liste_c1 = pool.starmap(decrypt, [(chiffre1, k) for k in cleatester])
    # Trie les listes liste_m1 et liste_c1
    with Pool() as pool:
        liste_m1c1 = pool.map(tri_rapide, [liste_m1, liste_c1])
    liste_m1 = liste_m1c1[0]
    liste_c1 = liste_m1c1[1]

    # Recupère les couples de clé (k1,k2)
    liste_key = find_common_tuples(liste_m1, liste_c1)

    # Teste les couples clés obtenues sur le couple (m2,c2).
    with Pool() as pool:
        liste_m2 = pool.starmap(verif, [(clair2, k) for k in liste_key])

    """
    Parcourt liste_m2, regarde si le chiffré obtenu par verif correspond
    au chiffré2. Boucle réalisée en 6 secondes en moyenne avec 6 coeurs.
    """
    for elt in liste_m2:
        if hex(elt[1])[2:] == chiffre2:
            cle_possible.append(elt[2])
            print("- Couple de clé(k1,k2) obtenu : k1 =", hex(elt[2][0])[2:], 'k2 =', hex(elt[2][1])[2:])
    print("Fin de l'attaque.")
