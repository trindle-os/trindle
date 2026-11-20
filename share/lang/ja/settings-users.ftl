# machine translated (Japanese), not yet reviewed by a person.

# Settings: the Users page (src/settings/panels/users.rs).

## The Users page (src/settings/panels/users.rs)

# Name of the Users page, in the page list and as its heading.
settings-users-title = ユーザー
# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = アカウント ユーザー 人 パスワード 写真 画像 管理者 accounts people password picture photo administrator
# Help line at the foot of the Users page.
settings-users-help = 人の追加、パスワードの変更、アカウントの削除には、管理者のパスワードが必要です。
# Beside a change that is waiting for the administrator's password to be typed.
settings-users-waiting = パスワードの入力を待っています…
# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = 管理者がいないため、全員のためにインストールや変更をできる人がいません。
# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = このコンピューターを使うのはあなただけで、あなたが管理者です。
# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = このコンピューターを使うのは1人だけです。
# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
       *[other] このコンピューターを使うのは { $n } 人で、あなたが管理者です。
    }
# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
       *[other] このコンピューターを使うのは { $n } 人で、そのうち1人が管理者です。
    }
# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
       *[other] このコンピューターを使うのは { $n } 人で、そのうち { $a } 人が管理者です。
    }
# Under a person's name on their card when the card is yours.
settings-users-this-is-you = あなたです。
# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = ログイン名
# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = 写真を選ぶ
# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = 写真
# Tip of the round button on a person's card that changes their password.
settings-users-change-password = パスワードを変更
# Tip of the round button on a person's card that deletes them and their files.
settings-users-delete = アカウントとファイルを削除
# Said beside a person once they are deleted.
settings-users-deleted = 削除しました
# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
settings-users-administrator = 管理者
# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
settings-users-cannot-take-away = 自分の管理者権限は外せません。
# Said beside a person after they are made an administrator.
settings-users-admin-now = 管理者になりました
# Said beside a person after they stop being an administrator.
settings-users-admin-no-longer = 管理者ではなくなりました
# Help text for a person's card (the information button).
settings-users-person-about = 管理者は、プログラムをインストールしたり、全員のために設定を変えたりできます。自分の管理者権限は外せません。変更のたびに、管理者のパスワードが必要です。
# Typing box in the change password form, for your own current password.
settings-users-current-password = 現在のパスワード
# Typing box in the change password form for the new password.
settings-users-new-password = 新しいパスワード
# Typing box in the change password form where the new password is typed again.
settings-users-again = もう一度入力
# Button in the change password form that saves the new password.
settings-users-save = 保存
# Spoken name of the Save button in the change password form.
settings-users-save-new = 新しいパスワードを保存
# Tip of the button that closes a form on the Users page without saving.
settings-users-cancel = キャンセル
# Red words in the change password form when your current password was left empty.
settings-users-need-current = 保存したログインが使い続けられるよう、現在のパスワードも入力してください。
# Red words in the change password form when the new password is too short.
settings-users-at-least-8 = 8文字以上にしてください。
# Red words in the change password form when the two new passwords differ.
settings-users-no-match = 2つが一致しません。
# Said beside a person once their password is changed.
settings-users-password-changed = パスワードを変更しました
# Red words after a password change when the saved logins could not take the new password.
settings-users-not-rekeyed = パスワードは変更しましたが、保存したログインは古いパスワードのままです。
# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = PNG、JPG、WebPのいずれかにしてください。
# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = 写真を変更できませんでした。
# Said beside a person once their picture is saved.
settings-users-saved = 保存しました
# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = 写真はログイン画面に出ますが、Trindleは他のプログラムと共有できませんでした。
# Heading of the form for adding a person.
settings-users-add-someone = 人を追加
# Help text for the Add someone form (the information button).
settings-users-add-about = フルネーム、ログイン名、8文字以上のパスワードを入力してください。ログイン名に使えるのは小文字と数字だけで、最初は文字にします。すぐにログインできます。管理者は、プログラムをインストールしたり、全員のために設定を変えたりできます。
# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = 新しいアカウント
# Typing box in the Add someone form for the person's full name.
settings-users-full-name = フルネーム
# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = ログイン名(小文字)
# Typing box in the Add someone form for the first password.
settings-users-password = パスワード
# Tip of the button that adds the new person.
settings-users-add-this = この人を追加
# Red words when the login name typed is not allowed.
settings-users-login-rules = ログイン名は、小文字と数字で、最初は文字にします。
# Red words when the new person's password is too short.
settings-users-password-8 = パスワードは8文字以上必要です。
# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name } はすでにあります。
# Said beside the Add someone form once the person is added.
settings-users-added = 追加しました
# Heading of the Saved logins card.
settings-users-saved-logins = 保存したログイン
# Help text for the Saved logins card (the information button).
settings-users-keyring-about = プログラムは、メールやWi-Fiなどのパスワードを、ログイン時に開くキーリングに保管します。ログインすると、Trindleはキーリングを自分のものに移します。うまくいかなかったときは、ここで移せます。保存したログインはすべてコピーされます。ログインでは開かないキーリングのパスワードを求められることがあります。GNOMEのファイルはそのまま残ります。
# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = Trindleのキーリング内
# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = GNOMEのキーリング内
# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = ログイン時にTrindleは移せませんでした。
# Tip of the button that moves saved logins to Trindle's keyring.
settings-users-move-keyring = Trindleのキーリングへ移す
# Typing box for your password in the form for moving saved logins.
settings-users-your-password = あなたのパスワード
# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = 今すぐ移す
# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
settings-users-password-for = { $name } のパスワード
# Red words when the move of saved logins was started without a password.
settings-users-your-password-please = パスワードを入力してください。
# Said beside the Saved logins line while they are copied.
settings-users-copying = 保存したログインをコピーしています…
# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = { $n } 件をコピーしました
# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
settings-users-nothing-moved =
    { $n ->
       *[other] まだ何も移していません。{ $names } のパスワードを入力してください。
    }
# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = 最後まで終わりませんでした。
# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = アカウントを確認しています…
# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = ユーザー
# Under the Administrator line of the placeholder card.
settings-users-looking = 読み込み中…
