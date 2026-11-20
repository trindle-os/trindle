# machine translated (Korean), not yet reviewed by a person.

# Trindle's session: the lock screen's words (the password and PIN card, the
# phone check and what it says when it fails), the "One moment, please" card
# shown while the desktop comes up, and the note on wallpapers that cost
# more memory than they need.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The lock screen (src/session/lock.rs)

# Heading of the card when the PIN is the way in.
session-lock-enter-pin = PIN을 입력하세요
# Heading of the card when the password is the way in.
session-lock-locked = 잠김
# Under the heading of the password card: what to do.
session-lock-type-password = 비밀번호를 입력하고 Enter를 누르세요
# Button under the PIN card that changes to typing the password.
session-lock-use-password = 비밀번호 사용
# Button under the password card that changes to typing a PIN.
session-lock-use-pin = PIN 사용
# Name shown beside the phone button, to unlock with the person's phone.
session-lock-use-phone = 휴대폰 사용
# Shown while the typed password or PIN is being checked.
session-lock-checking = 확인 중…
# Shown when the PIN has run out of time and the password is asked for instead.
session-lock-pin-timed-out = PIN 입력 시간이 지났어요. 비밀번호를 입력하세요.
# Shown after too many wrong PINs, and the password is asked for instead.
session-lock-too-many-pins = PIN이 너무 많이 틀렸어요. 비밀번호를 사용하세요.
# Shown after a wrong PIN, with how many tries are left (1 is one try left).
session-lock-wrong-pin =
    { $n ->
       *[other] PIN이 틀렸어요. { $n }번 남았어요.
    }
# Shown while the phone is asked to unlock the screen.
session-lock-asking-phone = 휴대폰에 물어보는 중…
# Shown when the phone did not unlock the screen and the PIN may be typed.
session-lock-phone-no-pin = Trindle이 휴대폰으로 잠금을 풀지 못했어요. PIN을 사용하세요.
# Shown when the phone did not unlock the screen and the password must be typed.
session-lock-phone-no-password = Trindle이 휴대폰으로 잠금을 풀지 못했어요. 비밀번호를 입력하세요.
# Shown when the password typed is not the person's password.
session-lock-wrong-password = 비밀번호가 맞지 않아요.

## One moment, please (src/session/starting.rs)

# The card shown while the desktop comes up, with a ring turning.
session-one-moment = 잠시만 기다려 주세요

## Wallpapers that cost more than they need (src/session/wallpaper_limits.rs)

# Said beside a picture bigger than the screen, which uses more memory than it needs.
session-wallpaper-bigger-than-screen = 이 사진은 화면보다 커서 메모리를 더 써요.
# Said beside a moving wallpaper, which uses more memory and battery than a still one.
session-wallpaper-moving-costs = 움직이는 배경화면은 메모리와 배터리를 더 써요.
# A reason a wallpaper was left out: the picture is larger than 4K.
session-wallpaper-larger-4k = 4K보다 큼
# A reason a wallpaper was left out: the film is bigger than the screen.
session-wallpaper-larger-screen = 화면보다 큼
# A reason a wallpaper was left out: its file is over 20 MB, or over 200 MB if it moves.
session-wallpaper-over-20-or-200 = 20 MB 초과 (움직이는 것은 200 MB 초과)
# A reason a wallpaper was left out: its file is over 20 MB.
session-wallpaper-over-20 = 20 MB 초과
# A reason a wallpaper was left out: its file is over 200 MB.
session-wallpaper-over-200 = 200 MB 초과
# Two reasons joined, as in "larger than 4K or over 20 MB"; the two come in.
session-wallpaper-reasons-or = { $first } 또는 { $second }
# The line that says how many wallpapers were left out and why; the count and the reasons come in.
session-wallpaper-left-out =
    { $count ->
       *[other] Trindle이 사진 { $count }장을 뺐어요. 이유: { $why }.
    }

# Under the lock screen when a phone or password check was given up on.
session-lock-check-abandoned = 시간이 너무 오래 걸렸어요. 다시 시도하세요.
# Under the lock screen when Enter is pressed while the last try, given up on, is still being checked.
session-lock-still-checking = 지난 시도를 아직 확인하는 중이에요. 잠시 뒤에 다시 시도하세요.
# Under the lock screen after several wrong tries, with how many seconds to wait (1 is one second).
session-lock-wait =
    { $n ->
       *[other] 너무 많이 틀렸어요. { $n }초 뒤에 다시 시도하세요.
    }

## The question before everything is restarted (src/session/restart_ask.rs)

# Heading of the question asked by Ctrl+Alt+Shift+Esc and by the Restart everything button in Settings.
session-restart-question = 데스크톱 전체를 다시 시작할까요?
# Under the heading: what restarting everything does to the open windows.
session-restart-windows = 열려 있는 창이 모두 닫혀요. 저장하지 않은 내용은 사라져요.
# Tip of the round button that leaves everything as it is.
session-restart-cancel = 취소
# Main button that restarts the whole desktop.
session-restart-go = 다시 시작
