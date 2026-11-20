# machine translated (Korean), not yet reviewed by a person.

# Trindle's keyring: the password window it opens, and the words it gives
# back to the person who typed the password. The keyring's own error names
# and the faults sent back to the program that asked are not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The password window (src/keyring/prompt.rs)

# Window title when the keyring is started for the first time, with no keyring yet.
keyring-title-start = 키링 시작
# Window title when the keyring is there and needs its password to open.
keyring-title-unlock = 키링 잠금 해제
# Heading over the password box, and the tip shown in the box while it is empty.
keyring-password = 비밀번호
# Main button at the bottom of the password window that checks the password.
keyring-unlock = 잠금 해제
# Button at the bottom of the password window that closes it without unlocking.
keyring-cancel = 취소
# Shown under the password box when Unlock is pressed with the box empty.
keyring-type-first = 먼저 비밀번호를 입력하세요
# Shown under the password box while the password is being checked.
keyring-checking = 확인 중…

## Why the password window is up (src/keyring/service.rs, why)

# Under the title when no keyring exists yet and a program wants to keep a password.
keyring-why-none = 프로그램이 비밀번호를 안전하게 보관하려고 해요. 로그인 비밀번호를 입력해 키링을 시작하세요. 앞으로는 로그인할 때 저절로 열려요.
# Under the title when the keyring was made with a different password from the login one.
keyring-why-old = 로그인 비밀번호가 이 키링을 만들 때의 비밀번호와 달라요. 예전 비밀번호를 한 번 입력하세요. 그 뒤로는 로그인할 때 키링이 저절로 열려요.
# Under the title when no password was typed at log in, for example with a fingerprint or automatic log in.
keyring-why-no-login = 로그인할 때 비밀번호를 입력하지 않아서(지문이나 자동 로그인을 썼어요) 키링이 저절로 열리지 못했어요. 로그인 비밀번호를 입력하세요.
# Under the title when a program asks for a saved password and the keyring is shut.
keyring-why-wanted = 프로그램이 저장된 비밀번호를 요청해요. 로그인 비밀번호를 입력해 키링을 잠금 해제하세요.

# Under the reason, naming the program that asked, when the keyring can find its name.
keyring-asked-by = 요청한 프로그램: { $program }
# Told to a program that asked while the keyring was taking over from the one before and many calls already waited.
keyring-starting = 키링을 시작하고 있어요. 다시 시도하세요.
# The program in "Asked for by", for a program on the bus: $name is the name it gives itself, which anything can claim.
keyring-named-itself = 스스로 { $name }(이)라고 하는 프로그램

## Words under the password box after a check (src/keyring/service.rs)

# Shown under the box when the keyring has stopped before it could check the password.
keyring-stopped = 키링이 멈췄어요
# Shown under the box when the password does not open the keyring.
keyring-not-the-keyring-password = 키링의 비밀번호가 아니에요
# Shown under the box when the keyring file is damaged and has been left as it is.
keyring-damaged-left = 키링 파일이 손상되었지만 Trindle은 바꾸지 않았어요
# Shown under the box when the password is not the person's login password.
keyring-not-login-password = 로그인 비밀번호가 아니에요
# Said to a program that gave a password with letters the keyring can't read, so no keyring is made with it.
keyring-password-unreadable = 키링이 읽을 수 없는 문자가 비밀번호에 있어요

## Moving the keyring from Settings (src/keyring/migrate.rs)

# Shown in Settings when the password typed there is not the person's password.
keyring-not-your-password = 내 비밀번호가 아니에요.
# Shown in Settings when the old keyring is still running after its time to stop has run out.
keyring-would-not-stop = 이전 키링이 멈추지 않았어요.

## Words the keyring gives when it is asked (src/keyring/store.rs, Error)

# Said when the keyring is locked and cannot be used.
keyring-error-locked = 키링이 잠겨 있어요
# Said when the password given does not open the keyring.
keyring-error-wrong-password = 그 비밀번호로는 키링이 열리지 않아요
# Said when the collection or item asked for is not there.
keyring-error-no-such = 그 항목이 없어요
# Said when the keyring file cannot be read as a keyring.
keyring-error-damaged = 키링 파일이 손상되었어요
