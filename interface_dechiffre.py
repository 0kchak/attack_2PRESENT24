from chiffrement import generate_key_known
from dechiffrement import dechiffrement

chiffre = input("Veuillez entrer le message à déchiffrer?\
               \nEn hexadécimal. Format : 000000. => ")
while len(chiffre) != 6:
    chiffre = input("Le format n'est pas bon. Quel est votre chiffré?\
        \nEn hexadécimal. Format: 000000. => ")

key = input("Veuillez entrer une clé pour le déchiffrement?\
        \nEn hexadécimal. Format : 000000. => ")
while len(key) != 6:
    key = input("Le format n'est pas bon. Quel est votre clé?\
        \nEn hexadécimal. Format: 000000. => ")


liste_key = generate_key_known(int(key, 16))
m = dechiffrement(chiffre, liste_key)
print(f"Le chiffré, \033[0;31m{chiffre}\033[0m donne le message \033[0;35m{hex(m)[2:]}\033[0m avec la clef \033[0;36m{key}\033[0m.")
