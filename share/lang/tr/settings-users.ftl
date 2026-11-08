
# machine translated (Turkish), not yet reviewed by a person.

# Name of the Users page, in the page list and as its heading.
settings-users-title = Kullanıcılar

# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = hesaplar kişiler parola resim fotoğraf yönetici

# Help line at the foot of the Users page.
# SAFETY: needs a human review before it ships
settings-users-help = Kişi eklemek, parola değiştirmek ve hesap silmek bir yöneticinin parolasını gerektirir.

# Beside a change that is waiting for the administrator's password to be typed.
# SAFETY: needs a human review before it ships
settings-users-waiting = Parola bekleniyor...

# Headline of the Users page when nobody on the computer is an administrator.
# SAFETY: needs a human review before it ships
settings-users-sum-nobody-admin = Burada kimse yönetici değil, bu yüzden herkes için hiçbir şey kurulamaz veya değiştirilemez.

# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = Bu bilgisayarı yalnızca siz kullanıyorsunuz ve yöneticisi sizsiniz.

# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = Bu bilgisayarı yalnızca bir kişi kullanıyor.

# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
        [one] Bu bilgisayarı { $n } kişi kullanıyor ve yönetici sizsiniz.
       *[other] Bu bilgisayarı { $n } kişi kullanıyor ve yönetici sizsiniz.
    }

# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
        [one] Bu bilgisayarı { $n } kişi kullanıyor ve biri yönetici.
       *[other] Bu bilgisayarı { $n } kişi kullanıyor ve biri yönetici.
    }

# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
        [one] Bu bilgisayarı { $n } kişi kullanıyor ve { $a } tanesi yönetici.
       *[other] Bu bilgisayarı { $n } kişi kullanıyor ve { $a } tanesi yönetici.
    }

# Under a person's name on their card when the card is yours.
settings-users-this-is-you = Bu sizsiniz.

# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = Oturum açma adı.

# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = Bir resim seçin

# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = Resimler

# Tip of the round button on a person's card that changes their password.
# SAFETY: needs a human review before it ships
settings-users-change-password = Parolayı değiştir

# Tip of the round button on a person's card that deletes them and their files.
# SAFETY: needs a human review before it ships
settings-users-delete = Dosyalarıyla birlikte sil

# Said beside a person once they are deleted.
# SAFETY: needs a human review before it ships
settings-users-deleted = Silindi

# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
# SAFETY: needs a human review before it ships
settings-users-administrator = Yönetici

# Under the Administrator line: what an administrator can do.
# SAFETY: needs a human review before it ships
settings-users-admin-can = Herkes için bir şeyler kurabilir ve değiştirebilir.

# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
# SAFETY: needs a human review before it ships
settings-users-cannot-take-away = Bunu kendinizden alamazsınız.

# Said beside a person after they are made an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-now = Artık yönetici

# Said beside a person after they stop being an administrator.
# SAFETY: needs a human review before it ships
settings-users-admin-no-longer = Artık yönetici değil

# Help text for a person's card (the information button).
# SAFETY: needs a human review before it ships
settings-users-person-about = Kişinin resmi, parolası ve sizden başka biri için dosyalarıyla birlikte silinmesi, adının yanındaki yuvarlak düğmelerdir. Yönetici program kurabilir ve herkes için bir şeyleri değiştirebilir; bunu kendinizden alamazsınız. Her değişiklik bir yöneticinin parolasını sorar.

# Typing box in the change password form, for your own current password.
# SAFETY: needs a human review before it ships
settings-users-current-password = Şimdiki parola

# Typing box in the change password form for the new password.
# SAFETY: needs a human review before it ships
settings-users-new-password = Yeni parola

# Typing box in the change password form where the new password is typed again.
# SAFETY: needs a human review before it ships
settings-users-again = Yeniden

# Button in the change password form that saves the new password.
settings-users-save = Kaydet

# Spoken name of the Save button in the change password form.
# SAFETY: needs a human review before it ships
settings-users-save-new = Yeni parolayı kaydet

# Tip of the button that closes a form on the Users page without saving.
# SAFETY: needs a human review before it ships
settings-users-cancel = İptal

# Red words in the change password form when your current password was left empty.
# SAFETY: needs a human review before it ships
settings-users-need-current = Kayıtlı oturum açma bilgilerinizin çalışmaya devam etmesi için şimdiki parolanız da gerekir.

# Red words in the change password form when the new password is too short.
# SAFETY: needs a human review before it ships
settings-users-at-least-8 = Lütfen en az 8 karakter.

# Red words in the change password form when the two new passwords differ.
# SAFETY: needs a human review before it ships
settings-users-no-match = İkisi eşleşmiyor.

# Said beside a person once their password is changed.
# SAFETY: needs a human review before it ships
settings-users-password-changed = Parola değiştirildi

# Red words after a password change when the saved logins could not take the new password.
# SAFETY: needs a human review before it ships
settings-users-not-rekeyed = Parola değiştirildi, ancak kayıtlı oturum açma bilgileri hâlâ eskisiyle açılıyor.

# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = Lütfen PNG, JPG veya WebP.

# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = Resim değiştirilemedi.

# Said beside a person once their picture is saved.
settings-users-saved = Kaydedildi

# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = Resim oturum açılırken görünür, ancak diğer programlarınıza verilemedi.

# Heading of the form for adding a person.
settings-users-add-someone = Biri ekle

# Help text for the Add someone form (the information button).
# SAFETY: needs a human review before it ships
settings-users-add-about = Başka biri için yeni hesap: tam adı, küçük harf ve rakamlardan oluşan, harfle başlayan bir oturum açma adı ve en az 8 karakterlik bir parola. Hemen oturum açabilir. Yönetici program kurabilir ve herkes için bir şeyleri değiştirebilir.

# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = Yeni hesap

# Under the New account heading.
# SAFETY: needs a human review before it ships
settings-users-new-account-sub = Bu parolayla hemen oturum açabilir.

# Typing box in the Add someone form for the person's full name.
settings-users-full-name = Tam ad

# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = Oturum açma adı, küçük harf

# Typing box in the Add someone form for the first password.
# SAFETY: needs a human review before it ships
settings-users-password = Parola

# Tip of the button that adds the new person.
settings-users-add-this = Bu kişiyi ekle

# Red words when the login name typed is not allowed.
settings-users-login-rules = Oturum açma adları küçük harf ve rakamlardan oluşur, harfle başlar.

# Red words when the new person's password is too short.
# SAFETY: needs a human review before it ships
settings-users-password-8 = Parola en az 8 karakter olmalı.

# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } zaten var.

# Said beside the Add someone form once the person is added.
settings-users-added = Eklendi

# Heading of the Saved logins card.
settings-users-saved-logins = Kayıtlı oturum açma bilgileri

# Help text for the Saved logins card (the information button).
# SAFETY: needs a human review before it ships
settings-users-keyring-about = Programların sizin için sakladığı parolalar, örneğin e-posta veya Wi-Fi için olanlar, oturum açışınızla açılan bir anahtarlıkta tutulur. Herkes oturum açarken kendiliğinden Trindle'ın anahtarlığına geçer. Sizinki taşınamadıysa taşıma her kayıtlı oturum açma bilgisini kopyalar, sizinkinin açmadığı anahtarlıkların parolasını sorar ve GNOME'un dosyaları olduğu gibi bırakılır.

# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = Trindle'ın anahtarlığında tutuluyor

# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = GNOME'un anahtarlığında tutuluyor

# Under the heading when the logins are kept by Trindle's keyring.
settings-users-opens-login = Oturum açışınızla açılır.

# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = Oturum açışınız bunları taşıyamadı.

# Tip of the button that moves saved logins to Trindle's keyring.
# SAFETY: needs a human review before it ships
settings-users-move-keyring = Trindle'ın anahtarlığına taşı

# Typing box for your password in the form for moving saved logins.
# SAFETY: needs a human review before it ships
settings-users-your-password = Parolanız

# Tip of the button that starts moving the saved logins.
# SAFETY: needs a human review before it ships
settings-users-go-ahead = Devam

# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
# SAFETY: needs a human review before it ships
settings-users-password-for = { $name } için parola

# Red words when the move of saved logins was started without a password.
# SAFETY: needs a human review before it ships
settings-users-your-password-please = Lütfen parolanız.

# Said beside the Saved logins line while they are copied.
settings-users-copying = Kayıtlı oturum açma bilgileriniz kopyalanıyor...

# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = { $n } kopyalandı

# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
# SAFETY: needs a human review before it ships
settings-users-nothing-moved =
    { $n ->
        [one] Henüz hiçbir şey taşınmadı: { $names } kendi parolasına gerek duyuyor.
       *[other] Henüz hiçbir şey taşınmadı: { $names } kendi parolalarına gerek duyuyor.
    }

# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = Yarıda durdu.

# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = Hesaplara bakılıyor…

# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = Kişiler

# Under the Administrator line of the placeholder card.
settings-users-looking = Aranıyor…

