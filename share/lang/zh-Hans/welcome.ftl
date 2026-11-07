
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = 欢迎使用 Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = 打开圆环

# Title of the third page, where the first apps are picked.
welcome-page-apps = 选择你的应用

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = 更多选择

# Title of the fifth page, about plugins.
welcome-page-plugins = 插件带来更多功能

# Title of the sixth page, about crash reports.
welcome-page-reports = 帮助改进 Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = 圆点

# Title of the last page, with a few things worth knowing.
welcome-page-know = 值得了解

# Sentence under the title of the first page.
welcome-sentence-hello = 你需要的一切都在一个圆圈里，也就是圆环。

# Sentence under the title of the second page.
welcome-sentence-ring = 双击屏幕的最边缘，或按 Super 键。

# Sentence under the title of the third page.
welcome-sentence-apps = 目前几乎什么都没有安装，请选择你想要的应用。

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = 多选几个，然后一键安装你选的所有应用。

# Sentence under the title of the fifth page.
welcome-sentence-plugins = 插件是来自“应用”的小型扩展，每个插件只做你允许的事。

# Sentence under the title of the sixth page.
welcome-sentence-reports = 出问题时，如果你愿意，可以发送一份报告。

# Sentence under the title of the dots page.
welcome-sentence-dots = 圆点表示有程序正在使用你的某样东西。

# Sentence under the title of the last page.
welcome-sentence-know = 让 Trindle 与众不同的几点。

# Top right of every page: where the person is in the tour.
welcome-step = 第 { $n } 步，共 { $total } 步

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }。第 { $n } 步，共 { $total } 步。{ $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
welcome-done-file-note = 欢迎页面已看过。它在“所有应用”中，名为“欢迎”；删除此文件后，下次登录时会再次显示。

# Name of the round button at the foot that closes the tour.
welcome-close = 关闭

# Name of the round button at the foot that goes to the page before.
welcome-back = 返回

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = 下一步

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = 安装所选应用

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = 开始使用 Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = 请连接互联网以安装应用。

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = 重试

# Foot while apps are going in: their names.
welcome-installing = 正在安装 { $names }。

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = 正在安装 { $names }，{ $left }。

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
       *[other] { $names } 已在圆环中。
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = 无法安装这些应用。{ $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak 拒绝了请求。

# Foot of the apps pages: the apps picked so far.
welcome-chosen = 已选择：{ $names }。

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }、{ $second }，以及另外 { $more } 个

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = 圆环

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = 里面有什么

# Name of the first thing the ring holds.
welcome-held-programs = 你的程序

# Line under it.
welcome-held-programs-line = 排在外圈。

# Name of the second thing the ring holds.
welcome-held-time = 时间和日期

# Line under it.
welcome-held-time-line = 位于中间。

# Name of the third thing the ring holds.
welcome-held-switches = 开关

# Line under it.
welcome-held-switches-line = 声音、屏幕、保持唤醒和电源。

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = 用指针

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = 用键盘

# Note under the picture of the Super key.
welcome-ring-super-note = Super 键上通常有 Windows 徽标。

# Head of a card of apps: web browsers.
welcome-kind-browser = 网页浏览器

# Head of a card of apps: email programs.
welcome-kind-email = 电子邮件

# Head of a card of apps: chat programs.
welcome-kind-chat = 聊天

# Head of a card of apps: office programs.
welcome-kind-office = 办公

# Head of a card of apps: music and video players.
welcome-kind-media = 音乐和视频

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = 图片和绘图

# Head of a card of apps: notes and writing.
welcome-kind-notes = 笔记和写作

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF 和文档

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = 目前 Flathub 上没有这些应用。

# Beside an app that has been installed.
welcome-app-installed = 已安装

# Beside an app that is going in now.
welcome-app-installing = 正在安装

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }，已选择

# Name of the round button on an app's row that picks it.
welcome-app-pick = 选择 { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = 在圆环中

# Head of the card that lists the plugins on offer.
welcome-plugins-card = 插件

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = 在“应用”的“插件”下。

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = 在“应用”中打开插件

# Said in the plugins card while there are none to show.
welcome-plugins-none = 可用的插件列在“应用”中。

# Head of the card on the crash reports page.
welcome-reports-card = 崩溃报告

# The one question about crash reports.
welcome-reports-question = 出问题时发送报告，帮助改进 Trindle？你总会先看到报告内容。

# Note under the question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-note = 未经你点击，什么都不会发送。你会先看到完整的报告，其中不包含文件、密码或 Wi-Fi 名称。

# Second note under the question about crash reports.
welcome-reports-settings = 你随时可以在“设置”的“崩溃报告”中更改。

# The answer that turns crash reports off.
welcome-reports-no = 否

# The answer that turns crash reports on.
welcome-reports-yes = 是，先询问我

# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = 出问题时发送报告

# Head of a card on the last page.
welcome-know-boxes = 应用住在各自的隔离环境中

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-know-boxes-line = 应用在访问你的文件、摄像头或麦克风之前会先询问。

# Head of a card on the last page.
welcome-know-updates = 安全更新会自动安装

# Line in that card.
welcome-know-updates-line = 每天自动安装，无需询问。其他更新会等你决定。

# Head of a card on the last page.
welcome-know-ring = “设置”和“应用”在圆环中

# Line in that card.
welcome-know-ring-line = 在圆环中间，时间的上方。

# Head of a card on the last page.
welcome-know-help = 帮助在“设置”中

# Line in that card.
welcome-know-help-line = 这个导览在“所有应用”中，名为“欢迎”，需要时可随时再看。

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = 一块小屏幕，上面只有中间的圆环：圆环中央是时间，周围是收藏的应用。

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = 小屏幕上打开的圆环，中间时间的下方并排放着几个插件的图标，插件安装后它的图标就位于这里。

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = 两幅小图，展示圆点出现的位置：环中窗口的标签上，以及屏幕顶部中间。

welcome-dots-where-tab = 窗口的标签上

welcome-dots-where-corner = 屏幕顶部中间

welcome-picture-ring = 一块小屏幕。指针移到最上边缘并双击，圆环在中间打开。

# What the screen reader says for the picture of the Super key.
welcome-picture-super = 按下 Super 键，圆环随之打开。


# Title of the first page, which asks how much the person knows.
welcome-page-level = 您了解多少？

# Sentence under that title.
welcome-sentence-level = 请选最接近的一项，导览会略过您已经知道的内容。

# Name of the answer for someone who has not used Linux.
welcome-level-new = 没用过 Linux

# Line in that answer's card.
welcome-level-new-line = 每一页都看，包括所有 Linux 桌面都有的功能。

# Name of the answer for someone who has used Linux.
welcome-level-linux = 用过 Linux

# Line in that answer's card.
welcome-level-linux-line = 只看 Trindle 独有的内容，跳过 Linux 基础知识。

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = 用过 Trindle

# Line in that answer's card.
welcome-level-trindle-line = 立即关闭导览，不再显示。

# Title of the page about files and folders.
welcome-page-files = 您的文件

# Sentence under that title.
welcome-sentence-files = 您的所有内容都放在主文件夹里。

# Title of the page about getting software.
welcome-page-software = 获取软件

# Sentence under that title.
welcome-sentence-software = 应用来自“应用”，而不是您下载的安装文件。

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = 管理员密码

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = 对整台电脑的更改会先要求输入密码。

# Title of the page about snapshots.
welcome-page-snapshots = 快照

# Sentence under that title.
welcome-sentence-snapshots = Trindle 会保留系统副本，因此更新可以撤销。

# Title of the page about the person's own words.
welcome-page-words = 您自己的用词

# Sentence under that title.
welcome-sentence-words = 可以修改 Trindle 显示的任何词语，或添加您自己的语言。

# Head of a card on the files page.
welcome-files-home = 主文件夹

# Line in that card.
welcome-files-home-line = 文档、下载和图片都在其中。“文件夹”会从这里打开。

# Head of a card on the files page.
welcome-files-drives = 没有盘符

# Line in that card.
welcome-files-drives-line = U 盘或其他磁盘插入后会出现在“文件夹”中。

# Head of a card on the files page.
welcome-files-hidden = 隐藏的名称

# Line in that card.
welcome-files-hidden-line = 以点开头的名称（例如 .config）是隐藏的，里面保存着应用的设置。

# Head of a card on the files page.
welcome-files-bin = 回收站

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = 删除的文件会留在回收站里，直到您清空它。

# Head of a card on the software page.
welcome-software-apps = 应用集中在一处

# Line in that card.
welcome-software-apps-line = 从圆环打开“应用”，即可查找、安装和移除程序。

# Head of a card on the software page.
welcome-software-exe = 没有安装文件

# Line in that card.
welcome-software-exe-line = Windows 安装文件（以 .exe 结尾）在这里无法运行。请到“应用”中查找该应用。

# Head of a card on the software page.
welcome-software-updates = 更新集中在一处

# Line in that card.
welcome-software-updates-line = “应用”和“设置”会让一切保持最新，因此任何应用都不需要自带更新程序。

# Head of a card on the software page.
welcome-software-remove = 移除很干净

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = 移除应用会把它完全清除，不留任何残余。

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-why = 为什么会询问

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = 为所有人安装软件或更改磁盘会影响每位用户，所以电脑会先询问。

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-who = 谁的密码

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = 管理员的密码，通常就是您登录时用的那个。

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-not = 什么时候不会询问

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = 只为您自己安装的应用和您自己的文件从不需要密码。

# Head of a card on the snapshots page.
welcome-snap-taken = 自动为您创建

# Line in that card.
welcome-snap-taken-line = 在 Btrfs 系统上，每次软件更改前后都会创建一个快照。

# Head of a card on the snapshots page.
welcome-snap-list = 在“设置”中列出

# Line in that card.
welcome-snap-list-line = “设置”中的“快照”会列出它们。相机图标可立即创建一个。

# Head of a card on the snapshots page.
welcome-snap-boot = 无法启动时使用

# Line in that card.
welcome-snap-boot-line = 在启动菜单中选择 Trindle 快照，再选择一个快照。

# Head of a card on the your own words page.
welcome-words-change = 修改词语

# Line in that card.
welcome-words-change-line = 找到 Trindle 显示的任何词语，输入您自己的说法。例如可以用“打开它，brew”代替“点击此处打开文件”。

# Head of a card on the your own words page.
welcome-words-share = 分享词语表

# Line in that card.
welcome-words-share-line = 把您的用词导出为文件，或导入朋友发来的文件。使用前会先检查。

# Head of a card on the your own words page.
welcome-words-all = 所有程序一次生效

# Line in that card.
welcome-words-all-line = Trindle 的所有程序都从同一处读取用词，所以改一次，处处生效。

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = 位置在“设置”的“区域和语言”中。“帮助”里有分步说明。

# Name of the round button that opens that Settings page.
welcome-words-open = 打开“区域和语言”
