# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Appearance": themes, highlight colour, reading the screen aloud, icons, fonts and the pointer. The names of the palette colours come from the toolkit.

## Page (src/settings/panels/appearance.rs)

# Name of the Settings page about appearance, in the list and as its heading.
settings-appearance-title = 外観
# Words a person might type into the Settings search to find the appearance page; separate them with spaces.
settings-appearance-keywords = 色 強調色 テーマ 外観 ダーク ライト 暗い 明るい アイコン フォント ポインター カーソル スクリーンリーダー 読み上げ ラベル アクセシビリティ colour colours color colors accent theme themes look skin glass dark light mode sunshine bright icons font cursor pointer screen reader blind read aloud accessibility labels tooltips names pointing
# Introduction under the heading of the appearance page, and the words behind the page i.
settings-appearance-help = ライトかダーク、色、アイコン、フォント、ポインターを選べます。スクリーンリーダーもオンにできます。変更は、プログラムを開き直すまで反映されないことがあります。
# Question along the bottom of the page while a theme is on trial.
settings-appearance-foot-keep = この外観を使い続けますか？
# Small words beside the question along the bottom while a theme is on trial.
settings-appearance-foot-escape = Esc キーですぐに元に戻せます。

## Looks and the top sentence

# Name of the dark look, on a theme's button and in the top sentence.
settings-appearance-look-dark = ダーク
# Name of the light look, on a theme's button and in the top sentence.
settings-appearance-look-light = ライト
# The dark look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-dark-lower = ダーク
# The light look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-light-lower = ライト
# Spoken name of a theme's Dark or Light button; { $name } is the theme and { $look } the look.
settings-appearance-theme-spoken = { $name }、{ $look }
# Spoken description of the small picture of a theme; { $name } is the theme and { $look } its look, such as dark.
settings-appearance-sample-spoken = { $name } の見た目、{ $look }
# Said aloud after Dark or Light was chosen; { $look } is dark or light.
settings-appearance-aloud-look = { $look } の外観です。
# Sentence at the top of the page when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-top-unusable = { $theme } テーマは使えないため、Trindle 標準の外観を使っています。{ $why }
# Sentence at the top of the page for a theme that has dark and light looks; { $theme } is the theme, { $look } dark or light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme-look = { $theme } テーマ、{ $look }、強調色は { $accent }、アイコンは { $shape } です。
# Sentence at the top of the page for a theme with one look; { $theme } is the theme, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme = { $theme } テーマ、強調色は { $accent }、アイコンは { $shape } です。
# Sentence at the top of the page when no theme is chosen; { $look } is Dark or Light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-look = { $look }、強調色は { $accent }、アイコンは { $shape } です。
# Shape of the icons in the top sentence.
settings-appearance-shape-square = 四角
# Shape of the icons in the top sentence.
settings-appearance-shape-round = 丸
# Name of Trindle's own highlight colour, first in the list of colours.
settings-appearance-trindle-yellow = Trindle イエロー

## Theme card

# Start of the words behind the i of the reading the screen card.
settings-appearance-reading-says = スクリーンリーダーは、画面に表示されている内容と、入力した文字を読み上げます。Super、Alt、S キーを同時に押すと、オンとオフを切り替えられます。
# Red words under the icon shape line when the icons could not be remade.
settings-appearance-no-icons = Trindle はすべてのアイコンを更新できず、一部は古い形のままです。もう一度アイコンセットを選んでやり直してください。
# Small words on a theme's tile when Trindle made some of its colours clearer.
settings-appearance-corrected = Trindle が一部の文字を読みやすくしました。
# Said aloud when a theme is tried; { $theme } is its name and { $seconds } how long until the old look comes back.
settings-appearance-aloud-trying =
    { $seconds ->
       *[other] { $theme } テーマを試しています。「使い続ける」を押さないと、{ $seconds } 秒後に元の外観に戻ります。Esc キーですぐに戻せます。
    }
# Name of Trindle's own theme when none is named.
settings-appearance-trindle = Trindle
# Said aloud when a theme was kept; { $theme } is its name.
settings-appearance-aloud-kept = { $theme } テーマを使い続けます。
# Red words under the Theme line when the old look could not be written back.
settings-appearance-back-failed = 時間が切れると、元の外観に戻ります。
# Said aloud when the look from before is back.
settings-appearance-aloud-back = 元の外観に戻りました。
# Title of the window for choosing a theme file.
settings-appearance-chooser-title = テーマを選ぶ
# Name of the filter in that window that shows only themes.
settings-appearance-chooser-filter = テーマ
# Green words under the Add a theme line after a theme was added; { $name } is its name.
settings-appearance-added = { $name } を上の一覧に追加しました。
# Red words under the Theme line when a theme could not be removed.
settings-appearance-not-removed = テーマを削除できませんでした。
# Red words under the theme tiles when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-cannot-use = Trindle は { $theme } テーマを使えません。{ $why }
# Name of the bin button that removes one of the person's own themes.
settings-appearance-remove-theme = このテーマを削除
# Name of the button that chooses a theme; { $name } is the theme.
settings-appearance-use-theme = { $name } を使う
# Small words on a theme's tile naming its author; { $author } is the name.
settings-appearance-by = 作者: { $author }
# Name of the button that adds a theme, and heading of its line.
settings-appearance-add-theme = テーマを追加
# Spoken name of the turning ring shown while themes are being found.
settings-appearance-looking = テーマを探しています
# Title of the Theme card.
settings-appearance-theme-card = テーマ
# Words behind the i of the Theme card.
settings-appearance-theme-about = テーマは、色、角、大きさ、間隔をまとめた1つのファイルです。テーマを押すと試せます。すべての Trindle のウィンドウが一度に変わります。「使い続ける」を押さないと、30秒後に元の外観に自動で戻るので、読めないテーマのままになることはありません。テーマにダークとライトの両方がある場合は、画像の横にダークとライトのボタンが表示されます。ライトは明るい日差しの中で見やすく、「アプリ」から入れたアプリは、次に開くときにライトを使います。外観が1つだけのテーマには、このボタンはありません。強調色は、どのテーマでも自分で選んだままです。Trindle は追加したテーマを読むだけで実行はせず、読みにくい文字は読みやすく直します。Trindle 標準の外観にターミナルから戻すには、trindle-settings --theme default を実行します。
# Heading of the line holding the theme tiles.
settings-appearance-theme-line = テーマ
# Added after the words behind the i of its card.
settings-appearance-add-theme-about = 誰かから共有された .css ファイルなど、1つの .css ファイルです。
# Words of the link that opens help about what a theme file holds.
settings-appearance-theme-file-help = テーマファイルの中身

## Colour

# Red words under the colour box when what was typed is not a colour.
settings-appearance-hex-bad = 0-9 と a-f を使って、#fbc711 のように入力してください。
# Spoken description of the small dot that shows the colour typed.
settings-appearance-dot = 入力した色
# Placeholder of the box for typing a colour code.
settings-appearance-hex-code = 16進コード
# Spoken name of the box for typing a colour as a hex code; the example is a code.
settings-appearance-hex-spoken = 好きな色を、#fbc711 のような16進コードで入力
# Heading of the line for typing any colour.
settings-appearance-any-colour = 好きな色
# Added after the words behind the i of its card.
settings-appearance-any-colour-about = #fbc711 のように、6桁または8桁で入力します。
# Title of the Colour card.
settings-appearance-colour-title = 色
# Words behind the i of the Colour card.
settings-appearance-colour-about = 強調色は、ボタン、スイッチ、満ちていくリング、いまの位置を示します。Trindle のウィンドウはすぐに変わり、リングは次に開くときに変わります。色が暗すぎてカード上で見えない場合は、読めるまで Trindle が明るくします。ライトの外観では、薄すぎる色を Trindle が濃くします。ダークとライトは、「テーマ」の中の各テーマのタイルで選びます。
# Heading of the line with the highlight colour swatches.
settings-appearance-highlight = 強調色

## Reading the screen

# Heading of the line, and name of its switch, for reading the screen aloud.
settings-appearance-reader = 画面を読み上げる
# Heading of the line, and name of its switch, for keeping labels while pointing.
settings-appearance-labels = ポイント中はラベルを表示し続ける
# Title of the Reading the screen card.
settings-appearance-reading-title = 画面の読み上げ
# Words behind the i of that card; { $says } is the first sentences, from the key reading-says.
settings-appearance-reading-about = { $says } ボタンの名前は、通常は1.5秒ほどで消えます。ラベルを表示し続けると、ポインターがボタンの上にある間は表示されたままです。画面を読み上げている間は、すべてのボタンの下に名前が表示されます。

## Icons

# Name of the set of round icons.
settings-appearance-icons-circles = 丸
# Name of the set of square icons.
settings-appearance-icons-squares = 四角
# Words beside the turning ring while the icons are remade.
settings-appearance-icons-working = アイコンを変更中
# Green words under the shape line after the icons were remade.
settings-appearance-icons-changed = 変更しました
# Name of the button that chooses an icon set; { $name } is the set, such as Circles.
settings-appearance-icons-set = { $name }のアイコン
# Heading of the line for the shape of the icons.
settings-appearance-shape = 形
# Title of the Icons card.
settings-appearance-icons-title = アイコン
# Words behind the i of the Icons card.
settings-appearance-icons-about = Trindle のアイコンは、丸か四角を選べます。アプリ用のアイコンがない場合は、Trindle が合うものを作ります。リングも新しい形になります。

## Text

# Name of the list of font families, and heading of the font line.
settings-appearance-font = フォント
# Name of the font line's choice before the font in use has been read, so a screen reader has a word to say.
settings-appearance-font-default = 標準
# Name of the list of font sizes.
settings-appearance-font-size = フォントサイズ
# Name of the button that adds fonts, and heading of its line.
settings-appearance-add-fonts = フォントを追加
# Words of the link that opens help about what a font file must contain.
settings-appearance-fonts-help = ファイルに必要な内容
# Title of the Text card.
settings-appearance-text-title = 文字
# Words behind the i of the Text card.
settings-appearance-text-about = プログラムが文字に使うフォントとサイズです。ここで追加した .ttf または .otf ファイル、またはそれらの zip は、あなた専用のフォント一覧に追加されます。プログラムを開き直すまで反映されないことがあります。
# Title of the window for choosing font files.
settings-appearance-chooser-fonts = フォントを選ぶ
# Red words when a font could not be replaced; { $name } is the file name.
settings-appearance-font-not-replaced = { $name } を置き換えられませんでした。
# Green words after one font took the place of the one there; { $name } is the file name.
settings-appearance-font-replaced-one = Trindle が { $name } を置き換えました。これから起動するプログラムで使われます。
# Green words after several fonts took the place of the ones there; { $n } of { $total } were done.
settings-appearance-font-replaced-some = Trindle が { $total } 個のうち { $n } 個のフォントを置き換えました。これから起動するプログラムで使われます。
# Green words after fonts were added; { $n } is how many.
settings-appearance-fonts-added =
    { $n ->
       *[other] { $n } 個のフォントを上の一覧に追加しました。
    }
# Green words after added fonts were already in the list.
settings-appearance-fonts-in-list = そのフォントはすでに一覧にあります。

## Pointer

# Choice in the list of cursor sizes: the normal size.
settings-appearance-cursor-normal = 標準
# Choice in the list of cursor sizes: a large size.
settings-appearance-cursor-large = 大
# Choice in the list of cursor sizes: the largest size.
settings-appearance-cursor-larger = 特大
# Spoken name of a picture of a cursor size; { $size } is its name, such as Large.
settings-appearance-pointer-spoken = ポインター（{ $size }）
# Name of the standard cursor theme (the one every system has).
settings-appearance-cursor-standard = 標準
# Name of the list of cursor themes, and heading of its line.
settings-appearance-cursor = ポインター
# Name of the list of cursor sizes, and heading of its line.
settings-appearance-cursor-size = ポインターのサイズ
# Title of the Pointer card.
settings-appearance-pointer-title = ポインター
