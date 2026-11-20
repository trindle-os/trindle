# machine translated (Korean), not yet reviewed by a person.

# Trindle's own words, in English: the source file for every language. The
# toolkit's words (keys starting kit-) are in trindle-toolkit/share/lang/en.ftl.
# See README.md in the toolkit's share/lang folder for how to translate.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## Report a problem (the window opened from the ring, and after a crash)

# Window title when the person opens it themselves from the ring.
report-title-note = 문제 보고
# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = 문제가 생겼어요. 보고서를 보낼까요?
# Under the title: what a report holds, and what it never holds.
report-summary = 보고서에는 내가 쓴 글, 컴퓨터 정보, 이번 부팅의 오류, Trindle의 로그가 들어가요. Trindle이 모은 내용에서 이름과 주소(이메일, 네트워크, Wi-Fi)는 지워요. 로그에는 파일 이름과 웹 주소가 남을 수 있어요. 내 파일과 비밀번호는 보내지 않아요.
# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = 무슨 일이 있었나요?
# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = 어떤 앱인가요?
# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = 모르겠어요
# Dropdown choice: the app is not one of Trindle's own.
report-app-other = 다른 프로그램
# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = 데스크톱
# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = 보내기
# Button at the bottom of the window when the person opened it themselves.
report-cancel = 취소
# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = 보내지 않기
# Button that opens the exact text of the report.
report-show-details = 자세히 보기
# Button that closes the exact text of the report.
report-hide-details = 자세히 숨기기
# Button under the shown report text that copies it.
report-copy = 복사
# Button under the shown report text that saves it as a file in Downloads.
report-save = 저장
# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = 아직 보고서를 보낼 수 없어요.
# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = 확인하는 중.
# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle은 베타 버전이에요. 보고서는 익명이에요. 이름, 이메일 주소, 홈 폴더는 지워요. 사용자 이름은 선택이에요.
# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Buildmorphic 사용자 이름 (선택)
# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle이 보고서를 보내지 못했어요. { $why } 보내기를 눌러 다시 시도하세요.
# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = 한꺼번에 들어온 보고서가 너무 많아요.
# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = 연결을 기다리는 중.
# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = 연결을 기다리는 중. { $why }
# Under the typing box after the report was sent.
report-sent = 보냈어요. 감사합니다.
report-sent-as-bug = 보냈어요. 버그 보고서 번호는 { $number }번이에요. 감사합니다.
# Under the typing box while the report is being posted.
report-sending = 보내는 중.
# Under the typing box after Save. $name is the file's name.
report-saved-as = Downloads/{ $name }(으)로 저장했어요.
# Red words when the Buildmorphic username holds an email address.
report-no-at = 이메일 주소가 아니라 Buildmorphic 사용자 이름을 입력하세요. @는 쓸 수 없어요.
# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = 사용자 이름은 2자에서 31자까지이고, 소문자, 숫자, 하이픈만 쓸 수 있어요.
# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = GitHub 이름에는 문자, 숫자, 하이픈만 쓸 수 있어요.
# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle이 보고서를 나중을 위해 저장하지 못했어요: { $reason }.
# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = 저장할 수 없었어요: { $reason }.
# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = 권한이 없어요
# The reason inside "Could not save it: ...": there is no room.
report-reason-full = 드라이브가 가득 찼어요
# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = 폴더가 없어요
# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = 드라이브 오류
# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl이 없어요.
# Why a report could not be posted: the post could not be started.
report-not-posted = 보낼 수 없었어요.
# Why a report could not be posted: the computer is offline.
report-no-connection = 연결이 없어요.
# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = 서버가 { $code }을(를) 보냈어요.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = 무슨 일이 있었는지 조금 더 써 주세요.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = 로그를 읽는 중.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = 보고서와 함께 보낼 로그가 없어서 보낼 수 없어요. 대신 저장하세요.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = 사진은 선택이에요. 프로젝트 소유자만 볼 수 있어요.
# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = PNG 사진 고르기
# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = 스크린샷 찍기
# Tooltip of the round button that takes the picture off the report.
report-picture-remove = 사진 제거
# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = 내 사진, { $width } x { $height }
# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = PNG 사진 고르기
report-picture-filter = PNG 사진
# Under the picture while it is being read.
report-picture-reading = 읽는 중.
# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = 스크린샷을 찍을 수 없었어요.
report-picture-no-chooser = Trindle이 파일 선택 창을 열 수 없었어요.
# Red words under the picture, each the reason it is refused.
report-picture-not-png = PNG 사진이 아니에요.
report-picture-too-big = { $megabytes } MB를 넘어요.
report-picture-too-large = 사진이 너무 커요. 가로 { $side } 픽셀, 세로 { $side } 픽셀까지 가능해요.
report-picture-damaged = 사진이 손상되었어요.
report-picture-unreadable = 읽을 수 없었어요.
