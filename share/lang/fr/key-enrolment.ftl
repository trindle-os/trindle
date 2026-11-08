
# machine translated (French), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = Approuver la clé d'un pilote au redémarrage

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen démarrage sécurisé clé écran bleu mot de passe pilote caméra écran noir

# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = Le pilote propriétaire de NVIDIA est compilé sur cet ordinateur : le démarrage sécurisé (Secure Boot) ne le chargera pas tant que vous n'aurez pas approuvé sa clé. Faites-le avant de redémarrer, sinon l'écran risque de rester noir.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = Un pilote complémentaire est compilé sur cet ordinateur : le démarrage sécurisé (Secure Boot) ne le chargera pas tant que vous n'aurez pas approuvé sa clé. Faites-le avant de redémarrer.

# Step 1: the password, made before the restart.
key-enrol-step1 = Inventez un mot de passe composé uniquement de lettres et de chiffres. L'écran bleu utilise un clavier américain, donc les symboles peuvent se trouver sur d'autres touches. Notez-le : il ne sert qu'une fois.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Redémarrez. Un écran bleu intitulé Enroll MOK apparaît avant le démarrage de Trindle. Il abandonne au bout d'environ 10 secondes, soyez donc prêt à appuyer sur une touche.

# SAFETY: needs a human review before it ships
# Step 3: the first two screens.
key-enrol-step3 = Choisissez Enroll MOK, puis Continue.

# SAFETY: needs a human review before it ships
# Step 4: the third screen and the password.
key-enrol-step4 = Choisissez Yes, puis saisissez le mot de passe que vous avez inventé.

# Step 5: the last screen.
key-enrol-step5 = Choisissez Reboot. Le pilote se charge une fois l'ordinateur redémarré.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Si vous manquez l'écran bleu, l'ordinateur démarre sans le pilote, comme avant. Rien n'est cassé : exécutez sudo mokutil --import /var/lib/dkms/mok.pub puis redémarrez pour qu'on vous le redemande.
