# machine translated (Korean), not yet reviewed by a person.

## Screenshots, the eye dropper and Trindle's own portals (src/screenshot): the picture window, what the desktop asks through the portals, and the file chooser

## The screenshot window (src/screenshot/mod.rs, toolbar.rs, marks.rs, pin.rs)

# Name of the screenshot overlay that covers the screen (what a screen reader calls it).
screenshot-name = 스크린샷
# Note on the overlay before any part is chosen.
screenshot-drag = 담을 부분을 드래그하세요
# Name of the round button that keeps the whole screen.
screenshot-whole = 전체 화면
# Name of the round button that closes the overlay or a pinned picture.
screenshot-close = 닫기
# Word on the button, shown when the report window asked for the picture, that hands the chosen part to the report.
screenshot-add-to-report = 보고서에 추가
# Name of the round button that gives the screenshot up.
screenshot-cancel = 취소
# Name of the round button that takes back the last mark.
screenshot-undo = 실행 취소
# Name of the round button that puts back the last mark taken away.
screenshot-redo = 다시 실행
# Name of the round button that copies the picture to the clipboard, and of the copy button of a colour.
screenshot-copy = 복사
# Name of a colour's button after it has been copied.
screenshot-copied = 복사함
# Word on the button that saves the picture in the Screenshots folder.
screenshot-save = 저장
# Word on the button that asks where to save the picture.
screenshot-save-as = 다른 이름으로 저장
# Name of the round button that keeps the picture on the screen as a small window.
screenshot-pin = 화면에 고정
# Name of the drawing tool that draws freehand.
screenshot-tool-pen = 펜
# Name of the drawing tool that draws a straight line.
screenshot-tool-line = 선
# Name of the drawing tool that draws an arrow.
screenshot-tool-arrow = 화살표
# Name of the drawing tool that draws a box.
screenshot-tool-box = 상자
# Name of the drawing tool that draws a circle.
screenshot-tool-circle = 원
# Name of the drawing tool that highlights.
screenshot-tool-highlighter = 형광펜
# Name of the drawing tool that writes words.
screenshot-tool-words = 글자
# Name of the drawing tool that puts numbered spots.
screenshot-tool-numbers = 번호
# Name of the drawing tool that blurs a patch.
screenshot-tool-blur = 흐리게
# Name of the round button that draws in red.
screenshot-colour-red = 빨강
# Name of the round button that draws in blue.
screenshot-colour-blue = 파랑
# Name of the round button that draws in green.
screenshot-colour-green = 초록
# Name of the round button that draws in orange.
screenshot-colour-orange = 주황
# Name of the round button that draws in purple.
screenshot-colour-purple = 보라
# Name of the round button that draws in white.
screenshot-colour-white = 하양
# Name of the round button that draws in black.
screenshot-colour-black = 검정
# Name of the round button that draws thin lines.
screenshot-thickness-thin = 얇게
# Name of the round button that draws lines of middle thickness.
screenshot-thickness-medium = 보통
# Name of the round button that draws thick lines.
screenshot-thickness-thick = 굵게
# Name the screen reader gives to a screenshot pinned on the screen.
screenshot-pinned = 고정된 스크린샷
# Title of the window that asks where to save the screenshot.
screenshot-save-title = 스크린샷 저장
# Title of the notice that says the screenshot could not be saved.
screenshot-not-saved = 스크린샷을 저장하지 못했어요
# Title of the notice that says the screenshot could not be copied.
screenshot-not-copied = 스크린샷을 복사하지 못했어요
# Words of that notice.
screenshot-clipboard-refused = Trindle이 복사할 수 없었어요.
# Words of the notice that the screenshot could not be saved: the picture was not made.
screenshot-no-picture = 스크린샷을 찍지 못했어요.
# Words of the notice that the screenshot could not be saved: no bus to ask on.
screenshot-no-bus = Trindle이 저장 위치를 물어볼 수 없어요.
# Words of the notice that the screenshot could not be saved: the file chooser could not be asked.
screenshot-no-chooser = 파일 선택 창을 열 수 없었어요.
# Words of the notice that the screenshot could not be saved: every file name was taken.
screenshot-no-free-name = 쓸 수 있는 이름이 없어요
# Words of the notice that the screenshot could not be saved: the path and why it failed.
screenshot-write-failed = { $path }: { $why }

## The eye dropper (src/screenshot/colours.rs)

# Name of the eye dropper's window, and its title.
screenshot-eye-title = 스포이트
# Words that float beside the i of the eye dropper's window.
screenshot-eye-help = 스포이트를 누른 다음 화면의 아무 곳이나 클릭하면 포인터 아래의 색을 가져와요. 취소하려면 Esc 키를 누르세요. 가져온 색은 바로 16진수 코드로 복사되고 아래 목록에 최신순으로 추가되어, 나중에 다시 복사할 수 있어요.
# Name of the i button beside the title.
screenshot-eye-about = 스포이트 정보
# Name of the main button that takes a colour.
screenshot-eye-take = 색 고르기
# Sentence under the title while no colour has been taken.
screenshot-eye-none = 화면에서 색을 고르세요.
# Sentence under the title once colours have been taken.
screenshot-eye-taken =
    { $n ->
       *[other] 색 { $n }개를 골랐어요. 가장 최근 색은 { $newest }이에요.
    }
# Shown in the big card while no colour has been taken.
screenshot-eye-latest-empty = 아직 고른 색이 없어요.
# Name of the card that holds the latest colour.
screenshot-eye-latest = 최근 색
# Name of the round button that clears the list of colours.
screenshot-eye-clear = 목록 지우기
# Head of the card with the list of colours taken.
screenshot-eye-list = 내 색
# Shown in the list card while no colour has been taken.
screenshot-eye-list-empty = 아직 고른 색이 없어요.
# Name of the round button that takes a colour out of the list.
screenshot-eye-remove = 제거
# What the saved list of colours is called when it is found damaged and set aside.
screenshot-eye-damaged = 스포이트 색

## The picker for sharing the screen (src/screenshot/cast_pick.rs)

# Title of the window that asks what an app may share.
screenshot-pick-title = { $app }와(과) 화면을 공유할까요?
# Sentence under the title when several things can be picked.
screenshot-pick-many = 하나 이상 고르세요. 앱이 공유를 멈추거나 닫힐 때까지 고른 것만 공유돼요.
# Sentence under the title when one thing can be picked.
screenshot-pick-one = 하나만 고르세요. 앱이 공유를 멈추거나 닫힐 때까지 고른 것만 공유돼요.
# Heading over the screens that can be shared.
screenshot-pick-screens = 화면
# Heading over the windows that can be shared.
screenshot-pick-windows = 창
# Shown when there is nothing to share.
screenshot-pick-nothing = 공유할 것이 없어요.
# Name of the choice that shares every screen at once.
screenshot-pick-desktop = 데스크톱 전체
# Words under that choice.
screenshot-pick-desktop-detail = 배치된 그대로 모든 화면
# Words under a screen's choice: its size in pixels.
screenshot-pick-size = { $width } x { $height }
# What the screen reader says for a choice that is picked.
screenshot-pick-chosen = { $name }, 선택함
# Name of the round button that refuses to share.
screenshot-pick-refuse = 공유 안 함
# Name of the main button that shares what is picked.
screenshot-pick-share = 공유

## The portals (src/screenshot/portal_*.rs)

# Name of the button that says no in a question from a program, and of its answer to the program.
screenshot-portal-deny = 거부
# Name of the main button that says yes in a question from a program.
screenshot-portal-allow = 허용
# Name of the round button that gives up a window from a program.
screenshot-portal-cancel = 취소
# Name of the main button that opens, and the title of a window that opens a file.
screenshot-portal-open = 열기
# Title of the window that saves a file, and the word of its main button.
screenshot-portal-save = 저장
# Name of the box where a name is typed.
screenshot-portal-name = 이름
# Heading when an unknown program asks who the person is.
screenshot-account-asks-unknown = 어떤 프로그램이 내 정보를 요청해요
# Heading when a program asks who the person is.
screenshot-account-asks = { $who }이(가) 내 정보를 요청해요
# Name the screen reader gives to the person's picture.
screenshot-account-picture = 내 사진
# Words under the person's name.
screenshot-account-signed-in = { $id }(으)로 로그인함
# Name of the search box in the window that asks which app should open a file.
screenshot-apps-search = 앱 검색
# Shown when no app can open the file.
screenshot-apps-none = 열 수 있는 앱이 없어요
# Name of the switch that remembers the choice.
screenshot-apps-always = 이 종류의 파일은 항상 이 앱으로 열기
# Title of the window that asks which app should open a file.
screenshot-apps-open-with = { $name } 열기에 사용할 앱
# Title of that window when the file's name is not known.
screenshot-apps-choose = 앱 고르기
# Name of the screen sharing that other programs see, and the name Trindle gives itself to PipeWire.
screenshot-cast-name = Trindle 화면 공유
# What other programs show for the shared screen.
screenshot-cast-node = 앱과 공유 중인 화면
# Name used for a program that cannot be named, in the window that asks what to share.
screenshot-cast-an-app = 앱
# Title of the window that asks what to share.
screenshot-cast-window = 화면 공유
# Title of the window that saves several files into a folder.
screenshot-files-save-many = 폴더에 저장
# Title of the window that chooses a folder.
screenshot-files-folder = 폴더 고르기
# What a program holding the computer awake is called when it gave no name.
screenshot-inhibit-who = 프로그램
# Heading when a program wants to add an app to the person's apps and cannot be named.
screenshot-launcher-ask = 내 앱에 추가할까요?
# Heading when a named program wants to add an app to the person's apps.
screenshot-launcher-asks = { $who }이(가) 내 앱에 추가하려고 해요
# Name the screen reader gives to the app's icon.
screenshot-launcher-icon = 앱 아이콘
# Words under the name, when the app opens a web address.
screenshot-launcher-opens = { $target } 열기
# Title of the window that asks to add an app.
screenshot-launcher-title = 앱 추가
# Name of the main button that adds the app.
screenshot-launcher-add = 추가
# Heading when a program that cannot be named offers a wallpaper.
screenshot-wallpaper-ask = 이 사진을 배경화면으로 설정할까요?
# Heading when a program offers a wallpaper.
screenshot-wallpaper-asks = { $who }이(가) 배경화면을 설정하려고 해요
# Name the screen reader gives to the picture offered.
screenshot-wallpaper-picture = 새 배경화면
# Name of the main button that sets the wallpaper.
screenshot-wallpaper-set = 배경화면으로 설정

## Printing (src/screenshot/portal_print.rs)

# Red line under the printer box when no printer is set up.
screenshot-print-no-printer = 설정된 프린터가 없어요. 설정의 프린터에서 추가하세요.
# Red line under the printer box when the printing system does not answer.
screenshot-print-not-answering = 인쇄 기능이 응답하지 않아요.
# Red line under the printer box when printing is not installed.
screenshot-print-not-installed = 이 컴퓨터에는 인쇄 기능이 설치되어 있지 않아요.
# Red line under the printer box when the chosen printer is paused.
screenshot-print-paused = { $name }이(가) 일시 중지되었어요.
# Label and name of the box where the printer is chosen.
screenshot-print-printer = 프린터
# Label and name of the choice of which pages are printed.
screenshot-print-pages = 페이지
# The choice to print all the pages.
screenshot-print-all = 전체
# The choice to print a range of pages.
screenshot-print-range = 범위
# Label and name of the box for the number of copies.
screenshot-print-copies = 매수
# Label and name of the choice of printing on both sides.
screenshot-print-sides = 양면
# Label and name of the choice of colour, and the choice for colour.
screenshot-print-colour = 컬러
# The choice to print in black and white.
screenshot-print-mono = 흑백
# Label and name of the choice of paper size.
screenshot-print-paper = 용지
# Label and name of the choice of how the page stands.
screenshot-print-page = 방향
# Name of the main button that prints, and the title when nothing else is known.
screenshot-print-go = 인쇄
# Title of the print window for a named document.
screenshot-print-doc = { $doc } 인쇄
# Title of the print window when the program has no document name.
screenshot-print-from = { $who }에서 인쇄
# Title of the notice that says a print did not go.
screenshot-print-failed = 인쇄하지 못했어요
# Words of that notice when the choices made were not kept.
screenshot-print-lost = 설정을 저장하지 못했어요. 다시 인쇄하세요.

## The file chooser (src/screenshot/file_chooser.rs)

# Line in the menu on a file or folder: change its name.
screenshot-chooser-rename = 이름 바꾸기
# Line in the menu on a file or folder: cut it.
screenshot-chooser-cut = 잘라내기
# Line in the menu: paste what was cut or copied.
screenshot-chooser-paste = 붙여넣기
# Line in the menu and name of the round button: make a new folder.
screenshot-chooser-new-folder = 새 폴더
# Line in the menu: move to the Trash.
screenshot-chooser-trash = 휴지통으로 이동
# Line in the menu on a file or folder: delete it for good, after asking.
screenshot-chooser-delete = 영구 삭제
# Question before deleting for good.
screenshot-chooser-delete-ask = 영구 삭제할까요?
# Line in the menu: copy the place of the thing as text.
screenshot-chooser-copy-location = 위치 복사
# Line in the menu on a folder: pin it to Folders' side bar.
screenshot-chooser-pin = 사이드바에 고정
# Line in the menu on a folder pinned already.
screenshot-chooser-unpin = 고정 해제
# Red line after a pin could not be kept.
screenshot-chooser-pin-not-kept = 고정을 저장하지 못했어요.
# Red line after some items could not be deleted.
screenshot-chooser-not-deleted =
    { $n ->
       *[other] Trindle이 항목 { $n }개를 삭제하지 못했어요: { $names }.
    }
# What the screen reader says for the list of places down the left of the file chooser.
screenshot-chooser-places = 위치
# What the screen reader says for the list of files and folders in the file chooser.
screenshot-chooser-files = 파일
# Name of the first place in the side bar, and of the only part of the path when Recent is shown.
screenshot-chooser-recent = 최근
# Name of the person's home folder in the side bar.
screenshot-chooser-home = 홈
# Name the screen reader gives to the path along the top.
screenshot-chooser-folder = 폴더
# Shown when a folder has nothing in it.
screenshot-chooser-empty = 비어 있어요
# Name of the choice of which kind of file is shown.
screenshot-chooser-kind = 파일 종류
# Said where a place on another computer is not answering.
screenshot-chooser-not-answering = 다른 컴퓨터가 응답하지 않아요.
# What the saved list of last folders is called when it is found damaged and set aside.
screenshot-chooser-damaged = 선택 창 폴더
# Words under the box when the folder a name points into is not there.
screenshot-chooser-no-folder = 그 폴더가 없어요.
# Words under the box when saving would replace a file.
screenshot-chooser-replace-ask = { $name }이(가) 이미 있어요. 바꾸려면 바꾸기를 누르거나 Enter 키를 한 번 더 누르세요.
# Words under the box when a key is pressed on Recent, which is not a folder.
screenshot-chooser-recent-not-folder = 최근은 폴더가 아니에요. 먼저 폴더를 여세요.
# Words under the box when a key is pressed with nothing picked.
screenshot-chooser-nothing-picked = 고른 것이 없어요.
# Words under the box while a file by that name is already there.
screenshot-chooser-there = { $name }이(가) 이미 있어요.
# Name of the round button that hides the hidden files.
screenshot-chooser-hide-hidden = 숨김 파일 가리기
# Name of the round button that shows the hidden files.
screenshot-chooser-show-hidden = 숨김 파일 보기
# Words of the main button when the program gave none, to choose a folder.
screenshot-chooser-choose-folder = 폴더 고르기
# Head of the question when a folder of that name is already there.
screenshot-chooser-clash-folder = { $name } 폴더가 이미 있어요
# Head of the question when a file of that name is already there.
screenshot-chooser-clash-file = { $name }이(가) 이미 있어요
# Name of the round button that leaves a pasted thing out.
screenshot-chooser-skip = 건너뛰기
# Name of the round button that keeps both under different names.
screenshot-chooser-both = 둘 다 유지
# Word on the button that replaces what is there.
screenshot-chooser-replace = 바꾸기
# What the screen reader says for the button that replaces a folder.
screenshot-chooser-replace-spoken = 바꾸기. 기존 폴더는 휴지통으로 이동해요.
# Name of the main button that merges two folders.
screenshot-chooser-merge = 병합
# Words in the question when folders are being pasted.
screenshot-chooser-rest-folders = 이미 있는 모든 폴더에 이렇게 하기.
# Words in the question when files are being pasted.
screenshot-chooser-rest-files = 이미 있는 모든 파일에 이렇게 하기.
# Name of the switch that gives the same answer to the rest.
screenshot-chooser-same = 나머지도 똑같이 하기
# Words under the box after Cut.
screenshot-chooser-was-cut = 이동 준비됨: { $what }
# Words under the box after Copy.
screenshot-chooser-was-copied = 복사 준비됨: { $what }
# Words under the box when Paste is pressed and there is nothing to paste.
screenshot-chooser-nothing-cut = 잘라내거나 복사한 것이 없어요.
# Words under the box when Paste is pressed while a paste is still going.
screenshot-chooser-paste-busy = Trindle이 아직 붙여넣는 중이에요.
# Red line after some items could not be moved.
screenshot-chooser-not-moved =
    { $n ->
       *[other] Trindle이 항목 { $n }개를 이동하지 못했어요: { $names }.
    }
# Red line after some items could not be copied.
screenshot-chooser-not-copied =
    { $n ->
       *[other] Trindle이 항목 { $n }개를 복사하지 못했어요: { $names }.
    }
# Red line after some items could not be put in the Trash.
screenshot-chooser-not-trashed =
    { $n ->
       *[other] Trindle이 항목 { $n }개를 휴지통으로 옮기지 못했어요: { $names }.
    }
# Name of the box where a file or folder is renamed.
screenshot-chooser-new-name = { $old }의 새 이름
# What several things are called in a line such as Cut.
screenshot-chooser-items =
    { $n ->
       *[other] 항목 { $n }개
    }

## Email (src/screenshot/portal_email.rs)

# Title of the notice when an email could not be started.
screenshot-mail-failed = 이메일을 만들지 못했어요
# Why: no mail program is chosen or installed.
screenshot-mail-no-app = 메일 프로그램을 고르지 않았어요. 설정의 기본 앱에서 고르세요.
# Why: a program asked to attach more files than allowed.
screenshot-mail-too-many = 첨부할 파일이 너무 많아요.
# Why: a file, or all the files together, are too big.
screenshot-mail-too-big = 파일이 너무 커서 첨부할 수 없어요.
# Why: a file could not be read as a plain file.
screenshot-mail-bad-file = 파일을 첨부하지 못했어요.
# Why: an address was not a plain address.
screenshot-mail-bad-address = 올바르지 않은 주소가 있어요.
# Why: the subject was too long or had odd characters.
screenshot-mail-bad-subject = 제목이 올바르지 않아요.
# Why: the words were too long or had odd characters.
screenshot-mail-bad-words = 내용이 올바르지 않아요.
# Why: no private place to keep the files.
screenshot-mail-no-place = 파일을 안전하게 보관하지 못했어요.
# Why: the mail program did not start.
screenshot-mail-not-started = 메일 프로그램이 시작되지 않았어요.
