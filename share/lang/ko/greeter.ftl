# machine translated (Korean), not yet reviewed by a person.

## The login screen (src/greeter): shown by greetd before anyone is logged in

# Name of the login screen's window (what a screen reader and the window list call it).
greeter-window-title = 로그인
# Sentence at the top of the panel, for someone this computer knows.
greeter-welcome-back = 다시 오신 것을 환영해요.
# Sentence at the top of the panel, when the name has not been typed yet.
greeter-welcome = 환영해요.
# Under the boxes, while there is nothing else to say.
greeter-ready = 비밀번호를 입력하고 Enter를 누르세요.
# Under the boxes, when a paired phone can stand in for the password.
greeter-ready-phone = 비밀번호를 입력하거나, Enter를 눌러 휴대폰으로 로그인하세요.
# Under the boxes, before a name is in.
greeter-ready-name = 이름과 비밀번호를 입력하고 Enter를 누르세요.
# Red line under the boxes after the phone did not log the person in.
greeter-phone-failed = 휴대폰으로 로그인하지 못했어요. 비밀번호를 입력하세요.
# Under the password box for a few seconds, where a phone is paired.
greeter-phone-hint = 휴대폰으로 로그인
# At the foot of the screen, for when nothing else works.
greeter-trouble = 로그인이 안 되나요? Ctrl+Alt+F2를 눌러 텍스트 로그인을 쓰세요.
# Beside the light drawing switch, on a computer with no graphics chip.
greeter-no-chip = Trindle이 그래픽 칩을 찾지 못해서, 데스크톱이 빠르게 반응하도록 더 단순한 그래픽을 쓰고 있어요.
# The main button's name (a round button with an arrow) that logs in.
greeter-sign-in = 로그인
# Red line under the boxes when the name and password are wrong, or the account is unknown.
greeter-no-match = 이름과 비밀번호가 맞지 않아요.
# Red line under the boxes after too many wrong passwords.
greeter-pam-max-tries = 너무 많이 시도했어요. 1분 기다렸다가 다시 시도하세요.
# Red line under the boxes when the account has run out.
greeter-pam-account-expired = 이 계정은 만료되었어요.
# Red line under the boxes when the password must be changed before logging in.
greeter-pam-password-expired = 이 비밀번호는 만료되었어요. 텍스트 콘솔(Ctrl+Alt+F2)에서 로그인해 바꾸세요.
# Red line under the boxes, or the sentence at the top, when the login failed and the system gave no reason.
greeter-login-failed = 로그인하지 못했어요.
# Red line under the name box when Enter is pressed with no name.
greeter-name-first = 먼저 이름을 입력하세요.
# Sentence at the top while waiting for the phone to answer.
greeter-asking-phone = 휴대폰에 묻는 중…
# Sentence at the top while the system checks the password.
greeter-logging-in = 로그인하는 중…
# Typing box for the person's name, shown quietly while empty.
greeter-name-box = 이름
# Typing box for the password, shown quietly while empty.
greeter-password-box = 비밀번호
# Typing box for an extra question the login asked (a code, for one), when the question itself is empty.
greeter-answer-box = 답
# Name and tip of the round button that logs in with a paired phone.
greeter-use-phone = 휴대폰 사용
# Name of the switch for light drawing on a computer with no graphics chip, and the word under it.
greeter-light-drawing = 저사양 그래픽
# Tip of the round button that shows or hides the on-screen keyboard.
greeter-keyboard-tip = 화상 키보드
# Word under the on-screen keyboard button.
greeter-keyboard = 키보드
# Tip and word under the round button that turns the screen reader on or off.
greeter-screen-reader = 화면 낭독기
# Word under a power button after the first press, and its tip.
greeter-press-again = 한 번 더 누르기
# Tip and word under the round button that restarts the computer.
greeter-restart = 다시 시작
# Tip of the restart button after the first press.
greeter-restart-sure = 한 번 더 누르면 다시 시작해요
# Tip and word under the round button that turns the computer off.
greeter-shut-down = 종료
# Tip of the shut down button after the first press.
greeter-shut-down-sure = 한 번 더 누르면 종료해요
# Red line under the boxes when a power button is pressed and the system has no bus to ask.
greeter-no-bus = Trindle이 시스템에 연결할 수 없어요.
# Red line under the boxes when the system refused to restart; the reason follows, as the system worded it.
greeter-restart-failed = 다시 시작하지 못했어요: { $why }
# Red line under the boxes when the system refused to turn off; the reason follows, as the system worded it.
greeter-shut-down-failed = 종료하지 못했어요: { $why }
# Red line under the boxes when the login service could not be spoken to.
greeter-service-no-talk = 로그인 서비스에 연결하지 못했어요.
# Red line under the boxes when the login service was too slow.
greeter-service-timeout = 로그인 서비스가 너무 오래 걸렸어요.
# Red line under the boxes when the login service hung up.
greeter-service-closed = greetd가 연결을 닫았어요.
# Red line under the boxes when the login service's answer was far too large.
greeter-service-too-big = 로그인 서비스가 너무 커서 읽을 수 없는 응답을 보냈어요.
# Red line under the boxes when the login screen was started by hand and not by the login service.
greeter-service-not-started = GREETD_SOCK이 설정되지 않아서 greetd가 이 로그인 화면을 시작하지 않았어요.
# Red line under the boxes when there is no connection to the login service.
greeter-service-no-connection = 연결 없음
