# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Settings page about appearance, in the list and as its heading.
settings-appearance-title = 外观

# Words a person might type into the Settings search to find the appearance page; separate them with spaces.
settings-appearance-keywords = 颜色 色彩 强调色 主题 外观 皮肤 玻璃 深色 浅色 模式 阳光 明亮 图标 字体 光标 指针 屏幕阅读器 盲人 朗读 无障碍 标签 提示 名称 指向 colour accent theme dark light font cursor

# Introduction under the heading of the appearance page, and the words behind the page i.
settings-appearance-help = 浅色或深色、颜色、图标、字体、指针，以及朗读屏幕。有些程序要重新打开后才会生效。

# SAFETY: needs a human review before it ships
# Question along the bottom of the page while a theme is on trial.
settings-appearance-foot-keep = 保留这个外观吗？

# SAFETY: needs a human review before it ships
# Small words beside the question along the bottom while a theme is on trial.
settings-appearance-foot-escape = 按 Escape 立即退回。

# Name of the dark look, on a theme's button and in the top sentence.
settings-appearance-look-dark = 深色

# Name of the light look, on a theme's button and in the top sentence.
settings-appearance-look-light = 浅色

# The dark look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-dark-lower = 深色

# The light look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-light-lower = 浅色

# Spoken name of a theme's Dark or Light button; { $name } is the theme and { $look } the look.
settings-appearance-theme-spoken = { $name }，{ $look }

# Spoken description of the small picture of a theme; { $name } is the theme and { $look } its look, such as dark.
settings-appearance-sample-spoken = { $name } 的外观，{ $look }

# Said aloud after Dark or Light was chosen; { $look } is dark or light.
settings-appearance-aloud-look = { $look }外观。

# Sentence at the top of the page when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-top-unusable = 使用 Trindle 自己的外观，因为无法使用 { $theme } 主题。{ $why }

# Sentence at the top of the page for a theme that has dark and light looks; { $theme } is the theme, { $look } dark or light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme-look = { $theme } 主题，{ $look }，高亮色为 { $accent }，图标为{ $shape }。

# Sentence at the top of the page for a theme with one look; { $theme } is the theme, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme = { $theme } 主题，高亮色为 { $accent }，图标为{ $shape }。

# Sentence at the top of the page when no theme is chosen; { $look } is Dark or Light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-look = { $look }，高亮色为 { $accent }，图标为{ $shape }。

# Shape of the icons in the top sentence.
settings-appearance-shape-round = 圆形

# Name of Trindle's own highlight colour, first in the list of colours.
settings-appearance-trindle-yellow = Trindle 黄

# Start of the words behind the i of the reading the screen card.
settings-appearance-reading-says = 会说出屏幕上有什么以及您输入了什么。同时按 Super、Alt 和 S 也可以将其打开或关闭。

# Red words under the icon shape line when the icons could not be remade.
settings-appearance-no-icons = 无法重新生成图标，所以有些仍保持旧的形状。请再选一次这套图标重试。

# Small words on a theme's tile when Trindle made some of its colours clearer.
settings-appearance-corrected = 有些颜色已调整得更清晰易读。

# SAFETY: needs a human review before it ships
# Said aloud when a theme is tried; { $theme } is its name and { $seconds } how long until the old look comes back.
settings-appearance-aloud-trying =
    { $seconds ->
       *[other] 正在试用 { $theme } 主题。除非您按“保留”，否则 { $seconds } 秒后会恢复原来的外观。按 Escape 立即退回。
    }

# Name of Trindle's own theme when none is named.
settings-appearance-trindle = Trindle

# Said aloud when a theme was kept; { $theme } is its name.
settings-appearance-aloud-kept = 已保留 { $theme } 主题。

# Red words under the Theme line when the old look could not be written back.
settings-appearance-back-failed = 时间到后会恢复原来的外观。

# Said aloud when the look from before is back.
settings-appearance-aloud-back = 已恢复原来的外观。

# Title of the window for choosing a theme file.
settings-appearance-chooser-title = 选择主题

# Name of the filter in that window that shows only themes.
settings-appearance-chooser-filter = 主题

# Green words under the Add a theme line after a theme was added; { $name } is its name.
settings-appearance-added = { $name } 已在上面的列表中。点按它即可使用。

# Red words under the Theme line when a theme could not be removed.
settings-appearance-not-removed = 无法移除该主题。

# Red words under the theme tiles when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-cannot-use = 无法使用 { $theme } 主题。{ $why }

# Heading of the line saying a theme is tried first.
settings-appearance-tried-first = 先试用主题

# Small words under that line; { $seconds } is how long Keep is asked for.
settings-appearance-tried-first-about =
    { $seconds ->
       *[other] 窗口底部会询问是否“保留”，等待 { $seconds } 秒。
    }

# SAFETY: needs a human review before it ships
# Name of the bin button that removes one of the person's own themes.
settings-appearance-remove-theme = 移除这个主题

# Name of the button that chooses a theme; { $name } is the theme.
settings-appearance-use-theme = 使用 { $name }

# Small words on a theme's tile naming its author; { $author } is the name.
settings-appearance-by = 作者：{ $author }

# Name of the button that adds a theme, and heading of its line.
settings-appearance-add-theme = 添加主题

# Spoken name of the turning ring shown while themes are being found.
settings-appearance-looking = 正在查找主题

# Title of the Theme card.
settings-appearance-theme-card = 主题

# Words behind the i of the Theme card.
settings-appearance-theme-about = 主题是把整个外观放在一个文件里：颜色、圆角、大小和间距。点按一个即可试用：每个 Trindle 窗口会同时改变，除非您按“保留”，否则 30 秒后会自动恢复原来的外观，所以读不出来的主题不会留下。同时有深色和浅色外观的主题，会在图片旁边显示“深色”和“浅色”；浅色在明亮的阳光下更容易看清，“应用”里的应用会在下次打开时采用。只有一种外观的主题没有这些按钮。无论哪个主题，您的高亮色都保持您自己的选择。您添加的主题只会被读取，不会被运行，会让文字难以阅读的颜色会被调整得更清晰。在终端里运行 trindle-settings --theme default，随时都能恢复 Trindle 自己的外观。

# Heading of the line holding the theme tiles.
settings-appearance-theme-line = 主题

# Small words under that line.
settings-appearance-theme-line-about = 整个外观。同时有两种外观的主题，旁边有深色或浅色选择。

# Small words under the Add a theme line.
settings-appearance-add-theme-about = 一个 .css 文件，例如别人分享的那种。

# Words of the link that opens help about what a theme file holds.
settings-appearance-theme-file-help = 主题文件包含什么

# Red words under the colour box when what was typed is not a colour.
settings-appearance-hex-bad = 六位 0-9 和 a-f，例如 #fbc711。

# Spoken description of the small dot that shows the colour typed.
settings-appearance-dot = 输入的颜色

# Placeholder of the box for typing a colour code.
settings-appearance-hex-code = 十六进制代码

# Spoken name of the box for typing a colour as a hex code; the example is a code.
settings-appearance-hex-spoken = 任意颜色，用十六进制代码表示，例如 #fbc711

# Heading of the line for typing any colour.
settings-appearance-any-colour = 任意颜色

# Small words under that line.
settings-appearance-any-colour-about = 六位或八位数字，例如 #fbc711。

# Title of the Colour card.
settings-appearance-colour-title = 颜色

# Words behind the i of the Colour card.
settings-appearance-colour-about = 高亮色标出按钮、开关、填充圆环以及您所在的位置。Trindle 自己的窗口会立即改变；圆环在下次打开时改变。在卡片上太暗而看不清的颜色会被调亮到能够阅读，在浅色外观中太淡的颜色会被调暗。深色或浅色在“主题”下，直接选择某个主题自己的图块。

# Heading of the line with the highlight colour swatches.
settings-appearance-highlight = 高亮色

# Small words under that line.
settings-appearance-highlight-about = 按钮、开关以及您所在的位置。

# Small words under the read the screen aloud switch when a screen reader is installed.
settings-appearance-reader-on = 同时按 Super、Alt 和 S 也可以将其打开或关闭。

# Heading of the line, and name of its switch, for reading the screen aloud.
settings-appearance-reader = 朗读屏幕

# Heading of the line, and name of its switch, for keeping labels while pointing.
settings-appearance-labels = 指向时保留标签

# Small words under that line.
settings-appearance-labels-about = 指向按钮时，它的名称会一直显示。

# Title of the Reading the screen card.
settings-appearance-reading-title = 朗读屏幕

# Words behind the i of that card; { $says } is the first sentences, from the key reading-says.
settings-appearance-reading-about = { $says } 按钮的名称通常在一秒半后消失；选择保留后，只要指针停在上面它就一直显示。屏幕朗读开启时，每个按钮下面都会显示它的名称。

# Name of the set of round icons.
settings-appearance-icons-circles = 圆形

# Words beside the turning ring while the icons are remade.
settings-appearance-icons-working = 正在更换图标

# Green words under the shape line after the icons were remade.
settings-appearance-icons-changed = 已更换

# Name of the button that chooses an icon set; { $name } is the set, such as Circles.
settings-appearance-icons-set = { $name }图标

# Heading of the line for the shape of the icons.
settings-appearance-shape = 形状

# Small words under that line.
settings-appearance-shape-about = Trindle 图标，圆形或方形。

# Title of the Icons card.
settings-appearance-icons-title = 图标

# Words behind the i of the Icons card.
settings-appearance-icons-about = Trindle 图标有圆形和方形两种。它们没有绘制的应用，会生成一个相配的图标；圆环和面板会按新的形状重新绘制。

# Name of the list of font families, and heading of the font line.
settings-appearance-font = 字体

# Name of the font line's choice before the font in use has been read, so a screen reader has a word to say.
settings-appearance-font-default = 默认

# Name of the list of font sizes.
settings-appearance-font-size = 字体大小

# Small words under the font line.
settings-appearance-font-about = 程序窗口里的文字。

# Name of the button that adds fonts, and heading of its line.
settings-appearance-add-fonts = 添加字体

# Small words under the Add fonts line.
settings-appearance-add-fonts-about = 一个 .ttf 或 .otf 文件，或它们的 zip 压缩包。

# Words of the link that opens help about what a font file must contain.
settings-appearance-fonts-help = 文件必须包含什么

# Title of the Text card.
settings-appearance-text-title = 文字

# Words behind the i of the Text card.
settings-appearance-text-about = 程序用来显示文字的字体及其大小。在这里添加的字体（.ttf 或 .otf 文件，或它们的 zip 压缩包）只会加入您自己的字体列表；有些程序要重新打开后才会生效。

# Title of the window for choosing font files.
settings-appearance-chooser-fonts = 选择字体

# Red words when a font could not be replaced; { $name } is the file name.
settings-appearance-font-not-replaced = 无法替换 { $name }。

# Green words after one font took the place of the one there; { $name } is the file name.
settings-appearance-font-replaced-one = 已替换 { $name }。之后启动的程序会使用它。

# Green words after several fonts took the place of the ones there; { $n } of { $total } were done.
settings-appearance-font-replaced-some = 已替换 { $total } 个中的 { $n } 个。之后启动的程序会使用它们。

# Green words after fonts were added; { $n } is how many.
settings-appearance-fonts-added =
    { $n ->
       *[other] 上面的列表中新增了 { $n } 个字体。请在上面选择。
    }

# Green words after added fonts were already in the list.
settings-appearance-fonts-in-list = 这些字体已经在列表中了。

# Choice in the list of cursor sizes: the normal size.
settings-appearance-cursor-normal = 标准

# Choice in the list of cursor sizes: a large size.
settings-appearance-cursor-large = 大

# Choice in the list of cursor sizes: the largest size.
settings-appearance-cursor-larger = 更大

# Spoken name of a picture of a cursor size; { $size } is its name, such as Large.
settings-appearance-pointer-spoken = { $size }指针

# Name of the standard cursor theme (the one every system has).
settings-appearance-cursor-standard = 标准

# Name of the list of cursor themes, and heading of its line.
settings-appearance-cursor = 光标

# Name of the list of cursor sizes, and heading of its line.
settings-appearance-cursor-size = 光标大小

# Small words under the cursor line.
settings-appearance-cursor-about = 指针的绘制方式。

# Small words under the cursor size line.
settings-appearance-cursor-size-about = 越大越容易找到。

# Title of the Pointer card.
settings-appearance-pointer-title = 指针
