
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = 显示器

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = 屏幕 显示器 分辨率 大小 缩放 放大 缩小 旋转 刷新率 夜间模式 暖色 蓝光 晚上 眼睛 screen monitor resolution size scale zoom bigger smaller rotate refresh night light warm blue light evening eyes

# Introduction under the heading of the display page, and the words behind the page i.
# SAFETY: needs a human review before it ships
settings-display-help = 更改会立即生效。如果屏幕显示异常，请不要操作：15 秒后会自动恢复，除非你点击“保留”。夜间模式会在晚上让屏幕颜色变暖。

# Question along the bottom of the page while a change of displays waits to be kept.
# SAFETY: needs a human review before it ships
settings-display-foot-keep = 保留此布局吗？

# Sentence at the top of the page while the displays are being found.
settings-display-looking = 正在查看显示器…

# Sentence at the top of the page when no display was found.
settings-display-none-found = 没有找到显示器。

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = 没有正在使用的显示器

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = 正在使用一台显示器

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
       *[other] 正在使用 { $n } 台显示器
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = 夜间模式已关闭

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = 夜间模式会在 { $from } 到 { $to } 之间让屏幕变暖

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = 夜间模式会在日落到日出之间让屏幕变暖

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used }；{ $warm }。

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = 正常

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = 向右旋转

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = 向左旋转

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = 上下颠倒

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = 在其右侧

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = 在其左侧

# Choice in the list of where a display sits: above the other.
settings-display-place-above = 在其上方

# Choice in the list of where a display sits: below the other.
settings-display-place-below = 在其下方

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height }，{ $hz } Hz

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  （推荐）

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent }%  （标准）

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = { $display }的分辨率

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = { $display }上的大小

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = { $display }的旋转

# Heading of the resolution line.
settings-display-resolution = 分辨率

# Small words under that line.
settings-display-resolution-about = 显示多少个点，刷新多快。

# Heading of the size line.
settings-display-size = 大小

# Small words under that line.
settings-display-size-about = 此屏幕上的所有内容看起来有多大。

# Heading of the rotation line.
settings-display-rotation = 旋转

# Small words under that line.
settings-display-rotation-about = 适用于竖放的屏幕。

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = 使用此显示器

# Small words under that line for the only display in use.
settings-display-use-only = 唯一正在使用的显示器无法关闭。

# Small words under that line otherwise.
settings-display-use-off = 已关闭，上面不显示任何内容。

# Words behind the i of a display's card.
# SAFETY: needs a human review before it ships
settings-display-card-about = 更改会立即生效。如果屏幕显示异常，请不要操作：15 秒后会自动恢复，除非你点击“保留”。推荐的分辨率是屏幕自身的分辨率；标准大小是 Trindle 为此屏幕选择的大小。

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = 放置{ $display }

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = 放置位置

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display }相对于{ $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = 排列

# Words behind the i of that card.
settings-display-arrangement-about = 显示器彼此相邻的位置，这样指针就能越过它们共用的边缘从一个移到另一个。

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = 使用两台显示器时，才能将一台放在另一台旁边。

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = 桌面拒绝了此更改。

# Said aloud after the displays changed; { $seconds } is how long until they go back.
# SAFETY: needs a human review before it ships
settings-display-aloud-changed =
    { $seconds ->
       *[other] 显示器已更改。除非你点击“保留”，否则将在 { $seconds } 秒后恢复。
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = 无法撤销此更改。{ $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = 此排列现已生效，但无法保留到下次登录。

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = 未做任何更改

# Small words under that line.
settings-display-refused-change = 桌面拒绝了此更改。

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = 更改会立即生效

# Small words under that line; { $seconds } is how long Keep is asked for.
# SAFETY: needs a human review before it ships
settings-display-keep-asked =
    { $seconds ->
       *[other] 窗口底部会询问是否保留，等待 { $seconds } 秒。
    }

# Title of the night light card.
settings-display-night-title = 夜间模式

# Words behind the i of that card.
settings-display-night-about = 屏幕在晚上变暖，对眼睛更友好。它根据时区推算日落和日出时间，也可以使用你自己设定的时间；如果时区无法确定这台电脑的位置，则使用设定的时间。

# Heading of the night light line, and name of its switch.
settings-display-night = 夜间模式

# Small words under that line.
settings-display-night-about-line = 晚上使用更暖的颜色。

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = { $zone }的日落和日出。

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = 时区没有给出位置：使用设定的时间。

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = 日落到日出

# Choice in the When list: between times the person sets.
settings-display-when-times = 设定时间

# Heading of the When line, and name of its list.
settings-display-when = 何时

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = 从

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = 到

# Heading of the warmth line, and name of its slider.
settings-display-warmth = 暖度

# Small words under the warmth line.
settings-display-warmth-about = 晚上屏幕变暖的程度。

# Small word under the left end of the warmth slider.
settings-display-little = 稍暖

# Small word under the right end of the warmth slider.
settings-display-very = 很暖

# Spoken description of the strip of colours night light runs through.
settings-display-strip = 从稍暖到很暖

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = 没有正在使用的显示器

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }，{ $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
       *[other] { $n } 台显示器，按排列方式并排
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = 无显示器

# Name of the large picture of the displays.
settings-display-your-displays = 你的显示器

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = 桌面没有列出任何显示器。

# Title of the card shown when no display was found.
settings-display-none-title = 未找到显示器

# Heading of the line in that card.
settings-display-none-heading = 没有可显示的地方

# Title of the card about the graphics card.
settings-display-graphics-title = 显卡

# Words behind the i of that card.
settings-display-graphics-about = 这台电脑有一块 NVIDIA 显卡，使用开源驱动。一切都能正常工作。如果是 2022 年前后及以后的显卡，会感觉较慢，因为开源驱动无法提高显卡的速度。可以安装 NVIDIA 自己的驱动。

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = NVIDIA 显卡，使用开源驱动

# Small words under that line.
settings-display-nvidia-about = 可以使用；2022 年及以后的显卡会感觉较慢。

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = 应该怎么做

