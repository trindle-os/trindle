# machine translated (Korean), not yet reviewed by a person.

# Apps: the window, its side bar and the front pages (src/store/mod.rs, catalogue.rs, dropped.rs).

## The window and its side bar (src/store/mod.rs)

# Title of the Apps window, and the program's name.
store-title = 앱
# Typing box at the top of Apps, shown quietly while empty.
store-search-placeholder = 앱 검색
# Side bar place: the front page of apps to try.
store-nav-discover = 둘러보기
# Side bar place: the apps that are installed.
store-nav-installed = 설치됨
# Side bar place: updates, when none are waiting.
store-nav-updates = 업데이트
# Side bar place: updates, with how many are waiting.
store-nav-updates-waiting = 업데이트 ({ $n })
# Side bar place: plugins.
store-nav-plugins = 플러그인
# Side bar place and title: the AppImages AppImageHub lists.
store-nav-browse = AppImage 찾아보기
# Side bar place and title: the one detailed search over Flathub, AppImages and GitHub.
store-nav-github = 검색
# Heading of the Flatpaks a detailed search found on Flathub.
store-gh-flathub = Flathub의 Flatpak ({ $n })
# Side bar place: where software comes from.
store-nav-sources = 소프트웨어 출처
# Side bar place, title of the page and its way back: moving apps and settings to another computer.
store-move = 앱 옮기기
# Small heading over the side bar's first group.
store-group-apps = 앱
# Small heading over the pages from Settings that are about apps.
store-group-settings = 앱 설정
# Small heading over the categories of apps.
store-group-categories = 분류
# Title of a page of search results; the words searched for are in the quotes.
store-grid-results = “{ $text }” 검색 결과
# Title of the front page of apps, and of a page before the catalogue has come.
store-grid-discover = 둘러보기
# Heading of the apps most people want, on the front page.
store-grid-popular = 인기
# Title of the page of apps from a source, before the catalogue has come.
store-grid-apps = 앱
# Red words under the search box when an app asked for is not in the catalogue.
store-not-in-catalogue = { $id }은(는) Flathub 목록에 없어요.

## Categories of apps (src/store/catalogue.rs)

# Side bar category: music, film and sound.
store-cat-audio-video = 소리와 영상
# Side bar category: drawing and photographs.
store-cat-graphics = 그림과 사진
# Side bar category: documents and spreadsheets.
store-cat-office = 사무
# Side bar category: browsers and chat.
store-cat-network = 인터넷
# Side bar category: programming tools.
store-cat-development = 개발
# Side bar category: teaching and learning.
store-cat-education = 배우기
# Side bar category: games.
store-cat-game = 게임
# Side bar category: science.
store-cat-science = 과학
# Side bar category: small everyday tools.
store-cat-utility = 유틸리티
# Side bar category: tools for the computer itself.
store-cat-system = 시스템
# Red words when the list of apps could not be read.
store-catalogue-unreadable = Trindle이 앱 목록을 읽지 못했어요: { $why }
# Red words when Flathub's list of apps could not be fetched. { $why } is what Flatpak said.
store-catalogue-not-fetched = Trindle이 Flathub에서 앱 목록을 가져오지 못했어요. 인터넷 연결을 확인하세요. { $why }

## Dropping a file on the window (src/store/dropped.rs)

# Red words under the search box when nothing that was dropped on the window is a kind Apps takes.
store-dropped-only = 여기에는 .deb, .AppImage, .flatpakref 파일만 놓을 수 있어요.

## Why an app was not updated (src/store/mod.rs)

# Red words under an app that Flatpak did not update; what Flatpak said follows.
store-not-updated-said = Flatpak이 업데이트하지 않았어요: { $why }
# Red words under an app that Flatpak did not update and gave no reason for.
store-not-updated-none = 업데이트되지 않았어요. 이유는 알려지지 않았어요.
