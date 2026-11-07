
# machine translated (Turkish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Heading of the card when the PIN is the way in.
session-lock-enter-pin = PIN'inizi girin

# Heading of the card when the password is the way in.
session-lock-locked = Kilitli

# SAFETY: needs a human review before it ships
# Under the heading of the password card: what to do.
session-lock-type-password = Parolanızı yazın, sonra Enter tuşuna basın

# SAFETY: needs a human review before it ships
# Button under the PIN card that changes to typing the password.
session-lock-use-password = Parola kullan

# SAFETY: needs a human review before it ships
# Button under the password card that changes to typing a PIN.
session-lock-use-pin = PIN kullan

# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = Telefon kullan

# Shown while the typed password or PIN is being checked.
session-lock-checking = Denetleniyor...

# SAFETY: needs a human review before it ships
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = PIN'in süresi doldu; parolanızı yazın.

# SAFETY: needs a human review before it ships
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = Çok fazla yanlış PIN girildi; parolanızı yazın.

# SAFETY: needs a human review before it ships
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
        [one] PIN yanlış. { $n } deneme hakkı kaldı.
       *[other] PIN yanlış. { $n } deneme hakkı kaldı.
    }

# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = Telefonunuza soruluyor...

# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = Telefonunuz kilidi açmadı. PIN'inizi kullanın.

# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = Telefonunuz kilidi açmadı. Parolanızı yazın.

# SAFETY: needs a human review before it ships
# Shown when the password typed is not the person's password.
session-lock-wrong-password = Bu parola doğru değil.

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = Lütfen bir dakika

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = Bu resim ekranınızdan büyük ve fazladan bellek kullanıyor.

# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = Hareketli duvar kâğıtları daha fazla bellek ve pil kullanır.

# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = 4K'dan büyük

# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = ekranınızdan büyük

# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = 20 MB üzeri (hareketliyse 200 MB)

# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = 20 MB üzeri

# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = 200 MB üzeri

# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } veya { $second }

# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out = { $count } tanesi dışarıda bırakıldı: { $why }.

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = Bu çok uzun sürdü. Tekrar deneyin.



# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = Tüm masaüstü yeniden başlatılsın mı?

# Under the heading: what restarting everything does to the open windows.
session-restart-windows = Açık her pencere kapanır ve içlerinde kaydedilmeyen her şey kaybolur.

# Tip of the round button that leaves everything as it is.
session-restart-cancel = İptal

# Main button that restarts the whole desktop.
session-restart-go = Yeniden başlat

