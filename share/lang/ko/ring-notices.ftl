# machine translated (Korean), not yet reviewed by a person.

# The ring's notices and notification circles: battery card, snooze choices, crash notices, the microphone switch and the magnifier. Part of the ring's own words (see ring.ftl).

## Battery card (src/ring/battery_alert.rs)

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = 배터리 { $pct }%
# Line under the low battery warning's title.
ring-battery-plug-in = 곧 충전기를 꽂아 주세요.
# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = 충전 중
# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = 충전기를 뽑았어요
# Line under the title when the charger comes out.
ring-battery-running = 배터리로 작동 중, { $pct }%

## Notification circles (src/ring/circles.rs)

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10분
# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1시간
# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6시간
# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = 내일 아침
# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = 10분 뒤에 다시 알림
# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = 1시간 뒤에 다시 알림
# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = 6시간 뒤에 다시 알림
# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = 아침에 다시 알림
# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = 메시지로 돌아가기
# Tip of the pin button when the notification is already pinned.
ring-act-unpin = 고정 해제
# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = 고정
# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = 나중에 알림
# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = 모두 지우기
# Tip of the copy button for a moment after the message was copied.
ring-act-copied = 복사했어요
# Tip of the button on a notification circle that copies its message.
ring-act-copy = 메시지 복사

## Crash notices (src/ring/crashes.rs)

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = 눌러서 도우미에게 넘기세요.
# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle
# Name of the desktop part in the crash notice.
ring-crash-desktop = Trindle 데스크톱
# Name of the login screen in the crash notice.
ring-crash-greeter = Trindle 로그인 화면
# Name of the lock screen in the crash notice.
ring-crash-lock = Trindle 잠금 화면
# Name of Settings in the crash notice.
ring-crash-settings = Trindle 설정
# Name of Apps in the crash notice.
ring-crash-apps = Trindle 앱
# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = Trindle { $name }
# Title of the notice that says a program crashed.
ring-crash-stopped = { $name }이(가) 예기치 않게 멈췄어요

## Drives (src/ring/drives.rs)

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = 그 드라이브가 더 이상 없어요
# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = { $name }을(를) 열지 못했어요

## Closing the lid (src/ring/lid.rs)

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = 절전
# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = 최대 절전
# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = 화면 잠금
# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = 아무것도 안 함
# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = 종료
# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = 덮개를 닫을 때 할 일은 설정의 전원에서 고를 수 있어요.

## Magnifier (src/ring/magnify.rs)

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = 화면 아무 곳이나 누르고 있으면 확대돼요
# Screen reader name of the magnifier's glass.
ring-magnify-name = 돋보기
# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = 확대 멈추기

## Microphone switch (src/ring/mic.rs)

# Tip of the switch in the ring that turns the microphone off for every program.
ring-mic-off-tip = 모든 프로그램의 마이크 끄기
# Tip of the switch in the ring that turns the microphone back on.
ring-mic-on-tip = 마이크 켜기
# Line in the ring when something other than the person pressed the microphone switch on.
ring-mic-own-press = 마이크는 직접 켜야만 켜져요.
# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = Trindle이 마이크 스위치를 바꾸지 못했어요: { $why }

## Clicking a notification (src/ring/note_goes.rs)

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = 프로그램을 찾지 못했어요
# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = 다른 프로그램이 보냈어요
# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = 시작하지 못했어요

## Notification stack (src/ring/notes/)

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = 도우미를 위해 복사했어요
# Screen reader name of a notification circle.
ring-notes-name = 알림
# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = 알림을 옮기는 중이에요. 놓으면 내려놓고, Esc를 누르면 제자리로 돌아가요.
# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = 옮겼어요
# Said when Escape put a held notification back.
ring-notes-put-back = 제자리로 돌렸어요
# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = 옮기려면 먼저 고정하세요

## Notification circles (src/ring/notify.rs)

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
       *[other] { $n }초
    }
# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = 닫을 때까지
# Time under a notification from yesterday.
ring-notify-yesterday = 어제 { $clock }
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = 오른쪽 아래
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = 왼쪽 아래
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = 왼쪽 위
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = 오른쪽 위

## Plugins in the ring (src/ring/plugins.rs)

# Title of the notice when a plugin that asks for permissions has been given none.
ring-plugin-needs = { $name }에 권한이 필요해요
# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
ring-plugin-cannot-work =
    { $asks ->
       *[other] 아직 작동하지 못해요. 요청한 권한 { $asks }개를 하나도 허용하지 않았어요.
    }

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = 복사했어요. { $said }

## Programs outside a box (src/ring/outside.rs)

# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = 네트워크에 접속했어요
# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = 내 파일을 열어 두었어요
# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
       *[other] { $names } 외 { $more }개
    }

# Said under the magnifier when the screen picture it took has the wrong size.
ring-magnify-wrong-size = 그림 크기가 맞지 않았어요.
