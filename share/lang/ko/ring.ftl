# machine translated (Korean), not yet reviewed by a person.

# The ring's own words (src/ring): the circle opened from the top of the screen, its pages, the notification circles, the tray and the small notices.
# Keys are ring-<page>-<thing>. Words shared by several pages are ring-<word>.

## Words shared by the ring's pages

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = 닫기
# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = 열기
# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = 닫을 창이 없어요
# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = 설정

## Command line (src/ring/mod.rs)

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = 사용법:
# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [OPTION…]
# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = 옵션:
# Help for the ring's --say option, printed in a terminal.
ring-cli-say = 마이크 없이 글로 음성 명령을 실행합니다
# Help for the ring's --background option, printed in a terminal.
ring-cli-background = 눈에 보이지 않게 시작해서, 처음 열 때 링이 바로 열리게 합니다
# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = 모든 창을 숨겨 바탕화면을 보이거나, 창을 다시 불러옵니다 (Super와 D 키도 같은 일을 합니다)
# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = 키보드 단축키처럼, 음성 명령을 듣는 상태로 링을 엽니다
# Help for the ring's --all option, printed in a terminal.
ring-cli-all = 모든 앱이 둘러선 상태로 링을 엽니다
# Help for the ring's --power option, printed in a terminal.
ring-cli-power = 링의 전원 옵션(로그아웃, 다시 시작, 종료)을 엽니다
# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = 설정의 즐겨찾기 페이지를 엽니다
# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = 설정에서 고른 새 모양(밝게 또는 어둡게, 새 강조색)을 바로 적용합니다
# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = 위쪽 모서리의 터치 영역을 다시 설정합니다
# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = 키보드 조작을 화면의 알림 원으로 옮깁니다 (Super와 N 키도 같은 일을 합니다)
# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = 모든 키보드 단축키 목록을 보이거나 숨깁니다 (설정에서 바꾸지 않았다면 Super와 K)
# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = 알 수 없는 옵션 { $option }
# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = --say에 필요한 인수가 없습니다

## The ring itself (src/ring/view/)

# Tip of the round button on an app's menu that removes the app.
ring-uninstall = 제거
# Line that replaces the uninstall button while the app is being removed.
ring-uninstalling = 제거하는 중
# Line that replaces the uninstall button once the app is removed.
ring-uninstalled = 제거됨
# Tip on the app menu's red button when removing the app did not work.
ring-uninstall-failed = 제거하지 못했어요.
# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = 링에서 빼기
# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = 빼는 중
# Line that replaces that button once the app is off the ring.
ring-taken-off = 뺐어요
# Name of the ring's round button that opens Apps.
ring-tool-apps = 앱
# Name of the ring's round button that opens Activity.
ring-tool-activity = 활동
# Name of the ring's round button that opens Drives.
ring-tool-drives = 드라이브
# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = 스크린샷
# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = 클립보드
# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = 알림
# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = 돋보기
# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = 색 추출
# Name of the ring's round button that opens the report a problem window.
ring-tool-report = 문제 알리기
# Line in the ring when the item that was pressed is not there any more.
ring-gone = 더 이상 없어요.
# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = Trindle이 그것을 열 수 없어요.
# Shorter name of the Sound and video piece of All apps, for when the full name does not fit along it.
ring-category-media-short = 소리
# Line in the ring when a spoken command, such as turning the sound down, did not work.
ring-voice-failed = 되지 않았어요.
# Line in the ring while it listens to a spoken command.
ring-listening = 듣는 중… 명령을 말해 주세요. 말을 멈추면 듣기도 멈춰요
# Line in the ring while it works out what was said.
ring-working-out = 알아보는 중…
# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = 닫을 것이 없어요
# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = { $date } { $time }입니다
# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = 이 화면의 밝기는 여기서 바꿀 수 없어요
# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = 화상 키보드가 설치되어 있지 않아요
# Screen reader name of the ring of open windows.
ring-open-windows = 열린 창
# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = 복원
# Tip of the button on a window's menu that minimises it.
ring-window-minimise = 최소화
# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = 원래 크기로
# Tip of the button on a window's menu that maximises it.
ring-window-maximise = 최대화
# Tip of the button on a window's menu that stops a program that does not answer.
ring-window-force-close = 강제 종료
# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = 모든 앱
# Name of the ring's round button that opens the power page.
ring-power = 전원
# Screen reader description of the ring's power button.
ring-power-spoken = 로그아웃, 다시 시작 또는 종료
# Screen reader name of the ring's results while something is typed.
ring-finding = { $typed } 찾는 중
# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = 날짜와 시간
# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = 새 창
# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
       *[other] 창 { $n }개 모두 닫기
    }
# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = 링에서 고정 해제
# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = 즐겨찾기에서 빼기
# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = 즐겨찾기에 추가

ring-startup-add = 로그인할 때 시작

ring-startup-remove = 로그인할 때 시작하지 않기
# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = 왜 안 됐나요?
# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    가운데에는 { $n }개까지 둘 수 있어요.
    하나를 빼서 자리를 만드세요.
# Choice on the ring's power page: lock the screen.
ring-power-lock = 잠금
# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = 화면 보호기
# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = 절전
# Choice on the ring's power page: hibernate.
ring-power-hibernate = 최대 절전
# Choice on the ring's power page: log out.
ring-power-log-out = 로그아웃
# Choice on the ring's power page: restart the computer.
ring-power-restart = 다시 시작
# Choice on the ring's power page: turn the computer off.
ring-power-shut-down = 종료
# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = 모든 앱을 누른 뒤 앱을 오른쪽 클릭하면 여기에 둘 수 있어요
# Line in the ring when a spoken or typed power choice needs a click to confirm.
ring-power-only-click = 클릭해서 확인하세요.
# Line in the ring when something other than the person tried to shut the computer down.
ring-power-only-own = 이 컴퓨터는 사용자만 종료할 수 있어요.
# Line in the ring when the power page closed because a program pressed a key.
ring-power-closed = 프로그램이 키를 눌러서 전원 페이지가 닫혔어요.

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = 다시 시작했어요
# Under the notice above.
ring-restarted-why = 링과 도우미 부분이 다시 시작되었어요.

# Said under the ring when sleep, log out, restart or shut down is pressed while Trindle is being installed.
ring-power-installing = Trindle을 설치하는 중이에요. 끝날 때까지 기다려 주세요.
