# Trindle's keyring: the password window it opens, and the words it gives
# back to the person who typed the password. The keyring's own error names
# and the faults sent back to the program that asked are not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The password window (src/keyring/prompt.rs)

# Window title when the keyring is started for the first time, with no keyring yet.
keyring-title-start = Start the keyring
# Window title when the keyring is there and needs its password to open.
keyring-title-unlock = Unlock the keyring
# Heading over the password box, and the tip shown in the box while it is empty.
keyring-password = Password
# Main button at the bottom of the password window that checks the password.
keyring-unlock = Unlock
# Button at the bottom of the password window that closes it without unlocking.
keyring-cancel = Cancel
# Shown under the password box when Unlock is pressed with the box empty.
keyring-type-first = Type the password first
# Shown under the password box while the password is being checked.
keyring-checking = Checking…

## Why the password window is up (src/keyring/service.rs, why)

# Under the title when no keyring exists yet and a program wants to keep a password.
keyring-why-none = A program wants to keep a password safe. Type your login password to start the keyring; from now on it opens by itself when you log in.
# Under the title when the keyring was made with a different password from the login one.
keyring-why-old = Your login password is not the one this keyring was made with. Type the old one once; after that it opens by itself at log in.
# Under the title when no password was typed at log in, for example with a fingerprint or automatic log in.
keyring-why-no-login = No password was typed at log in (a fingerprint or automatic log in), so the keyring could not open by itself. Type your login password.
# Under the title when a program asks for a saved password and the keyring is shut.
keyring-why-wanted = A program wants a saved password. Type your login password to unlock the keyring.

# Under the reason, naming the program that asked, when the keyring can find its name.
keyring-asked-by = Asked for by { $program }
# Told to a program that asked while the keyring was taking over from the one before and many calls already waited.
keyring-starting = The keyring is starting. Ask again.
# The program in "Asked for by", for a program on the bus: $name is the name it gives itself, which anything can claim.
keyring-named-itself = a program that calls itself { $name }

## Words under the password box after a check (src/keyring/service.rs)

# Shown under the box when the keyring has stopped before it could check the password.
keyring-stopped = The keyring has stopped
# Shown under the box when the password does not open the keyring.
keyring-not-the-keyring-password = That is not the keyring's password
# Shown under the box when the keyring file is damaged and has been left as it is.
keyring-damaged-left = The keyring file is damaged, so it has been left alone
# Shown under the box when the password is not the person's login password.
keyring-not-login-password = That is not your login password

## Moving the keyring from Settings (src/keyring/migrate.rs)

# Shown in Settings when the password typed there is not the person's password.
keyring-not-your-password = That is not your password.
# Shown in Settings when the old keyring is still running after its time to stop has run out.
keyring-would-not-stop = The old keyring would not stop.

## Words the keyring gives when it is asked (src/keyring/store.rs, Error)

# Said when the keyring is locked and cannot be used.
keyring-error-locked = The keyring is locked
# Said when the password given does not open the keyring.
keyring-error-wrong-password = That password does not open the keyring
# Said when the collection or item asked for is not there.
keyring-error-no-such = No such collection or item
# Said when the keyring file cannot be read as a keyring.
keyring-error-damaged = The keyring file is damaged
