# machine translated (Korean), not yet reviewed by a person.

# Settings page "Keyboard and mouse": layouts, Caps Lock, changed keys, programs' keyboards, game controllers, mouse and touchpad. The names of keys that a person types (Escape, Ctrl, Caps Lock) are read by the program and are not here.

## Page (src/settings/panels/keyboard.rs)

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = 키보드와 마우스
# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = layout language typing caps lock on-screen keyboard button ring mouse touchpad trackpad pointer speed tap click scroll wheel slower faster natural left handed plugin shortcut keys remap change swap ctrl escape super alt mac side buttons tapped held made by programs keyd 배열 언어 입력 화상 키보드 버튼 링 마우스 터치패드 포인터 속도 탭 클릭 스크롤 휠 느리게 빠르게 왼손잡이 플러그인 단축키 키 바꾸기
# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = 여기서 바꾸는 내용은 바로 적용돼요. Alt와 Shift를 함께 눌러 두 번째 배열로 바꿀 수 있어요.

## Top sentence

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = { $layout }(으)로 입력해요.
# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt와 Shift를 누르면 { $layout }(으)로 바뀌어요.
# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock은 아무 일도 하지 않아요.
# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock은 { $key } 키예요.
# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = 또 하나의 Ctrl
# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape
# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace
# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = 이 컴퓨터에는 키보드 배열 목록이 없어요.
# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = 키보드
# Heading of the line in that card.
settings-keyboard-no-list-heading = 키보드 배열 목록을 쓸 수 없어요
# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = 설치하려면 이 명령을 실행하세요: sudo apt install xkb-data
# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
       *[other] 키와 버튼 { $n }개를 바꿨어요.
    }

## Layout

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = 배열
# Heading of the layout line, and the name of its list.
settings-keyboard-layout = 배열
# Heading of the variant line, and the name of its list.
settings-keyboard-variant = 변형
# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = 표준
# First choice in the list of second layouts: no second layout.
settings-keyboard-none = 없음
# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = 바꿀 배열
# Words behind the i of the layout card.
settings-keyboard-layout-about = 각 키가 입력하는 글자를 정해요. 배열과 그 변형을 고를 수 있어요. Alt와 Shift를 함께 누르면 두 번째 배열로 바뀌고, 다시 누르면 돌아와요. 바로 적용돼요.
# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = 지금 쓰는 중

## Other keys

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock
# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = 또 하나의 Ctrl
# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape
# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace
# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = 없음
# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Caps Lock을 이렇게 쓰기
# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = 링의 키보드 버튼
# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = 다른 키
# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock을 두 번째 Ctrl이나 Escape 같은 다른 키로 쓸 수 있어요. 키보드 버튼은 링 가운데에 있고 화면 키보드를 열어요. 마우스나 터치 화면으로 입력할 때 쓰세요. 링을 다음에 열 때부터 보여요.

## Keys and buttons

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = 프로그램은 이것을 바꿀 수 없어요.
# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = 바꿀 키나 마우스 버튼을 누르세요.
# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = 아무것도 누르지 않아서 추가된 것이 없어요.
# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = 대신 쓸 키를 입력하세요. Escape나 Ctrl+C 같은 것이나 Nothing이에요.
# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle가 어떤 키인지 알아내지 못했어요. 목록에서 키를 고르세요.
# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = 모든 키보드와 마우스
# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = 아래에서 이 키를 직접 바꿨어요. 그 변경이 쓰여요.
# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = { $key }을(를) 바꿀 기기
# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = { $key }을(를) 바꿀 내용
# Name of the bin button that takes a change away.
settings-keyboard-take-away = 이 변경 없애기
# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = { $n }번째 줄은 무시했어요: { $why }
# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = 바꾼 키를 읽을 수 없어요
# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Trindle가 변경 내용을 읽을 수 있을 때까지 키는 평소대로 작동해요. { $why }
# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = 저장하지 못했어요. 바꾼 키 파일을 읽을 수 없어서 그대로 두었어요.
# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = 바꿀 키나 버튼 고르기
# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = 바꿀 키나 버튼을 누르세요
# Heading of the line for changing a key or button.
settings-keyboard-keys-add = 키나 버튼 바꾸기
# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key }은(는) 이미 바꿨어요.
# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = 키와 버튼
# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = 모든 키나 마우스 버튼이 다른 일을 할 수 있어요. 다른 키나 버튼, Ctrl+C처럼 함께 누르는 키, H, I처럼 차례로 누르는 키가 될 수 있어요. 아무 일도 안 하게 하거나, 톡 누르면 하나, 누르고 있으면 다른 하나를 하게 할 수도 있어요. 예를 들어 톡 누르면 Escape, 누르고 있으면 Ctrl이에요. 로그인 화면부터 어디서나 작동해요. Ctrl, Alt와 F 키를 함께 누르면 항상 텍스트 콘솔이 열리고, 거기서 trindle-settings --keys default를 입력하면 모든 키가 원래대로 돌아와요.

## Ready-made swaps

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Ctrl과 Caps Lock 바꾸기
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock: 톡 누르면 Escape, 누르고 있으면 Ctrl
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Escape와 Caps Lock 바꾸기
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Mac 키보드처럼 Alt와 Super 바꾸기
# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = 마우스 옆 버튼으로 복사와 붙여넣기

## Programs' keyboards and mice

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
settings-keyboard-made-switch = { $name } 믿기
# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = 아직 없어요
# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = 프로그램이 만든 키보드와 마우스
# Words behind the i of that card.
settings-keyboard-made-about = 키 바꾸기 도구와 게임 런처는 자체 키보드나 마우스를 만들 수 있어요. 이들의 키와 클릭은 입력과 게임에는 쓸 수 있지만, 종료나 허용 같은 보호된 버튼을 누르거나 사용자에게 하는 질문에 답할 수는 없어요. 여기서 켜면 사용자로 인정해요.

## Game controllers

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = 컨트롤러 { $vendor }:{ $product }
# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $title }의 { $name }
# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = { $title }이(가) 하는 일
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = A와 B 바꾸기
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = X와 Y 바꾸기
# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = 오른쪽 스틱의 위아래 뒤집기

## Plugin shortcuts

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = 플러그인 단축키
# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin }은(는) 쓰지 않아요: { $why }
# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = { $label }의 키
# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = 저장됨

## The screenshot and the shortcut list

# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = 스크린샷, 보고서, 단축키
# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = 스크린샷 찍기
# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = 단축키 목록 보기
# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = 지금 스크린샷 찍기
# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = 문제 알리기
# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = 지금 문제 알리기
# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = 지금 목록 보기

## Restarting the desktop

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = 바탕 화면 다시 시작
# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = 링과 도우미 다시 시작
# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = 모두 다시 시작
# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = { $label }의 키
# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = 지금 다시 시작해요. 열린 창은 그대로예요.
# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = 물어본 뒤 모두 다시 시작
# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = 다시 시작했어요
# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = 다른 단축키가 이미 { $keys }을(를) 써요.
# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = 저장하지 못했어요. { $why }

## Mouse and touchpad

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = 터치패드에도 적용돼요.
# Title of the mouse card.
settings-keyboard-mouse-title = 마우스
# Words behind the i of the mouse card.
settings-keyboard-mouse-about = 포인터 속도는 마우스를 움직일 때 포인터가 움직이는 거리예요. 터치패드에도 적용돼요. 스크롤 속도는 휠을 한 번 돌릴 때 페이지가 움직이는 거리예요. 느리게, 보통, 빠르게 중에서 고르거나 슬라이더로 원하는 값을 정하세요. 자연스러운 스크롤을 켜면 휠을 돌리는 쪽으로 페이지가 움직여요. 휴대폰과 같아요. 왼손잡이용은 왼쪽 버튼과 오른쪽 버튼을 바꿔요.
# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = 포인터 속도
# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = 스크롤 속도
# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = 자연스러운 스크롤
# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = 왼손잡이용
# Heading and switch name for the clicking helper.
settings-keyboard-clicker = 클릭 도우미
# Small words under the clicking helper line.
settings-keyboard-clicker-about = 머무르기나 스위치 클릭 도구의 클릭도 사용자의 클릭으로 인정해요.
# Title of the touchpad card.
settings-keyboard-touchpad-title = 터치패드
# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = 스크롤 속도는 두 손가락으로 스크롤할 때 페이지가 움직이는 거리예요. 탭하여 클릭을 켜면 가볍게 탭해도 클릭으로 인정해요. 자연스러운 스크롤을 켜면 손가락이 움직이는 쪽으로 페이지가 움직여요. 휴대폰과 같아요. 입력 중 끄기는 입력하는 동안 손바닥이 포인터를 움직이지 않게 해요.
# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = 탭하여 클릭
# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = 입력 중 끄기
# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = 느리게
# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = 보통
# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = 빠르게

## USB ports

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB 포트
# Words behind the i of that card.
settings-keyboard-usb-about = USB 포트가 시작되지 않으면 Trindle가 한 번 껐다가 다시 켜서 다시 시도해요. 포트마다 한 번보다 많이 하지는 않아요.
# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle가 변경을 되돌리지도 못했어요: { $undo }
