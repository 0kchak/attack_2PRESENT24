
def substitution_inverse(binaire: int):
    # Récupère les 4 premiers bits grâce aux opérateurs bit à bit et applique
    # le tableau de substitution inversé dessus. On réitère
    # et on return un int substitué.
    liste_sub_inverse = [5, 14, 15, 8, 12, 1, 2, 13, 11, 4, 6, 3, 0, 7, 9, 10]
    result = 0
    w5 = (binaire >> 4*5) & 0xF
    w5 = liste_sub_inverse[w5]
    w4 = (binaire >> 4*4) & 0xF
    w4 = liste_sub_inverse[w4]
    w3 = (binaire >> 4*3) & 0xF
    w3 = liste_sub_inverse[w3]
    w2 = (binaire >> 4*2) & 0xF
    w2 = liste_sub_inverse[w2]
    w1 = (binaire >> 4*1) & 0xF
    w1 = liste_sub_inverse[w1]
    w0 = (binaire >> 4*0) & 0xF
    w0 = liste_sub_inverse[w0]
    result = (w5 << 20) | (w4 << 16) | (w3 << 12) | (w2 << 8) | (w1 << 4) | (w0)
    return result


def permutation_inverse(num):
    nvl_num = num & 0b1 # récupère le bit 0 et le positionne en 0
    nvl_num |= (num >> 5) & 0b10  # récupère le 6ème bit et le positionne en 1er
    nvl_num |= (num >> 10) & 0b100  # récupère le 12ème bit et le positionne en 2ème
    nvl_num |= (num >> 15) & 0b1000  # récupère le 18ème bit et le positionne en 3ème
    nvl_num |= (num << 3) & 0b10000  # récupère le 1er bit et le positionne en 4ème
    nvl_num |= (num >> 2) & 0b100000  # récupère le 7ème bit et le positionne en 5ème
    nvl_num |= (num >> 7) & 0b1000000  # récupère le 13ème bit et le positionne en 6ème
    nvl_num |= (num >> 12) & 0b10000000  # récupère le 19ème bit et le positionne en 7ème
    nvl_num |= (num << 6) & 0b100000000  # récupère le 2ème bit et le positionne en 8ème
    nvl_num |= (num << 1) & 0b1000000000  # récupère le 8ème bit et le positionne en 9ème
    nvl_num |= (num >> 4) & 0b10000000000  # récupère le 14ème bit et le positionne en 10ème
    nvl_num |= (num >> 9) & 0b100000000000  # récupère le 20ème bit et le positionne en 11ème
    nvl_num |= (num << 9) & 0b1000000000000  # récupère le 3ème bit et le positionne en 12ème
    nvl_num |= (num << 4) & 0b10000000000000  # récupère le 9ème bit et le positionne en 13ème
    nvl_num |= (num >> 1) & 0b100000000000000  # récupère le 15ème bit et le positionne en 14ème
    nvl_num |= (num >> 6) & 0b1000000000000000  # récupère le 21ème bit et le positionne en 15ème
    nvl_num |= (num << 12) & 0b10000000000000000  # récupère le 4ème bit et le positionne en 16ème
    nvl_num |= (num << 7) & 0b100000000000000000  # récupère le 10ème bit et le positionne en 17ème
    nvl_num |= (num << 2) & 0b1000000000000000000  # récupère le 16ème bit et le positionne en 18ème
    nvl_num |= (num >> 3) & 0b10000000000000000000  # récupère le 22ème bit et le positionne en 19ème
    nvl_num |= (num << 15) & 0b100000000000000000000  # récupère le 5ème bit et le positionne en 20ème
    nvl_num |= (num << 10) & 0b1000000000000000000000  # récupère le 11ème bit et le positionne en 21ème
    nvl_num |= (num << 5) & 0b10000000000000000000000  # récupère le 17ème bit et le positionne en 22ème
    nvl_num |= (num) & 0b100000000000000000000000  # récupère le 23ème bit et le positionne en 23ème
    return nvl_num


def dechiffrement(chiffre, list_key):
    # algorithme de chiffrement dans le sens inverse.
    etat = int(chiffre, 16)
    etat = etat ^ list_key[10]
    for i in range(10, 0, -1):
        etat = permutation_inverse(etat)
        etat = substitution_inverse(etat)
        etat = etat ^ list_key[i-1]
    dechiffre = etat
    return dechiffre
