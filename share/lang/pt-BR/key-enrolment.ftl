# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = Aprovar a chave de um driver ao reiniciar

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot chave mok cadastrar registrar tela azul senha driver nvidia virtualbox câmera dkms mokutil tela preta

# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = O driver próprio da NVIDIA é compilado neste computador, então o Secure Boot não o carrega até você aprovar a chave dele. Faça isso antes de reiniciar, ou a tela pode ficar preta.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = Um driver adicional é compilado neste computador, então o Secure Boot não o carrega até você aprovar a chave dele. Faça isso antes de reiniciar.

# Step 1: the password, made before the restart.
key-enrol-step1 = Crie uma senha só com letras e números. A tela azul usa um teclado americano, então os símbolos podem estar em outras teclas. Anote a senha; ela é usada uma única vez.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Reinicie. Uma tela azul com o título Enroll MOK aparece antes de o Trindle iniciar. Ela desiste após cerca de 10 segundos, então esteja pronto para apertar uma tecla.

# Step 3: the first two screens.
key-enrol-step3 = Escolha Enroll MOK e depois Continue.

# Step 4: the third screen and the password.
key-enrol-step4 = Escolha Yes e digite a senha que você criou.

# Step 5: the last screen.
key-enrol-step5 = Escolha Reboot. O driver é carregado quando o computador voltar.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Se você perder a tela azul, o computador inicia sem o driver, como antes. Nada está danificado; execute sudo mokutil --import /var/lib/dkms/mok.pub e reinicie para ser perguntado de novo.
