# machine translated (Korean), not yet reviewed by a person.

# Settings page "Audio": speakers, microphone, volume, mute and a test sound.

## Page (src/settings/panels/audio.rs)

# Name of the Settings page about sound, in the list and as its heading.
settings-audio-title = 오디오
# Words a person might type into the Settings search to find the audio page; separate them with spaces.
settings-audio-keywords = 소리 볼륨 스피커 헤드폰 마이크 음소거 테스트 sound volume speakers headphones microphone mic mute test
# Introduction under the heading of the audio page, and the words behind the page i.
settings-audio-help = 소리를 낼 스피커나 헤드폰과 사용할 마이크를 고르세요.

## Top sentence

# Red sentence at the top of the page when PipeWire, the sound system, is not installed.
settings-audio-top-missing = PipeWire가 설치되어 있지 않아서 소리가 나지 않아요.
# Red sentence at the top of the page when PipeWire is installed but not answering.
settings-audio-top-failed = 소리 서비스인 PipeWire가 응답하지 않아서 소리가 나지 않아요.
# Red sentence at the top of the page when there are no speakers or headphones.
settings-audio-top-no-output = 스피커나 헤드폰이 없어요.
# Plain sentence at the top of the page while the page is still finding out about sound.
settings-audio-top-listening = 소리를 확인하는 중…
# Warning at the top of the page when sound is muted; { $device } is the speakers or headphones.
settings-audio-top-muted = { $device }이(가) 음소거되어 있어요.
# Warning at the top of the page when the volume is at nothing; { $device } is the speakers or headphones.
settings-audio-top-down = { $device }의 볼륨이 맨 아래에 있어요.
# Sentence at the top of the page when all is well; { $device } is the speakers or headphones and { $percent } the volume.
settings-audio-top-playing = { $device }(으)로 볼륨 { $percent }%로 소리가 나고 있어요.

## Cards

# Title of the card for speakers and headphones.
settings-audio-out-title = 스피커와 헤드폰
# Title of the card for microphones.
settings-audio-in-title = 마이크
# Heading of the line shown when no speakers or headphones are found.
settings-audio-out-none = 스피커를 찾지 못했어요
# Heading of the line shown when no microphone is found.
settings-audio-in-none = 마이크 없음
# Small words under that line for speakers.
settings-audio-out-plug = 스피커나 헤드폰을 연결하세요.
# Small words under that line for microphones.
settings-audio-in-plug = 마이크나 헤드셋을 연결하세요.
# Heading of the line with the list of devices.
settings-audio-device = 장치
# Name of the list of speakers and headphones.
settings-audio-out-device-name = 소리가 나는 곳
# Name of the list of microphones.
settings-audio-in-device-name = 마이크
# Heading of the volume line.
settings-audio-volume = 볼륨
# Name of the volume slider for the speakers.
settings-audio-out-volume-name = 소리 볼륨
# Name of the volume slider for the microphone.
settings-audio-in-volume-name = 마이크 볼륨
# Heading of the mute line.
settings-audio-mute = 음소거
# Name of the switch that mutes the sound.
settings-audio-out-mute-name = 소리 음소거
# Name of the switch that mutes the microphone.
settings-audio-in-mute-name = 마이크 음소거
# Heading of the line that plays a test sound.
settings-audio-test = 테스트
# Name of the button that plays a test sound.
settings-audio-test-button = 테스트 소리 재생
# Words behind the i of the speakers card.
settings-audio-out-about = 소리를 낼 스피커나 헤드폰을 고르세요. 여기서 바꾸는 볼륨과 음소거는 키보드 키나 링의 소리 원과 똑같이 작동해요.
# Heading of the line with the microphone level bar.
settings-audio-level = 입력 수준
# Words behind the i of the microphone card.
settings-audio-in-about = 사용할 마이크를 고르세요. 이 페이지가 열려 있는 동안 막대가 마이크로 들어오는 소리를 보여 줘요. Trindle은 아무것도 녹음하거나 저장하지 않아요.
# Title of the card shown when PipeWire is not answering.
settings-audio-failed-title = 소리
# Heading of the line in that card.
settings-audio-failed-heading = PipeWire가 응답하지 않아요
# Small words under that line.
settings-audio-failed-about = 로그인할 때 소리가 시작되지 않았을 수 있어요.

## Red words

# Red words under the volume line when the volume could not be changed.
settings-audio-no-volume = Trindle이 볼륨을 바꾸지 못했어요. 소리가 시작되지 않았을 수 있어요. 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요.
# Red words under the mute line when the sound could not be muted.
settings-audio-no-mute = Trindle이 소리를 음소거하지 못했어요. 소리가 시작되지 않았을 수 있어요. 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요.
# Red words under the mute line when the sound could not be turned back on.
settings-audio-no-unmute = Trindle이 소리를 다시 켜지 못했어요. 소리가 시작되지 않았을 수 있어요. 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요.
# Red words under the device line when the device could not be chosen.
settings-audio-not-chosen = 그 장치로 바꾸지 못했어요.
# Red words under the test line when the test sound would not play.
settings-audio-test-failed = 이 장치에서 테스트 소리가 재생되지 않았어요.

## Where sound goes

# A device's name with what kind it is; { $name } is the device and { $kind } one of the kind words below.
settings-audio-with-kind = { $name } ({ $kind })
# Kind of output: a Bluetooth device.
settings-audio-kind-bluetooth = Bluetooth
# Kind of output: a screen or TV on a cable.
settings-audio-kind-hdmi = HDMI
# Kind of output: a device on a USB cable.
settings-audio-kind-usb = USB
# Kind of output: headphones or a headset on a cable.
settings-audio-kind-headphones = 헤드폰
# Kind of output: the computer's own speakers.
settings-audio-kind-speakers = 스피커
# Heading of the line shown while Bluetooth plays the sound.
settings-audio-bt-now = 소리가 Bluetooth로 나가고 있어요
# Tooltip of the button that moves the sound to the speakers; { $device } is the speakers, kind included.
settings-audio-use-speakers = { $device }(으)로 재생
# Heading of the line when Bluetooth devices are connected but have no sound output; { $names } lists them.
settings-audio-bt-silent = { $names }이(가) 연결되어 있지만 소리를 낼 수 없어요.
# Words under that heading.
settings-audio-bt-silent-about = 소리 설정이 안 되어 있을 수 있어요. 연결을 끊었다가 다시 연결하세요.
# Red words when the sound devices could not be read.
settings-audio-not-read = Trindle이 소리 장치를 찾지 못했어요.
