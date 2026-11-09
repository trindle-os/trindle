# machine translated (Russian), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = Одобрение ключа драйвера при перезагрузке

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen безопасная загрузка ключ синий экран пароль драйвер камера чёрный экран

# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = Собственный драйвер NVIDIA собирается на этом компьютере, поэтому Secure Boot не загрузит его, пока вы не одобрите его ключ. Сделайте это до перезагрузки, иначе экран может остаться чёрным.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = Дополнительный драйвер собирается на этом компьютере, поэтому Secure Boot не загрузит его, пока вы не одобрите его ключ. Сделайте это до перезагрузки.

# Step 1: the password, made before the restart.
# SAFETY: needs a human review before it ships
key-enrol-step1 = Придумайте пароль только из латинских букв и цифр. На синем экране американская раскладка клавиатуры, поэтому символы могут находиться на других клавишах. Запишите пароль: он нужен один раз.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Перезагрузите компьютер. Перед запуском Trindle появится синий экран с заголовком Enroll MOK. Он закрывается примерно через 10 секунд, поэтому будьте готовы нажать клавишу.

# Step 3: the first two screens.
key-enrol-step3 = Выберите Enroll MOK, затем Continue.

# Step 4: the third screen and the password.
# SAFETY: needs a human review before it ships
key-enrol-step4 = Выберите Yes, затем введите пароль, который вы придумали.

# Step 5: the last screen.
key-enrol-step5 = Выберите Reboot. Драйвер загрузится, когда компьютер включится снова.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Если пропустить синий экран, компьютер запустится без драйвера, как раньше. Ничего не сломано; выполните sudo mokutil --import /var/lib/dkms/mok.pub и перезагрузите компьютер, чтобы запрос появился снова.
