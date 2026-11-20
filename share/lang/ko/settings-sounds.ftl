# machine translated (Korean), not yet reviewed by a person.

# Settings page "Sounds" (event sounds), with the names of the sounds and moments from src/common/sounds.rs.

## Page (src/settings/panels/sounds.rs)

# Name of the Settings page about event sounds, in the list and as its heading.
settings-sounds-title = 소리
# Words a person might type into the Settings search to find the sounds page; separate them with spaces.
settings-sounds-keywords = 소리 효과음 알림음 시작음 시작 로그인 로그아웃 종료 충전기 연결 연결 해제 배터리 알림 경고음 끔 조용히 직접 추가 파일 wav ogg mp3 sound sounds chime startup login logout shutdown charger battery alert beep
# Introduction under the heading of the sounds page, and the words behind the page i.
settings-sounds-help = 로그인할 때처럼 상황마다 소리를 고르세요. 내 소리를 추가할 수도 있어요.
# First choice in each moment's list of sounds: that moment makes no sound.
settings-sounds-off = 끔

## Top sentence

# Sentence at the top of the page when no moment makes a sound.
settings-sounds-top-none = 모든 소리가 꺼져 있어요.
# Sentence at the top of the page when every moment makes a sound.
settings-sounds-top-all = 모든 소리가 켜져 있어요.
# Sentence at the top of the page when some moments make a sound; { $n } is how many of the { $total } do.
settings-sounds-top-some =
    { $n ->
       *[other] 소리 { $total }개 중 { $n }개가 켜져 있고, 나머지는 꺼져 있어요.
    }

## Moments and sounds

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-open = Trindle 시작
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-close = Trindle 종료
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-sunrise = 일출
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-sunset = 일몰
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-rising = 올라가는 소리
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-falling = 내려가는 소리
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-alert = 경고
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-bell = 종
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-glass = 유리
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-pop = 팝
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-login = 로그인할 때
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-logout = 로그아웃, 다시 시작, 종료할 때
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-plug = 충전기를 연결했을 때
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-unplug = 충전기를 뺐을 때
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-battery = 배터리가 부족할 때
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-notification = 알림이 나타날 때
# Heading of the logging out line.
settings-sounds-moment-logout-short = 로그아웃
# Heading of the notifications line.
settings-sounds-moment-notification-short = 알림
# Added to the Log in and out card's i.
settings-sounds-moment-logout-about = 다시 시작과 종료도 포함돼요.

## Lines and buttons

# Heading and slider name for the volume of the logging in and out sounds.
settings-sounds-volume-coming = 로그인과 로그아웃 소리 크기
# Heading and slider name for the volume of notification sounds.
settings-sounds-volume-notifications = 알림 소리 크기
# Heading and slider name for the volume of the charger and battery sounds.
settings-sounds-volume-power = 전원 소리 크기
# Name of the round button that plays the sound chosen for a moment, or an added sound.
settings-sounds-hear = 재생
# Name of the button that opens a window to add sound files.
settings-sounds-add = 소리 파일 추가
# Name of the button that opens the folder where added sounds are kept.
settings-sounds-open-folder = 추가한 소리 폴더 열기
# Red words under the add sounds line when the folder could not be opened.
settings-sounds-folder-not-opened = 폴더를 열 수 없었어요.
# Red words under the add sounds line when the folder for added sounds could not be made.
settings-sounds-folder-not-made = Trindle이 내 소리용 폴더를 만들 수 없었어요.
# Name of the button that plays the sound being added, when one of the same name is already there.
settings-sounds-hear-new = 새 소리 재생
# Name of the button that plays the sound already there, when one of the same name is being added.
settings-sounds-hear-old = 기존 소리 재생
# Heading of the line for adding sounds.
settings-sounds-add-heading = 소리 추가
# Name of the bin button that removes a sound added by the person.
settings-sounds-remove = 이 소리 제거
# Heading of the line shown when no sounds have been added.
settings-sounds-none-yet = 아직 추가한 소리가 없어요

## Red and green words

# Red words under a moment's line when its chosen sound file is gone.
settings-sounds-gone = 그 소리가 이 컴퓨터에 더 이상 없어요. 다른 소리를 고르세요.
# Red words under a line when a sound would not play.
settings-sounds-would-not-play = 소리가 재생되지 않았어요. 오디오에서 스피커를 확인하세요.
# Title of the window for choosing sound files.
settings-sounds-chooser-title = 소리 파일 선택
# Name of the filter in the window for choosing sound files that shows only sounds.
settings-sounds-chooser-filter = 소리
# Red words under the add sounds line when a chosen file is not a sound; { $name } is the file name.
settings-sounds-not-playable = Trindle이 파일 형식을 인식하지 못해서 { $name }을(를) 재생할 수 없어요.
# Red words under the add sounds line when a file could not be copied in; { $name } is the file name.
settings-sounds-not-copied = { $name }을(를) 복사할 수 없었어요.
# Green words under the add sounds line after sounds were added; { $n } is how many.
settings-sounds-added =
    { $n ->
       *[other] 소리 { $n }개를 추가했어요. 이제 위의 어느 상황에든 고를 수 있어요.
    }
# Red words under the question about a sound of the same name when it could not be replaced; { $name } is the file name.
settings-sounds-not-replaced = { $name }을(를) 바꿀 수 없었어요.
# Green words after one sound took the place of the one there; { $name } is the file name.
settings-sounds-replaced-one = { $name }을(를) 바꿨어요.
# Green words after several sounds took the place of the ones there; { $n } of { $total } were done.
settings-sounds-replaced-some = { $total }개 중 { $n }개를 바꿨어요.
# Red words under a sound's line when it could not be removed.
settings-sounds-not-removed = 제거할 수 없었어요.
# Red words under a moment's line when its sound was removed and the moment could not be set back to its own sound.
settings-sounds-not-set-back = Trindle이 소리를 제거했지만, 그 상황의 기본 소리로 되돌리지 못했어요.

## Cards

# Title of the card with the sounds for logging in and out and notifications.
settings-sounds-coming-title = 로그인과 로그아웃
# Words behind the i of that card.
settings-sounds-coming-about = 소리 크기는 0에서 100까지 10단위로 바꿀 수 있어요. 손을 떼면 소리가 한 번 나요.
# Title of the card with the charger and battery sounds.
settings-sounds-power-title = 전원
# Words behind the i of that card.
settings-sounds-power-about = 충전기를 연결하거나 뺄 때, 배터리가 부족할 때 나는 소리예요. 소리 크기는 0에서 100까지 10단위로 바꿀 수 있어요. 손을 떼면 소리가 한 번 나요.
# Title of the card with the sounds the person added.
settings-sounds-own-title = 내 소리
# Words behind the i of that card.
settings-sounds-own-about = WAV, OGG, Opus, FLAC, MP3 파일을 쓸 수 있어요. Trindle이 내 폴더에 복사하므로 원본은 옮기거나 지워도 돼요. 다른 소리처럼 어느 상황에든 고를 수 있어요. 소리를 제거하면 그 소리를 쓰던 상황은 기본 소리로 돌아가요.
