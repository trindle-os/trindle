# machine translated (Korean), not yet reviewed by a person.

# Settings page "Power": battery, power mode, screen dimming and locking, PIN, log out and shut down; also the PIN time names from src/common/pin.rs.

## Page (src/settings/panels/power.rs)

# Name of the Settings page about power, in the list and as its heading.
settings-power-title = 전원
# Words a person might type into the Settings search to find the power page; separate them with spaces.
settings-power-keywords = 배터리 절전 잠자기 종료 다시 시작 로그아웃 화면 꺼짐 유휴 어둡게 잠금 비밀번호 PIN 키패드 카페인 화면 켜 두기 충전 제한 80 배터리 상태 battery suspend sleep shut down restart log out screen off idle dim lock password pin keypad caffeine keep awake charge charging limit 80 battery health
# Introduction under the heading of the power page, and the words behind the page i.
settings-power-help = 전원 모드를 고르고, 화면이 언제 어두워지고 잠기고 꺼질지 정해요. 노트북에서는 배터리일 때와 충전기를 꽂았을 때를 따로 정해요. 링 안의 컵은 화면을 켜 둬요. 여기서 잠금, 다시 시작, 종료도 할 수 있어요.

## Times in the lists

# Choice in a list of how long to wait.
settings-power-time-never = 안 함
# Choice in a list of how long to wait.
settings-power-time-1m = 1분
# Choice in a list of how long to wait.
settings-power-time-2m = 2분
# Choice in a list of how long to wait.
settings-power-time-5m = 5분
# Choice in a list of how long to wait.
settings-power-time-10m = 10분
# Choice in a list of how long to wait.
settings-power-time-15m = 15분
# Choice in a list of how long to wait.
settings-power-time-30m = 30분
# Choice in a list of how long to wait.
settings-power-time-45m = 45분
# Choice in a list of how long to wait.
settings-power-time-1h = 1시간
# Choice in a list of how long to wait.
settings-power-time-90m = 1시간 30분
# Choice in a list of how long to wait.
settings-power-time-2h = 2시간
# Choice in the lock after list: lock at no other time than when the computer sleeps.
settings-power-time-only-sleep = 절전할 때만

## Session buttons and lid

# Heading of the line, and name of its round button, that suspends the computer.
settings-power-suspend = 절전
# Heading of the line, and name of its round button, that restarts the computer.
settings-power-restart = 다시 시작
# Heading of the line, and name of its round button, that shuts the computer down.
settings-power-shut-down = 종료
# Choice in the list of what closing the lid does: the computer sleeps.
settings-power-lid-sleep = 절전
# Choice in the list of what closing the lid does: the computer hibernates.
settings-power-lid-hibernate = 최대 절전
# Choice in the list of what closing the lid does: the screen locks.
settings-power-lid-lock = 화면 잠금
# Choice in the list of what closing the lid does: nothing.
settings-power-lid-nothing = 아무 일도 안 함
# Choice in the list of what closing the lid does: the computer shuts down.
settings-power-lid-shut-down = 종료

## Idle timers

# Heading of an idle timer line, and the name of its lists.
settings-power-timer-dim = 화면을 어둡게 할 때까지
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-clock = 움직이는 시계를 보여 줄 때까지
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-off = 화면을 끌 때까지
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-suspend = 절전할 때까지

## Warnings and states

# Red words under the keep the screen awake switch when turning it on could not be saved.
settings-power-no-awake = 선택을 저장하지 못해서 Trindle이 화면을 켜 두지 못했어요.
# Red words under the keep the screen awake switch when turning it off could not be saved.
settings-power-no-sleep = Trindle이 선택을 저장하지 못해서 화면이 계속 켜져 있어요.
# Name of the power mode that saves the battery.
settings-power-profile-saver = 배터리 절약
# Name of the balanced power mode.
settings-power-profile-balanced = 균형
# Name of the power mode that favours speed.
settings-power-profile-performance = 성능
# What the battery is doing: charging.
settings-power-state-charging = 충전 중
# What the battery is doing: running on its own charge.
settings-power-state-battery = 배터리 사용 중
# What the battery is doing: empty.
settings-power-state-empty = 방전됨
# What the battery is doing: full.
settings-power-state-full = 충전 완료
# What the battery is doing: the charger is in but it is not charging.
settings-power-state-plugged = 연결됨, 충전 안 함
# Time left on the battery; { $h } hours and { $m } minutes. The h and min are short for hours and minutes.
settings-power-hours-minutes = { $h }시간 { $m }분
# Time left on the battery when under an hour; { $m } minutes. The min is short for minutes.
settings-power-minutes = { $m }분

## Top sentence

# Sentence at the top of the page on a computer with no battery.
settings-power-top-no-battery = 전원 연결됨. 이 컴퓨터에는 배터리가 없어요.
# Sentence at the top of the page on a computer with no battery; { $mode } is the power mode.
settings-power-top-no-battery-mode = 전원 연결됨, { $mode } 모드. 이 컴퓨터에는 배터리가 없어요.
# Sentence at the top of the page while charging; { $percent } is how full.
settings-power-top-charging = 충전 중, { $percent }%.
# Sentence at the top of the page while charging; { $percent } is how full and { $left } how long until full.
settings-power-top-charging-left = 충전 중, { $percent }%, 완충까지 { $left }.
# Sentence at the top of the page when the battery is full.
settings-power-top-full = 배터리가 가득 찼어요.
# Sentence at the top of the page when plugged in but not charging; { $percent } is how full.
settings-power-top-plugged = 전원 연결됨, { $percent }%, 충전 안 함.
# Sentence at the top of the page on battery; { $percent } is how full.
settings-power-top-battery = 배터리 { $percent }%.
# Sentence at the top of the page on battery; { $percent } is how full and { $left } how long is left.
settings-power-top-battery-left = 배터리 { $percent }%, 약 { $left } 남음.
# Sentence at the top of the page while the battery is being read.
settings-power-top-looking = 배터리를 확인하는 중…

## Power mode and battery card

# Red words under the power mode line when the mode could not be changed.
settings-power-mode-failed = 전원 모드를 바꾸지 못했어요.
# Heading of the power mode line, and the name of its list.
settings-power-mode = 전원 모드
# Said in green under a line after its choice was kept.
settings-power-saved = 저장됨
# Name of the list of battery levels to warn at.
settings-power-warn-name = 배터리가 이 정도로 줄면 알림
# Heading of the line for the battery warning.
settings-power-warn = 이 정도로 줄면 알림
# Choice in the list of battery levels: never warn.
settings-power-never = 안 함
# Heading of the line, and name of its switch, for stopping charging at 80%.
settings-power-limit = 80%에서 충전 멈춤
# Title of the battery card on a laptop.
settings-power-card-battery = 배터리
# Title of the battery card when there is no battery picture and no power modes.
settings-power-card-looking-after = 배터리 관리
# Words behind the i of the battery card.
settings-power-battery-about = 배터리 절약은 열이 적고 배터리가 더 오래가요. 성능은 더 빠르게 작동해요. 균형은 그 중간이에요. 배터리가 줄어들면 알림이 화면 가운데에 한 번 나타나요. 80%에서 멈추면 대부분 전원을 꽂고 쓰는 노트북에 더 좋고, 배터리를 몇 년 더 오래 쓰는 데 도움이 돼요. 하루 종일 콘센트 없이 지낼 날에는 끄세요.
# Words under the charge limit line while the password window is open.
settings-power-limit-waiting = 비밀번호를 기다리는 중…
# Green words under the charge limit line after charging was limited to 80%.
settings-power-limit-on = 80%에서 멈춤
# Green words under the charge limit line after the limit was removed.
settings-power-limit-off = 완충까지 충전

## Screen cards

# Heading of the keep the screen awake line, and name of its switch.
settings-power-awake = 화면 켜 두기
# Title of the card about dimming and the moving clock.
settings-power-screen-title = 화면
# Words behind the i of the Screen card.
settings-power-screen-about = 화면 켜 두기가 켜져 있으면 아래 동작은 일어나지 않아요. 링 안의 컵으로도 켜고 끌 수 있어요. 노트북 화면처럼 밝기를 직접 조절하는 화면만 어두워질 수 있어요. 움직이는 시계는 화면을 어둡게 하고 시간만 천천히 돌아다니며 보여 줘요. 마우스를 움직이거나 키를 누르면 모두 원래대로 돌아와요. 노트북에서는 시간을 두 번 정해요. 배터리일 때와 충전기를 꽂았을 때예요.
# Title of the card about turning the screen off and sleeping.
settings-power-sleep-title = 화면 끄기와 절전
# Words behind the i of the Screen off and sleep card.
settings-power-sleep-about = 키를 누르거나 마우스를 움직이면 화면이 다시 켜져요. 절전은 깨울 때까지 컴퓨터를 멈춰 둬요. 노트북에서는 시간을 두 번 정해요. 배터리일 때와 충전기를 꽂았을 때예요. 두 번째 화면을 연결했다면 덮개를 닫아도 아무 일도 없어요. 더 큰 화면으로 계속 작업할 수 있어요.
# Heading of the line for what closing the lid does.
settings-power-lid = 덮개를 닫으면

## Battery or charger captions

# Caption above a list that applies while the laptop runs on its battery.
settings-power-when-battery = 배터리 사용 중
# Caption above a list that applies while the laptop is plugged in.
settings-power-when-charger = 전원 연결됨
# Name of a list that applies on battery; { $title } is the name of its line.
settings-power-named-battery = { $title }: 배터리 사용 중
# Name of a list that applies while plugged in; { $title } is the name of its line.
settings-power-named-charger = { $title }: 전원 연결됨

## Locking

# Heading of the lock the screen line, and name of its switch.
settings-power-lock = 화면 잠금
# Heading of the lock after line, and the name of its list.
settings-power-lock-after-name = 잠금까지 시간
# Heading of the line, and name of its switch, for locking when the computer sleeps.
settings-power-lock-sleep = 컴퓨터가 절전할 때 잠금
# Heading of the line, and name of its button, that locks the screen now.
settings-power-lock-now = 지금 잠그기
# Small words under the lock now line.
settings-power-lock-now-about = 또는 Super와 L을 누르세요.
# Red words under the lock now line when the lock screen would not start.
settings-power-lock-now-failed = 잠금 화면이 시작되지 않아서 Trindle이 화면을 잠그지 못했어요.
# Title of the locking card.
settings-power-lock-title = 잠금
# Words behind the i of the locking card.
settings-power-lock-card-about = 자리를 비우면 화면이 잠겨요. 비밀번호나, 정해 둔 경우 네 자리 PIN으로 풀기 전에는 아무것도 보이지 않아요. 로그인에는 항상 비밀번호가 필요해요. 잠금까지 시간은 화면 끄기나 절전 시간과 상관없이, 컴퓨터를 그냥 두면 얼마 뒤에 잠길지 정하는 값이에요. 그 시간 전에 돌아오면 PIN이 필요 없어요. 잠금을 끄면 PIN도 지워져요. PIN 사용 시간이 지나거나 다섯 번 틀리면 비밀번호가 필요해요.

## PIN

# Heading of the line for adding a PIN.
settings-power-pin-add = PIN으로도 잠금 풀기
# Heading of the line, and name of its list, for how long a PIN works after the screen locks.
settings-power-pin-works-name = PIN 사용 시간
# Name of the bin button that stops using a PIN.
settings-power-pin-stop = PIN 사용 중지
# Heading of the line shown when a PIN is kept.
settings-power-pin-kept = PIN이 설정돼 있어요
# Placeholder of the box for typing a new PIN.
settings-power-pin-new = 새 PIN
# Name of the button that keeps the PIN typed.
settings-power-pin-keep = 이 PIN 저장
# Placeholder of the box for typing your account password before a PIN is saved.
settings-power-pin-password = 내 비밀번호
# Red words under the PIN box when no password was typed.
settings-power-pin-need-password = 먼저 비밀번호를 입력한 다음 PIN을 저장하세요.
# Red words under the PIN box when the password typed was wrong.
settings-power-pin-wrong-password = 비밀번호가 맞지 않아서 Trindle이 PIN을 저장하지 않았어요.
# Red words under the PIN line when the PIN could not be removed.
settings-power-pin-not-removed = Trindle이 PIN을 지우지 못해서 아직 화면 잠금을 풀 수 있어요.
# Red words under the PIN box when it is not the right length; { $n } is how many digits a PIN has.
settings-power-pin-digits =
    { $n ->
       *[other] { $n }자리로 입력하세요.
    }

## Session

# Heading of the log out line, and name of its round button.
settings-power-log-out = 로그아웃
# Title of the card with log out, suspend, restart and shut down.
settings-power-session-title = 세션
# Words behind the i of the session card.
settings-power-session-about = 로그아웃하면 모든 창이 닫히고 로그인 화면으로 돌아가요. 절전은 깨울 때까지 컴퓨터를 멈춰 둬요. 링 안의 전원 선택처럼 각 버튼은 한 번 더 눌러야 실행돼서, 실수로 눌러도 아무 일도 없어요.
# Red words under a session button when the system bus is not there.
settings-power-no-bus = 시스템 서비스에 연결할 수 없어요.
# Red words under a session button when the computer would not do it.
settings-power-would-not = 컴퓨터가 그 작업을 하지 못했어요.

## Pictures

# Spoken description of the battery picture while the battery is being read.
settings-power-art-looking-tip = 배터리를 확인하는 중이에요.
# Spoken description of the battery picture; { $percent } is how full and { $state } what it is doing, such as Charging.
settings-power-art-tip = 배터리 { $percent }%. { $state }
# Word drawn in the empty battery picture while the battery is being read.
settings-power-art-looking = 불러오는 중
# Word drawn under the battery figure when its state is not known.
settings-power-art-battery = 배터리
# Words drawn in the battery picture: { $left } is the time left and { $state } is either empty or full.
settings-power-art-left =
    { $state ->
        [empty] 방전까지 { $left }
       *[full] 완충까지 { $left }
    }
# Spoken description of the battery range bar when there is no warning; { $top } is the percent it charges to.
settings-power-limits-tip = { $top }%까지 충전, 알림 없음.
# Spoken description of the battery range bar; { $top } is the percent it charges to and { $warn } where a warning comes.
settings-power-limits-tip-warn = { $top }%까지 충전, { $warn }%에서 알림.

## PIN times (src/common/pin.rs)

# A length of time in the list of how long a PIN works, in minutes; { $n } is the number.
settings-power-pin-minutes =
    { $n ->
       *[other] { $n }분
    }
# A length of time in the list of how long a PIN works, in whole hours; { $n } is the number.
settings-power-pin-hours =
    { $n ->
       *[other] { $n }시간
    }
# A length of time in the list of how long a PIN works, in hours and minutes; { $hours } and { $minutes } are the two parts already written out, such as 1 hour and 15 minutes.
settings-power-pin-hours-minutes = { $hours } { $minutes }
