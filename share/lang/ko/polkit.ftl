# machine translated (Korean), not yet reviewed by a person.

# Trindle's administrator password question, the box that asks for the
# password when a step needs the administrator (polkit's agent in src/polkit).
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The password question (src/polkit/mod.rs)

# Heading of the card asking for the administrator's password.
polkit-title = 관리자 비밀번호가 필요해요
# Under the heading: whose password the box is for; the person's name comes in.
polkit-password-for = { $user }의 비밀번호
# Button that lets the step go ahead once the password is typed.
polkit-allow = 허용
# Button that stops the step and closes the question.
polkit-cancel = 취소
# Said when the password typed is not accepted, and the question is asked again.
polkit-not-accepted = 비밀번호가 맞지 않아요. 다시 시도하세요.
# Name a screen reader speaks for the password question window.
polkit-spoken = 내 비밀번호
# Under the password box, when other requests for a password are waiting for
# this one to end. { $n } is how many.
polkit-waiting-behind =
    { $n ->
       *[other] 요청 { $n }개가 더 기다리고 있어요.
    }

## Why Drives asks (src/polkit/mod.rs)

# Under the heading, when Drives asks for the password to take the snapshot before making room.
polkit-reason-drives-snapshot = Drives가 공간을 만들기 전에 스냅샷을 찍으려면 관리자 비밀번호가 필요해요.
# Under the heading, when Drives asks for the password to switch a swap file off before making room.
polkit-reason-drives-swap-off = Drives가 공간을 만드는 동안 스왑 파일을 끄려면 관리자 비밀번호가 필요해요.
# Under the heading, when Drives asks for the password to switch a swap file back on.
polkit-reason-drives-swap-on = Drives가 스왑 파일을 다시 켜려면 관리자 비밀번호가 필요해요.
