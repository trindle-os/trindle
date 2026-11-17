# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = 关于这台电脑

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = 主机名 电脑名称 网络名称 关于 系统 版本 debian trindle 内存 ram 处理器 cpu 磁盘 显卡 屏幕 型号 内核 滚动发布 活动 运行中的程序 繁忙 任务管理器 系统监视器 驱动器 usb 空间

# Text behind the info button of the About page.
settings-about-help = 这一页显示这台电脑在网络上被其他电脑看到的名称、它由什么组成，以及启动要多久。“活动”显示电脑此刻在做什么，以及哪些程序在占用内存和处理器。“驱动器”显示你的驱动器及其空间。它们各自在单独的窗口中打开。点“更多详情”里的箭头即可打开。

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = 未知

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
       *[other] { $model }，{ $cores } 个线程
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel 显卡

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD 显卡

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA 显卡

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = 虚拟机显卡

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = 显卡

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height }，{ $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = 已用 { $amount }

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = 可用 { $amount }

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = 共 { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }，已开机 { $on }。

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }，已开机 { $on }。磁盘快满了。

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } 秒

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } 分钟

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
       *[other] { $n } 分钟
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
       *[other] { $n } 小时
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
       *[other] { $n } 天
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = 只能使用字母、数字和连字符。不要以连字符开头或结尾。

# Words under the computer name box while the password window is open.
settings-about-name-waiting = 正在等待你输入密码……

# Red words under the computer name box when renaming failed.
settings-about-name-failed = 无法重命名这台电脑。

# Words under the computer name box after renaming worked.
settings-about-name-done = 已打开的程序会在你下次登录后使用新名称。

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = 电脑名称

# Tip of the save button beside the computer name box.
settings-about-rename = 重命名这台电脑

# Bold line of the computer name setting.
settings-about-name = 电脑名称

# Heading of the card with the computer's name.
settings-about-name-group = 名称

# Text behind the info button of the Name card.
settings-about-name-about = 这是这台电脑在网络上以及在终端提示符处的名称。只能使用字母、数字和连字符，且不能以连字符开头或结尾。更改时需要输入你的密码。

# Heading of the card about the computer's hardware.
settings-about-hardware = 电脑

# Text behind the info button of the hardware card.
settings-about-hardware-about = 这里显示电脑的制造商和型号、处理器和显卡，还有处理器可以同时运行的线程数，以及正在使用的每块屏幕的尺寸和刷新率。

# Heading of the card about the software.
settings-about-software = 软件

# Text behind the info button of the software card.
settings-about-software-about = Trindle 是运行在 Debian 之上的桌面，Debian 负责管理下面的程序。内核负责与硬件打交道。Trindle 自己的合成器使用 Wayland 绘制桌面。

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle 自己的合成器

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
       *[other] { $n } 个应用
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = 更多详情

# Text behind the info button of the More detail card.
settings-about-more-about = “活动”显示电脑此刻在做什么，以及哪些程序在占用内存和处理器。“驱动器”显示每个驱动器、它有多满、状况如何，以及所有 U 盘。

# Bold line of the setting that opens Activity.
settings-about-activity = 查看正在运行的内容

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = 活动

# Bold line of the setting that opens Drives.
settings-about-drives = 驱动器和空间

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = 驱动器

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle 无法把启动用时保存到“下载”。

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = 日期和时间,秒数

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle 启动时间.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = 在“下载”中，文件名为 { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = 保存

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = 把每次启动用时保存到“下载”

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = 保存每次启动用时

# Quiet line under that setting.
settings-about-times-keep-sub = 以电子表格保存到“下载”。

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = 每一列是一次启动。最新的在右边。

# Heading of the card with the last ten start-up times.
settings-about-times = 最近十次启动

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle 从按下电源键开始计时，到登录界面为止，记录每次启动。输入磁盘密码所花的时间不计在内。最新的在最上面。

# Heading of the memory tile.
settings-about-memory = 内存

# Heading of the disk tile.
settings-about-disk = 磁盘

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = 问题报告

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = 报告会发送到 Buildmorphic，这是帮助构建 Trindle 的社区。只需一个电子邮件地址就能加入。你第一个报告的问题，即使你无法修复，也能获得积分和等级。报告只包含一份固定的内容清单，不按“发送”就不会发送任何内容。用户名或 GitHub 名称是可选的，用户名绝不会是你的电子邮件地址。

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = 出问题时询问是否发送报告

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic 用户名

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub 名称

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = 关于社区

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
       *[other] 已关闭。{ $n } 份等待发送的报告已删除。
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
       *[other] 已关闭。{ $n } 份等待发送的报告无法删除。
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = 更新

# Text behind the info button of the updates card.
settings-about-updates-about = 登录后不久，Trindle 会检查应用、插件、固件和自身的新版本，之后每隔几个小时再检查一次。如果你关闭它，Trindle 只会在你于“应用”中点击“检查更新”，或点击圆环上的更新按钮时才检查。Debian 的安全修复每天安装，并有自己的开关。如果你关闭那个开关，它们将不再自动安装，已知的安全漏洞会一直不修复，直到你更新。更改任一开关都需要管理员密码，这样整台电脑都会采用你的选择。

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = 检查更新

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = 正在等待你输入密码……


# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = 安全更新

# Warning line under the Security updates switch when it is off.
settings-about-security-off = 安全修复不再自动安装。

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = 已关闭，但 Reporting 附加组件中等待发送的报告没有被删除。
