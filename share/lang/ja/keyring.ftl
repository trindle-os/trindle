# machine translated (Japanese), not yet reviewed by a person.

# Trindle's keyring: the password window it opens, and the words it gives
# back to the person who typed the password. The keyring's own error names
# and the faults sent back to the program that asked are not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The password window (src/keyring/prompt.rs)

# Window title when the keyring is started for the first time, with no keyring yet.
keyring-title-start = キーリングを開始
# Window title when the keyring is there and needs its password to open.
keyring-title-unlock = キーリングのロックを解除
# Heading over the password box, and the tip shown in the box while it is empty.
keyring-password = パスワード
# Main button at the bottom of the password window that checks the password.
keyring-unlock = ロック解除
# Button at the bottom of the password window that closes it without unlocking.
keyring-cancel = キャンセル
# Shown under the password box when Unlock is pressed with the box empty.
keyring-type-first = 先にパスワードを入力してください
# Shown under the password box while the password is being checked.
keyring-checking = 確認中…

## Why the password window is up (src/keyring/service.rs, why)

# Under the title when no keyring exists yet and a program wants to keep a password.
keyring-why-none = あるプログラムがパスワードを安全に保管しようとしています。ログインパスワードを入力して、キーリングを開始してください。次回からは、ログイン時に自動で開きます。
# Under the title when the keyring was made with a different password from the login one.
keyring-why-old = ログインパスワードは、このキーリングを作ったときのパスワードと違います。以前のパスワードを 1 回入力してください。その後は、ログイン時にキーリングが自動で開きます。
# Under the title when no password was typed at log in, for example with a fingerprint or automatic log in.
keyring-why-no-login = ログイン時にパスワードを入力しなかった (指紋認証か自動ログインを使った) ため、キーリングが自動で開けませんでした。ログインパスワードを入力してください。
# Under the title when a program asks for a saved password and the keyring is shut.
keyring-why-wanted = あるプログラムが保存したパスワードを必要としています。ログインパスワードを入力して、キーリングのロックを解除してください。

# Under the reason, naming the program that asked, when the keyring can find its name.
keyring-asked-by = 要求元: { $program }
# Told to a program that asked while the keyring was taking over from the one before and many calls already waited.
keyring-starting = キーリングを起動しています。もう一度お試しください。
# The program in "Asked for by", for a program on the bus: $name is the name it gives itself, which anything can claim.
keyring-named-itself = { $name } と名乗るプログラム

## Words under the password box after a check (src/keyring/service.rs)

# Shown under the box when the keyring has stopped before it could check the password.
keyring-stopped = キーリングが停止しました
# Shown under the box when the password does not open the keyring.
keyring-not-the-keyring-password = キーリングのパスワードではありません
# Shown under the box when the keyring file is damaged and has been left as it is.
keyring-damaged-left = キーリングのファイルが壊れていますが、Trindle は変更していません
# Shown under the box when the password is not the person's login password.
keyring-not-login-password = ログインパスワードではありません
# Said to a program that gave a password with letters the keyring can't read, so no keyring is made with it.
keyring-password-unreadable = そのパスワードには、キーリングが読めない文字が含まれています

## Moving the keyring from Settings (src/keyring/migrate.rs)

# Shown in Settings when the password typed there is not the person's password.
keyring-not-your-password = あなたのパスワードではありません。
# Shown in Settings when the old keyring is still running after its time to stop has run out.
keyring-would-not-stop = 古いキーリングが停止しませんでした。

## Words the keyring gives when it is asked (src/keyring/store.rs, Error)

# Said when the keyring is locked and cannot be used.
keyring-error-locked = キーリングがロックされています
# Said when the password given does not open the keyring.
keyring-error-wrong-password = そのパスワードではキーリングを開けません
# Said when the collection or item asked for is not there.
keyring-error-no-such = その項目は存在しません
# Said when the keyring file cannot be read as a keyring.
keyring-error-damaged = キーリングのファイルが壊れています
