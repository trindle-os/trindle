# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the screenshot overlay that covers the screen (what a screen reader calls it).
screenshot-name = 截图

# Note on the overlay before any part is chosen.
screenshot-drag = 拖动选出要保留的部分

# Name of the round button that keeps the whole screen.
screenshot-whole = 整个屏幕

# Name of the round button that closes the overlay or a pinned picture.
screenshot-close = 关闭

# SAFETY: needs a human review before it ships
# Name of the round button that gives the screenshot up.
screenshot-cancel = 取消

# Name of the round button that takes back the last mark.
screenshot-undo = 撤销

# Name of the round button that puts back the last mark taken away.
screenshot-redo = 重做

# Name of the round button that copies the picture to the clipboard, and of the copy button of a colour.
screenshot-copy = 复制

# Name of a colour's button after it has been copied.
screenshot-copied = 已复制

# Word on the button that saves the picture in the Screenshots folder.
screenshot-save = 保存

# Word on the button that asks where to save the picture.
screenshot-save-as = 另存为

# Name of the round button that keeps the picture on the screen as a small window.
screenshot-pin = 固定在屏幕上

# Name of the drawing tool that draws freehand.
screenshot-tool-pen = 画笔

# Name of the drawing tool that draws a straight line.
screenshot-tool-line = 直线

# Name of the drawing tool that draws an arrow.
screenshot-tool-arrow = 箭头

# Name of the drawing tool that draws a box.
screenshot-tool-box = 方框

# Name of the drawing tool that draws a circle.
screenshot-tool-circle = 圆圈

# Name of the drawing tool that highlights.
screenshot-tool-highlighter = 荧光笔

# Name of the drawing tool that writes words.
screenshot-tool-words = 文字

# Name of the drawing tool that puts numbered spots.
screenshot-tool-numbers = 编号

# Name of the drawing tool that blurs a patch.
screenshot-tool-blur = 模糊

# Name of the round button that draws in red.
screenshot-colour-red = 红色

# Name of the round button that draws in blue.
screenshot-colour-blue = 蓝色

# Name of the round button that draws in green.
screenshot-colour-green = 绿色

# Name of the round button that draws in orange.
screenshot-colour-orange = 橙色

# Name of the round button that draws in purple.
screenshot-colour-purple = 紫色

# Name of the round button that draws in white.
screenshot-colour-white = 白色

# Name of the round button that draws in black.
screenshot-colour-black = 黑色

# Name of the round button that draws thin lines.
screenshot-thickness-thin = 细

# Name of the round button that draws lines of middle thickness.
screenshot-thickness-medium = 中

# Name of the round button that draws thick lines.
screenshot-thickness-thick = 粗

# Name the screen reader gives to a screenshot pinned on the screen.
screenshot-pinned = 已固定的截图

# Title of the window that asks where to save the screenshot.
screenshot-save-title = 保存截图

# Title of the notice that says the screenshot could not be saved.
screenshot-not-saved = 截图未保存

# Title of the notice that says the screenshot could not be copied.
screenshot-not-copied = 截图未复制

# Words of that notice.
screenshot-clipboard-refused = 剪贴板无法接收。

# Words of the notice that the screenshot could not be saved: the picture was not made.
screenshot-no-picture = 无法生成图片

# Words of the notice that the screenshot could not be saved: no bus to ask on.
screenshot-no-bus = 没有可供询问的会话总线

# Words of the notice that the screenshot could not be saved: the file chooser could not be asked.
screenshot-no-chooser = 无法询问文件选择器

# Words of the notice that the screenshot could not be saved: every file name was taken.
screenshot-no-free-name = 没有可用的名称

# Words of the notice that the screenshot could not be saved: the path and why it failed.
screenshot-write-failed = { $path }：{ $why }

# Name of the eye dropper's window, and its title.
screenshot-eye-title = 取色器

# Words that float beside the i of the eye dropper's window.
screenshot-eye-help = 按下取色器，然后点击屏幕上的任意位置，即可取得指针下的颜色。按 Escape 放弃。每种颜色会立即以十六进制代码复制，并保存在下面的列表中，最新的在前，以后可以再次复制。

# Name of the i button beside the title.
screenshot-eye-about = 关于取色器

# Name of the main button that takes a colour.
screenshot-eye-take = 取色

# Sentence under the title while no colour has been taken.
screenshot-eye-none = 从屏幕上的任何位置取一种颜色。

# Sentence under the title once colours have been taken.
screenshot-eye-taken =
    { $n ->
       *[other] 已取 { $n } 种颜色；最新的是 { $newest }。
    }

# Shown in the big card while no colour has been taken.
screenshot-eye-latest-empty = 您取的颜色会大大地显示在这里，随时可以复制。

# Name of the card that holds the latest colour.
screenshot-eye-latest = 最新的颜色

# SAFETY: needs a human review before it ships
# Name of the round button that clears the list of colours.
screenshot-eye-clear = 清空列表

# Head of the card with the list of colours taken.
screenshot-eye-list = 已取的颜色

# Shown in the list card while no colour has been taken.
screenshot-eye-list-empty = 用取色器取的颜色保存在这里，最新的在前。

# SAFETY: needs a human review before it ships
# Name of the round button that takes a colour out of the list.
screenshot-eye-remove = 移除

# What the saved list of colours is called when it is found damaged and set aside.
screenshot-eye-damaged = 取色器颜色

# SAFETY: needs a human review before it ships
# Title of the window that asks what an app may share.
screenshot-pick-title = 要与 { $app } 共享您的屏幕吗？

# SAFETY: needs a human review before it ships
# Sentence under the title when several things can be picked.
screenshot-pick-many = 请选择一项或多项。只有您选的内容会被共享，直到应用停止或关闭。

# SAFETY: needs a human review before it ships
# Sentence under the title when one thing can be picked.
screenshot-pick-one = 请选择一项。只有您选的内容会被共享，直到应用停止或关闭。

# Heading over the screens that can be shared.
screenshot-pick-screens = 屏幕

# Heading over the windows that can be shared.
screenshot-pick-windows = 窗口

# Shown when there is nothing to share.
screenshot-pick-nothing = 没有可共享的内容。

# SAFETY: needs a human review before it ships
# Name of the choice that shares every screen at once.
screenshot-pick-desktop = 整个桌面

# Words under that choice.
screenshot-pick-desktop-detail = 所有屏幕，按实际摆放

# Words under a screen's choice: its size in pixels.
screenshot-pick-size = { $width } × { $height }

# What the screen reader says for a choice that is picked.
screenshot-pick-chosen = { $name }，已选择

# SAFETY: needs a human review before it ships
# Name of the round button that refuses to share.
screenshot-pick-refuse = 不共享

# SAFETY: needs a human review before it ships
# Name of the main button that shares what is picked.
screenshot-pick-share = 共享

# SAFETY: needs a human review before it ships
# Name of the button that says no in a question from a program, and of its answer to the program.
screenshot-portal-deny = 拒绝

# SAFETY: needs a human review before it ships
# Name of the main button that says yes in a question from a program.
screenshot-portal-allow = 允许

# SAFETY: needs a human review before it ships
# Name of the round button that gives up a window from a program.
screenshot-portal-cancel = 取消

# Name of the main button that opens, and the title of a window that opens a file.
screenshot-portal-open = 打开

# Title of the window that saves a file, and the word of its main button.
screenshot-portal-save = 保存

# Name of the box where a name is typed.
screenshot-portal-name = 名称

# SAFETY: needs a human review before it ships
# Heading when an unknown program asks who the person is.
screenshot-account-asks-unknown = 有程序想知道您是谁

# SAFETY: needs a human review before it ships
# Heading when a program asks who the person is.
screenshot-account-asks = { $who } 想知道您是谁

# Name the screen reader gives to the person's picture.
screenshot-account-picture = 您的头像

# Words under the person's name.
screenshot-account-signed-in = 已登录为 { $id }

# Name of the search box in the window that asks which app should open a file.
screenshot-apps-search = 搜索应用

# Shown when no app can open the file.
screenshot-apps-none = 这里没有能打开它的应用

# Name of the switch that remembers the choice.
screenshot-apps-always = 以后这类文件总是用这个应用打开

# Title of the window that asks which app should open a file.
screenshot-apps-open-with = 用以下应用打开 { $name }

# Title of that window when the file's name is not known.
screenshot-apps-choose = 选择应用

# Name of the screen sharing that other programs see, and the name Trindle gives itself to PipeWire.
screenshot-cast-name = Trindle 屏幕共享

# What other programs show for the shared screen.
screenshot-cast-node = 与应用共享的屏幕

# Name used for a program that cannot be named, in the window that asks what to share.
screenshot-cast-an-app = 某个应用

# SAFETY: needs a human review before it ships
# Title of the window that asks what to share.
screenshot-cast-window = 共享屏幕

# Title of the window that saves several files into a folder.
screenshot-files-save-many = 保存到文件夹

# Title of the window that chooses a folder.
screenshot-files-folder = 选择文件夹

# What a program holding the computer awake is called when it gave no name.
screenshot-inhibit-who = 某个程序

# SAFETY: needs a human review before it ships
# Heading when a program wants to add an app to the person's apps and cannot be named.
screenshot-launcher-ask = 要把它添加到您的应用中吗？

# SAFETY: needs a human review before it ships
# Heading when a named program wants to add an app to the person's apps.
screenshot-launcher-asks = { $who } 想把它添加到您的应用中

# Name the screen reader gives to the app's icon.
screenshot-launcher-icon = 它的图标

# Words under the name, when the app opens a web address.
screenshot-launcher-opens = 打开 { $target }

# SAFETY: needs a human review before it ships
# Title of the window that asks to add an app.
screenshot-launcher-title = 添加应用

# SAFETY: needs a human review before it ships
# Name of the main button that adds the app.
screenshot-launcher-add = 添加

# SAFETY: needs a human review before it ships
# Heading when a program that cannot be named offers a wallpaper.
screenshot-wallpaper-ask = 要把这张图片设为壁纸吗？

# SAFETY: needs a human review before it ships
# Heading when a program offers a wallpaper.
screenshot-wallpaper-asks = { $who } 提供了一张新壁纸

# Name the screen reader gives to the picture offered.
screenshot-wallpaper-picture = 新壁纸

# SAFETY: needs a human review before it ships
# Name of the main button that sets the wallpaper.
screenshot-wallpaper-set = 设为壁纸

# Red line under the printer box when no printer is set up.
screenshot-print-no-printer = 没有设置打印机。请在“设置”的“打印机”中添加一台。

# Red line under the printer box when the printing system does not answer.
screenshot-print-not-answering = 打印服务没有响应。

# Red line under the printer box when printing is not installed.
screenshot-print-not-installed = 这台电脑没有安装打印功能。

# Red line under the printer box when the chosen printer is paused.
screenshot-print-paused = { $name } 已暂停。

# Label and name of the box where the printer is chosen.
screenshot-print-printer = 打印机

# Label and name of the choice of which pages are printed.
screenshot-print-pages = 页面

# The choice to print all the pages.
screenshot-print-all = 全部

# The choice to print a range of pages.
screenshot-print-range = 范围

# Label and name of the box for the number of copies.
screenshot-print-copies = 份数

# Label and name of the choice of printing on both sides.
screenshot-print-sides = 双面

# Label and name of the choice of colour, and the choice for colour.
screenshot-print-colour = 彩色

# The choice to print in black and white.
screenshot-print-mono = 黑白

# Label and name of the choice of paper size.
screenshot-print-paper = 纸张

# Label and name of the choice of how the page stands.
screenshot-print-page = 页面方向

# Name of the main button that prints, and the title when nothing else is known.
screenshot-print-go = 打印

# Title of the print window for a named document.
screenshot-print-doc = 打印 { $doc }

# Title of the print window when the program has no document name.
screenshot-print-from = 打印来自 { $who } 的内容

# Title of the notice that says a print did not go.
screenshot-print-failed = 无法打印

# Words of that notice when the choices made were not kept.
screenshot-print-lost = 您的选择没有保留。请重新打印。

# Line in the menu on a file or folder: change its name.
screenshot-chooser-rename = 重命名

# Line in the menu on a file or folder: cut it.
screenshot-chooser-cut = 剪切

# Line in the menu: paste what was cut or copied.
screenshot-chooser-paste = 粘贴

# Line in the menu and name of the round button: make a new folder.
screenshot-chooser-new-folder = 新建文件夹

# SAFETY: needs a human review before it ships
# Line in the menu: move to the Trash.
screenshot-chooser-trash = 移到回收站

# Name of the first place in the side bar, and of the only part of the path when Recent is shown.
screenshot-chooser-recent = 最近

# Name of the person's home folder in the side bar.
screenshot-chooser-home = 主文件夹

# Name the screen reader gives to the path along the top.
screenshot-chooser-folder = 文件夹

# Shown when a folder has nothing in it.
screenshot-chooser-empty = 这里什么也没有

# Name of the choice of which kind of file is shown.
screenshot-chooser-kind = 文件类型

# Said where a place on another computer is not answering.
screenshot-chooser-not-answering = 另一台电脑没有响应。

# What the saved list of last folders is called when it is found damaged and set aside.
screenshot-chooser-damaged = 选择器文件夹

# Words under the box when the folder a name points into is not there.
screenshot-chooser-no-folder = 没有那个文件夹。

# SAFETY: needs a human review before it ships
# Words under the box when saving would replace a file.
screenshot-chooser-replace-ask = { $name } 已经存在。按“替换”，或再按一次 Enter，即可用这个替换它。

# Words under the box when a key is pressed on Recent, which is not a folder.
screenshot-chooser-recent-not-folder = “最近”不是文件夹。请先打开一个文件夹。

# Words under the box when a key is pressed with nothing picked.
screenshot-chooser-nothing-picked = 没有选择任何内容。

# Words under the box while a file by that name is already there.
screenshot-chooser-there = { $name } 已经存在。

# Name of the round button that hides the hidden files.
screenshot-chooser-hide-hidden = 隐藏隐藏文件

# Name of the round button that shows the hidden files.
screenshot-chooser-show-hidden = 显示隐藏文件

# Words of the main button when the program gave none, to choose a folder.
screenshot-chooser-choose-folder = 选择文件夹

# SAFETY: needs a human review before it ships
# Head of the question when a folder of that name is already there.
screenshot-chooser-clash-folder = 已经有一个名为 { $name } 的文件夹

# SAFETY: needs a human review before it ships
# Head of the question when a file of that name is already there.
screenshot-chooser-clash-file = { $name } 已经存在

# Name of the round button that leaves a pasted thing out.
screenshot-chooser-skip = 跳过

# Name of the round button that keeps both under different names.
screenshot-chooser-both = 两者都保留

# SAFETY: needs a human review before it ships
# Word on the button that replaces what is there.
screenshot-chooser-replace = 替换

# SAFETY: needs a human review before it ships
# What the screen reader says for the button that replaces a folder.
screenshot-chooser-replace-spoken = 替换：原来的文件夹会被移到回收站

# SAFETY: needs a human review before it ships
# Name of the main button that merges two folders.
screenshot-chooser-merge = 合并

# Words in the question when folders are being pasted.
screenshot-chooser-rest-folders = 对每个已存在的文件夹都采用同样的答案。

# Words in the question when files are being pasted.
screenshot-chooser-rest-files = 对每个已存在的文件都采用同样的答案。

# Name of the switch that gives the same answer to the rest.
screenshot-chooser-same = 其余的都这样处理

# Words under the box after Cut.
screenshot-chooser-was-cut = 已剪切 { $what }。请把它粘贴到一个文件夹里。

# Words under the box after Copy.
screenshot-chooser-was-copied = 已复制 { $what }。请把它粘贴到一个文件夹里。

# Words under the box when Paste is pressed and there is nothing to paste.
screenshot-chooser-nothing-cut = 没有剪切或复制任何内容。

# Words under the box when Paste is pressed while a paste is still going.
screenshot-chooser-paste-busy = 仍在粘贴中。

# Red line after some items could not be moved.
screenshot-chooser-not-moved =
    { $n ->
       *[other] 有 { $n } 项无法移动：{ $names }。
    }

# Red line after some items could not be copied.
screenshot-chooser-not-copied =
    { $n ->
       *[other] 有 { $n } 项无法复制：{ $names }。
    }

# Red line after some items could not be put in the Trash.
screenshot-chooser-not-trashed =
    { $n ->
       *[other] 有 { $n } 项无法放入回收站：{ $names }。
    }

# Name of the box where a file or folder is renamed.
screenshot-chooser-new-name = { $old } 的新名称

# What several things are called in a line such as Cut.
screenshot-chooser-items =
    { $n ->
       *[other] { $n } 项
    }

# Title of the notice when an email could not be started.
screenshot-mail-failed = 无法启动电子邮件

# Why: no mail program is chosen or installed.
screenshot-mail-no-app = 没有设置邮件程序。请在“设置”的“默认应用”中选择一个。

# Why: a program asked to attach more files than allowed.
screenshot-mail-too-many = 要附加的文件太多。

# Why: a file, or all the files together, are too big.
screenshot-mail-too-big = 文件太大，无法附加。

# Why: a file could not be read as a plain file.
screenshot-mail-bad-file = 有一个文件无法附加。

# Why: an address was not a plain address.
screenshot-mail-bad-address = 有一个地址无效。

# Why: the subject was too long or had odd characters.
screenshot-mail-bad-subject = 主题无效。

# Why: the words were too long or had odd characters.
screenshot-mail-bad-words = 内容无效。

# Why: no private place to keep the files.
screenshot-mail-no-place = 无法安全地保存这些文件。

# Why: the mail program did not start.
screenshot-mail-not-started = 邮件程序没有启动。

# Line in the menu on a file or folder: delete it for good, after asking.
screenshot-chooser-delete = 永久删除

# Question before deleting for good.
screenshot-chooser-delete-ask = 永久删除？

# Line in the menu: copy the place of the thing as text.
screenshot-chooser-copy-location = 复制位置

# Line in the menu on a folder: pin it to Folders' side bar.
screenshot-chooser-pin = 固定到侧边栏

# Line in the menu on a folder pinned already.
screenshot-chooser-unpin = 取消固定

# Red line after a pin could not be kept.
screenshot-chooser-pin-not-kept = 无法保存固定。

# Red line after some items could not be deleted.
screenshot-chooser-not-deleted =
    { $n ->
       *[other] 有 { $n } 个项目无法删除：{ $names }。
    }

