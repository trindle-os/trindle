
# machine translated (Indonesian), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = Menyetujui kunci driver saat memulai ulang

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen layar biru kata sandi kunci kamera layar hitam

# First paragraph before the steps when the driver is NVIDIA's own.
# SAFETY: needs a human review before it ships
key-enrol-why-nvidia = Driver NVIDIA dibangun di komputer ini, jadi Secure Boot tidak akan memuatnya sampai Anda menyetujui kuncinya. Lakukan ini sebelum memulai ulang, atau layar bisa tetap hitam.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
# SAFETY: needs a human review before it ships
key-enrol-why-addon = Driver tambahan dibangun di komputer ini, jadi Secure Boot tidak akan memuatnya sampai Anda menyetujui kuncinya. Lakukan ini sebelum memulai ulang.

# Step 1: the password, made before the restart.
# SAFETY: needs a human review before it ships
key-enrol-step1 = Buat kata sandi yang hanya berisi huruf dan angka. Layar biru memakai papan ketik Amerika, jadi simbol bisa berada di tombol lain. Tuliskan; kata sandi ini dipakai sekali saja.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Mulai ulang. Layar biru berjudul Enroll MOK muncul sebelum Trindle dimulai. Layar ini menyerah setelah sekitar 10 detik, jadi bersiaplah menekan sebuah tombol.

# Step 3: the first two screens.
key-enrol-step3 = Pilih Enroll MOK, lalu Continue.

# Step 4: the third screen and the password.
# SAFETY: needs a human review before it ships
key-enrol-step4 = Pilih Yes, lalu ketik kata sandi yang Anda buat.

# Step 5: the last screen.
key-enrol-step5 = Pilih Reboot. Driver dimuat setelah komputer menyala kembali.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Jika layar biru terlewat, komputer menyala tanpa driver, seperti sebelumnya. Tidak ada yang rusak; jalankan sudo mokutil --import /var/lib/dkms/mok.pub dan mulai ulang untuk ditanya lagi.
