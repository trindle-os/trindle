# machine translated (Korean), not yet reviewed by a person.

# Trindle's voice control: the words the ring says back after a spoken
# request, the voice set-up's progress lines, and the reasons it did not start.
# The words the person says are matched in English and are not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## What the ring says back after a spoken request (src/voice/commands.rs)

# Said under the ring when a radio is switched on by speaking; the radio's name comes first.
voice-radio-on = { $name } 켜짐
# Said under the ring when a radio is switched off by speaking; the radio's name comes first.
voice-radio-off = { $name } 꺼짐
# Said under the ring after "brighter" turns the screen up.
voice-brighter = 더 밝게
# Said under the ring after "dimmer" turns the screen down.
voice-dimmer = 더 어둡게
# Said under the ring when the request is to let the screen rest again.
voice-keep-awake-off = 화면이 다시 꺼질 수 있어요
# Said under the ring when the screen is kept awake.
voice-keep-awake-on = 화면을 켜 둘게요
# Said under the ring when the request is to look for updates.
voice-updates = 업데이트를 확인하는 중
# Said under the ring when nothing was heard at all.
voice-nothing-heard = Trindle이 듣지 못했어요. 링 가운데를 누르고 말씀해 주세요.
# Said under the ring when the volume goes up.
voice-louder = 더 크게
# Said under the ring when the volume goes down.
voice-quieter = 더 작게
# Said under the ring when the sound is switched back on.
voice-sound-on = 소리 켜짐
# Said under the ring when the sound is muted.
voice-muted = 음소거됨
# Said under the ring when the wallpaper changes to the next one.
voice-next-wallpaper = 다음 배경 화면
# Said under the ring after a screenshot is taken; Pictures is the folder's name.
voice-screenshot-saved = 스크린샷을 사진 폴더에 저장했어요
# Said under the ring when the request is to show every app.
voice-all-apps = 모든 앱
# Said under the ring before shutting down, which waits for a click to confirm.
voice-confirm-power-off = 종료할까요? 확인하려면 누르세요
# Said under the ring before restarting, which waits for a click to confirm.
voice-confirm-reboot = 다시 시작할까요? 확인하려면 누르세요
# Said under the ring before logging out, which waits for a click to confirm.
voice-confirm-logout = 로그아웃할까요? 확인하려면 누르세요
# Said under the ring when the request is to find an app in Apps; the words the person said for it come in.
voice-looking-for-app = 앱에서 { $what } 찾는 중
# Said under the ring when the request opens Apps (the store).
voice-apps = 앱
# Said under the ring when the request opens the disks page of Settings; Drives is the page's name.
voice-drives = 드라이브
# Said under the ring when the request opens Settings.
voice-settings = 설정
# Said under the ring when the request closes the window in front.
voice-closing-window = 창을 닫는 중
# Said under the ring when an open app is closed; the app's name comes in.
voice-closing-app = { $name } 닫는 중
# Said under the ring when no app has the name the person said; the name they said comes in.
voice-no-app = { $name }(이)라는 앱이 없어요
# Said under the ring when a window is brought to the front; the app's name comes in.
voice-showing-app = { $name } 보여 주는 중
# Said under the ring when an app is opened; the app's name comes in.
voice-opening-app = { $name } 여는 중
# Said under the ring when the request was heard but not understood; what was heard comes in.
voice-heard-unknown = Trindle이 “{ $heard }” 명령을 알아듣지 못했어요

## Why voice did not start (src/voice/mod.rs)

# Said in the ring when voice has not been added yet.
voice-optional = 음성은 선택 기능이에요. 설정의 링 페이지에서 추가하세요.
# Said in the ring when voice has been added but is switched off.
voice-switched-off = 음성이 꺼져 있어요. 설정의 링 페이지에서 켜세요.
# Said in the ring when the microphone cannot be opened; the reason comes after it.
voice-mic-failed = Trindle이 마이크를 사용할 수 없었어요: { $why }
# The reason after "Trindle couldn't access the microphone:" when the session has no private folder to keep a recording in.
voice-no-private-folder = 사용할 수 있는 비공개 폴더가 없어요.
# Said in the ring when the microphone's recorder would not start.
voice-mic-not-started = 녹음 프로그램이 시작되지 않아서 Trindle이 마이크를 사용할 수 없었어요
# Said in the ring when listening stopped and nothing was recorded.
voice-nothing-recorded = 녹음된 내용이 없어요.
# Said in the ring when the recording had no speech in it.
voice-nothing-heard-after = Trindle이 듣지 못했어요. 링 가운데를 누르고 말씀해 주세요.
# Said in the ring when the speech engine failed and gave no reason.
voice-whisper-failed = Whisper가 실패했어요

## Adding and removing voice (src/voice/setup.rs)

# Progress line while the build tools are added for the build; the names of the packages come in.
voice-installing-tools = 빌드 도구를 임시로 설치하는 중: { $names }
# Progress line while whisper.cpp is built; its version comes in.
voice-building = 음성 제어용 whisper.cpp { $version } 빌드 중이에요. 1~2분 걸려요.
# Progress line while the speech model is downloaded (148 MB).
voice-fetching-model = 음성 모델 다운로드 중 (148 MB)
# Progress line while the build tools are taken away again.
voice-removing-tools = 빌드 도구를 지우는 중
# Said once voice is added and ready to use.
voice-ready = 음성 제어를 쓸 수 있어요. 링을 열고 가운데를 누른 뒤 말씀해 주세요.
# Said once voice is removed.
voice-removed = 음성 제어를 제거했어요.
# Shown in the terminal window that stays open, asking for Enter to close it.
voice-press-enter = 닫으려면 Enter를 누르세요.
# Shown when the command is run with neither install nor remove.
voice-usage = usage: trindle-voice-setup install | remove

# Said when a step of the voice set-up fails. { $program } is the program that was run, with its first word.
voice-step-failed = { $program }이(가) 실패했어요.
# Said when the whisper.cpp download isn't the one Trindle checks for, so nothing is built from it.
voice-whisper-fingerprint = 받은 whisper.cpp가 Trindle이 예상한 것과 달라서 빌드하지 않았어요.
# Said when the speech model download isn't the one Trindle checks for, so it isn't installed.
voice-model-fingerprint = 받은 음성 모델이 Trindle이 예상한 것과 달라서 설치하지 않았어요.
