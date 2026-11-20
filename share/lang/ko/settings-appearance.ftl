# machine translated (Korean), not yet reviewed by a person.

# Settings page "Appearance": themes, highlight colour, reading the screen aloud, icons, fonts and the pointer. The names of the palette colours come from the toolkit.

## Page (src/settings/panels/appearance.rs)

# Name of the Settings page about appearance, in the list and as its heading.
settings-appearance-title = 화면 모양
# Words a person might type into the Settings search to find the appearance page; separate them with spaces.
settings-appearance-keywords = 색 색상 강조 테마 모양 스킨 유리 어둡게 밝게 모드 햇빛 밝은 아이콘 글꼴 폰트 커서 포인터 화면 낭독기 시각장애 읽기 소리 접근성 라벨 이름 툴팁 가리키기
# Introduction under the heading of the appearance page, and the words behind the page i.
settings-appearance-help = 밝게 또는 어둡게, 색, 아이콘, 글꼴, 포인터를 고르거나 화면 낭독기를 켜세요. 일부 프로그램은 다시 열어야 바뀐 내용이 보여요.
# Question along the bottom of the page while a theme is on trial.
settings-appearance-foot-keep = 이 모양을 유지할까요?
# Small words beside the question along the bottom while a theme is on trial.
settings-appearance-foot-escape = Escape를 누르면 바로 되돌려요.

## Looks and the top sentence

# Name of the dark look, on a theme's button and in the top sentence.
settings-appearance-look-dark = 어둡게
# Name of the light look, on a theme's button and in the top sentence.
settings-appearance-look-light = 밝게
# The dark look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-dark-lower = 어두운
# The light look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-light-lower = 밝은
# Spoken name of a theme's Dark or Light button; { $name } is the theme and { $look } the look.
settings-appearance-theme-spoken = { $name }, { $look }
# Spoken description of the small picture of a theme; { $name } is the theme and { $look } its look, such as dark.
settings-appearance-sample-spoken = { $name }의 모양, { $look }
# Said aloud after Dark or Light was chosen; { $look } is dark or light.
settings-appearance-aloud-look = { $look } 모양이에요.
# Sentence at the top of the page when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-top-unusable = { $theme } 테마를 쓸 수 없어서 Trindle 기본 모양을 쓰고 있어요. { $why }
# Sentence at the top of the page for a theme that has dark and light looks; { $theme } is the theme, { $look } dark or light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme-look = { $theme } 테마, { $look } 모양, 강조 색 { $accent }, { $shape } 아이콘이에요.
# Sentence at the top of the page for a theme with one look; { $theme } is the theme, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme = { $theme } 테마, 강조 색 { $accent }, { $shape } 아이콘이에요.
# Sentence at the top of the page when no theme is chosen; { $look } is Dark or Light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-look = { $look }, 강조 색 { $accent }, { $shape } 아이콘이에요.
# Shape of the icons in the top sentence.
settings-appearance-shape-square = 사각
# Shape of the icons in the top sentence.
settings-appearance-shape-round = 둥근
# Name of Trindle's own highlight colour, first in the list of colours.
settings-appearance-trindle-yellow = Trindle 노랑

## Theme card

# Start of the words behind the i of the reading the screen card.
settings-appearance-reading-says = 화면 낭독기는 화면에 있는 내용과 입력하는 내용을 소리 내어 읽어 줘요. Super, Alt, S를 함께 누르면 켜고 끌 수 있어요.
# Red words under the icon shape line when the icons could not be remade.
settings-appearance-no-icons = Trindle이 아이콘을 모두 바꾸지 못해서 일부는 예전 모양 그대로예요. 다시 시도하려면 아이콘 세트를 다시 고르세요.
# Small words on a theme's tile when Trindle made some of its colours clearer.
settings-appearance-corrected = Trindle이 일부 글자를 읽기 쉽게 고쳤어요.
# Said aloud when a theme is tried; { $theme } is its name and { $seconds } how long until the old look comes back.
settings-appearance-aloud-trying =
    { $seconds ->
       *[other] { $theme } 테마를 써 보는 중이에요. 유지를 누르지 않으면 { $seconds }초 뒤에 예전 모양으로 돌아가요. 지금 돌아가려면 Escape를 누르세요.
    }
# Name of Trindle's own theme when none is named.
settings-appearance-trindle = Trindle
# Said aloud when a theme was kept; { $theme } is its name.
settings-appearance-aloud-kept = { $theme } 테마를 유지했어요.
# Red words under the Theme line when the old look could not be written back.
settings-appearance-back-failed = 시간이 지나면 예전 모양으로 돌아가요.
# Said aloud when the look from before is back.
settings-appearance-aloud-back = 예전 모양으로 돌아왔어요.
# Title of the window for choosing a theme file.
settings-appearance-chooser-title = 테마 고르기
# Name of the filter in that window that shows only themes.
settings-appearance-chooser-filter = 테마
# Green words under the Add a theme line after a theme was added; { $name } is its name.
settings-appearance-added = { $name }이(가) 위 목록에 추가되었어요.
# Red words under the Theme line when a theme could not be removed.
settings-appearance-not-removed = 테마를 지우지 못했어요.
# Red words under the theme tiles when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-cannot-use = Trindle이 { $theme } 테마를 쓸 수 없어요. { $why }
# Name of the bin button that removes one of the person's own themes.
settings-appearance-remove-theme = 이 테마 지우기
# Name of the button that chooses a theme; { $name } is the theme.
settings-appearance-use-theme = { $name } 사용
# Small words on a theme's tile naming its author; { $author } is the name.
settings-appearance-by = 만든 이: { $author }
# Name of the button that adds a theme, and heading of its line.
settings-appearance-add-theme = 테마 추가
# Spoken name of the turning ring shown while themes are being found.
settings-appearance-looking = 테마 찾는 중
# Title of the Theme card.
settings-appearance-theme-card = 테마
# Words behind the i of the Theme card.
settings-appearance-theme-about = 테마는 색, 모서리, 크기, 간격까지 담은 하나의 파일이에요. 테마를 누르면 써 볼 수 있어요. 모든 Trindle 창이 한꺼번에 바뀌어요. 유지를 누르지 않으면 30초 뒤에 예전 모양으로 저절로 돌아가므로, 읽을 수 없는 테마에 갇힐 일이 없어요. 테마에 어두운 모양과 밝은 모양이 모두 있으면 그림 옆에 어둡게와 밝게 버튼이 나타나요. 밝게는 햇빛이 강한 곳에서 보기 쉽고, 앱에서 설치한 프로그램은 다음에 열 때부터 밝게를 써요. 모양이 하나뿐인 테마에는 이 버튼이 없어요. 강조 색은 어떤 테마에서도 내가 고른 색 그대로예요. Trindle은 추가한 테마를 읽기만 하고 실행하지 않으며, 읽기 어려운 글자는 더 잘 보이게 고쳐요. 터미널에서 Trindle 기본 모양으로 되돌리려면 trindle-settings --theme default를 실행하세요.
# Heading of the line holding the theme tiles.
settings-appearance-theme-line = 테마
# Added after the words behind the i of its card.
settings-appearance-add-theme-about = .css 파일 하나예요. 누군가 공유해 준 파일 같은 것이에요.
# Words of the link that opens help about what a theme file holds.
settings-appearance-theme-file-help = 테마 파일에 들어 있는 내용

## Colour

# Red words under the colour box when what was typed is not a colour.
settings-appearance-hex-bad = 0-9와 a-f를 쓰세요. 예: #fbc711.
# Spoken description of the small dot that shows the colour typed.
settings-appearance-dot = 입력한 색
# Placeholder of the box for typing a colour code.
settings-appearance-hex-code = 16진수 코드
# Spoken name of the box for typing a colour as a hex code; the example is a code.
settings-appearance-hex-spoken = 16진수 코드로 입력하는 모든 색, 예: #fbc711
# Heading of the line for typing any colour.
settings-appearance-any-colour = 아무 색
# Added after the words behind the i of its card.
settings-appearance-any-colour-about = #fbc711처럼 6자리 또는 8자리예요.
# Title of the Colour card.
settings-appearance-colour-title = 색
# Words behind the i of the Colour card.
settings-appearance-colour-about = 강조 색은 버튼, 스위치, 차오르는 링, 현재 위치를 표시해요. Trindle 창은 바로 바뀌고, 링은 다음에 열 때 바뀌어요. 카드 위에서 너무 어두워 안 보이는 색은 읽을 수 있을 때까지 Trindle이 밝게 해 줘요. 밝은 모양에서는 너무 옅은 색을 어둡게 해 줘요. 어둡게나 밝게는 테마 아래 각 테마 타일에서 고르세요.
# Heading of the line with the highlight colour swatches.
settings-appearance-highlight = 강조 색

## Reading the screen

# Heading of the line, and name of its switch, for reading the screen aloud.
settings-appearance-reader = 화면 소리 내어 읽기
# Heading of the line, and name of its switch, for keeping labels while pointing.
settings-appearance-labels = 가리키는 동안 라벨 유지
# Title of the Reading the screen card.
settings-appearance-reading-title = 화면 읽기
# Words behind the i of that card; { $says } is the first sentences, from the key reading-says.
settings-appearance-reading-about = { $says } 버튼 이름은 보통 1.5초 뒤에 사라져요. 라벨을 유지하면 포인터가 버튼 위에 있는 동안 계속 보여요. 화면을 소리 내어 읽는 동안에는 모든 버튼 아래에 이름이 나타나요.

## Icons

# Name of the set of round icons.
settings-appearance-icons-circles = 원형
# Name of the set of square icons.
settings-appearance-icons-squares = 사각형
# Words beside the turning ring while the icons are remade.
settings-appearance-icons-working = 아이콘 바꾸는 중
# Green words under the shape line after the icons were remade.
settings-appearance-icons-changed = 바꿨어요
# Name of the button that chooses an icon set; { $name } is the set, such as Circles.
settings-appearance-icons-set = { $name } 아이콘
# Heading of the line for the shape of the icons.
settings-appearance-shape = 모양
# Title of the Icons card.
settings-appearance-icons-title = 아이콘
# Words behind the i of the Icons card.
settings-appearance-icons-about = Trindle 아이콘은 둥근 모양과 사각 모양이 있어요. 앱의 아이콘이 없으면 Trindle이 어울리는 아이콘을 만들어요. 링도 새 모양을 따라가요.

## Text

# Name of the list of font families, and heading of the font line.
settings-appearance-font = 글꼴
# Name of the font line's choice before the font in use has been read, so a screen reader has a word to say.
settings-appearance-font-default = 기본
# Name of the list of font sizes.
settings-appearance-font-size = 글꼴 크기
# Name of the button that adds fonts, and heading of its line.
settings-appearance-add-fonts = 글꼴 추가
# Words of the link that opens help about what a font file must contain.
settings-appearance-fonts-help = 파일에 들어 있어야 하는 내용
# Title of the Text card.
settings-appearance-text-title = 글자
# Words behind the i of the Text card.
settings-appearance-text-about = 프로그램이 글자에 쓰는 글꼴과 크기예요. 여기서 추가하는 글꼴은 .ttf 또는 .otf 파일이나 그 압축 파일이며, 내 글꼴 목록에만 추가돼요. 일부 프로그램은 다시 열어야 보여요.
# Title of the window for choosing font files.
settings-appearance-chooser-fonts = 글꼴 고르기
# Red words when a font could not be replaced; { $name } is the file name.
settings-appearance-font-not-replaced = { $name }을(를) 바꾸지 못했어요.
# Green words after one font took the place of the one there; { $name } is the file name.
settings-appearance-font-replaced-one = Trindle이 { $name }을(를) 바꿨어요. 지금부터 시작하는 프로그램이 이 글꼴을 써요.
# Green words after several fonts took the place of the ones there; { $n } of { $total } were done.
settings-appearance-font-replaced-some = Trindle이 글꼴 { $total }개 중 { $n }개를 바꿨어요. 지금부터 시작하는 프로그램이 이 글꼴을 써요.
# Green words after fonts were added; { $n } is how many.
settings-appearance-fonts-added =
    { $n ->
       *[other] 글꼴 { $n }개가 위 목록에 추가되었어요.
    }
# Green words after added fonts were already in the list.
settings-appearance-fonts-in-list = 그 글꼴은 이미 목록에 있어요.

## Pointer

# Choice in the list of cursor sizes: the normal size.
settings-appearance-cursor-normal = 보통
# Choice in the list of cursor sizes: a large size.
settings-appearance-cursor-large = 크게
# Choice in the list of cursor sizes: the largest size.
settings-appearance-cursor-larger = 더 크게
# Spoken name of a picture of a cursor size; { $size } is its name, such as Large.
settings-appearance-pointer-spoken = 포인터 { $size }
# Name of the standard cursor theme (the one every system has).
settings-appearance-cursor-standard = 표준
# Name of the list of cursor themes, and heading of its line.
settings-appearance-cursor = 포인터
# Name of the list of cursor sizes, and heading of its line.
settings-appearance-cursor-size = 포인터 크기
# Title of the Pointer card.
settings-appearance-pointer-title = 포인터
