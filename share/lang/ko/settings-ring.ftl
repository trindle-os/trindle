# machine translated (Korean), not yet reviewed by a person.

# Settings page "Ring": how the ring opens, notifications, the clipboard and voice control.

## Page (src/settings/panels/ring.rs)

# Name of the Settings page about the ring, in the list and as its heading.
settings-ring-title = 링
# Words a person might type into the Settings search to find the ring page; separate them with spaces.
settings-ring-keywords = favourites favorites launcher edge click hot corner voice speech windows open windows clipboard copy paste history forget notifications 즐겨찾기 런처 가장자리 클릭 음성 창 클립보드 복사 붙여넣기 기록 알림
# Introduction under the heading of the ring page, and the words behind the page i.
settings-ring-help = 즐겨찾기는 링의 가운데 주변에 놓여요. 즐겨찾기는 링에서 직접 고르세요. 링에서 앱을 오른쪽 클릭하면 추가하거나 뺄 수 있고, 즐겨찾기를 다른 곳으로 끌면 순서가 바뀌어요.
# Sentence at the top of the ring page saying how the ring opens.
settings-ring-headline = 링은 화면 가장자리에서 열리고, 열려 있는 창이 링에 나타나요.
## Opening the ring

# Line saying how many favourites the ring has; { $n } is the number.
settings-ring-favourites =
    { $n ->
       *[other] 즐겨찾기 { $n }개
    }
# Title of the card about opening the ring.
settings-ring-opening = 링 열기
# Words behind the i of the card about opening the ring.
settings-ring-opening-about = 포인터를 화면 가장자리로 옮겨 더블클릭하거나 Super 키를 누르면 링이 열려요. 즐겨찾기는 링 가운데 주변에 놓이고, 열려 있는 창은 그 둘레의 링에 놓여요. 창을 클릭하면 앞으로 나오고, 오른쪽 클릭하면 버튼이 보여요. 링에서 앱을 오른쪽 클릭하면 추가하거나 뺄 수 있어요.
## Notifications

# Heading of the switch line, and the spoken name of the switch, for showing notifications as they arrive.
settings-ring-notify-show = 도착하면 바로 표시
# Heading of the list line, and the name of the list, for how long a notification stays.
settings-ring-notify-long = 표시 시간
# Added to the Notifications card's i.
settings-ring-notify-long-about = 긴급한 알림은 닫을 때까지 남아요.
# Heading of the list line, and the name of the list, for which corner notifications appear in.
settings-ring-notify-where = 표시 위치
# Added to the Notifications card's i.
settings-ring-notify-where-about = 이 모서리부터 쌓여요.
# Title of the card about notifications.
settings-ring-notify-title = 알림
# Words behind the i of the card about notifications.
settings-ring-notify-about = 프로그램이 알려 줄 것이 있으면 알림이 몇 초 동안 나타났다가 사라져요. 이 기능을 끄면 아무것도 나타나지 않지만, 모든 알림은 링의 알림 원에 계속 모여요. 긴급한 알림은 여기서 어떻게 정하든 닫을 때까지 남아서 경고를 놓치지 않아요.
## Clipboard

# Heading of the list line, and the name of the list, for when copied lines are forgotten.
settings-ring-clip-forget = 복사한 내용 지우기
# Heading of the line saying how many lines the clipboard holds now.
settings-ring-clip-now = 지금 복사된 것
# Small words under that line; { $n } is how many lines are kept and { $pinned } how many of them are pinned.
settings-ring-clip-count =
    { $n ->
       *[other] { $n }줄, 고정 { $pinned }줄.
    }
# Title of the card about the clipboard.
settings-ring-clip-title = 클립보드
# Words behind the i of the card about the clipboard.
settings-ring-clip-about = 링의 클립보드 원은 복사한 내용을 보관해서 이전에 복사한 것을 붙여넣을 수 있어요. 고정한 줄은 여기서 어떻게 정하든 항상 보관해요.
## Voice

# Small words under the voice line while voice is being added.
settings-ring-voice-adding = 음성을 추가하는 중이에요. 계속 작업해도 돼요.
# Small words under the voice line while its build tools are being removed.
settings-ring-voice-removing-tools = 빌드 도구를 제거하는 중이에요.
# Small words under the voice line while voice is being removed.
settings-ring-voice-removing = 음성을 제거하는 중이에요.
# Heading of the voice line, and the spoken name of its switch.
settings-ring-voice-control = 음성 제어
# Said, in green, under the voice switch after voice is turned on.
settings-ring-voice-onoff-on = 켜짐
# Said, in green, under the voice switch after voice is turned off.
settings-ring-voice-onoff-off = 꺼짐
# Heading of the line, and the name of its button, that removes voice.
settings-ring-voice-remove = 음성 제거
# Added to the Voice card's i once voice is added.
settings-ring-voice-remove-about = 약 150 MB가 비워져요.
# Words on the line offering to remove the build tools that stayed after voice was added.
settings-ring-voice-tools-left = 준비됐어요. 빌드 도구는 남겨 뒀어요.
# Name of the button that removes the build tools voice left behind.
settings-ring-voice-tools-remove = 빌드 도구 제거
# Name of the button that adds voice control to the computer.
settings-ring-voice-add = 음성 제어 추가
# Added to the Voice card's i before voice is added.
settings-ring-voice-optional = 선택 사항이에요. 약 150 MB가 필요해요.
# Title of the card about voice.
settings-ring-voice-title = 음성
# Words behind the i of the voice card; the quoted words are examples of what to say.
settings-ring-voice-card-about = 클릭하는 대신 앱 이름, “소리 크게” 또는 “Wi-Fi 설정”이라고 말해 보세요. 링의 가운데를 누른 뒤 말하세요. 말을 멈추면 Trindle도 듣기를 멈춰요. 음성은 이 컴퓨터에만 있고 어디로도 보내지 않아요. 음성을 추가하는 데 약 150 MB와 몇 분이 걸리고, 그동안 계속 작업해도 돼요. 제거하면 음성 엔진과 모델이 삭제돼요.
# Words beside the round progress dial while voice is being added.
settings-ring-voice-working-add = 음성 추가 중
# Words beside the round progress dial while voice build tools are being removed.
settings-ring-voice-working-tools = 빌드 도구 제거 중
# Words beside the round progress dial while voice is being removed.
settings-ring-voice-working-remove = 음성 제거 중

## Clipboard history choices (src/common/clipboard.rs)

# Choice in the list of when copied lines are forgotten: every time the computer restarts.
settings-ring-forget-reboot = 컴퓨터를 다시 시작할 때마다
# Choice in the list of when copied lines are forgotten: after a day.
settings-ring-forget-day = 하루 뒤
# Choice in the list of when copied lines are forgotten: after a week.
settings-ring-forget-week = 일주일 뒤
# Choice in the list of when copied lines are forgotten: after two weeks.
settings-ring-forget-fortnight = 2주 뒤
# Choice in the list of when copied lines are forgotten: never.
settings-ring-forget-never = 지우지 않음

# Heading of the switch line, and the spoken name of the switch, for showing a window's title under its picture when pointing at it in the ring.
settings-ring-names = 가리킬 때 창 제목 표시
# Added to the Opening card's i.
settings-ring-names-about = 꺼도 화면 읽기 프로그램은 제목을 읽어 줘요.
# Red words when voice could not be added.
settings-ring-voice-not-added = Trindle이 음성을 추가하지 못했어요.
# Red words when voice could not be removed.
settings-ring-voice-not-removed = Trindle이 음성을 제거하지 못했어요.
# Red words when the build tools left after adding voice could not be removed.
settings-ring-voice-tools-not-removed = Trindle이 빌드 도구를 제거하지 못했어요.
