# machine translated (Turkish), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = Yeniden başlatırken bir sürücünün anahtarını onaylama

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen güvenli önyükleme anahtar mavi ekran parola şifre sürücü kamera siyah ekran

# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = NVIDIA'nın kendi sürücüsü bu bilgisayarda derlenir, bu yüzden Güvenli Önyükleme anahtarını onaylayana kadar yüklemez. Bunu yeniden başlatmadan önce yapın, yoksa ekran siyah kalabilir.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = Eklenti sürücüsü bu bilgisayarda derlenir, bu yüzden Güvenli Önyükleme anahtarını onaylayana kadar yüklemez. Bunu yeniden başlatmadan önce yapın.

# Step 1: the password, made before the restart.
# SAFETY: needs a human review before it ships
key-enrol-step1 = Yalnızca harf ve rakamlardan oluşan bir parola belirleyin. Mavi ekranda Amerikan klavyesi vardır, bu yüzden simgeler başka tuşlarda olabilir. Parolayı yazın; yalnızca bir kez kullanılır.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Yeniden başlatın. Trindle başlamadan önce Enroll MOK başlıklı mavi bir ekran görünür. Yaklaşık 10 saniye sonra vazgeçer, bu yüzden bir tuşa basmaya hazır olun.

# Step 3: the first two screens.
key-enrol-step3 = Enroll MOK'u, ardından Continue'yu seçin.

# Step 4: the third screen and the password.
# SAFETY: needs a human review before it ships
key-enrol-step4 = Yes'i seçin, sonra belirlediğiniz parolayı yazın.

# Step 5: the last screen.
key-enrol-step5 = Reboot'u seçin. Bilgisayar açılınca sürücü yüklenir.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Mavi ekranı kaçırırsanız bilgisayar eskisi gibi sürücü olmadan başlar. Hiçbir şey bozulmaz; yeniden sorulması için sudo mokutil --import /var/lib/dkms/mok.pub komutunu çalıştırın ve yeniden başlatın.
