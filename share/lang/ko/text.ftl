# machine translated (Korean), not yet reviewed by a person.

## Text (src/text): the plain text and Markdown editor, its menus, its find bar, its spelling and its settings.

## The window, the tabs and the menus (src/text/mod.rs)

# The program's name: its window title, and the name a screen reader gives the text box.
text-title = Text
# Name of a tab that has no file yet, and the name of a file made from one.
text-untitled = 제목 없음
# Button that opens the file chooser to open a file; also the title of that chooser.
text-open = 열기
# Button in the menu for a new file, with its shortcut in brackets.
text-new-key = 새로 만들기 (Ctrl+N)
# Button in the menu for opening a file, with its shortcut in brackets.
text-open-key = 열기 (Ctrl+O)
# Button in the menu that shows the files opened or saved recently. Also the heading of that list.
text-recent-files = 최근 파일
# Button in the menu that saves the file as it is.
text-save = 저장
# The words a screen reader gives the Save button, with its shortcut.
text-save-key = 저장 (Ctrl+S)
# Button in the menu that saves the file under a new name; also the title of the chooser that asks for the name.
text-save-as = 다른 이름으로 저장
# The words a screen reader gives the Save as button, with its shortcut.
text-save-as-key = 다른 이름으로 저장 (Ctrl+Shift+S)
# Line under the Save button when the file was changed by something else after Text opened, saved or reloaded it. Nothing has been saved.
text-changed-elsewhere = 파일을 연 뒤에 내용이 바뀌었어요.
# Button beside that line: save the words here over the changed file.
text-save-anyway = 그래도 저장
# Button beside that line: read the file again from disk and drop the unsaved changes here.
text-reload = 다시 불러오기
# Button in the menu that undoes the last change, with its shortcut.
text-undo-key = 실행 취소 (Ctrl+Z)
# Button in the menu that redoes the last undone change, with its shortcut.
text-redo-key = 다시 실행 (Ctrl+Shift+Z)
# Button in the menu that opens the find bar, with its shortcut.
text-find-key = 찾기 (Ctrl+F)
# Button in the menu that opens the find and replace bar, with its shortcut.
text-find-replace-key = 찾아 바꾸기 (Ctrl+H)
# Button in the menu that opens the go to line bar, with its shortcut.
text-go-to-line-key = 줄로 이동 (Ctrl+G)
# Button in the menu for a Markdown file that shows it as it will look finished, with its shortcut.
text-finished-key = 미리 보기 (Ctrl+E)
# Button in the menu for a Markdown file that exports it as a web page. Also the title of the chooser for that.
text-export-html = 웹 페이지로 내보내기
# Button in the menu for a Markdown file that exports it as a PDF. Also the title of the chooser for that.
text-export-pdf = PDF로 내보내기
# Button in the menu that turns on wrapping of long lines.
text-wrap = 긴 줄 줄바꿈
# Button in the menu that shows or hides the line numbers.
text-line-numbers = 줄 번호
# Button in the menu that opens the Text settings.
text-settings = 설정
# Tip of the button on a tab that closes it. $name is the tab's file name.
text-close-tab = { $name } 닫기
# Tip of the button that closes the open bar.
text-close-bar = 표시줄 닫기
# Words the screen reader gives the button that shows the finished look of the text, with its shortcut.
text-finished-look = 미리 보기
# Menu item that copies everything in the text.
text-copy-all = 모든 텍스트 복사
# Menu items for the text box: cut, copy, paste and select all the words.
text-cut = 잘라내기
text-copy = 복사
text-paste = 붙여넣기
text-select-all = 모두 선택
# Menu item for a misspelt word in the text box that opens the spelling check. The same words label the Settings switch that turns spelling checking on or off.
text-check-spelling = 맞춤법 검사
# Shown when a character cannot be saved in the encoding the file is kept in. $c is the character, $encoding the encoding's name.
text-encoding-cannot-write = ‘{ $c }’은(는) { $encoding }(으)로 저장할 수 없어요. 상태 표시줄에서 다른 인코딩을 고르세요.
# The words on the status bar: the line and column of the caret.
text-line-column = { $line }행, { $col }열
# The words on the status bar: how many words and letters there are. Each count has its own singular and plural.
text-counts = { $words ->
       *[other] 단어 { $words }개
    }, { $letters ->
       *[other] 글자 { $letters }개
    }
# The words on the status bar when some words are selected: how many words and letters there are, then that they are selected.
text-counts-selected = { $words ->
       *[other] 단어 { $words }개
    }, { $letters ->
       *[other] 글자 { $letters }개
    } 선택됨
# Status bar item: the line ending of a file that uses Windows line endings.
text-windows-endings = Windows 줄 끝
# Status bar item: the encoding the file is read and written in. $name is the encoding's name.
text-encoding = 인코딩: { $name }
# Words in place of the file's name when the text has never been saved to a file.
text-no-file = 이 텍스트는 아직 파일이 없어요.

## The find bar (src/text/mod.rs)

# Words on the find bar: the find box's label and placeholder.
text-find = 찾기
# Words on the find bar: a switch to find only the same capitals.
text-match-case = 대소문자 구분
# Words on the find bar: a switch to find only whole words.
text-whole-words = 전체 단어
# Tip of the button that goes to the previous find, with its shortcut.
text-previous-key = 이전 (Shift+F3)
# Tip of the button that goes to the next find, with its shortcut.
text-next-key = 다음 (F3)
# Words on the replace bar: the replace box's label.
text-replace-with = 바꿀 내용
# Button on the replace bar that replaces the word found.
text-replace = 바꾸기
# The words a screen reader gives the Replace button.
text-replace-spoken = 찾은 단어 바꾸기
# Button on the replace bar that replaces every word found.
text-replace-all = 모두 바꾸기
# The words a screen reader gives the Replace all button.
text-replace-all-spoken = 찾은 단어 모두 바꾸기
# Words on the find bar when a find has been asked for and has no match.
text-none-found = 일치하는 항목 없음
# Words on the find bar when a find has matches. $at is which one is shown, $n how many there are.
text-of-found = { $at } / { $n }
# Words on the find bar when a find has matches and none is shown. $n is how many there are.
text-found = { $n }개 찾음
# Words on the find bar when the words replaced are counted. $n is how many, and it is never zero here.
text-replaced = { $n ->
       *[other] { $n }개 바꿈
    }
# Words on the go to line bar when the number typed is not a line number.
text-type-line = 줄 번호를 입력하세요
# Placeholder of the go to line box: the range of lines that can be typed. $lines is the number of lines.
text-goto-placeholder = 1 ~ { $lines }
# Words on the go to line bar after the box: how many lines there are. $lines is the number.
text-of-lines = / { $lines }
# Button on the go to line bar that goes to the line typed.
text-go = 이동
# Words on the go to line bar: the box's label.
text-go-to-line = 줄로 이동
# Button that goes back from a bar, a settings page or a list to the text.
text-back = 텍스트로 돌아가기

## The spelling bar (src/text/mod.rs)

# Words on the spelling bar, after the word: the word was added to the person's own words.
text-spell-added = 내 단어에 추가했어요
# Words on the spelling bar, after the word: the word is not in the dictionary and there are no suggestions.
text-spell-none = 사전에 없고 추천할 단어도 없어요
# Words on the spelling bar, after the word: the word is not in the dictionary and there are suggestions.
text-spell-offers = 사전에 없어요. 이 중에 있나요?
# Button on the spelling bar that uses the spelling suggested.
text-use-spelling = 이 철자 사용
# Button on the spelling bar that adds the word to the person's own words.
text-add-word = 내 단어에 추가
# Heading over the list of suggested spellings.
text-suggestions = 추천 철자

## Recent files (src/text/mod.rs)

# Words in the list of recent files when none has been opened or saved yet.
text-files-here = 열거나 저장한 파일이 여기에 표시돼요.
# Button that clears the list of recent files.
text-clear-list = 목록 지우기

## The Text settings (src/text/mod.rs)

# Heading of the Text settings page.
text-settings-title = Text 설정
# Heading of the group on the settings page for when Text starts.
text-starting = 시작
# Label of the choice on the settings page: what happens when Text opens.
text-when-opens = Text를 열 때
# First choice: Text opens the files it had open last time.
text-reopen = 이전 파일 다시 열기
# Second choice: Text opens with an empty page.
text-start-blank = 새로 시작
# Note under the start choice: work not saved to a file is kept whichever choice is made.
text-keep-note = 어느 쪽을 골라도 저장하지 않은 작업은 그대로 복원돼요.
# Heading of the group on the settings page for how the text looks.
text-showing = 텍스트 모양
# Label of the size slider: how big the text is.
text-size = 크기
# Name of the size slider, read out by a screen reader.
text-text-size = 글자 크기
# Heading of the group on the settings page for spelling.
text-spelling = 맞춤법
# Note on the settings page when no spelling dictionary is found on the computer.
text-no-hunspell = 설치된 Hunspell 사전이 없어요.
# Label of the list of spelling languages.
text-language = 언어
# Name of the list of spelling languages, read out by a screen reader.
text-spelling-language = 맞춤법 검사 언어
# Automatic choice of the spelling language, with the language it picked. $lang is the language.
text-automatic-language = 자동 ({ $lang })
# Heading of the font list on the settings page.
text-font = 글꼴
# The first font in the list: the font Trindle ships with.
text-own-font = Trindle 기본 글꼴

## Saving and the file dialogs (src/text/mod.rs)

# Heading over the encodings a file can be read as, when it is opened by a chosen encoding.
text-read-as = 파일 읽는 인코딩
# Heading over the encodings a file is saved in.
text-save-in = 저장 인코딩
# Note under the save encoding: the next save writes the file in this encoding.
text-next-save = 다음에 저장할 때 파일에 이 인코딩을 사용해요.
# Words saying the text has changes that are not saved to its file. $path is the file.
text-changes-not-saved = { $name }에 저장하지 않은 변경 사항이 있어요.
# Words saying the text has changes and no file yet.
text-not-saved = 이 텍스트는 파일에 저장되지 않았어요.
# Heading of the question shown before closing a tab with changes. $name is the tab's file name.
text-save-named = { $name }을(를) 저장할까요?
# Words in the question before closing: how many other tabs with changes are left after this one. $left is the number.
text-more-after =
    { $left ->
       *[other] 저장할 탭이 { $left }개 더 있어요.
    }
# Button in the question before closing that closes the tab without saving it.
text-close-no-save = 저장하지 않고 닫기
# Button in the question before closing that keeps the tab open.
text-keep-open = 열어 두기
# Two problems shown one after the other. $said is the first, $problem the next.
text-two-problems = { $said } { $problem }
# Shown when the file chosen is a folder. $name is the file's path.
text-is-folder = { $name }은(는) 폴더예요.
# Shown when a link in the finished look is pressed and is not a web address, or no browser started.
text-link-not-opened = 웹 주소만 여기서 열려요.

# Shown when the file is too big to open in Text. $name is the file's path.
text-too-big = { $name }은(는) 너무 커서 여기서 열 수 없어요.
# Shown when a file could not be opened. $name is the file's path and $reason why.
text-cannot-open = Trindle이 { $name }을(를) 열 수 없어요: { $reason }
# Shown when a file could not be written. $name is the file's path and $reason why.
text-cannot-write = Trindle이 { $name }에 쓸 수 없어요: { $reason }
# Shown when a file could not be saved. $name is the file's path and $reason why.
text-cannot-save = Trindle이 { $name }을(를) 저장할 수 없어요: { $reason }
# Shown when a file could not be read again. $name is the file's path and $reason why.
text-cannot-read = { $name }을(를) 읽을 수 없어요: { $reason }
# Shown when the list of open work could not be written. $folder is the folder and $reason why.
text-work-not-kept = Trindle이 { $folder }에 작업을 저장할 수 없어요: { $reason }
# Shown when the settings could not be saved. $reason says why.
text-settings-not-saved = 설정을 저장할 수 없어요: { $reason }
# Shown when a word could not be added to the person's own words. $word is the word and $reason why.
text-word-not-added = Trindle이 { $word }을(를) 내 사전에 추가할 수 없어요: { $reason }
# Shown when the list of recent files could not be cleared. $reason says why.
text-list-not-cleared = Trindle이 목록을 지울 수 없어요: { $reason }
# Shown when the file chooser could not be opened.
text-chooser-failed = 파일 선택 창을 열 수 없어요.
# Shown when the file chooser cannot be used because there is no session bus.
text-no-bus = Trindle이 지금은 파일 선택 창을 열 수 없어요.
# Shown when the spelling language asked for is not on the computer. $lang is its name.
text-no-dictionary = { $lang }(이)라는 사전이 없어요.
# Shown when a spelling dictionary could not be read. $lang is its name and $reason why.
text-dictionary-unreadable = Trindle이 { $lang } 사전을 읽을 수 없어요: { $reason }
# Shown when a save was stopped and the text it held before was kept in a folder. $reason says what went wrong and $path is where the old text is kept.
text-kept-before = { $reason }. 이전 텍스트는 { $path }에 저장되어 있어요.
# Shown when a save was stopped part way and the old text is kept. $why says why, and $path is where the old text is kept.
text-kept-before-that = { $why } 저장하기 전의 텍스트는 { $path }에 있어요.
# Shown when a save was stopped and what was kept of it cannot be read. $name is the file's path.
text-saved-unreadable = { $name }을(를) 저장하는 중에 Text가 닫혔고, Trindle이 저장된 부분을 읽을 수 없어요.
# Shown when a save was stopped and the file has changed since. $name is the file's path.
text-saved-changed = { $name }을(를) 저장하는 중에 Text가 닫혔어요. 그 뒤로 파일이 바뀌어서 Trindle이 그대로 두었어요.
# Shown when a save was stopped and the old text could not be put back. $name is the file's path, $reason why, and $path where the old text is kept.
text-saved-not-put-back = { $name }을(를) 저장하는 중에 Text가 닫혔고, Trindle이 파일을 원래대로 되돌릴 수 없어요: { $reason }. 저장하기 전의 텍스트는 { $path }에 있어요.

# Quiet word beside what was just saved, exported or changed.
text-done = 완료

# Suggested file name in the Save as chooser for text never saved before.
text-untitled-file = 제목 없음.txt

# The misspelt word, shown in quotation marks on the spelling bar. { $word } is the word.
text-spell-word = ‘{ $word }’
