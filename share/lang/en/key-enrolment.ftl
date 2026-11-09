# Words said before a restart that leads into approving a driver's key (the blue Enroll MOK screen), for every path that leads there (src/common/key_enrolment.rs). Shown as steps in the Help topic "secure-boot-key" and wherever a driver that needs a key is offered.

## The steps (src/common/key_enrolment.rs)

# Title of the Help topic about approving a driver's key.
key-enrol-title = Approving a driver's key at restart
# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen
# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = NVIDIA's own driver is built on this computer, so Secure Boot will not load it until you approve its key. Do this before you restart, or the screen may stay black.
# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = An add-on driver is built on this computer, so Secure Boot will not load it until you approve its key. Do this before you restart.
# Step 1: the password, made before the restart.
key-enrol-step1 = Make up a password of letters and digits only. The blue screen has an American keyboard, so symbols may sit on other keys. Write it down; it is used once.
# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Restart. A blue screen headed Enroll MOK appears before Trindle starts. It gives up after about 10 seconds, so be ready to press a key.
# Step 3: the first two screens.
key-enrol-step3 = Choose Enroll MOK, then Continue.
# Step 4: the third screen and the password.
key-enrol-step4 = Choose Yes, then type the password you made.
# Step 5: the last screen.
key-enrol-step5 = Choose Reboot. The driver loads once the computer is back.
# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Miss the blue screen and the computer starts without the driver, as before. Nothing is broken; run sudo mokutil --import /var/lib/dkms/mok.pub and restart to be asked again.
