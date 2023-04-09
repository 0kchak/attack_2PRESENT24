from chiffrement import chiffrement, generate_key_known
import random

message = input("Veuillez entrer le message à chiffrer?\
               \nEn hexadécimal. Format : 000000. => ")
while len(message) != 6:
    message = input("Le format n'est pas bon. Quel est votre message?\
        \nEn hexadécimal. Format: 000000. => ")

start = input("Souhaitez vous donner une clé pour le chiffrement?\
        \n yes|no\n")

if start.lower() == 'no':
    key = hex(random.randint(0, 0xFFFFFF))[2:]
elif start.lower() == 'yes':
    key = input("Veuillez entrer une clé pour le chiffrement?\
            \nEn hexadécimal. Format : 000000. => ")
    while len(key) != 6:
        key = input("Le format n'est pas bon. Quel est votre clé?\
            \nEn hexadécimal. Format: 000000. => ")


liste_key = generate_key_known(int(key, 16))
c = chiffrement(message, liste_key)
print(f"Le message, \033[0;31m{message}\033[0m donne le chiffré \033[0;35m{hex(c)[2:]}\033[0m avec la clef \033[0;36m{key}\033[0m.")
