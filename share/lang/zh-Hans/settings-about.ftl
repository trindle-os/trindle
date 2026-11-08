# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = 关于这台电脑

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = 主机名 电脑名称 网络名称 关于 系统 版本 debian trindle 内存 ram 处理器 cpu 磁盘 显卡 屏幕 型号 内核 滚动发布 活动 运行中的程序 繁忙 任务管理器 系统监视器 驱动器 usb 空间

# Text behind the info button of the About page.
settings-about-help = 这台电脑在网络上显示的名称、它由什么组成，以及启动要多久。它此刻在做什么、哪些程序在占用内存和处理器，可在“活动”中查看；驱动器及其空间可在“驱动器”中查看，它们各是一个单独的窗口：点“更多详情”里的箭头即可打开。

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
settings-about-name-bad = 只能用字母、数字和连字符，且不能以连字符开头或结尾。

# Words under the computer name box while the password window is open.
settings-about-name-waiting = 正在等待输入密码……

# Red words under the computer name box when renaming failed.
settings-about-name-failed = 无法重命名这台电脑。

# Words under the computer name box after renaming worked.
settings-about-name-done = 已打开的程序要到下次登录后才会看到新名称。

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = 电脑名称

# Tip of the save button beside the computer name box.
settings-about-rename = 重命名这台电脑

# Bold line of the computer name setting.
settings-about-name = 电脑名称

# Quiet line under the computer name setting.
settings-about-name-sub = 字母、数字和连字符。

# Heading of the card with the computer's name.
settings-about-name-group = 名称

# Text behind the info button of the Name card.
settings-about-name-about = 这台电脑在网络上以及在终端提示符处显示的样子。只能用字母、数字和连字符，且不能以连字符开头或结尾。更改时会要求输入您的密码。

# Heading of the card about the computer's hardware.
settings-about-hardware = 电脑

# Text behind the info button of the hardware card.
settings-about-hardware-about = 制造商和型号、处理器及其同时运行的线程数、显卡，以及正在使用的每块屏幕的尺寸和刷新率。

# Quiet line under the maker and model.
settings-about-hw-computer = 电脑

# Quiet line under the processor.
settings-about-hw-processor = 处理器

# Quiet line under the graphics.
settings-about-hw-graphics = 负责绘制画面

# Quiet line under the screens.
settings-about-hw-screen = 屏幕

# Heading of the card about the software.
settings-about-software = 软件

# Text behind the info button of the software card.
settings-about-software-about = Trindle 是建立在 Debian 之上的桌面，Debian 负责管理下面的程序。内核负责与硬件沟通，Trindle 自己的合成器在 Wayland 上绘制桌面。

# Quiet line under the Debian version.
settings-about-sw-debian = 在底层运行

# Quiet line under the kernel version.
settings-about-sw-kernel = 内核

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle 自己的合成器

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = 在 Wayland 上绘制桌面

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
       *[other] { $n } 个应用
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = 已安装，就在圆环里

# Heading of the card with the ways to Activity and Drives.
settings-about-more = 更多详情

# Text behind the info button of the More detail card.
settings-about-more-about = 它此刻在做什么、哪些程序在占用内存和处理器，可在“活动”中查看。每个驱动器、它有多满、状况如何，以及 U 盘，可在“驱动器”中查看。

# Bold line of the setting that opens Activity.
settings-about-activity = 查看正在运行的内容

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = 占用内存和处理器的程序。

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = 活动

# Bold line of the setting that opens Drives.
settings-about-drives = 驱动器和空间

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = 每个驱动器、它有多满、状况如何。

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = 驱动器

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = 无法把启动用时保存到“下载”。

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = 在“下载”中，文件名为 { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = 保存

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = 把每次启动用时保存到“下载”

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = 每一次启动，留作记录

# Quiet line under that setting.
settings-about-times-keep-sub = 以电子表格保存到“下载”。

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = 每一列是一次启动，最新的在右边。

# Heading of the card with the last ten start-up times.
settings-about-times = 最近十次启动

# Text behind the info button of the start-up times card.
settings-about-times-about = 每次启动都从按下电源键计时到登录界面，输入磁盘密码所花的时间不计在内。最新的在最上面。

# Heading of the memory tile.
settings-about-memory = 内存

# Heading of the disk tile.
settings-about-disk = 磁盘


# Heading of the card with the switch for looking for updates.
settings-about-updates = 更新

# Text behind the info button of the updates card.
settings-about-updates-about = 登录后不久以及之后每隔几个小时，Trindle 会查找应用、插件、固件和自身的新版本，Debian 的安全修复则每天安装。关闭后，Trindle 只有在你于“应用”中点击“检查更新”，或点击圆环上的更新按钮时才会查找。Debian 的安全修复有自己的开关；关闭后，它们不再自动安装，已知的故障会一直存在，直到你更新。更改任一开关都需要管理员密码，这样计算机自己的服务也能知道这个选择。

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = 检查更新

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = 正在等待密码...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = 关闭后，除非你要求，否则不会查找。



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = 安全更新

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = 安全修复每天自动安装。

# Warning line under the Security updates switch when it is off.
settings-about-security-off = 安全修复不再自动安装。

