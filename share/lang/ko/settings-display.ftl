# machine translated (Korean), not yet reviewed by a person.

# Settings page "Display": screens, resolution, size, rotation, arrangement and night light.

## Page (src/settings/panels/display.rs)

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = 디스플레이
# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = 디스플레이 화면 모니터 해상도 크기 배율 확대 축소 회전 주사율 야간 조명 따뜻한 블루라이트 저녁 눈 screen monitor resolution size scale zoom bigger smaller rotate refresh night light warm blue light evening eyes
# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = 변경 사항은 바로 적용돼요. 화면이 이상해지면 아무것도 하지 마세요. 유지를 누르지 않으면 15초 뒤에 저절로 돌아가요. 야간 조명은 저녁에 화면을 따뜻한 색으로 바꿔 줘요.
# Question along the bottom of the page while a change of displays waits to be kept.
settings-display-foot-keep = 이 배치를 유지할까요?
# Sentence at the top of the page while the displays are being found.
settings-display-looking = 디스플레이를 확인하는 중…
# Sentence at the top of the page when no display was found.
settings-display-none-found = 디스플레이를 찾을 수 없어요.

## Top sentence

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = 사용 중인 디스플레이가 없고
# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = 디스플레이 한 대를 사용 중이고
# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
       *[other] 디스플레이 { $n }대를 사용 중이고
    }
# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = 야간 조명은 꺼져 있어요
# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = 야간 조명이 { $from }부터 { $to }까지 화면을 따뜻하게 해요
# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = 야간 조명이 해 질 때부터 해 뜰 때까지 화면을 따뜻하게 해요
# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used } { $warm }.

## Lists

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = 기본
# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = 오른쪽으로 회전
# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = 왼쪽으로 회전
# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = 거꾸로
# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = 오른쪽
# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = 왼쪽
# Choice in the list of where a display sits: above the other.
settings-display-place-above = 위
# Choice in the list of where a display sits: below the other.
settings-display-place-below = 아래

## Resolutions and sizes

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }
# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height }, { $hz } Hz
# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (권장)
# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz
# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  (표준)
# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = { $display }의 해상도
# Name of the list of sizes; { $display } is the display.
settings-display-size-on = { $display }의 크기
# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = { $display }의 회전
# Heading of the resolution line.
settings-display-resolution = 해상도
# Heading of the size line.
settings-display-size = 크기
# Heading of the rotation line.
settings-display-rotation = 회전
# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = 이 디스플레이 사용
# Small words under that line for the only display in use.
settings-display-use-only = 사용 중인 하나뿐인 디스플레이는 끌 수 없어요.
# Words behind the i of a display's card.
settings-display-card-about = 변경 사항은 바로 적용돼요. 화면이 이상해지면 아무것도 하지 마세요. 유지를 누르지 않으면 15초 뒤에 저절로 돌아가요. 권장 해상도는 화면에 가장 알맞게 만들어진 해상도예요. 표준 크기는 Trindle이 이 화면에 맞게 고른 크기예요.

## Arrangement

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = { $display } 위치
# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = 위치
# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $other } 기준 { $display } 위치
# Title of the card about where displays sit.
settings-display-arrangement-title = 배치
# Words behind the i of that card.
settings-display-arrangement-about = 디스플레이가 서로 어느 쪽에 놓이는지 정해요. 그러면 맞닿은 가장자리를 넘어 포인터가 움직일 수 있어요.
# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = 한쪽 옆에 놓으려면 디스플레이가 두 대 필요해요.

## Changes and keeping

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = 바꿀 수 없었어요.
# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
       *[other] 디스플레이가 바뀌었어요. 유지를 누르지 않으면 { $seconds }초 뒤에 돌아가요.
    }
# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = 변경을 되돌릴 수 없었어요. { $why }
# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = 배치는 지금 적용 중이지만 Trindle이 다음 로그인을 위해 저장하지 못했어요.
# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = 바뀐 것이 없어요
# Small words under that line.
settings-display-refused-change = 그 변경을 할 수 없었어요.
# Heading of the line under the picture of the displays.
settings-display-apply-at-once = 변경 사항은 바로 적용돼요

## Night light

# Title of the night light card.
settings-display-night-title = 야간 조명
# Words behind the i of that card.
settings-display-night-about = 저녁에 화면이 따뜻한 색으로 바뀌어 눈이 편해요. 시간대로 계산한 해 질 때와 해 뜰 때를 따르거나, 직접 정한 시간을 따라요. Trindle이 시간대로 위치를 알 수 없으면 직접 정한 시간을 사용해요.
# Heading of the night light line, and name of its switch.
settings-display-night = 야간 조명
# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = { $zone }의 해 질 때와 해 뜰 때예요.
# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Trindle이 해 지는 시각을 몰라서 정한 시간을 사용해요.
# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = 해 질 때부터 해 뜰 때까지
# Choice in the When list: between times the person sets.
settings-display-when-times = 정한 시간
# Heading of the When line, and name of its list.
settings-display-when = 언제
# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = 시작
# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = 끝
# Heading of the warmth line, and name of its slider.
settings-display-warmth = 따뜻함
# Small word under the left end of the warmth slider.
settings-display-little = 약간
# Small word under the right end of the warmth slider.
settings-display-very = 아주 따뜻하게
# Spoken description of the strip of colours night light runs through.
settings-display-strip = 약간 따뜻한 색부터 아주 따뜻한 색까지

## Pictures and cards

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = 사용 중인 디스플레이 없음
# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }
# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
       *[other] 배치된 디스플레이 { $n }대
    }
# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = 디스플레이 없음
# Name of the large picture of the displays.
settings-display-your-displays = 내 디스플레이
# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle이 디스플레이를 찾지 못했어요.
# Title of the card shown when no display was found.
settings-display-none-title = 디스플레이를 찾지 못했어요
# Heading of the line in that card.
settings-display-none-heading = 디스플레이를 찾지 못했어요
# Title of the card about the graphics card.
settings-display-graphics-title = 그래픽
# Words behind the i of that card.
settings-display-graphics-about = 이 컴퓨터에는 오픈 드라이버를 쓰는 NVIDIA 그래픽 카드가 있어요. 작동은 하지만 2022년 무렵 이후의 카드는 오픈 드라이버가 카드 속도를 높일 수 없어서 느리게 느껴질 수 있어요. 대신 NVIDIA의 드라이버를 설치할 수 있어요.
# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = 오픈 드라이버를 쓰는 NVIDIA 카드
# Small words under that line.
settings-display-nvidia-about = 2022년 이후 카드는 느릴 수 있어요.
# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = 해결 방법
