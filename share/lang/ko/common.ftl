# machine translated (Korean), not yet reviewed by a person.

# Words of the shared parts that no one program owns: why an app cannot be uninstalled, what updating and renewing say, what the keyring says, the X11 rooms, and the "in use by" lines. Each program shows them where it needs them.

## Uninstalling (src/common/removal.rs)

# Said instead of an Uninstall button on an app that is part of Trindle itself.
common-removal-trindle = Trindle의 일부라서 그대로 둬요.
# Said instead of an Uninstall button on an app that the computer cannot work without.
common-removal-needed = 컴퓨터에 꼭 필요해서 그대로 둬요.
# Said instead of an Uninstall button on an app whose removal would take a part of Trindle with it.
common-removal-trindle-needs = Trindle의 일부가 필요로 해서 그대로 둬요.
# Red words when an app's saved data was not deleted because its id is not a plain name.
common-removal-data-name = 앱 이름에 특이한 문자가 있어서 데이터를 삭제하지 않았어요.
# Red words when an app's saved data folder is a link or not a folder, so nothing was deleted.
common-removal-data-link = 폴더가 다른 곳으로 연결된 링크라서 데이터를 삭제하지 않았어요.
# Red words when an app's saved data could not be deleted.
common-removal-data-failed = 데이터를 삭제하지 못했어요.
# Said when only an app's saved data is to go and the app is running.
common-removal-data-running = 앱이 실행 중이라서 데이터를 삭제하지 않았어요. 먼저 앱을 닫으세요.
# Said when the app's data path is a file, not a folder.
common-removal-data-not-folder = 폴더가 있어야 할 자리에 파일이 있어서 데이터를 삭제하지 않았어요.

## One update at a time (src/common/updating.rs)

# Name of the updates button, and its tip, while the ring's own update button is putting updates in.
common-updating-short-ring = 업데이트 중
# Name of an update button while Apps is putting updates in.
common-updating-short-apps = 앱에서 업데이트 중
# Name of an update button while the daily security fixes for the apps' boxes are going in.
common-updating-short-boxes = 보안 수정 추가 중
# Name of an update button while Debian's own installer is at work.
common-updating-short-system = Debian 업데이트 중
# Name of an update button while something not named is installing updates.
common-updating-short-unknown = 다른 곳에서 업데이트 중
# Headline of an updates page while the ring is installing updates.
common-updating-sentence-ring = 링에서 업데이트를 설치하고 있어요.
# Headline of an updates page while Apps is installing updates.
common-updating-sentence-apps = 앱에서 업데이트를 설치하고 있어요.
# Headline of an updates page while the daily security fixes are going into the apps' boxes.
common-updating-sentence-boxes = 앱의 보안 수정을 설치하고 있어요.
# Headline of an updates page while Debian's installer is at work (the daily security updates, apt in a terminal, or another person).
common-updating-sentence-system = Debian 설치 프로그램이 실행 중이에요. 보안 업데이트를 위해 매일 실행돼요.
# Headline of an updates page while something not named is installing updates.
common-updating-sentence-unknown = 다른 곳에서 업데이트를 설치하고 있어요.

## What an update does to a version (src/common/updating.rs)

# Beside an app in a list of updates, when what waits is a newer build of the version already installed. { $version } is the version.
common-updating-newer-build-of = 버전 { $version }의 새 빌드
# Beside an app in a list of updates: the installed version, then the version that waits.
common-updating-version-to = 버전 { $from }에서 { $to }(으)로
# Beside an app in a list of updates, when only the installed version is known. { $version } is that version.
common-updating-now-version = 현재 버전 { $version }
# Beside an app in a list of updates, when only the new version is known. { $version } is that version.
common-updating-to-version = 버전 { $version }(으)로
# Beside a thing whose own version is shown, when what waits is a newer build of the same version.
common-updating-newer-waiting = 새 빌드가 기다리고 있어요
# Beside a thing whose own version is shown: the version that waits.
common-updating-version-waiting = 버전 { $version }이(가) 기다리고 있어요
# Beside a thing whose own version is shown, when the new version is not known.
common-updating-update-waiting = 업데이트가 기다리고 있어요

## Why the administrator's password is asked for (src/common/updating.rs)

# Said in the password box when Debian's and Trindle's updates are put in together.
common-updating-reason-all = Debian과 Trindle 업데이트는 이 컴퓨터를 쓰는 모든 사람에게 영향을 주므로 관리자 비밀번호가 필요해요. 직접 설치한 앱은 비밀번호 없이 업데이트돼요.
# Said in the password box when only Debian's updates are put in.
common-updating-reason-debian = Debian 업데이트는 이 컴퓨터를 쓰는 모든 사람에게 영향을 주므로 관리자 비밀번호가 필요해요.
# Said in the password box when only Trindle's own update is put in.
common-updating-reason-trindle = Trindle 업데이트는 이 컴퓨터를 쓰는 모든 사람에게 영향을 주므로 관리자 비밀번호가 필요해요.
# Said in the password box when new firmware is put in.
common-updating-reason-firmware = 새 펌웨어는 컴퓨터 자체를 바꾸므로 관리자 비밀번호가 필요해요.
# Said in the password box when the lists of the whole computer's updates are fetched away from the computer's own screen.
common-updating-reason-refresh = 컴퓨터 전체의 업데이트를 확인하려면 관리자 비밀번호가 필요해요. 컴퓨터 화면에서 직접 확인할 때는 필요 없어요.

## How an update ended (src/common/updating.rs)

# The end of a sentence saying why nothing was installed. { $why } is the reason, such as "no administrator's password was given".
common-updating-because = { $why } 때문이에요.
# Red words beside an update that did not finish and gave no reason.
common-updating-no-reason = 끝나지 않았어요. 이유는 알려지지 않았어요.
# Red words beside an update that did not finish. { $why } is the reason the system gave.
common-updating-failed = 끝나지 않았어요. { $why }
# Red words beside an update that was never started. { $why } is the end of a sentence such as "because no administrator's password was given."
common-updating-not-installed = 설치하지 않았어요. { $why }

## Why a step behind the administrator's password was refused (src/common/refused.rs)

# End of a sentence saying why a step was refused: the password box was closed. It follows "because".
common-refused-why-cancelled = 관리자 비밀번호를 입력하지 않았기
# End of a sentence saying why a step was refused: the password was asked for and not accepted. It follows "because".
common-refused-why-wrong = 관리자 비밀번호가 맞지 않았기
# End of a sentence saying why a step was refused: this login may never do it. It follows "because".
common-refused-why-not-allowed = 이 계정은 비밀번호가 맞아도 그 작업을 할 수 없기
# End of a sentence saying why a step was refused: nothing was there to ask for the password. It follows "because".
common-refused-why-unasked = 비밀번호 창이 나타나지 않았기
# End of a sentence saying why a step was refused: it is not known which of the two reasons it was. It follows "because".
common-refused-why-unclear = 관리자 비밀번호가 맞지 않았거나 이 계정이 그 작업을 할 수 없기
# Sentence of its own saying a step was refused: the password box was closed.
common-refused-sentence-cancelled = 관리자 비밀번호를 입력하지 않았어요
# Sentence of its own saying a step was refused: the password was not accepted.
common-refused-sentence-wrong = 관리자 비밀번호가 맞지 않았어요
# Sentence of its own saying a step was refused: this login may never do it.
common-refused-sentence-not-allowed = 이 계정은 비밀번호가 맞아도 그 작업을 할 수 없어요
# Sentence of its own saying a step was refused: nothing was there to ask for the password.
common-refused-sentence-unasked = 비밀번호 창이 나타나지 않았어요
# Sentence of its own saying a step was refused: it is not known which of the two reasons it was.
common-refused-sentence-unclear = 관리자 비밀번호가 맞지 않았거나 이 계정이 그 작업을 할 수 없어요
# Red words when a step was refused because the password box was closed.
common-refused-not-done-cancelled = 관리자 비밀번호를 입력하지 않아서 작업하지 않았어요.
# Red words when a step was refused because the password was not accepted.
common-refused-not-done-wrong = 관리자 비밀번호가 맞지 않아서 작업하지 않았어요.
# Red words when a step was refused because this login may never do it; the last sentence says what may help.
common-refused-not-done-not-allowed = 작업하지 않았어요. 이 계정은 비밀번호가 맞아도 그 작업을 할 수 없어요. 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요.
# Red words when a step was refused because nothing was there to ask for the password; the last sentence says what may help.
common-refused-not-done-unasked = 비밀번호 창이 나타나지 않아서 작업하지 않았어요. 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요.
# Red words when a step was refused and it is not known why; the last sentence says what may help.
common-refused-not-done-unclear = 작업하지 않았어요. 관리자 비밀번호가 맞지 않았거나 이 계정이 그 작업을 할 수 없어요. 비밀번호 창이 나타나지 않았다면 로그아웃했다가 다시 로그인한 뒤 다시 시도하세요.
# Red words when a step behind the administrator's password failed and said nothing of why.
common-refused-no-reason = 작업하지 않았어요. 문제가 생겼는데 이유는 알려지지 않았어요.

## The keyring (src/common/keyring_said.rs)

# Red words when the keyring will not open without the person's password.
common-keyring-locked = 키링이 잠겨 있어요. 로그아웃했다가 다시 로그인하거나 키링을 잠금 해제한 뒤 다시 시도하세요.
# Red words when this computer has no keyring to keep a login or key in.
common-keyring-missing = 저장할 키링이 이 컴퓨터에 없어요.
# Red words when the keyring is there and gives no answer in time.
common-keyring-silent = 키링이 제때 응답하지 않았어요.
# Red words when work with the keyring never ended, long after its deadline.
common-keyring-never = 키링이 끝내 응답하지 않았어요.
# Red words when the turn with the keyring could not be taken. { $why } is the system's reason.
common-keyring-turn-failed = Trindle이 키링에 접근하지 못해서 키링을 사용하지 않았어요({ $why }).
# Line shown while the keyring has not answered whether it kept or removed a login or key. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-waiting =
    { $action ->
        [removing] 키링이 아직 응답하지 않아서 { $what }을(를) 삭제하지 않았어요. 키링이 나중에 삭제하면 이 메시지로 알려드려요.
       *[other] 키링이 아직 응답하지 않아서 { $what }을(를) 저장하지 않았어요. 키링이 나중에 저장하면 이 메시지로 알려드려요.
    }
# Line shown when the keyring answered late and did what was asked. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-done =
    { $action ->
        [removing] 키링이 늦게 응답했고, { $what }을(를) 삭제했어요.
       *[other] 키링이 늦게 응답했고, { $what }을(를) 저장했어요.
    }
# Red words when the keyring never answered. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping".
common-keyring-late-never =
    { $action ->
        [removing] 키링이 응답하지 않아서 { $what }을(를) 삭제하지 않았어요.
       *[other] 키링이 응답하지 않아서 { $what }을(를) 저장하지 않았어요.
    }
# Red words when the keyring answered late and would not do it. { $what } is what is kept, such as "login"; { $action } is "removing" or "keeping"; { $why } is the reason, as a sentence.
common-keyring-late-refused =
    { $action ->
        [removing] 키링이 늦게 응답했지만 { $what }을(를) 삭제하지 않았어요: { $why }
       *[other] 키링이 늦게 응답했지만 { $what }을(를) 저장하지 않았어요: { $why }
    }

## After an update, which parts are still the old program (src/common/renewed.rs)

# Name of a part of Trindle, said in the sentence about what is not yet the new version: the ring.
common-renewed-ring = 링
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the window that opens and saves files for other programs.
common-renewed-portal = 열기와 저장 창
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the wallpaper.
common-renewed-wallpaper = 배경화면
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the clipboard.
common-renewed-clipboard = 클립보드
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the keyring.
common-renewed-keyring = 키링
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that dims the screen and locks it.
common-renewed-idle = 화면 어둡게 하기와 잠금
# Name of a part of Trindle, said in the sentence about what is not yet the new version: night light.
common-renewed-nightlight = 야간 모드
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the part that lets other computers find this one.
common-renewed-wsdd = 네트워크 검색
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the plugins that run in the background.
common-renewed-plugin = 백그라운드에서 실행되는 플러그인
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the desktop itself.
common-renewed-compose = 데스크톱 자체
# Name of a part of Trindle, said in the sentence about what is not yet the new version: the link an AI assistant is using.
common-renewed-mcp = AI 도우미가 쓰는 연결
# Joins two or more names of parts in a list, all but the last: "the ring, the keyring". { $first } is the list so far; { $next } is the next name.
common-renewed-join = { $first }, { $next }
# Joins the last name of a list of parts to the rest: "the ring, the keyring and night light". { $most } is the list so far.
common-renewed-join-last = { $most } 및 { $last }
# Sentence on Settings and Apps after an update, when some parts stay the old program until the next log out. { $list } is the names of the parts.
common-renewed-sentence = 이 업데이트는 이미 적용됐어요. 다만 { $list }은(는) 다음에 로그아웃한 뒤에 새 버전으로 바뀌어요.
# Sentence on Settings and Apps after an update, when some parts may still be the old program and will change at the next safe moment or the next log out. { $list } is the names of the parts.
common-renewed-sentence-latest = 이 업데이트는 이미 적용됐어요. 다만 { $list }은(는) 늦어도 다음에 로그아웃할 때 새 버전으로 바뀌어요.

## Older programs in boxes (src/common/x11_rooms.rs)

# Title of the notice, and red words in Apps, when the older-programs display for a box would not start.
common-x11-would-not-start = Xwayland가 시작되지 않았어요
# Second line of that notice, when the desktop will try the older-programs display again soon.
common-x11-try-again = 1분 뒤에 다시 시도하세요

## What is using the microphone, camera, screen, input or copied files (src/common/in_use.rs)

# Head of the card in Settings, Permissions, and of the page in Welcome, that shows each mark with what it means.
common-in-use-key = 표시의 의미
# Under the marks in that key: where a mark shows up. On the screen it is a half circle hung from the top edge; on a window tab it is a small dot.
common-in-use-where = 반원은 화면 위쪽 가운데에 걸려 있어요. 창 탭에는 작은 점이 보여요.

# Screen reader name of the microphone dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-microphone = 마이크 사용 중
# Line on pointing at the dots, and its screen reader name, when programs use the microphone. { $who } is the programs' names.
common-in-use-microphone-by = { $who }이(가) 마이크 사용 중
# Screen reader name of the camera dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-camera = 카메라 사용 중
# Line on pointing at the dots, and its screen reader name, when programs use the camera. { $who } is the programs' names.
common-in-use-camera-by = { $who }이(가) 카메라 사용 중
# Screen reader name of the screen dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-screen = 화면 녹화 중
# Line on pointing at the dots, and its screen reader name, when programs record the screen. { $who } is the programs' names.
common-in-use-screen-by = { $who }이(가) 화면 녹화 중
# Screen reader name of the input dot and its row, and the line shown on pointing at the dots, when no program is named: the mouse and keyboard are in another program's hands.
common-in-use-input = 마우스와 키보드 사용 중
# Line on pointing at the dots, and its screen reader name, when programs have taken the mouse and keyboard. { $who } is the programs' names.
common-in-use-input-by = { $who }이(가) 키보드와 마우스를 사용 중이에요
# Screen reader name of the copied files dot and its row, and the line shown on pointing at the dots, when no program is named.
common-in-use-files = 복사한 파일을 읽고 있어요
# Line on pointing at the dots, and its screen reader name, when programs read the files the person just copied. { $who } is the programs' names.
common-in-use-files-by = { $who }이(가) 복사한 파일을 읽고 있어요

# The ring's who-is-using-what row while a plugin reaches devices on the local network.
common-in-use-home-network = 로컬 네트워크 사용 중

# The same with the plugin's name.
common-in-use-home-network-by = { $who }이(가) 로컬 네트워크 사용 중

# The ring's who-is-using-what row while a plugin uses scanners and USB devices.
common-in-use-usb = 스캐너와 USB 장치 사용 중

# The same with the plugin's name.
common-in-use-usb-by = { $who }이(가) 스캐너와 USB 장치 사용 중
# Line on pointing at the dots when the microphone is switched off and the program listening hears nothing.
common-in-use-microphone-off = 마이크 꺼짐
# Line on pointing at the dots when the microphone is switched off but programs want it. { $who } is the programs' names.
common-in-use-microphone-off-by = 마이크 꺼짐. { $who }이(가) 사용하려고 했어요.
# Line under a microphone's mute switch in Settings and the ring's sound page while the microphone is off for every program.
common-in-use-microphone-off-note = 링의 마이크 버튼으로 모든 프로그램에서 마이크를 껐어요.

## A settings file that is damaged or cannot be read (src/common/damaged.rs). Help section: settings-unreadable

# Said where the person is working when a file of their own was damaged, was put aside beside itself and a new one begun. { $what } names what the file keeps, such as "Launch counts".
common-damaged-reset = 파일이 손상되어 { $what }을(를) 초기화했어요. 손상된 파일은 옆에 보관했어요.
# Said where the person is working when a file of their own is there but cannot be read, so nothing is saved into it and it is left as it is. { $what } names what the file keeps, such as "Launch counts".
common-unreadable-kept = Trindle이 파일을 읽을 수 없어서 { $what }을(를) 저장하지 않았어요. 아무것도 바꾸지 않았어요.
# Said where the person is working when one of Steam's own settings files is there but cannot be read, so it is left as it is.
common-steam-unreadable = Steam 설정을 읽을 수 없어서 아무것도 바꾸지 않았어요.
# Said when a permission is asked to be changed for something that is not an app's name.
common-flatpak-not-an-app = 앱 이름이 아니에요.
# Said when a permission is asked for that Trindle doesn't know.
common-flatpak-not-a-permission = Trindle이 바꿀 수 있는 권한이 아니에요.
# Said where fonts are being added when the file chosen holds far more than a set of fonts is, so none of it was unpacked.
common-fonts-too-big = 글꼴 모음이라고 하기엔 파일이 너무 커서 아무것도 풀지 않았어요.
# Said where fonts are being added when the file chosen could not be unpacked.
common-fonts-unpack-failed = 파일을 풀 수 없어요. 파일이 손상되었거나 종류가 다를 수 있어요.
# Said where fonts are being added when the program that unpacks files could not run. { $error } is the system's reason.
common-fonts-unpack-error = 파일을 풀 수 없어요: { $error }
# Said where fonts are being added when the file chosen holds no fonts. Help has a section on adding fonts.
common-fonts-none = 이 파일에는 글꼴이 없어요. 도움말의 글꼴 추가를 보세요.
## Displays (src/common/outputs.rs)

# Said where displays are set when the desktop refused the change and gave no reason.
common-displays-refused = 데스크톱이 디스플레이를 그렇게 설정할 수 없었어요.
# Said where displays are set when the desktop refused the change. { $why } is the desktop's own reason.
common-displays-refused-why = 데스크톱이 디스플레이를 그렇게 설정할 수 없었어요: { $why }.
# Said where displays are set when the displays changed again and again while the change was being made.
common-displays-kept-changing = 디스플레이가 계속 바뀌어서 아무것도 바꾸지 않았어요.
# Said where displays are set when Settings is not running in a desktop.
common-displays-no-desktop = Trindle이 데스크톱을 찾지 못했어요.
# Said where displays are set when the desktop has no way to list its displays.
common-displays-cannot-say = 이 데스크톱은 어떤 디스플레이가 있는지 알려 주지 않아요.
# Said where displays are set when the desktop gave no list of its displays.
common-displays-did-not-say = 데스크톱이 어떤 디스플레이가 있는지 알려 주지 않았어요.
# Said where displays are set when the change would switch every display off.
common-displays-none-on = 그렇게 하면 켜진 디스플레이가 없어져요.
# Said where displays are set when the desktop gave no answer within a few seconds, or went away.
common-displays-stopped-answering = 데스크톱이 응답하지 않아요.
# Said where the lock screen PIN is set when what was typed is not the right length. { $n } is how many digits a PIN has.
common-pin-digits = PIN은 { $n }자리예요.
# Said where the lock screen PIN is set when it could not be turned into the secret form it is kept in.
common-pin-not-hashed = PIN을 저장하지 못했어요.
# Said where a theme is chosen when no theme has the name asked for.
common-themes-no-such = 그 이름의 테마가 없어요.
# Said where a theme is saved when it could not be kept. { $error } is the system's reason.
common-themes-not-kept = 테마를 저장하지 못했어요: { $error }.
# Said where a theme file is added, when its name doesn't end in .css. { $name } is the file's name.
common-theme-add-not-css = { $name }은(는) 테마가 아니에요. 테마는 이름이 .css로 끝나는 파일 하나예요.
# Said where a theme file is added, when it can't be read. { $name } is the file's name.
common-theme-add-unreadable = Trindle이 { $name }을(를) 읽지 못했어요.
# Said where a theme file is added, when it is over 64 KB. { $name } is the file's name.
common-theme-add-too-big = { $name }은(는) 테마라고 하기엔 너무 커요. 테마는 최대 64 KB예요.
# Said where a theme file is added, when it holds more than plain text. { $name } is the file's name.
common-theme-add-not-a-sheet = { $name }은(는) 스타일 시트가 아니에요. 일반 텍스트 외의 내용이 들어 있어요.
# Said where a theme file is added, when reading it took too long. { $name } is the file's name.
common-theme-add-too-slow = { $name }을(를) 읽는 데 너무 오래 걸렸어요.

# Under Trindle's own look on Settings, Appearance: a few words about it.
common-theme-trindle-about = Trindle 고유의 모양: 깊은 푸른 밤, 부드러운 모서리.
# Under the Bold theme on Settings, Appearance.
common-theme-bold-about = 선명한 테두리의 흑백, 더 큰 글자와 더 큰 버튼. 저시력 사용자용이에요.
# Under the Midnight theme on Settings, Appearance.
common-theme-midnight-about = 어둡고 평평한 회색 단계: 검정과 회색, 테두리 없음, 작은 모서리.
# Under the Paper theme on Settings, Appearance.
common-theme-paper-about = 밝은 방을 위한, 크림색 책상 위의 따뜻한 흰 종이. 밝은 모양이에요.
# Under the Snug theme on Settings, Appearance.
common-theme-snug-about = 더 작고 더 촘촘하며, 각지고 차가운 회색. 작은 화면에 더 많이 들어가요.
## Drives (src/common/volumes.rs)

# Said where a drive is unlocked when the passphrase did not open it.
common-volumes-wrong-passphrase = 암호가 맞지 않아 열지 못했어요.
# Said where a drive is opened when it is open already.
common-volumes-already-open = 이미 열려 있어요.
# Said where a drive is opened when it is locked and no passphrase has been given.
common-volumes-needs-passphrase = 열려면 암호가 필요해요.
# Said where a drive is opened when the small window that asks for its passphrase could not be shown.
common-volumes-window-failed = 암호 창을 열지 못했어요.
# What the file kept by the list of programs a box may use is called, in the words above.
common-unreadable-name-box-permissions = 앱 권한
# What the file that notes which questions an app has already put to the person is called, in the words above.
common-unreadable-name-box-asked = 이미 한 질문
# What the file that notes which apps came with the image is called, in the words above.
common-unreadable-name-box-offered = 처음 로그인 때 제안한 앱
# What the lists of a fenced program (its folders, hosts, settings and programs) are called, in the words above.
common-unreadable-name-fence = 울타리 목록
# What the list of icons the desktop found missing is called, in the words above.
common-unreadable-name-missing-icons = 없는 아이콘 목록
# What the screen arrangement an older version kept in display.sh is called, in the words above.
common-unreadable-name-old-display = 화면 배치

## Hardware with no driver (src/common/no_driver.rs)

# Said where the person works when a known chip has no driver yet. The kernel is a version number.
common-no-driver-known-driver = { $chip }에 쓸 드라이버가 아직 없어요. Linux { $kernel }부터 드라이버가 들어 있어요.
# Said when a known chip is missing a firmware file. The package and version name where it comes from.
common-no-driver-known-firmware = { $chip }의 펌웨어가 없어요. { $package } { $version }에 들어 있어요.
# Said of a chip nothing is known about that has no driver. The class is Wi-Fi, Bluetooth, sound or graphics; the id is two hex numbers.
common-no-driver-generic = 이 { $class } 칩({ $id })에 쓸 드라이버가 없어요.
# Said of a chip nothing is known about whose firmware file could not be loaded.
common-no-driver-generic-firmware = { $class } 펌웨어 없음: { $file }.
# Added after a Wi-Fi line.
common-no-driver-stick = 당분간은 USB Wi-Fi 어댑터를 쓸 수 있어요.
# Tip of the small round i button that opens Help.
common-no-driver-more = 자세히 보기
common-no-driver-class-wifi = Wi-Fi
common-no-driver-class-bluetooth = Bluetooth
common-no-driver-class-sound = 사운드
common-no-driver-class-graphics = 그래픽
# Said of a device with no driver that nothing is known about. The name is its name, or its two hex numbers.
common-no-driver-device-none = { $name }에 쓸 드라이버가 없어요.

# The small round button beside a failure message that opens Help at the page
# about it. Its tip, and the name a screen reader says.
common-more = 더 보기
common-more-spoken = 자세히 보기, 도움말이 열려요

## Pocketkey's "ask for the password again after" (src/common/pocketkey.rs)

# First choice of "Ask for the password again after": the phone stays good for as long as it likes.
common-pocketkey-never = 사용 안 함
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } minutes.
common-pocketkey-minutes =
    { $n ->
       *[other] { $n }분
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } hours.
common-pocketkey-hours =
    { $n ->
       *[other] { $n }시간
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } days.
common-pocketkey-days =
    { $n ->
       *[other] { $n }일
    }
# A choice of "Ask for the password again after", and the time in the screen's label: { $n } weeks.
common-pocketkey-weeks =
    { $n ->
       *[other] { $n }주
    }
# On the lock and login screens, when the time chosen has passed and the phone is not accepted: { $time } is such as "1 hour".
common-pocketkey-due = { $time } 넘게 지났어요. 비밀번호를 다시 입력해야 해요.
# Said under the greyed Earlier versions card.
common-snap-no-snapper = 이전 버전을 쓰려면 snapper가 필요한데, 이 컴퓨터에는 설치되어 있지 않아요.
# Said under the greyed Earlier versions card.
common-snap-from-snapshot = 스냅샷으로 컴퓨터를 시작한 동안에는 쓸 수 없어요.
# Said under the greyed Earlier versions card.
common-snap-part-of-system = 이 컴퓨터에서는 홈 폴더가 별도 디스크 영역이 아니라 시스템의 일부예요. 그래서 시스템과 따로 이전 버전을 보관할 수 없어요.
# Said under the greyed Earlier versions card.
common-snap-network = 홈 폴더가 다른 컴퓨터에 있어서 여기에는 이전 버전을 보관할 수 없어요.
# Said under the greyed Earlier versions card: { $fs } is the kind of file system, such as ext4.
common-snap-not-btrfs = 이전 버전을 쓰려면 홈 폴더가 Btrfs에 있어야 하는데, 지금은 { $fs }에 있어요.
# Said under the greyed Earlier versions card.
common-snap-not-whole = 홈 폴더가 별도 영역이 아니라 디스크의 다른 영역 안에 있어서 Trindle이 이전 버전을 보관할 수 없어요.
# Said under the greyed Earlier versions card.
common-snap-read-only = 지금은 홈 폴더에 쓸 수 없어서 아무것도 보관할 수 없어요.
# Said under the greyed Earlier versions card.
common-snap-locked-files = 여기서는 홈 폴더가 잠긴 파일 하나로 저장돼요. 매일 복사본을 만들면 디스크가 가득 차요.
# Said under the greyed Earlier versions card.
common-snap-elsewhere = 홈 폴더가 복사본을 만드는 /home 안에 있지 않아요.
# Said under the greyed Earlier versions card.
common-snap-apart = 홈 폴더가 잠겨 있거나 다른 폴더와 따로 보관되어 있어서 복사본에 넣을 수 없어요.
# Said under the greyed Earlier versions card.
common-snap-own-part = 홈 폴더가 별도 디스크 영역이라서 /home 복사본에 들어가지 않아요.
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written.
common-firewall-not-protocol-port = "{ $text }"은(는) 프로토콜:포트 형식이 아니에요. 예: tcp:24810.
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written before the colon.
common-firewall-bad-protocol = "{ $text }"은(는) tcp나 udp가 아니에요. 소문자로 써 주세요.
# Why a plugin's port line in plugin.ini is refused. { $text } is what was written, { $lowest } the lowest port allowed.
common-firewall-bad-port = "{ $text }"은(는) { $lowest }부터 65535까지의 포트가 아니에요.
# Said when a port you open by hand isn't written as protocol:port.
common-firewall-own-form = 프로토콜:포트 형식으로 써 주세요. 예: tcp:37666.
# Said when a port you open by hand starts with something other than tcp or udp.
common-firewall-own-protocol = tcp나 udp를 쓰세요.
# Said when a port you open by hand isn't a number from 1 to 65535.
common-firewall-own-port = 1부터 65535까지의 포트가 아니에요.
# Joins the last port to the others in a list of ports. { $first } is the others, comma separated.
common-firewall-and = { $first } 및 { $last }
# Heads a list of ports. { $count } is how many, { $list } is the ports, such as TCP 24810 and UDP 24811.
common-firewall-ports =
    { $count ->
       *[other] 포트 { $list }
    }
# Why a plugin's ports are refused. { $max } is the most it may name.
common-firewall-too-many = 플러그인은 포트를 최대 { $max }개까지 지정할 수 있어요.
# Why a plugin's ports are refused. { $port } is a port such as TCP 24810.
common-firewall-twice = { $port }이(가) 두 번 지정되었어요.
# Why a plugin's ports are refused. { $port } is a port such as TCP 80, { $lowest } the lowest allowed.
common-firewall-too-low = { $port }은(는) { $lowest }보다 낮아요. 그 포트는 시스템용으로 남겨 두었어요.
# Why a plugin's port is refused. { $port } is the number.
common-firewall-reserved = { $port } 포트는 Trindle이나 일반 서비스가 쓰고 있어서 플러그인이 요청할 수 없어요.
# Why a plugin's port can't be opened. { $port } is such as TCP 24810, { $plugin } is the other plugin's name.
common-firewall-taken = { $port }은(는) 이미 { $plugin }이(가) 쓰고 있어서 이 플러그인에는 열 수 없어요.
# Why a plugin cannot open another port: { $max } is how many may be open.
common-firewall-full = 플러그인용으로 열 수 있는 포트 최대 { $max }개가 이미 열려 있어요.
# Told by a notice when the ring has been started again.
common-ring-held-back = 링이 여러 번 연속으로 멈춰서 Trindle이 잠시 중지했어요. 지금은 다시 실행 중이에요.
# Told by a notice when the ring has been started again.
common-ring-not-started = 링을 한동안 시작하지 못했어요. 지금은 다시 실행 중이에요.
# Told by a notice when the ring has been started again.
common-ring-not-ended = 링이 응답을 멈췄고 한동안 종료하지 못했어요. 지금은 다시 실행 중이에요.
# Told by a notice.
common-ring-not-done = 컴퓨터에 절전, 다시 시작 또는 종료를 요청했지만 실행되지 않았어요.
# Told by a notice; the ring is what the round menu is called.
common-ring-by-name = 데스크톱이 이 링을 직접 시작하지 못해서 전체 화면 창 위에 열 수 없고, 알림도 맨 위에 유지되지 않아요. 로그아웃했다가 다시 로그인하면 해결될 수 있어요.
# A group of apps in the ring.
common-category-internet = 인터넷
# A group of apps in the ring (as above).
common-category-office = 사무
# A group of apps in the ring (as above).
common-category-graphics = 그래픽
# A group of apps in the ring (as above).
common-category-media = 소리와 영상
# A group of apps in the ring (as above).
common-category-games = 게임
# A group of apps in the ring (as above).
common-category-development = 개발
# A group of apps in the ring (as above).
common-category-learning = 학습
# A group of apps in the ring (as above).
common-category-utilities = 유틸리티
# A group of apps in the ring (as above).
common-category-system = 시스템
# Why an app cannot be started: its launcher names nothing to run.
common-app-no-command = 실행할 명령이 없어요.
# Why a launcher was left alone: { $why } is the reason.
common-launcher-unchanged = { $why } 때문에 아무것도 바뀌지 않았어요.
# The ring's notice when something is plugged in.
common-drive-arrived-usb = USB 드라이브가 연결되었어요
# The ring's notice when something is plugged in (as above).
common-drive-arrived-card = 메모리 카드가 연결되었어요
# The ring's notice when something is plugged in (as above).
common-drive-arrived-disc = 디스크가 삽입되었어요
# The ring's notice when something is plugged in (as above).
common-drive-arrived-other = 드라이브가 연결되었어요
# Said in the password box after what the password is for.
common-drive-every-time = 드라이브를 바꿀 때마다 물어봐요. 이 창을 닫으면 아무것도 바뀌지 않아요.
# Said in the password box: { $rest } is what is being done, such as "format this drive".
common-drive-password-needed-to = { $rest }하려면 관리자 비밀번호가 필요해요
# Said in the password box when the action is not named.
common-drive-password-needed = 이 드라이브를 바꾸려면 관리자 비밀번호가 필요해요
# Why a folder cannot be shared.
common-share-not-yours = 내 폴더만 공유할 수 있어요.
# Why a share cannot have the name asked for.
common-share-name-is-login = 그 이름은 이 컴퓨터의 다른 사용자 로그인 이름이라서 공유 이름으로 쓸 수 없어요. 다른 이름을 고르세요.
# Why a share cannot have the name asked for.
common-share-bad-name = 공유 이름에는 % < > * ? | / \ + = ; : " 또는 쉼표를 쓸 수 없어요.
# Why a folder cannot be shared.
common-share-no-guests = 이 컴퓨터의 Samba 설정이 비밀번호 없는 공유를 허용하지 않아요.
# Why a folder cannot be shared.
common-share-most = 이 컴퓨터가 허용하는 만큼의 폴더를 이미 공유하고 있어요.
# Why a folder cannot be shared.
common-share-gone = 그 폴더가 더 이상 없어요.
# Why a folder cannot be shared, when the program said nothing.
common-share-no-reason = Samba가 이유를 알려 주지 않았어요.
# Why a folder cannot be shared: { $said } is what the sharing program said.
common-share-said = Samba가 알려 준 내용: { $said }
# Why a screenshot could not be taken.
common-shot-no-desktop = 촬영할 데스크톱이 없어요.
# Why a screenshot could not be taken.
common-shot-no-display = 촬영할 화면이 없어요.
# Why a screenshot could not be taken.
common-shot-no-display-there = 데스크톱의 그 부분에는 화면이 없어요.
# Why a screenshot could not be taken.
common-shot-cannot = 이 데스크톱은 스크린샷을 찍을 수 없어요.
# Why a screenshot could not be taken.
common-shot-too-big = 스크린샷이 너무 커요.
# Why a screenshot could not be taken.
common-shot-no-size = 화면이 빈 그림을 돌려줘서 Trindle이 스크린샷을 찍지 못했어요.
# Why a screenshot could not be taken.
common-shot-unreadable = 데스크톱이 읽을 수 있는 그림을 주지 않았어요.
# Why a screenshot could not be taken.
common-shot-no-memory = 그림을 담을 메모리가 부족했어요.
# Why a screenshot could not be taken.
common-shot-stopped = 데스크톱이 화면 촬영을 중단했어요.
# Why a screenshot could not be taken.
common-shot-failed = 데스크톱이 화면을 촬영하지 못했어요.
# Said where the person is working when no screen reader is installed.
common-reader-none-yet = 스크린 리더가 설치되어 있지 않아요.
# Said wherever hiding a drive is offered or shown.
common-drive-not-a-lock = 숨기기는 잠금이나 보호가 아니에요. 똑같이 자신을 알리는 쌍둥이 드라이브도 함께 숨겨져요.
# Said when typed words are not a change to a key.
common-remap-not-a-key = Trindle이 아는 키가 아니에요. Escape, Ctrl+C, Left Click을 써 보세요.
# Said when typed words are not a change to a key.
common-remap-too-long = 한 번의 변경에 담을 수 있는 키보다 많아요.
# Said when typed words are not a change to a key or a button.
common-no-equals = 변경에는 누르는 것, 등호, 바뀔 대상이 필요해요.
# Said when typed words are not a change to a key.
common-remap-tapped-held = 짧게 누를 때와 길게 누를 때를 각각 쓰세요. 예: Escape tapped, Ctrl held.
# Said when typed words are not a change to a controller button.
common-pads-not-a-button = Trindle이 아는 버튼이 아니에요. South, Left Trigger, Escape를 써 보세요.
# Said when typed words are not a change to a controller.
common-pads-not-a-zone = 데드존은 0%에서 50%까지예요.
# Said of a line in the controllers file that is not understood.
common-pads-not-a-heading = Trindle이 이 컨트롤러 제목을 알아보지 못해요.
# Said of a line in the controllers file that is not understood.
common-pads-no-heading = 어떤 컨트롤러 제목 아래에도 있지 않아요.
# What a theme's tile says when its file holds more than a theme may.
common-theme-too-big = 이 파일은 테마로 쓰기에는 너무 커서 사용하지 않아요. 테마는 최대 64 KB예요.
# What a theme's tile says when it is not an ordinary file.
common-theme-not-a-sheet = 일반 텍스트 파일이 아니라서 테마가 될 수 없고 사용하지 않아요.
# What is said of a theme that was chosen and whose file has since gone.
common-theme-gone = 그 파일이 더 이상 없어요.
# What is said of a theme on a disk that does not answer.
common-theme-too-slow = 파일을 읽는 데 너무 오래 걸려서 Trindle이 사용하지 않아요.
# What is said when the folders of themes are on a disk that does not answer.
common-theme-list-slow = 테마 폴더를 읽는 데 너무 오래 걸려서 Trindle 기본 모양만 보여 줘요.
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-big = 내 스타일 시트 style.css가 256 KB를 넘어서 사용하지 않아요.
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-not-a-sheet = 내 스타일 시트 style.css가 일반 텍스트 파일이 아니라서 사용하지 않아요.
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-too-slow = 내 스타일 시트 style.css를 읽는 데 너무 오래 걸려서 사용하지 않아요.
# What is said when colours in the person's own style sheet fall short of what can be read.
common-theme-own-hard-to-read = 읽기 어려운 색이 있어요.
# What is said under the tiles when a folder holds more themes than are shown.
common-theme-not-all = 폴더에 목록보다 많은 테마가 있을 수 있어요. 폴더마다 처음 40개 테마만 보여 주고, 처음 400개 파일만 살펴봐요.
# What is said of the person's own style sheet when it cannot be used.
common-theme-own-no-target = 내 스타일 시트 style.css가 없는 파일로 연결되어 있어서 사용하지 않아요.
# The name of a file set aside when damaged, said in a sentence: Clipboard history.
common-damaged-clipboard = 클립보드 기록
# The name of a file set aside when damaged, said in a sentence: Last-used apps.
common-damaged-last-used = 최근에 쓴 앱
# What kind of virtual machine this is, said in a sentence.
common-vm-qemu = QEMU 가상 머신(GNOME Boxes, virt-manager 등)
# What kind of virtual machine this is, said in a sentence.
common-vm-apple = Mac의 가상 머신
# What kind of virtual machine this is, said in a sentence: { $name } is the name systemd gives it.
common-vm-other = 가상 머신({ $name })
# What a program offered in Software sources is for, in a line.
common-source-claude = Anthropic의 Claude 앱: 채팅, Cowork, Claude Code
# What a program offered in Software sources is for, in a line.
common-source-tailscale = 내 기기끼리 잇는 개인 네트워크
# What a program offered in Software sources is for, in a line.
common-source-syncthing = 클라우드 없이 내 컴퓨터들의 폴더를 똑같이 맞춰 줘요. Debian 기본 버전은 더 오래된 버전이에요.
# What a program offered in Software sources is for, in a line.
common-source-jellyfin = 집 안 어느 화면에서나 볼 수 있는 내 영화, 음악, 사진 서버
# What a program offered in Software sources is for, in a line.
common-source-nodejs = 개발자용 Node.js 최신 장기 지원 버전. Debian 기본 버전은 20이에요.
# What a program offered in Software sources is for, in a line.
common-source-github-cli = 터미널에서 쓰는 GitHub, 개발자용. Debian 기본 버전은 더 오래된 버전이에요.
# What a program offered in Software sources is for, in a line.
common-source-docker = 프로그램을 컨테이너에서 실행해요. 개발자용
# Said in the box that asks for a drive's passphrase: { $name } is the drive.
common-volumes-passphrase-for = { $name }의 암호문
# The title of the box that asks for a drive's passphrase.
common-volumes-unlock-title = 드라이브 잠금 해제

# What a snapshot is, in the Earlier versions list: made just before software was changed.
common-snap-before-change = 소프트웨어 변경 전
# What a snapshot is, in the Earlier versions list: made just after software was changed.
common-snap-after-change = 소프트웨어 변경 후
# What a snapshot is, in the Earlier versions list: made when the computer started.
common-snap-boot = 컴퓨터를 시작했을 때
# What a snapshot is, in the Earlier versions list: one taken on a schedule.
common-snap-regular = 정기 복사본
# What a snapshot is, in the Earlier versions list, when it has no note.
common-snap-plain = 스냅샷
# Said when a font file can't be added. { $name } is the file's name, { $error } the system's reason.
common-pack-not-copied = Trindle이 { $name }을(를) 복사하지 못했어요: { $error }
# Said when a file given as a font isn't one. { $name } is the file's name.
common-pack-not-fonts = { $name }은(는) 글꼴이나 글꼴 압축 파일이 아니에요.
# Said when a drive can't be hidden because it has no name of its own to remember it by.
common-hidden-no-name = 이 드라이브는 고유한 이름이 없어서 Trindle이 기억할 수 없어요.
# Said when the list of hidden drives can't be read. { $error } is the system's reason.
common-hidden-left = 숨긴 드라이브 목록을 그대로 두었어요: { $error }
# Said when a drive can't be released because your login session can't be found.
common-leave-no-session = Trindle이 로그인 세션을 찾지 못해서 아무것도 바뀌지 않았어요.
# Said under the sound choices when PipeWire gives no answer and says nothing about why.
common-audio-no-answer = PipeWire가 응답하지 않았어요.
# Said when the screen reader can't be turned on or off. { $error } is the first problem, { $undo } the second.
common-screen-reader-undo-failed = { $error } Trindle이 변경을 되돌리지도 못했어요: { $undo }
# How long copies are kept, in a sentence such as "kept for a week".
common-keep-week = 1주일
# How long copies are kept, in a sentence such as "kept for two weeks".
common-keep-two-weeks = 2주일
# How long copies are kept, in a sentence such as "kept for a month".
common-keep-month = 1개월
# How long copies are kept, in a sentence such as "kept for 3 days". { $n } is the number of days.
common-keep-days =
    { $n ->
       *[other] { $n }일
    }

# Said when an app can't be started. { $program } is the command, { $error } the system's reason.
common-app-not-started = Trindle이 { $program }을(를) 시작하지 못했어요: { $error }

# Said when a settings file is too big to read.
common-keyfile-too-big = 그 파일은 너무 커서 읽을 수 없어요.
# Said when a file Trindle was about to write at login isn't yours.
common-login-file-not-yours = 그곳의 파일은 내 것이 아니에요.
# What the security updates switch says on a hover while it is on.
common-auto-updates-on = 자동 업데이트 켜짐
# What the security updates switch says on a hover while it is off.
common-auto-updates-off = 자동 업데이트 꺼짐
# Said when polkit's list of the passwords it is keeping for this login couldn't be read, so Trindle can't make sure a password given for a drive is forgotten.
common-leave-list-unread = 아직 보관 중인 비밀번호를 읽을 수 없었어요.
