# machine translated (Korean), not yet reviewed by a person.

# Apps: the pages of apps, the installed list, sources and moving apps (src/store/pages/).

## Pages of apps (src/store/pages/)

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
       *[other] 앱 { $count }개
    }
# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
       *[other] 업데이트 { $count }개 대기 중
    }
# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = Flathub에서 고를 수 있는 앱: { $apps }, { $more }.
# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = Flathub와 AppImageHub에서 고를 수 있는 앱: { $apps }, { $more }.
# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = AppImageHub에서 고를 수 있는 앱: { $apps }.
# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = 업데이트 확인이 꺼져 있어요
# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = 내 앱은 모두 최신이에요
# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.
# Line under the title of a search that found nothing.
store-grid-none-found = “{ $text }”에 맞는 앱이 없어요.
# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = OpenAI가 다운로드에 서명하면 ChatGPT가 다시 나와요.
# Line under the title of an empty page of apps.
store-grid-none-here = 아직 여기에 앱이 없어요.
# Advice in the empty page of a search.
store-grid-try-shorter = 더 짧은 단어로 찾거나 왼쪽에서 분류를 고르세요.
# Advice in the empty page of a kind of apps.
store-grid-search-or-look = 위에서 이름으로 찾거나 왼쪽에서 다른 분류를 고르세요.
# Under a page of apps: how many are shown of how many there are.
store-grid-showing = { $apps }개 중 { $shown }개를 보여 줘요.
# Link under a page of apps that shows more of them.
store-grid-show-more = { $next }개 더 보기
# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Flathub에서 앱 목록을 가져오고 있어요. 처음에는 1분쯤 걸려요.
# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = 앱이 Flathub에 연결하지 못했어요.
# Under the title of the waiting page when the computer is not connected.
store-wait-offline = 이 컴퓨터는 인터넷에 연결되어 있지 않아요. 연결되면 앱이 다시 시도해요.
# In the waiting page when the computer is not connected.
store-wait-comes-from = 고를 수 있는 앱은 인터넷을 통해 Flathub에서 와요.
# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages }와 왼쪽의 앱 설정은 이것 없이도 쓸 수 있어요.
# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages }와 왼쪽의 앱 설정은 Flathub 없이도 쓸 수 있어요.
# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages }와 왼쪽의 앱 설정은 인터넷 없이도 쓸 수 있어요.
# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = 다시 시도
# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = 결과
# Red words under the search box when no web browser opens a link.
store-no-browser = { $url }을 열 웹 브라우저가 없어요. 먼저 앱에서 하나를 설치하세요.

## The Installed page (src/store/pages/)

# Said in the card or under the title while the lists are being read.
store-looking = 찾는 중…
# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = 이제 설치됨 목록과 링에 있어요
# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = 삭제됨
# The word on an app's dial when installing or removing did not work.
store-done-not = 실패
# The word on an app's dial when it was installed.
store-done-installed = 설치됨
# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak이 실패했어요.
# Tab of the Installed page that shows everything.
store-tab-all = 전체
# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak 파일
# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage
# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Deb 파일
# Tab of the Installed page that shows plugins.
store-tab-plugins = 플러그인
# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = 아직 설치된 것이 없어요. 둘러보기에서 써 볼 앱을 찾아보세요.
# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
       *[other] 플러그인 { $n }개가 설치되어 있어요
    }
# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
       *[other] 프로그램 { $n }개가 설치되어 있어요
    }
# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = { $n }개가 설치되어 있어요
# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
       *[other] { $start }, 모두 Flathub에서 설치
    }
# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
       *[other] { $start }, 모두 Flatpak 파일에서 설치
    }
# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
       *[other] { $start }, 모두 AppImage
    }
# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
       *[other] { $start }, 모두 .deb 파일에서 설치
    }
# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }
# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }
# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = Flathub { $n }개
# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
       *[other] Flatpak 파일 { $n }개
    }
# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
       *[other] AppImage { $n }개
    }
# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
       *[other] 앱 { $n }개
    }
# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
       *[other] .deb 파일 { $n }개
    }
# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
       *[other] 플러그인 { $n }개
    }
# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }
# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = 최신 상태예요.
# End of the Installed line when everything is up to date.
store-installed-state-all = 모두 최신 상태예요.
# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
       *[other] { $n }개에 업데이트가 있어요.
    }
# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = 프로그램 { $total }개 중 { $shown }개가 “{ $find }”와 맞아요.
# The same when plugins are among the things.
store-installed-match-things = { $total }개 중 { $shown }개가 “{ $find }”와 맞아요.
# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }
# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = AppImage 추가
# Placeholder of the typing box on the Installed page.
store-installed-search = 설치된 것 검색
# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = AppImage를 추가하지 못했어요.
# In the card of the Installed page when nothing is installed.
store-installed-empty = 설치한 것은 모두 여기에 나와요. 둘러보기나 GitHub에서 설치한 앱, AppImage, .deb 파일의 프로그램, 플러그인이에요.
# In the card of the Installed page when the search matches nothing.
store-installed-no-match = “{ $find }”와 맞는 설치된 것이 없어요.
# On an installed thing's line: its version.
store-installed-version = 버전 { $version }
# On an installed thing's line: its updates are held back.
store-installed-held = 업데이트 보류 중
# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = 업데이트 보류 중, { $waits }
# Tip and spoken name of the button while an app is updating.
store-installed-updating = { $name } 업데이트하는 중
# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = { $name } 업데이트
# Tip of the round button that shows an installed app's page.
store-installed-about = 정보
# Under an installed thing's row while it is being taken off.
store-installed-removing = 삭제하는 중…

## Adding an AppImage or a Flatpak file (src/store/pages/)

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Trindle이 지금 파일 선택 창을 열 수 없어요.
# Title of the window that asks which AppImage to add.
store-add-choose-title = AppImage 고르기
# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage
# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Trindle이 파일 선택 창을 열 수 없어요.
# Words in the Installed card while an AppImage is being added.
store-add-adding = 추가하는 중…
# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name }은(는) 이미 설치되어 있어요. 설치됨에서 찾을 수 있어요.
# Words once an AppImage is in the ring.
store-add-in-ring = 추가했어요. 이제 내 목록에 있어요.
# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = 이 컴퓨터에서 쓸 수 있는 Flatpak 참조가 아니에요.
# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name }을(를) 설치했어요.
# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak이 설치하지 못했어요.
# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = Trindle이 { $name }을(를) 설치하지 못했어요. { $why }
# Words in the Installed card while an AppImage replaces another.
store-add-replacing = 바꾸는 중…
# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name }이(가) 이미 있어요
# Small heading over the AppImage being brought.
store-replace-new = 새 것
# Small heading over the AppImage already there.
store-replace-old = 이미 있는 것
# Tip of the round button that keeps the AppImage already there.
store-replace-skip = 건너뛰고 기존 것 유지
# Button that replaces the AppImage already there with the new one.
store-replace-do = 바꾸기
# Heading of the card that asks what to do with a Flatpak being removed.
store-flatpak-ask-head = { $name }을(를) 삭제할까요?
# Under that heading: how much saved data the app has.
store-flatpak-ask-size = 저장된 데이터가 { $size }예요.
# Under that heading when the app has saved no data.
store-flatpak-ask-none = 저장된 데이터가 없어요.
# Under that heading when the folder of the saved data cannot be read, so its size is not known.
store-flatpak-ask-uncounted = 저장된 데이터의 크기를 잴 수 없었어요.
# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = 저장된 데이터의 크기를 재는 중…
# Under the choices: the two that delete data cannot be undone.
store-flatpak-ask-undone = 삭제한 데이터는 되돌릴 수 없어요.
# Button: remove the app and leave the data it saved.
store-flatpak-keep = 앱만 삭제하고 데이터 유지
# Button (red): remove the app and delete its saved data.
store-flatpak-wipe = 앱과 데이터 삭제
# Button (red): delete only the saved data and keep the app.
store-flatpak-data-only = 데이터만 삭제
# Why Delete its data only is greyed.
store-flatpak-data-only-none = 삭제할 저장된 데이터가 없어요.
# Tip of the round button that shuts the card with nothing done.
store-flatpak-cancel = 취소
# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = 저장된 데이터를 삭제했어요.
# Words while only the data is being deleted.
store-flatpak-data-going = 저장된 데이터를 삭제하는 중…
# Red words under a button when a job ended and gave no reason.
store-job-no-reason = 끝내지 못했어요. 이유는 알려지지 않았어요.
# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = 문제가 있는 곳 보기

## The GitHub page (src/store/pages/)

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = 앱 검색 (Enter를 누르면 GitHub도 검색)
# Tick box under the Search page's box: look on Flathub, for Flatpaks.
store-gh-place-flathub = Flathub (Flatpak)
# Tick box: look on AppImageHub, for AppImages.
store-gh-place-hub = AppImageHub (AppImage)
# Tick box: look on GitHub, which needs Enter.
store-gh-place-github = GitHub
# Heading of the card of matches from AppImageHub.
store-gh-hub = AppImageHub ({ $n })
# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = 이 컴퓨터가 인터넷에 연결되어 있지 않아서 Trindle이 GitHub을 검색하지 못했어요.
# Note on the GitHub page while GitHub is being asked.
store-gh-asking = GitHub 확인 중…
# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = GitHub에 더 이상 없어요.
# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
       *[other] 프로젝트 { $n }개에서 AppImage와 Flatpak 파일을 찾는 중…
    }
# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = GitHub의 검색 결과 중 이 컴퓨터용 AppImage나 Flatpak 파일이 있는 것이 없어요.
# Heading of the card of matches from GitHub.
store-gh-on = GitHub
# Heading of the card that holds the search box.
store-gh-search-card = GitHub 검색
# Line under the title of the GitHub page.
store-gh-line = AppImage나 Flatpak 파일로 된 GitHub의 앱이에요. 아무도 검사하지 않으니 믿을 수 있는 사람의 것만 설치하세요.
# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = 이 컴퓨터가 인터넷에 연결되어 있지 않으면 GitHub을 검색할 수 없어요.

## A project on GitHub and the GitHub key (src/store/pages/)

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak과 AppImage
# Tip of the round button that shows or hides a project's files.
store-gh-see-files = 파일 보기
# Button that installs a GitHub project or file.
store-gh-install = 설치
# Link to a project's page on GitHub.
store-gh-open = GitHub에서 열기
# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, 릴리스 { $tag }
# Name of the GitHub key's button and of its box.
store-gh-key = GitHub 키
# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = GitHub 키가 저장되어 있어요
# In the GitHub key's box: what the key is for.
store-gh-key-words = 대부분은 필요하지 않아요. 키가 없으면 GitHub은 이 컴퓨터에서 한 시간에 60번까지 요청을 받아요. 키가 있으면 5,000번까지예요. 키는 이 한도를 높이는 일만 해요. 공개 파일을 보는 데만 쓰고 다른 데는 쓰지 않아요. Trindle이 내 계정에 들어갈 수는 없고, 키를 만들 때 권한을 선택하지 않아요. 키는 이 컴퓨터의 키링에 저장되고 GitHub에만 보내져요.
# In the GitHub key's box: how to make a key.
store-gh-key-how = GitHub에서 키를 만들고, 이름은 아무거나 정하고, 권한은 그대로 두세요. 토큰 생성을 누른 다음 나온 것을 여기에 붙여 넣으세요.
# Red words under the GitHub key's box when the key could not be removed.
store-gh-key-not-removed = 삭제하지 못했어요: { $why }
# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = 저장하지 못했어요: { $why }
# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = 비워 두면 그대로 유지돼요
# Placeholder of the GitHub key's box.
store-gh-key-paste = 키 붙여 넣기
# Tip of the round button that removes the stored GitHub key.
store-gh-key-remove = 키 삭제
# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = GitHub에서 키 만들기
# Tip of the round button that keeps the GitHub key typed.
store-gh-key-keep = 키 저장
# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = 최신 릴리스에는 이 컴퓨터에 설치할 수 있는 것이 없어요.

## The Move your apps page (src/store/pages/)

# Word on the file's dial and under a GitHub file when it was added.
store-added = 추가됨
# Line under the title of the Move your apps page.
store-bk-line = 내 앱과 설정을 어디든 가져갈 수 있는 파일 하나로 담아요.
# Heading of the card that makes a backup.
store-bk-card = 백업
# Tip of the round button while the backup is being made.
store-bk-making-tip = 백업을 만드는 중
# Main button that makes the backup.
store-bk-make = 백업 만들기
# Tip of the round button that shows the backup in Folders.
store-bk-show = Folders에서 보기
# Name of the switch for the list of apps in a backup.
store-bk-apps = 앱
# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
       *[other] 내 Flatpak 앱 { $n }개의 목록이에요.
    }
# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list }
# Name of the switch for the apps' own data in a backup.
store-bk-data = 앱 자체 데이터
# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage
# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle 설정
# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } 지금 { $size }예요.
# Name of the row that says where the backup goes.
store-bk-where = 저장 위치
# Under that row: the folder and the file's name.
store-bk-where-sub = 내 홈 폴더, 파일 이름은 { $name }
# Heading of the card that puts a backup back.
store-bk-back-card = 백업 복원
# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = 백업 파일 고르기
# Note in the card that puts a backup back.
store-bk-back-note = 복원하면 백업에 있는 앱을 설치하고 데이터와 설정을 되돌려요. 문서와 사진은 백업에 들어 있지 않으니 직접 복사하세요.
# Said while a backup is being made.
store-bk-making = 백업하는 중이에요. 몇 분 걸릴 수 있어요.
# Said when a backup is made: the file and its size.
store-bk-made = 백업은 { $path } ({ $size })예요. USB 메모리나 클라우드 저장소에 복사해서 가져가세요.
# Red words when no connection to the file chooser exists.
store-bk-no-bus = Trindle이 파일 선택 창에 연결할 수 없어서 백업을 고르지 못했어요.
# Title of the window that asks which backup to use.
store-bk-choose-title = 백업 고르기
# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle 및 Flat 백업
# Red words when the file chooser gave no answer.
store-bk-no-answer = 파일 선택 창이 응답하지 않아서 백업을 고르지 못했어요.
# Said while a backup file is read.
store-bk-reading = 백업을 읽는 중…
# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } 모든 설정을 보려면 로그아웃했다가 다시 로그인하세요.
# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
       *[other] 앱 { $apps }개를 찾았고, 그중 { $with }개에 설정이 있어요. 시작하는 중…
    }

## The Software sources page (src/store/pages/)

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] 추가한 소프트웨어 소스가 아직 없어요.
       *[other] 소프트웨어 소스 { $n }개를 추가했고, Debian 업데이트와 함께 최신으로 유지돼요.
    }
# Under a maker's source that is added and installed.
store-src-added = { $what }이(가) 추가되어 최신으로 유지돼요.
# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. 소스는 추가했지만 프로그램은 설치되지 않았어요.
# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. 다른 방법으로 설치되었어요.
# Tip of the round button that removes a maker's source and its program.
store-src-remove-both = 프로그램과 소스 삭제
# Link to a maker's page, naming the program.
store-src-open = { $name } 열기
# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Tailscale 로그인
# Tip of the round button that adds a maker's source, naming the program.
store-src-add = { $name } 추가
# Heading of the card of makers' sources that can be added.
store-src-ready = 추가할 수 있어요
# Name of the field for the address of a source.
store-src-address = 주소
# Name of the field for the suite of a source.
store-src-suite = Suite
# Name of the field for the components of a source.
store-src-components = Components
# Name of the field for a source's signing key.
store-src-key = 서명 키
# Name of the field for the program to install from a source.
store-src-program = 설치할 프로그램
# Placeholder of the program field.
store-src-program-hint = 선택 사항
# Tip of the round button that adds the source typed.
store-src-add-this = 이 소스 추가
# Heading of the card that adds another source.
store-src-another = 다른 소스 추가
# Note in that card.
store-src-another-note = 소스는 컴퓨터 전체의 업데이트를 설치할 수 있으니 믿을 수 있는 제작자의 것만 추가하세요. 추가할 때마다 관리자 비밀번호를 물어요.
# Tip of the round button that removes a source that is already on the computer.
store-src-remove-this = 이 소스 삭제
# Heading of the card of sources already on the computer.
store-src-already = 이 컴퓨터에 이미 있는 소스
# Red words when the addresses of a new source are not secure.
store-src-bad-address = 두 주소 모두 https://로 시작해야 해요
# Red words when the suite or components are not plain words.
store-src-bad-words = Suite와 Components는 일반 단어여야 해요.
# Red words when the program is not a package name.
store-src-bad-package = 패키지 이름이 아니에요.
# Word on a source's dial while its job starts.
store-src-starting = 시작 중
# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Tailscale 로그인 페이지를 여는 중…
# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = 열린 페이지에서 로그인하세요. 로그인하면 Trindle이 알아서 이어서 해요.
# Said under Tailscale's row when it is connected.
store-src-connected = 연결됨

# Said under the backup row when its apps come from an http address on the local network: the link is not encrypted, and the restore goes on.
store-bk-http-line = 앱으로 가는 연결이 암호화되어 있지 않아요.
# Title of the card shown before a restore from an http address over the internet.
store-bk-http-title = 암호화되지 않음
# Words of that card.
store-bk-http-card = 이 연결은 암호화되어 있지 않아요. 누군가 중간에 오가는 내용을 볼 수 있고, 백업을 다른 것으로 바꿔치기할 수도 있어요.
# Label of the tick box on that card; the restore button works only once it is on.
store-bk-http-tick = 위험을 이해하고 받아들여요
# Button on that card that restores the backup.
store-bk-http-go = 복원
# Button on that card that gives the restore up.
store-bk-http-cancel = 취소
# Said before every Add of an AppImage from AppImageHub, in plain ink: what is and is not checked, and how Trindle boxes it.
store-hub-warning = AppImageHub는 프로젝트 목록만 보여 줘요. 아무도 검토하거나 검사하지 않고, 서명할 필요도 없어요. Trindle은 AppImage마다 전용 홈 폴더가 있는 상자 안에서 실행해요. 처음에는 인터넷과 소리를 쓸 수 있지만 내 파일은 볼 수 없어요.
# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = AppImage가 밝힌 출처는 참고용일 뿐이에요. Trindle은 이를 확인하지 않아요.
# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = GitHub: { $repo }
# The same, with how many stars the project has.
store-from-github-stars = GitHub: { $repo }, 별 { $stars }개
# Where an AppImage comes from: a website, by its name.
store-from-site = GitHub가 아니에요. 출처는 { $site }
# Where an AppImage comes from, for one added by hand.
store-from-unknown = 출처를 알 수 없어요
# Round button that opens the website an AppImage is fetched from.
store-from-site-open = 웹사이트 열기
# When an app was last updated.
store-age-updated = 마지막 업데이트: { $date }
# The same, said in red, when that is more than six months ago.
store-age-updated-old = 마지막 업데이트: { $date }. 6개월 넘게 업데이트되지 않아서 고쳐지지 않은 보안 문제가 있을 수 있어요.
# When the source does not say when an app was last updated.
store-age-updated-unknown = 마지막 업데이트: 알 수 없음
# When an app's project was made.
store-age-created = 만든 날짜: { $date }
# When the source does not say when an app's project was made.
store-age-created-unknown = 만든 날짜: 알 수 없음
# A date the source does not give, on a line of Facts.
store-age-not-known = 알 수 없음
# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = 추가하기 전에
# Title of the card that lists the AppImages.
store-browse-list = AppImage
# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
       *[other] AppImageHub에 AppImage { $count }개가 올라와 있어요.
    }
# The same while AppImageHub's list is being fetched.
store-browse-fetching = AppImageHub 목록을 가져오는 중…
# The same when the list could not be fetched.
store-browse-failed = Trindle이 AppImageHub 목록을 가져오지 못했어요.
# The same with no internet and no copy kept.
store-browse-offline = 이 컴퓨터가 인터넷에 연결되어 있지 않으면 AppImageHub 목록을 가져올 수 없어요.
# In the list when the kind chosen has none.
store-browse-none = AppImageHub에 해당하는 것이 없어요.
# Under the rows: how many are shown of how many.
store-browse-showing = { $n }개 중 { $shown }개를 보여 줘요.
# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Trindle 앱에 추가
# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = 자세히와 파일 보기
# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = 둘 다
# The same: only Flatpaks.
store-show-flatpaks = Flatpak
# The same: only AppImages.
store-show-appimages = AppImage
# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }
# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage
# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage는 검사하지 않아요. 아무도 검토하거나 검사하지 않아요.
# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle은 AppImage마다 전용 홈 폴더가 있는 상자 안에서 실행해요. 처음에는 인터넷과 소리를 쓸 수 있지만 내 파일은 볼 수 없어요.
# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = 다시 보지 않기
# Button on that card that goes on with the install.
store-appimage-warn-ok = 확인
# Button on that card that gives the install up.
store-appimage-warn-cancel = 취소

# Printed by the command line after a backup. { $size } is how big the file is.
store-cli-backed-up = 백업 완료 ({ $size })

# Printed by the command line after Trindle's own settings were put back.
store-cli-settings-back = Trindle 설정을 되돌렸어요
