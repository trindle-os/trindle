# machine translated (Japanese), not yet reviewed by a person.

# Settings page "Display": screens, resolution, size, rotation, arrangement and night light.

## Page (src/settings/panels/display.rs)

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = ディスプレイ
# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = 画面 モニター 解像度 サイズ 拡大 縮小 回転 リフレッシュ ナイトライト 暖色 ブルーライト 夜 目 screen monitor resolution size scale zoom rotate refresh night light
# Introduction under the heading of the display page, and the words behind the page i.
settings-display-help = 変更はすぐに反映されます。画面がおかしくなったら、何もしないでください。「保持」を押さないと、15 秒後に自動で元に戻ります。ナイトライトは、夕方になると画面を暖かい色にします。
# Question along the bottom of the page while a change of displays waits to be kept.
settings-display-foot-keep = この配置を保持しますか?
# Sentence at the top of the page while the displays are being found.
settings-display-looking = ディスプレイを確認中…
# Sentence at the top of the page when no display was found.
settings-display-none-found = ディスプレイが見つかりません。

## Top sentence

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = 使用中のディスプレイはありません
# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = ディスプレイを 1 台使用中です
# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
       *[other] ディスプレイを { $n } 台使用中です
    }
# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = ナイトライトはオフです
# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = ナイトライトは { $from } から { $to } まで画面を暖かくします
# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = ナイトライトは日没から日の出まで画面を暖かくします
# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }。{ $warm }。

## Lists

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = そのまま
# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = 右に回転
# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = 左に回転
# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = 上下逆さま
# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = 右側
# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = 左側
# Choice in the list of where a display sits: above the other.
settings-display-place-above = 上側
# Choice in the list of where a display sits: below the other.
settings-display-place-below = 下側

## Resolutions and sizes

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }
# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height }、{ $hz } Hz
# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (推奨)
# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz
# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  (標準)
# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = { $display } の解像度
# Name of the list of sizes; { $display } is the display.
settings-display-size-on = { $display } のサイズ
# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = { $display } の回転
# Heading of the resolution line.
settings-display-resolution = 解像度
# Heading of the size line.
settings-display-size = サイズ
# Heading of the rotation line.
settings-display-rotation = 回転
# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = このディスプレイを使う
# Small words under that line for the only display in use.
settings-display-use-only = 使用中のディスプレイが 1 台だけのときは、オフにできません。
# Words behind the i of a display's card.
settings-display-card-about = 変更はすぐに反映されます。画面がおかしくなったら、何もしないでください。「保持」を押さないと、15 秒後に自動で元に戻ります。推奨の解像度は、その画面に合わせて作られたものです。標準のサイズは、Trindle がこの画面向けに選んだものです。

## Arrangement

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = { $display } の位置
# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = 位置
# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $other } に対する { $display } の位置
# Title of the card about where displays sit.
settings-display-arrangement-title = 配置
# Words behind the i of that card.
settings-display-arrangement-about = ディスプレイが並ぶ位置を設定します。接している端で、ポインターが行き来できるようになります。
# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = 並べて配置するには、ディスプレイが 2 台必要です。

## Changes and keeping

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = 変更できませんでした。
# Said aloud after the displays changed; { $seconds } is how long until they go back.
settings-display-aloud-changed =
    { $seconds ->
       *[other] ディスプレイが変わりました。「保持」を押さないと、{ $seconds } 秒後に元に戻ります。
    }
# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = 変更を元に戻せませんでした。{ $why }
# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = この配置は今は使われていますが、Trindle は次回のログイン用に保存できませんでした。
# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = 何も変わっていません
# Small words under that line.
settings-display-refused-change = その変更はできませんでした。
# Heading of the line under the picture of the displays.
settings-display-apply-at-once = 変更はすぐに反映されます

## Night light

# Title of the night light card.
settings-display-night-title = ナイトライト
# Words behind the i of that card.
settings-display-night-about = 夕方になると画面が暖かい色になり、目にやさしくなります。タイムゾーンから求めた日没と日の出に合わせるか、自分で決めた時刻に合わせます。タイムゾーンから場所がわからないときは、自分で決めた時刻が使われます。
# Heading of the night light line, and name of its switch.
settings-display-night = ナイトライト
# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = { $zone } の日没と日の出です。
# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = 日没の時刻がわからないため、Trindle は決めた時刻を使っています。
# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = 日没から日の出まで
# Choice in the When list: between times the person sets.
settings-display-when-times = 時刻を決める
# Heading of the When line, and name of its list.
settings-display-when = いつ
# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = 開始
# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = 終了
# Heading of the warmth line, and name of its slider.
settings-display-warmth = 暖かさ
# Small word under the left end of the warmth slider.
settings-display-little = 少し
# Small word under the right end of the warmth slider.
settings-display-very = とても暖かい
# Spoken description of the strip of colours night light runs through.
settings-display-strip = 少し暖かい色から、とても暖かい色まで

## Pictures and cards

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = 使用中のディスプレイはありません
# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }、{ $res }
# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
       *[other] ディスプレイ { $n } 台、配置のとおり
    }
# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = ディスプレイなし
# Name of the large picture of the displays.
settings-display-your-displays = お使いのディスプレイ
# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Trindle はディスプレイを見つけられませんでした。
# Title of the card shown when no display was found.
settings-display-none-title = ディスプレイが見つかりません
# Heading of the line in that card.
settings-display-none-heading = ディスプレイが見つかりません
# Title of the card about the graphics card.
settings-display-graphics-title = グラフィックス
# Words behind the i of that card.
settings-display-graphics-about = このコンピューターには、オープンドライバーを使う NVIDIA のグラフィックスカードがあります。動作はしますが、2022 年ごろ以降のカードでは、オープンドライバーがカードの速度を上げられないため、動きが遅く感じます。代わりに NVIDIA 純正のドライバーをインストールできます。
# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = オープンドライバーを使う NVIDIA カード
# Small words under that line.
settings-display-nvidia-about = 2022 年以降のカードは、動きが遅くなることがあります。
# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = 対処のしかた
