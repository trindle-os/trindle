
# machine translated (Spanish), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = Aprobar la clave de un controlador al reiniciar

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = arranque seguro secure boot clave mok enroll enrol pantalla azul contraseña controlador driver nvidia virtualbox cámara dkms mokutil pantalla negra

# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = El controlador de NVIDIA se compila en este equipo, así que el arranque seguro no lo cargará hasta que apruebes su clave. Hazlo antes de reiniciar, o la pantalla podría quedarse en negro.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = Un controlador adicional se compila en este equipo, así que el arranque seguro no lo cargará hasta que apruebes su clave. Hazlo antes de reiniciar.

# SAFETY: needs a human review before it ships
# Step 1: the password, made before the restart.
key-enrol-step1 = Inventa una contraseña solo con letras y dígitos. La pantalla azul usa un teclado estadounidense, así que los símbolos pueden estar en otras teclas. Anótala; se usa una sola vez.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Reinicia. Antes de que arranque Trindle aparece una pantalla azul titulada Enroll MOK. Se rinde pasados unos 10 segundos, así que ten lista una tecla.

# Step 3: the first two screens.
key-enrol-step3 = Elige Enroll MOK y luego Continue.

# SAFETY: needs a human review before it ships
# Step 4: the third screen and the password.
key-enrol-step4 = Elige Yes y escribe la contraseña que inventaste.

# Step 5: the last screen.
key-enrol-step5 = Elige Reboot. El controlador se carga cuando el equipo vuelve a arrancar.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Si pierdes la pantalla azul, el equipo arranca sin el controlador, como antes. No se ha roto nada; ejecuta sudo mokutil --import /var/lib/dkms/mok.pub y reinicia para que te lo pidan de nuevo.

