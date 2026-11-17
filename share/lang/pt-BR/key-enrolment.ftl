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
key-enrol-step1 = Crie uma senha usando só letras e números. A tela azul usa um layout de teclado americano, então os símbolos podem estar em outras teclas. Anote a senha. Você só vai precisar dela uma vez.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Reinicie. Uma tela azul chamada Enroll MOK aparece antes de o Trindle iniciar. Você tem cerca de 10 segundos para apertar uma tecla, então esteja pronto.

# Step 3: the first two screens.
key-enrol-step3 = Escolha Enroll MOK e depois Continue.

# Step 4: the third screen and the password.
key-enrol-step4 = Escolha Yes e digite a senha que você criou.

# Step 5: the last screen.
key-enrol-step5 = Escolha Reboot. O driver será carregado quando o computador reiniciar.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Se você perder a tela azul, o computador inicia sem o driver, como antes. Nada está danificado. Para ser perguntado de novo, execute sudo mokutil --import /var/lib/dkms/mok.pub e reinicie.
