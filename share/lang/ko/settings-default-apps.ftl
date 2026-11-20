# machine translated (Korean), not yet reviewed by a person.

# Settings page "Default apps": which program opens each kind of file or link.

## Page (src/settings/panels/default_apps.rs)

# Name of the Settings page about default apps, in the list and as its heading.
settings-default-apps-title = 기본 앱
# Words a person might type into the Settings search to find the default apps page; separate them with spaces.
settings-default-apps-keywords = browser mail email files pictures music video pdf calendar open with terminal 브라우저 메일 이메일 파일 사진 음악 비디오 달력 열기 터미널
# Introduction under the heading of the default apps page, and the words behind the page i.
settings-default-apps-help = 웹 링크, 이메일, 사진, 달력 등을 열 프로그램을 고르세요. 터미널은 ~/.config/xdg-terminals.list 에서 고릅니다. 고르지 않으면 Trindle은 설치된 첫 번째 터미널을 써요.

## Kinds of file

# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-web = 웹
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-mail = 메일
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-folders = 폴더
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-text = 텍스트
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pictures = 사진
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-music = 음악
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-video = 비디오
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-pdf = PDF 문서
# Name of a kind of thing to open, as the heading of a line and the name of its list on the default apps page.
settings-default-apps-kind-calendar = 달력
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-web-in-sentence = 웹
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-mail-in-sentence = 메일
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-folders-in-sentence = 폴더
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-text-in-sentence = 텍스트
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pictures-in-sentence = 사진
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-music-in-sentence = 음악
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-video-in-sentence = 비디오
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-pdf-in-sentence = PDF 문서
# The same kind of thing in the middle of a sentence ("Nothing is chosen yet to open ..."), so in lower case where the language does that.
settings-default-apps-kind-calendar-in-sentence = 달력

## Cards

# Title of a card on the default apps page.
settings-default-apps-card-internet = 인터넷
# Words behind the i of a card on the default apps page.
settings-default-apps-card-internet-about = 웹 링크, 이메일 주소, 달력 초대를 열 프로그램을 고르세요.
# Title of a card on the default apps page.
settings-default-apps-card-files = 파일과 문서
# Words behind the i of a card on the default apps page.
settings-default-apps-card-files-about = 폴더, 일반 텍스트 파일, PDF 문서를 열 프로그램을 고르세요.
# Title of a card on the default apps page.
settings-default-apps-card-media = 사진과 소리
# Words behind the i of a card on the default apps page.
settings-default-apps-card-media-about = 사진, 음악, 비디오를 열 프로그램을 고르세요.

## Page

# Sentence at the top of the page when every kind has a program; { $web } is the program that opens web links.
settings-default-apps-all-set-web = 모든 항목에 여는 프로그램이 있고, { $web }이(가) 웹 링크를 열어요.
# Sentence at the top of the page when every kind has a program and the one for web links is not known.
settings-default-apps-all-set = 모든 항목에 여는 프로그램이 있어요.
# Sentence at the top of the page when one kind has no program chosen; { $kind } is its name in lower case.
settings-default-apps-one-missing = { $kind }을(를) 열 프로그램을 고르세요.
# Sentence at the top of the page when several kinds have no program chosen; { $kinds } is their names as a list joined with "or".
settings-default-apps-many-missing = { $kinds }을(를) 열 프로그램을 아직 고르지 않았어요.
# Small words under a kind that no installed program can open.
settings-default-apps-none-installed = 이것을 열 수 있는 설치된 프로그램이 아직 없어요.
# First choice in a list when no program is chosen for that kind yet.
settings-default-apps-not-chosen = 아직 고르지 않음
# Said in green under a kind's list after the choice was kept.
settings-default-apps-saved = 저장됨
