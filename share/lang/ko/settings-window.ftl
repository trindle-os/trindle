# machine translated (Korean), not yet reviewed by a person.

# The Settings window itself: its title, the side bar with its search box and group headings, and the words printed by its command line options. Each page's own words are in the settings-<page>.ftl files.

## Window (src/settings/mod.rs)

# Title of the Settings window, spoken by a screen reader and shown in the window list.
settings-window-title = 설정
# Typing box at the top of the Settings side bar; it filters the pages.
settings-window-search = 설정 검색
# Small grey heading over the side bar's first group of pages: how it looks and sounds.
settings-window-group-personal = 개인
# Small grey heading over the side bar's group of pages for the computer's parts.
settings-window-group-devices = 장치
# Small grey heading over the side bar's group of pages for networks.
settings-window-group-network = 네트워크
# Small grey heading over the side bar's group of pages for the system.
settings-window-group-system = 시스템
# Small grey heading over the side bar's last group of pages: about Trindle, thanks and help.
settings-window-group-about = 정보
# Small grey heading over pages a search finds that are in the Apps program.
settings-window-in-apps = 앱에서
# Small grey heading over the row a search finds that opens the Folders program's settings.
settings-window-in-folders = Folders에서
# The side bar row that opens the Folders program at its own settings.
settings-window-in-folders-name = Folders 자체 설정
# Words the Settings search matches the Folders settings row by, in lower case, separated by spaces.
settings-window-in-folders-keywords =
    folders 폴더 파일 파일 관리자 시작 패널 홈 최근 드라이브 usb 메모리 sd 카드 연결 열기 꺼내기 목록 확장자 이름 체크 상자 선택 행 촘촘하게 보통 넓게 files file manager start pane recent drives stick plugged in eject list endings extensions names tick boxes checkboxes picking selecting rows compact normal roomy
# Quiet line in the side bar when a search finds no page.
settings-window-no-page = 맞는 페이지가 없어요. 더 짧은 단어로 검색하거나 도움말을 확인해 보세요.

## Command line (src/settings/mod.rs)

# Printed in a terminal by trindle-settings --themes for a theme that cannot be used; id is its short name, name its full name, why the reason.
settings-window-theme-refused = {"  "}{ $id }  { $name } (사용할 수 없음: { $why })
# Printed in a terminal when trindle-settings --theme is given no theme name.
settings-window-theme-name = 테마 이름을 입력하세요. 예: trindle-settings --theme default
# Printed in a terminal after trindle-settings --theme worked; theme is the theme's name.
settings-window-theme-now = 현재 테마: { $theme }
# The name printed for the built-in theme in the line above.
settings-window-theme-own = Trindle 기본
# Printed in a terminal when the theme could not be chosen; error says why and ends with a full stop.
settings-window-themes-list = { $error } 목록을 보려면 trindle-settings --themes 를 실행하세요.
# Printed in a terminal when trindle-settings --keys is not given the word default.
settings-window-keys-how = 모든 키와 버튼을 원래대로 되돌리려면 다음을 실행하세요: trindle-settings --keys default
# Printed in a terminal after every key and button was put back.
settings-window-keys-back = 모든 키와 버튼이 원래대로 돌아왔어요.
# Printed in a terminal when the keys could not be put back; error says why.
settings-window-keys-failed = Trindle이 키를 되돌리지 못했어요: { $error }
