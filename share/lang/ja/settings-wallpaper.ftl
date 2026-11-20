# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Wallpaper": a plain colour, a picture, or a folder of pictures that change by themselves.

## Page (src/settings/panels/wallpaper.rs)

# Name of the Settings page about the wallpaper, in the list and as its heading.
settings-wallpaper-title = 壁紙
# Words a person might type into the Settings search to find the wallpaper page; separate them with spaces.
settings-wallpaper-keywords = 背景 画像 スライドショー 写真 background picture slideshow photo
# Introduction under the heading of the wallpaper page, and the words behind the page i.
settings-wallpaper-help = 単色、1 枚の画像、または、好きな間隔で切り替わる画像のフォルダーを使えます。

## Lists

# Choice in the list of how long each picture stays.
settings-wallpaper-every-10s = 10 秒
# Choice in the list of how long each picture stays.
settings-wallpaper-every-30s = 30 秒
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1m = 1 分
# Choice in the list of how long each picture stays.
settings-wallpaper-every-5m = 5 分
# Choice in the list of how long each picture stays.
settings-wallpaper-every-15m = 15 分
# Choice in the list of how long each picture stays.
settings-wallpaper-every-30m = 30 分
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1h = 1 時間
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1d = 1 日
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-fill = 画面いっぱいに表示
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-fit = 画像全体を収める
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-center = 中央、実際のサイズ
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-stretch = 引き伸ばす
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-tile = 並べて表示
# Choice in the list of the order pictures change in: at random.
settings-wallpaper-order-shuffle = ランダム
# Choice in the list of the order pictures change in: by file name.
settings-wallpaper-order-name = ファイル名順
# Red words when the program that draws the wallpaper would not start.
settings-wallpaper-no-changer = Trindle は壁紙を切り替えるプログラムを起動できませんでした。

## Sentences about the pictures

# Under the large picture, when counting; { $used } of { $all } pictures are used.
settings-wallpaper-count =
    { $all ->
       *[other] { $all } 枚中 { $used } 枚を使用
    }
# Under the large picture; { $folder } is a folder with no pictures in it.
settings-wallpaper-empty-folder = { $folder } に画像がありません
# Under the large picture, for the pictures that come with Trindle; { $used } of { $all } are used.
settings-wallpaper-count-own =
    { $all ->
       *[other] Trindle 付属の画像 { $all } 枚中 { $used } 枚を使用
    }
# Under the large picture, for a folder; { $used } of { $all } pictures are used and { $folder } is the folder.
settings-wallpaper-count-folder =
    { $all ->
       *[other] { $folder } の画像 { $all } 枚中 { $used } 枚を使用
    }
# Under the large picture, for one of Trindle's own pictures; { $name } is its name.
settings-wallpaper-file-own = { $name }、Trindle 付属の画像の 1 枚
# Under the large picture when there is only a colour.
settings-wallpaper-plain-below = 単色です。下で設定します。
# Under the large picture when the picture or folder has gone.
settings-wallpaper-gone-below = 画像またはフォルダーが見つかりません。ほかのものを選んでください。
# Sentence about the wallpaper when it is a plain colour.
settings-wallpaper-plain = 単色だけです。
# Sentence about the wallpaper when Trindle's own pictures change; { $every } is how often, { $used } of { $all } pictures are used and { $extra } is any added sentences, each after a space.
settings-wallpaper-changing-own =
    { $all ->
       *[other] { $every }ごとに、Trindle 付属の画像 { $all } 枚中 { $used } 枚から新しい画像に切り替えます。{ $extra }
    }
# Sentence about the wallpaper when a folder's pictures change; { $every } is how often, { $used } of { $all } pictures are used, { $folder } is the folder's name and { $extra } is any added sentences, each after a space.
settings-wallpaper-changing-folder =
    { $all ->
       *[other] { $every }ごとに、{ $folder } の画像 { $all } 枚中 { $used } 枚から新しい画像に切り替えます。{ $extra }
    }
# Sentence about the wallpaper when its folder has no pictures; { $folder } is the folder.
settings-wallpaper-empty-folder-choose = フォルダー { $folder } に画像がありません。ほかのものを選んでください。
# Added to the sentence about the wallpaper when a folder holds more than Trindle looks through.
settings-wallpaper-more-than-looked-at = フォルダーの画像が Trindle で確認できる数より多いため、一部が除かれることがあります。
# Sentence about the wallpaper for one of Trindle's own pictures; { $name } is its name.
settings-wallpaper-file-own-said = { $name }、Trindle 付属の画像の 1 枚です。
# Sentence about the wallpaper for one chosen picture; { $name } is its name, written to read in the middle of a sentence.
settings-wallpaper-file-one = { $name } を常に表示します。
# Sentence about the wallpaper when the chosen picture cannot be used; { $why } is the reason.
settings-wallpaper-file-refused = Trindle はその画像を使っていません。{ $why } 代わりに Trindle 付属の画像を表示します。
# Sentence about the wallpaper when the picture has gone.
settings-wallpaper-file-gone = 画像が見つかりません。画像かフォルダーを選んでください。

## Lines and buttons

# Said in green under a line after its choice was kept.
settings-wallpaper-saved = 保存しました
# Said in green under the next picture line after the picture changed.
settings-wallpaper-changed = 切り替えました
# Small word drawn in the corner of a moving picture's tile.
settings-wallpaper-animated = 動く画像
# Title of the window for choosing a picture.
settings-wallpaper-chooser-title = 画像を選ぶ
# Name of the filter in that window that shows only pictures.
settings-wallpaper-chooser-filter = 画像
# Title of the window for choosing a folder of pictures.
settings-wallpaper-chooser-folder = 画像のフォルダーを選ぶ
# Spoken description of the large picture when the wallpaper is a plain colour.
settings-wallpaper-plain-name = 単色
# Name of the large picture of what is on the screen.
settings-wallpaper-on-screen = 画面の表示
# Title of the Wallpaper card.
settings-wallpaper-card-title = 壁紙
# Heading of the wallpaper line, and name of its switch.
settings-wallpaper-wallpaper = 壁紙
# Heading of the line for choosing where the picture comes from.
settings-wallpaper-picture = 画像
# Name of the round button that chooses one picture.
settings-wallpaper-choose-picture-button = 画像を選ぶ
# Name of the round button that chooses a folder of pictures.
settings-wallpaper-choose-folder = フォルダーを選ぶ
# Name of the round button that chooses Trindle's own pictures.
settings-wallpaper-own-pictures = Trindle 付属の画像
# Heading of the fit line.
settings-wallpaper-fit = 表示方法
# Name of the list of how the picture fits the screen.
settings-wallpaper-fit-name = 画面への合わせ方
# Heading of the colour line.
settings-wallpaper-colour = 色
# Small words under the colour line when a picture covers it.
settings-wallpaper-colour-hidden = 画像で隠れています。
# Placeholder and name of the box for typing the colour as a hex code; the example is a code.
settings-wallpaper-colour-hex = 16 進コード (例: #09090b)
# Words behind the i of the Wallpaper card.
settings-wallpaper-card-about = 単色、1 枚の画像、または、好きな間隔で切り替わる画像のフォルダーを使えます。色は、画像がないときと、画面全体を覆わない画像の周りに表示されます。#09090b のように、16 進コードで入力します。
# Small words on the cards that are greyed while the wallpaper is off.
settings-wallpaper-waits = 使うには、壁紙をオンにしてください。
# Heading of the line, and name of its switch, for changing the picture now and then.
settings-wallpaper-rotate = 画像を切り替える
# Heading of the line, and name of its list, for how long each picture stays.
settings-wallpaper-every = 切り替える間隔
# Heading of the line, and name of its list, for the order pictures change in.
settings-wallpaper-order = 順番
# Heading of the line, and name of its button, for moving to the next picture.
settings-wallpaper-next = 次の画像
# Title of the Changing pictures card.
settings-wallpaper-changing-title = 切り替わる画像
# Words behind the i of that card.
settings-wallpaper-changing-about = フォルダー内の画像を、ここで選んだ間隔で、ランダムまたはファイル名順に、1 枚ずつ表示します。
# Words behind the i of the Pictures card when pictures change by themselves.
settings-wallpaper-pictures-rotating = Trindle は、暗くなっていない画像をすべて使います。画像を押すと除外され、もう一度押すと戻ります。何も削除されません。大きな画像が、今表示されているものです。
# Words behind the i of the Pictures card when one picture is on.
settings-wallpaper-pictures-one = 画像を押すと、それを使います。
# Title of the Pictures card.
settings-wallpaper-pictures-title = 画像
# Spoken description of the empty frame shown when there are no pictures.
settings-wallpaper-no-pictures-name = 画像はまだありません
# Heading of the line shown when there are no pictures to choose from.
settings-wallpaper-no-pictures = 選べる画像がありません
