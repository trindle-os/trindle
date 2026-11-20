# machine translated (Korean), not yet reviewed by a person.

# Apps: the page for one app (src/store/app_page.rs).

## The page of one app (src/store/app_page.rs)

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }.
# Under the app's name when only the maker is known.
store-app-line-by = 만든 곳: { $by }.
# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, 만든 곳: { $by }
# The state beside the dot while the app is being taken off.
store-app-state-removing = 삭제하는 중
# The state beside the dot while the app is being fetched.
store-app-state-downloading = 내려받는 중
# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = 설치하는 중
# The state beside the dot while the app's own update goes in.
store-app-state-updating = 업데이트하는 중
# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = 설치됨, 업데이트 보류 중
# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = 설치됨, 버전 { $version }에서 업데이트 보류 중
# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = 설치됨, { $waits }
# The same, with the installed version.
store-app-state-waiting-version = 설치됨, 버전 { $version }, { $waits }
# The state beside the dot for an installed app that is up to date.
store-app-state-current = 설치됨, 최신 상태
# The same, with the installed version.
store-app-state-current-version = 설치됨, 버전 { $version }, 최신 상태
# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = 설치 안 됨
# The same, with the size of the download.
store-app-state-absent-size = 설치 안 됨, 내려받기 { $size }
# Word beside the spinner while the app is being taken off.
store-app-busy-removing = 삭제하는 중
# Word beside the spinner while the app is being put in.
store-app-busy-installing = 설치하는 중
# Tip and spoken name of the round button that gives up a download.
store-app-stop = 중지
# Tip and spoken name of the round button that takes the app off.
store-app-remove = 삭제
# Round button beside Update or Release that opens the app.
store-app-open-round = 열기
# Main button that releases an app whose updates are held back.
store-app-release = 업데이트 허용
# Main button that updates the app.
store-app-update = 업데이트
# Main button that opens the app.
store-app-open = 열기
# Main button that installs the app.
store-app-install = 설치
# Button that opens the app's website.
store-app-website = 웹사이트
# In the picture frame when the app has no pictures.
store-app-no-pictures = 이 앱에는 사진이 없어요.
# In the picture frame when the picture could not be fetched.
store-app-picture-missed = Trindle이 이 사진을 가져오지 못했어요.
# In the picture frame while the picture is being fetched.
store-app-picture-fetching = 사진을 가져오는 중…
# In the picture frame when the computer has no internet.
store-app-picture-offline = 사진을 보려면 인터넷에 연결하세요.
# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = 사진 { $at } / { $n }
# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = 사진 { $at } / { $n }: { $caption }
# Tip of the round button that shows the picture before.
store-app-picture-earlier = 이전 사진
# Tip of the round button that shows the picture after.
store-app-picture-later = 다음 사진
# Licence shown in Facts for an app that is not open source.
store-app-proprietary = 독점 소프트웨어
# Name of the Facts card.
store-app-facts = 정보
# Facts line: the size of the download.
store-app-fact-download = 내려받기 크기
# Facts line: where the app comes from.
store-app-fact-source = 출처
# Facts line: the version.
store-app-fact-version = 버전
# Facts line: the licence of an open source app.
store-app-fact-licence = 라이선스
# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = 라이선스, 오픈 소스 아님
# Facts line: who made the app.
store-app-fact-made-by = 만든 곳
# Facts line: when the app's project was made.
store-app-fact-created = 만든 날짜
# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  { $n ->
       *[other] 평가 { $count }개
    }
# Under the stars when nobody has rated the app.
store-app-rating-none = 아직 평가가 없어요.
# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = 보여 줄 평가가 없어요.
# Under the stars while the ratings are being fetched.
store-app-rating-looking = 평가를 찾는 중…
# Last line of the Rating card: where the ratings come from.
store-app-rating-source = GNOME의 오픈 리뷰 서비스에서 가져왔어요.
# Last line of the Rating card when there is no internet.
store-app-rating-offline = 이 컴퓨터는 인터넷에 연결되어 있지 않아요.
# Name of the Rating card.
store-app-rating = 평가
# Text of the About card when the app says nothing of itself.
store-app-no-description = 이 앱에는 설명이 없어요.
# Name of the About card.
store-app-about = 소개
# Heading of a review that has no title.
store-app-review-no-title = 제목 없음
# Under What people say when nobody has written.
store-app-people-none = 아직 이 앱에 대한 글이 없어요.
# Under What people say when there is no internet.
store-app-people-offline = 리뷰를 읽으려면 인터넷에 연결하세요.
# Under What people say while reviews are being fetched.
store-app-people-looking = 리뷰를 불러오는 중…
# Name of the card of reviews.
store-app-people = 리뷰
