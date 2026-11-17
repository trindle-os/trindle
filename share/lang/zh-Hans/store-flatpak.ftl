# machine translated (Simplified Chinese), not yet reviewed by a person.

# The page shown before a Flatpak program is installed, with what it will be able to reach.
# Headline over the page; the source of the program is given.
store-fp-headline-from = 从 { $host } 安装 { $name }？

# Headline over the page for a program that has been downloaded.
store-fp-headline = 安装 { $name }？

# Name of the card that lists what the program will be able to reach.
store-fp-card = 它能访问的内容

# Tooltip of the round button that leaves without installing.
store-fp-cancel = 不安装

# The main button.
store-fp-install = 安装

# Said while Flatpak is being asked what the program wants.
store-fp-reading = 正在读取它的请求…

# Red words when what the program asks for could not be read.
store-fp-unread = Trindle 无法读取它的请求。

# Red words after Install was pressed once for a program whose list could not be read.
store-fp-unread-again = 再按一次“安装”才会继续。

# Said when the program is already on this computer; the name is given.
store-fp-have = { $name } 已经安装。

# Said when the program asks to reach nothing outside its own box.
store-fp-none = 除了它自己的沙盒，什么都不访问。

# Each line below is one thing the program will be able to reach.
store-fp-network = 互联网和你的本地网络

store-fp-x11 = 其他应用的窗口，以及你在其中输入的内容

store-fp-sound = 声音和你的麦克风

store-fp-devices = 你的摄像头和其他所有已插入的设备

store-fp-input = 游戏手柄和其他输入设备

store-fp-kvm = 快速虚拟机

store-fp-shm = 与其他应用共享的内存

store-fp-usb = USB 设备

store-fp-canbus = CAN 总线设备

store-fp-cups = 你的打印机

store-fp-multiarch = 运行 32 位程序

store-fp-escape = 运行这台电脑上的任何程序

# A reference that also adds a second place to get apps from, named by its address.
store-fp-adds-source = 把 { $host } 加入你的应用来源

# Last line, when the program asks for more than the lines above say.
store-fp-other = 另外还有其他设置

store-fp-bluetooth = 蓝牙设备

store-fp-smartcard = 智能卡

store-fp-ssh = 你的 SSH 密钥

store-fp-gpg = 你的 GPG 密钥

store-fp-devel = 调试工具，能监视和控制它启动的程序

store-fp-session-bus = 你的会话中运行的每个程序

store-fp-system-bus = 这台电脑上的每个服务

# A place the program can open and change; the place is given, as below.
store-fp-files = 打开并修改{ $what }

# The same for a place it can only look at.
store-fp-files-read = 查看{ $what }

store-fp-where-host = 这台电脑上的所有文件

store-fp-where-system = 这台电脑的系统文件

store-fp-where-home = 你的整个主文件夹，包括隐藏文件

store-fp-where-path = 文件夹 { $path }

# A program on your session that it can talk to; the name is given.
store-fp-talk = 与 { $name } 通信

# A service the program runs on your session.
store-fp-own = 运行服务 { $name }

store-fp-talk-system = 与系统服务 { $name } 通信

store-fp-own-system = 运行系统服务 { $name }

# Said in red when Install failed and taking the program out again failed too.
store-fp-undo-failed = 它可能只安装了一部分。

# Said in red when a bundle could not be copied for a safe read.
store-fp-copy-failed = 无法复制该文件以进行安全读取。
