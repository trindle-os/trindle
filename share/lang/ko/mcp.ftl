# machine translated (Korean), not yet reviewed by a person.

# Trindle's assistant access (MCP): the question window that asks the person
# before an assistant changes anything, and the words of its answer buttons.
# What the assistant reads back (its tool descriptions and errors) is written
# for the assistant, not the person, so it is not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The question window (src/mcp/ask.rs)

# Window title of the question an assistant's request raises.
mcp-title-ask = 어시스턴트 요청
# Heading over the words of what the assistant wants to do.
mcp-ask-heading = Trindle에 연결된 어시스턴트가 다음을 하려고 해요:
# Button that refuses the change the assistant asked for.
mcp-no = 아니요
# Button that lets the change the assistant asked for go ahead.
mcp-yes = 예, 진행해요

## What the question says, one line for each kind of change (src/mcp/tools.rs)

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = { $name } 창을 맨 앞으로 가져오기
# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = { $key } 설정을 { $from }에서 { $to }(으)로 바꾸기
# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = { $look } 테마 사용하기
# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = { $look } 테마를 쓰고 강조 색을 { $accent }(으)로 하기
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = Flathub에서 { $id } 설치하기
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = { $id }을(를) 제거하고 파일은 남기기
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = Wi-Fi 네트워크 { $name }에 연결하기
# The question for pairing a Bluetooth device that has no name; its address comes in.
mcp-ask-pair = Bluetooth 기기 { $address }와(과) 페어링하기
# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
mcp-ask-pair-named = Bluetooth 기기 { $name } ({ $address })와(과) 페어링하기
# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = { $name } 닫기
# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = { $name } 열기
# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
mcp-ask-allow = { $id }이(가) { $what }을(를) 사용하도록 허용하기
# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
mcp-ask-deny = { $id }이(가) { $what }을(를) 사용하지 못하게 하기
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = 설정과 앱 권한을 이전 상태로 되돌리기: { $note }
# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = 설정, 앱 권한, { $id }의 파일을 이전 상태로 되돌리기: { $note }. { $id }이(가) 열려 있으면 먼저 닫아요
# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = { $id }을(를) 닫고 다시 시작하기. 저장하지 않은 내용은 사라져요
# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = { $id }의 { $file }에서 { $key }을(를) { $from }에서 { $to }(으)로 바꾸기. { $id }이(가) 열려 있으면 먼저 닫아요
# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = { $id }의 { $file }에 { $key }을(를) { $to }(으)로 추가하기. { $id }이(가) 열려 있으면 먼저 닫아요
# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = { $id }의 캐시 지우기. { $id }이(가) 열려 있으면 먼저 닫고, 되돌릴 수 있도록 캐시 사본을 남겨요
