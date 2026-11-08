# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = 安装 { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = 欢迎

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = 磁盘

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = 隐私

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = 密码

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = 您

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = 就绪

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = 正在安装

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = 完成

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = 正在准备

# Red line at the foot when the installer's helper could not be started.
installer-no-start = 安装程序无法启动：{ $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = 无法读取安装计划。

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = 无法读取安装计划：{ $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = 安装已停止，之后没有再写入任何内容。

# Name of the round button at the foot that goes back a page.
installer-back = 返回

# Name of the main button at the foot that goes to the next page.
installer-next = 下一步

# SAFETY: needs a human review before it ships
# Name of the main button at the foot of the Ready page that starts the install.
installer-install = 安装

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = 立即重启

# Beside the keyboard test box on the first page.
installer-try-keyboard = 试试键盘

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = 在此输入以试试键盘

# SAFETY: needs a human review before it ships
# Note beside Install after one press, when the second press writes beside another system.
installer-arm = 再点一次即可安装

# SAFETY: needs a human review before it ships
# Note beside Install after one press, when the second press erases a whole disk.
installer-arm-erase = 再点一次即可清除整个磁盘并安装

# Note beside Next on the first page while no language is chosen.
installer-missing-language = 请选择语言

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = 请选择键盘

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = 请选择时区

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = 请选择安装 Trindle 的位置

# SAFETY: needs a human review before it ships
# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = 请输入两次磁盘密码，两次必须相同

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = 需要填写登录名

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = 请给这台电脑起个名字

# SAFETY: needs a human review before it ships
# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = 请输入两次密码，两次必须相同

# Title of the first page.
installer-welcome-title = 欢迎使用 { $name }

# Sentence under the title of the first page.
installer-welcome-help = 请选择语言；键盘和时间会随之设置，您也可以分别修改。

# Head of the card for choosing the language, and the name of its search box.
installer-language = 语言

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = 键盘

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = 时区

# Under a card's head while nothing has been chosen in its list.
installer-select = 选择

# Title of the disk page.
installer-disks-title = { $name } 的安装位置

# SAFETY: needs a human review before it ships
# Sentence under the title of the disk page.
installer-disks-help = 您所选磁盘上的全部内容都会被替换，除非您在现有内容旁边腾出空间。

# Shown on the disk page when no disk is big enough.
installer-disks-none = 没有找到足够大的磁盘。Trindle 需要 20 GB。

# SAFETY: needs a human review before it ships
# Name and words of the choice to erase a whole disk.
installer-disks-whole = 使用整个磁盘，替换其上的所有内容

# SAFETY: needs a human review before it ships
# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = 保留 { $what }，与其共用磁盘

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = 给现有内容留 { $kept }，给 Trindle { $mine }

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }，{ $size }

# Name of a partition with nothing on it.
installer-empty-space = 空闲空间

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest }和{ $last }

# Under a disk's name: what is on it.
installer-disk-has = 上面有 { $list }

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = 上面有 { $list }，已加密

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = 上面有一个已加密的系统

# Under a disk's name: there is nothing on it.
installer-disk-empty = 空

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = 上面有文件，但没有 Trindle 能识别的系统

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = { $gb } GB 的交换文件

# Name of the switch that makes a swap file.
installer-swap-switch = 创建交换文件

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = 安全启动已开启，所以这台电脑无法休眠，但内存不足时交换文件仍然有用。只有磁盘空间充裕时才会创建。

# Under the swap file's head when the computer can hibernate.
installer-swap-note = 可以休眠，即断电后保持所有打开的内容，内存不足时也有地方可用。没有它，电脑就无法休眠。只有磁盘空间充裕时才会创建。

# Title of the page about encrypting the disk.
installer-privacy-title = 加密磁盘

# Sentence under the title of that page.
installer-privacy-help = 加密后的磁盘，即使被人从这台电脑上取下，也无法被读取。

# Head of the card with the three choices.
installer-privacy-locked = 磁盘的锁定方式

# SAFETY: needs a human review before it ships
# Name and words of the choice not to encrypt the disk.
installer-privacy-none = 不加密磁盘

# SAFETY: needs a human review before it ships
# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = 加密，并在电脑启动时要求输入密码

# SAFETY: needs a human review before it ships
# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = 加密，并让这台电脑的 TPM 芯片来解锁

# Head of the card about the TPM chip.
installer-privacy-about = 关于芯片

# In the card about the chip, on a computer that has one.
installer-privacy-chip = 这台电脑有 TPM 芯片。安装后的第一次启动会要求输入一次密码，让芯片认识这台电脑；之后芯片会自动解锁磁盘，无需输入。请保管好这个密码：如果更换芯片，或把磁盘装到另一台电脑上，就要靠它才能打开。

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = 这台电脑没有 Trindle 能使用的 TPM 芯片，所以每次启动都会要求输入密码。在有 TPM 芯片的电脑上，芯片可以自动解锁磁盘。

# Title of the disk password page.
installer-disk-title = 磁盘密码

# Sentence under the title of the disk password page.
installer-disk-help = 电脑启动时，它会最先被输入。

# SAFETY: needs a human review before it ships
# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = 磁盘密码

# Name of the round button that makes up a passphrase.
installer-disk-suggest = 建议一个密码短语

# Label and name of the second box of a password, where it is typed again.
installer-same-again = 再输入一次

# SAFETY: needs a human review before it ships
# Red line under the password boxes when the two do not match.
installer-mismatch = 两次输入的密码不一致。

# Head of the card that tells the person to write the password down.
installer-disk-write-down = 请记下来

# SAFETY: needs a human review before it ships
# Words in that card.
installer-disk-write-down-text = 请把这个密码写在纸上，放在远离电脑的安全地方。没有它，磁盘就永远无法打开，无论是您还是其他任何人。没有任何办法可以重新进入。四个普通的词就能组成一个容易输入、又难以猜到的密码短语。

# Title of the account page.
installer-you-title = 关于您

# Sentence under the title of the account page.
installer-you-help = 这是您用来登录的账户。

# Head of the card with the name boxes.
installer-account = 您的账户

# Label and name of the box for the person's name.
installer-your-name = 您的姓名

# Label and name of the box for the sign-in name.
installer-user-name = 登录名

# Label and name of the box for the computer's name.
installer-host-name = 这台电脑的名称

# SAFETY: needs a human review before it ships
# Head of the card with the password boxes.
installer-your-password = 您的密码

# SAFETY: needs a human review before it ships
# Label and name of the first password box.
installer-password = 密码

# Label of the strength meter and the name a screen reader gives it.
installer-strength = 猜中的难度

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = 需要填写登录名。

# Red line under the account boxes: a name is too long.
installer-name-too-long = 这个名称太长。

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = 请以小写字母开头。

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = 只能用小写字母、数字、- 和 _。

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = 系统已经在使用这个名称。

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = 请给这台电脑起个名字。

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = 只能用字母、数字和 -。

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = 电脑马上就能猜中它。

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
       *[other] 电脑大约 { $n } 分钟就能猜中它。
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
       *[other] 电脑大约 { $n } 小时就能猜中它。
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
       *[other] 电脑大约 { $n } 天就能猜中它。
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
       *[other] 电脑大约需要 { $n } 年才能猜中它。
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = 电脑要花比有记载的历史还长的时间才能猜中它。

# Title of the Ready page.
installer-ready-title = 准备安装

# Head of the card that lists what will happen.
installer-ready-nothing-changed = 目前还没有更改任何内容

# Head of the card about security updates.
installer-ready-security = 安全更新

# In the card about security updates.
installer-ready-security-text = { $name } 每天都会自动安装 Debian 的安全修复。此项无法关闭。

# SAFETY: needs a human review before it ships
# Sentence under the title when a whole disk will be replaced.
installer-ready-replaced = { $disk } 上的所有内容都将被替换。

# SAFETY: needs a human review before it ships
# Sentence under the title when the install goes beside what is there.
installer-ready-kept = { $part } 上的所有内容都会保留，但请先备份：缩小过程中如果断电，可能会丢失文件。

# Sentence under the title when no disk has been chosen.
installer-ready-none = 还没有选择磁盘。

# SAFETY: needs a human review before it ships
# Name of a row in the list: the whole disk will be used.
installer-ready-whole = 整个 { $disk }

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-whole-text = 其上的所有内容都会被 { $name } 替换。

# SAFETY: needs a human review before it ships
# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = 在 { $part } 旁边

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-beside-text = { $part } 会被缩小到 { $size }，{ $name } 会装进腾出的空间。

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = 没有磁盘

# Words of that row.
installer-ready-no-disk-text = 没有选择磁盘。

# SAFETY: needs a human review before it ships
# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = 未加密

# SAFETY: needs a human review before it ships
# Words of that row.
installer-ready-plain-text = 任何人取下磁盘后都可以读取其中的内容。

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = 已加密

# Words of that row when a password is typed at each start.
installer-ready-locked-password = 每次电脑启动时，您都要输入它的密码。

# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = 先输入一次密码，之后由这台电脑的 TPM 芯片来解锁。

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = 键盘 { $keyboard }，时区 { $zone }。

# Title of the page shown while the install runs.
installer-installing-title = 正在安装 { $name }

# Head of the card that holds the short tour.
installer-tour = 简短导览

# Title of the last page.
installer-done-title = { $name } 已安装

# Sentence under the title of the last page.
installer-done-help = 请取出 U 盘，然后重启。

# SAFETY: needs a human review before it ships
# Head of the card that repeats the warning about the disk password.
installer-done-keep = 请妥善保管磁盘密码

# SAFETY: needs a human review before it ships
# In that card when the TPM chip unlocks the disk.
installer-keep-chip = 磁盘已加密。第一次启动会要求输入一次磁盘密码，让芯片认识这台电脑；之后它会自行打开。请把这个密码写在纸上，放在远离这台电脑的安全地方。没有它，磁盘就永远无法打开，无论是您还是其他任何人。

# SAFETY: needs a human review before it ships
# In that card when the password is typed at each start.
installer-keep-password = 磁盘已加密。如果还没有记下磁盘密码，请现在就把它写在纸上，放在远离这台电脑的安全地方。没有它，磁盘就永远无法打开，无论是您还是其他任何人。

# Title of the first slide.
installer-slide-ring = 一切尽在一个圆环

# Words of the first slide.
installer-slide-ring-text = 把指针移到屏幕边缘并双击，圆环就会打开：您的程序在外圈，时间和各种开关在中间。

# Title of the second slide.
installer-slide-snapshots = 更新出问题，可以退回

# Words of the second slide.
installer-slide-snapshots-text = 每次软件变更前后都会为系统拍一个快照。设置可以把整个系统恢复到其中任何一个；电脑无法启动时，也可以在启动菜单中恢复。

# Title of the third slide.
installer-slide-apps = 不碍事的应用

# Words of the third slide.
installer-slide-apps-text = Flathub 应用、Debian 自带的应用、AppImage 和软件源都在一处，备份还能带到另一台电脑上。

# Title of the fourth slide.
installer-slide-yours = 快速变成您的风格

# Words of the fourth slide.
installer-slide-yours-text = 浅色或深色、您自己的声音，以及记得您所复制内容的剪贴板。

# Title of the fifth slide.
installer-slide-small = 小巧安静

# Words of the fifth slide.
installer-slide-small-text = 桌面占用的内存很少，几秒钟就能启动，旧笔记本也会焕然一新。

# Title of the tour slide about the ring.
installer-tour-ring = 圆环就是整个桌面

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = 把指针放到这个角上，圆环就会打开

# Note on the ring picture, at the time.
installer-tour-ring-time = 中间是时间和日期

# Note on the ring picture, at the programs.
installer-tour-ring-programs = 您的程序在外圈

# Note on the ring picture, at the switches.
installer-tour-ring-switches = 声音、屏幕、所有应用、保持唤醒和关机

# Title of the tour slide about Settings.
installer-tour-settings = 设置，用平实的话说

# Note on the Settings picture, at the search box.
installer-tour-settings-search = 在此输入即可找到设置，不用到处翻找

# Note on the Settings picture, at Network.
installer-tour-settings-network = 在“网络”里连接 Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = 浅色还是深色，随时可换

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = 二十种颜色，也可以输入您自己的

# Title of the tour slide about Apps.
installer-tour-apps = 应用，满足其他一切需要

# Note on the Apps picture, at the search box.
installer-tour-apps-search = 按名称或用途搜索程序

# Note on the Apps picture, at the list.
installer-tour-apps-many = 来自 Flathub 和 Debian 的数千个免费程序

# Note on the Apps picture, at updates.
installer-tour-apps-updates = 所有软件的更新，包括 Trindle 本身

# Title of the tour slide about voice control.
installer-tour-voice = 您也可以直接对它说话

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = 语音控制在这里：设置，然后是圆环

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = 点一次“添加语音”。大约 150 MB，需要几分钟

# Note on the voice picture, about what to say.
installer-tour-voice-say = 然后点按圆环中间，说出应用的名称、“调大声音”或“Wi-Fi 设置”

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = 您说的话只在这台电脑上识别，不会发往别处

# Title of the tour slide about snapshots.
installer-tour-snapshots = 在任何更改之前，先留好退路

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = 每次更新前，电脑都会拍一个快照

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = 如果出了问题，可以在启动菜单中退回原来的样子

# Name of the picture of the ring, and the head of its card.
installer-live-caption = 圆环，一切所在之处

# Sentence under the title of the welcome from a stick.
installer-live-help = 您正在从 U 盘运行，所以这台电脑上的任何内容都没有被更改。

# Head of the card with the three first things to do.
installer-live-begin = 开始

# Name of the first thing to do.
installer-live-open = 打开圆环

# Line under it.
installer-live-open-line = 双击屏幕边缘，或按 Super 键。

# Name of the second thing to do.
installer-live-speak = 对它说话

# Line under it when voice control is there.
installer-live-speak-line = 点按圆环中间，说出您想要的。

# Line under it when voice control has to be added.
installer-live-speak-add = 在“设置”的“圆环”中添加语音，然后点按中间并说话。

# Name of the third thing to do.
installer-live-tab = 全都在圆环里

# Line under it.
installer-live-tab-line = 已打开的内容、通知和剪贴板都在这里。

# The choice of the dark look.
installer-live-dark = 深色

# The choice of the light look.
installer-live-light = 浅色

# Name the screen reader gives to the choice between dark and light.
installer-live-look = 整个桌面的浅色或深色

# Name of the round button that closes this window and opens the ring.
installer-live-around = 先随便看看

# Name of the main button that opens the installer.
installer-live-install = 安装 { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = 正在腾出空间

# Foot of the install page while the disk is prepared.
installer-step-prepare = 正在准备磁盘

# Foot of the install page while a place is made beside another system.
installer-step-place = 正在为 Trindle 腾出位置

# Foot of the install page while the disk is looked at.
installer-step-check = 正在检查磁盘上已有的内容

# Foot of the install page while another system is made smaller.
installer-step-smaller = 正在缩小另一个系统

# Foot of the install page while the disk is encrypted.
installer-step-lock = 正在锁定磁盘

# Foot of the install page while the TPM chip is set up.
installer-step-chip = 正在设置电脑的芯片

# Foot of the install page while the system is copied.
installer-step-copy = 正在把 Trindle 复制到磁盘

# Foot of the install page while settings are written.
installer-step-settings = 正在写入设置

# Foot of the install page while the person's account is made.
installer-step-account = 正在创建您的账户

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = 正在移除 U 盘专用的部分

# Foot of the install page while the start-up files are made.
installer-step-startup = 正在生成启动文件

# Foot of the install page while the boot menu is set up.
installer-step-boot = 正在设置启动菜单

# Foot of the install page while snapshots are set up.
installer-step-snapshots = 正在设置快照

# Foot of the install page at the very end.
installer-step-finishing = 正在收尾

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = { $program } 无法启动：{ $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } 未能完成。{ $why }

# Red line at the foot: a file could not be written.
installer-write-failed = 无法写入 { $path }：{ $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } 不是磁盘。

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } 不是这台电脑上的磁盘。

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = 没有选择磁盘。

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Trindle 无法使用这个登录名。

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Trindle 无法使用这个电脑名称。

# Red line at the foot: the language cannot be used.
installer-bad-language = Trindle 无法使用这种语言。

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Trindle 无法使用这个键盘。

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Trindle 无法使用这个时区。

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = 这个姓名中含有 Trindle 无法写入的内容。

# Red line at the foot: a password has a line break in it.
installer-bad-password = 密码中不能包含换行。

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = 安装程序的后半部分必须以 root 身份运行。

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = 这个磁盘没有可共用的启动分区，所以 Trindle 无法装在现有内容旁边。

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = 找不到磁盘上新建的分区。

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = 无法读取 { $part } 的大小。

# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle 无法缩小 { $fs } 分区。

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle 无法缩小未知类型的分区。

# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = 加密磁盘需要一个密码。

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = 无法复制系统：{ $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = 无法复制系统。{ $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = 没有空间创建交换文件。

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = 无法读取交换文件在磁盘上的位置。

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = 无法识别新建的分区。

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = 无法设置时区：{ $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = 无法在这台电脑上安装启动菜单。

# Name of that button after one press, when a second press goes ahead.
installer-sure = 确定吗？



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = 已打开的内容、剪贴板，以及一个清空屏幕的按钮

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } 是 Trindle 正在运行的那个 U 盘。

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } 正在使用。请关闭正在使用它的程序，然后重试。

# Said beside how far the install has got, once it has started.
installer-started = 安装已开始

# The button that ends an install under way.
installer-halt = 停止

# Beside Stop after its first press.
installer-halt-sure = 要停止安装吗？这块磁盘上原来的内容已经被清除了。

# While an install is being stopped.
installer-halting = 正在停止

# When Stop could not let go of the disk.
installer-halt-stuck = 无法释放这块磁盘。请重新启动电脑后再试。

# The copy is done and the person is still answering.
installer-step-waiting = 已复制。正在等待你的回答。



installer-already-running = 这台计算机上已经有另一个安装程序在运行。


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle 至少需要 1 GB 内存，而这台电脑的内存不足。

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = 只有通过 UEFI 启动的电脑才能加密磁盘，而这台电脑不是。

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = { $part } 上的 Windows 处于休眠状态或停留在快速启动状态，缩小它可能会丢失其中的文件。请启动 Windows，选择“重启”，然后再试一次。

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle 已安装，但无法设置快照：{ $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle 已安装，但无法让启动菜单列出快照：{ $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = 并非所有内容都已设置好

