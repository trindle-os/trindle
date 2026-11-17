
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = 驱动器

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = 关于此页面

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = 这台电脑内部和插入的每个驱动器，都画成一条由各个分区组成的横条。每个分区的宽度对应它的大小，填充的长度对应已用的空间，所以已满的分区一眼就能看出来。

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = 图下方，每个驱动器都有一张卡片。你可以打开分区、挂载或卸载分区、安全移除驱动器，或将驱动器断电。分区上的铅笔可以让你重命名、选择文件夹、设为开机时挂载、调整大小、检查、修复、格式化或删除它。空闲空间中的加号用来新建分区。

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = 你可以看到系统所在的驱动器，但不能对它做任何会擦除或移动内容的更改。对任何驱动器的每次更改，每次都会要求输入管理员密码。眼睛图标可以让驱动器或分区在 Folders 和圆环中隐藏。它仍会留在这里，显示为灰色，也没有被锁定。

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = 你的驱动器

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = 现在无法读取驱动器。

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = 重试

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = 重新查找

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = 没有找到驱动器。插入一个，它就会出现在这里。

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = 驱动器服务

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = 系统总线

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what }不可用。请用以下命令安装：sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = 正在查看驱动器…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } 正在损坏。请立即把想保留的文件复制到另一个驱动器。

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } 需要查看：{ $troubles }。

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] 你的驱动器及其上的空间。
       *[other] 你的驱动器及其上的空间，另有 { $plugged } 个驱动器已插入。
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] 这台电脑快满了，只剩 { $free } 可用。请尽快删除或移走一些文件。
       *[other] 这台电脑快满了，只剩 { $free } 可用。已插入 { $plugged } 个驱动器。请尽快删除或移走一些文件。
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] 这台电脑空间渐满，还有 { $free } 可用。
       *[other] 这台电脑空间渐满，还有 { $free } 可用。已插入 { $plugged } 个驱动器。
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] 这台电脑空间充足，还有 { $free } 可用。
       *[other] 这台电脑空间充足，还有 { $free } 可用。已插入 { $plugged } 个驱动器。
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = 这台电脑

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = 你的文件

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] 系统所在的磁盘还有 { $free } 可用。
       *[files] 存放你自己文件的位置还有 { $free } 可用。
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }：已用 { $full }，共 { $whole }，可用 { $free }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = 共 { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = 驱动器还没有报告任何健康读数

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = 状况良好，由驱动器自检得出

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
       *[other] 全部 { $n } 个驱动器都通过了自检
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
       *[other] 有 { $n } 个需要查看：请见其卡片
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
       *[other] { $n } 个驱动器，共 { $total }。{ $health }。
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = 可用 { $size }

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = 未挂载

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = 空闲空间

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = 里面没有光盘或存储卡

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = 上面还没有内容

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }，USB 或可移动

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }，在这台电脑内部

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }，USB 或可移动，已隐藏

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }，在这台电脑内部，已隐藏

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }，已用 { $full }，{ $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }，{ $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }，{ $words }：{ $parts }。

# Name given to a part that has no label and no device name.
disks-part-name = 分区

# Name given to a drive that gives no maker or model.
disks-drive-name = 驱动器

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }，{ $size }

# Kind of a part that is swap space.
disks-kind-swap = 交换空间

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = 已加密，已解锁

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = 已加密，已锁定

# Kind of a part that holds other parts.
disks-kind-container = 包含分区

# Kind of a part that has no filesystem on it.
disks-kind-none = 没有文件系统

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }，EFI，电脑用来启动的分区
        [legacy] { $kind }，旧式电脑用来启动的分区
       *[loader] { $kind }，引导程序自己的分区
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }：{ $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = 任何电脑

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = 仅限 Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = 旧设备

# Trouble a drive reports about its spare space.
disks-trouble-spare = 备用空间即将用完

# Trouble a drive reports about its heat.
disks-trouble-temperature = 温度过高

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = 正在磨损

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = 已变为只读

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = 其内存备份已失效

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = 部分已变为只读

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = 磨损已超出厂商的预期

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
       *[other] { $n } 处损坏
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
       *[other] 已使用 { $hours } 小时
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = 正在损坏：请立即把想保留的内容复制出来

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = 需要注意：{ $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = 状况良好

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }，{ $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = 没有响应

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = 外壳有响应，但里面的驱动器没有。请拔下它，等待十秒，再用自带的线缆插到蓝色接口或 USB-C 接口；如果仍是这样，里面的驱动器可能松脱或已损坏。

# SAFETY: needs a human review before it ships
# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = 安全移除

# SAFETY: needs a human review before it ships
# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = 将其断电

# SAFETY: needs a human review before it ships
# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
disks-erase-whole = 擦除整个驱动器

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] 显示此驱动器
       *[partition] 显示此分区
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] 隐藏此驱动器。{ $lock }
       *[partition] 隐藏此分区。{ $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = 已在 Folders 和圆环中隐藏。{ $lock }

# SAFETY: needs a human review before it ships
# Note on the card of the drive the system runs from.
disks-system-note = 系统运行在它上面，所以这里的任何内容都不能擦除。

# Note on the card of a drive that has no partition table.
disks-no-table = 这个驱动器没有分区表，所以整个驱动器都是空闲的。

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = 未挂载

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = 开机时挂载到 { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = 出现在 { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = 位于 { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = 打开

# Tip of the round button that mounts a part.
disks-mount = 挂载

# SAFETY: needs a human review before it ships
# Tip of the round button that unmounts a part.
disks-unmount = 卸载

# Tip of the round button on a part that opens the box of changes.
disks-change-it = 进行更改

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = 名称

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = 名称（可选）

# Tip of the button that saves a part's new name.
disks-rename-tip = 重命名

# Note under the Name row of a part.
disks-name-note = 它在这里和其他电脑上的名称。

# Placeholder of the typing box for a part's folder.
disks-folder-name = 文件夹名称

# Tip of the button that saves a part's folder.
disks-use-folder = 使用此文件夹

# Label of the row where a part's folder is chosen.
disks-folder = 文件夹

# Note under the Folder row of a part.
disks-folder-note = 它的文件出现的位置。留空则由电脑选择。

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = 开机时挂载

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = 电脑每次启动时都会挂载它。如果它不在，电脑照常启动。

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = 大小（GB）

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = 从 { $least } 到 { $most }。

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = 从 { $least } 到 { $most }。挂载后可查看它最小能缩到多少。

# SAFETY: needs a human review before it ships
# Tip of the button that resizes a part.
disks-resize-tip = 调整大小

# Label of the row where a part is resized.
disks-size = 大小

# SAFETY: needs a human review before it ships
# Said above the Format and Erase buttons at all times. { $words } names what is erased.
disks-format-erases = 这会擦除{ $words }上的所有内容，且无法撤销。

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning = 请再点按一次：{ $drive } 上的 { $name } 将从 { $from } 调整为 { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = 为新分区腾出空间

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = 空间（GB）

# Tip of the button that makes room under a part that is in use.
disks-room-tip = 腾出空间

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
disks-room-warning = 请再点按一次，从 { $drive } 上的 { $name } 中腾出 { $size }。Trindle 会先创建快照。

# SAFETY: needs a human review before it ships
# Note under the row that makes room.
disks-room-note = Trindle 会先创建快照，再在使用中缩小分区。腾出的空间保持空闲。

# Tip of the button that checks a part for faults.
disks-check-tip = 检查是否有故障

# SAFETY: needs a human review before it ships
# Tip of the button that repairs a part.
disks-repair-tip = 修复

# Label of the row where a part is checked and repaired.
disks-faults = 故障

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
disks-repair-warning = 请再点按一次以修复 { $drive } 上的 { $name }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = 类型

# Label of the row where a part is formatted.
disks-format = 格式化

# SAFETY: needs a human review before it ships
# Tip of the button that formats a part.
disks-format-tip = 格式化

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the format or erase button. { $words } names what is erased.
disks-format-warning = 请再点按一次：{ $words }上的所有内容都将被擦除

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $drive } 上的 { $name }，{ $size }

# SAFETY: needs a human review before it ships
# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-delete-name = 删除 { $drive } 上的 { $name }，{ $size }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = 删除分区

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = 在这里新建分区

# Label of the row where a new part is made.
disks-new-part = 新分区

# Tip of the button that makes a new part.
disks-make-tip = 创建

# SAFETY: needs a human review before it ships
# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = 请再点按一次，在 { $drive } 上新建一个 { $size } 的分区

# SAFETY: needs a human review before it ships
# Title of the box for erasing a whole drive. { $name } is the drive.
disks-sheet-erase = 擦除 { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }，位于 { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = 新分区

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = 可用 { $size }，位于 { $drive }

# Tip of the round button that closes the box of changes.
disks-close = 关闭

# SAFETY: needs a human review before it ships
# Words on a row while a part is being mounted.
disks-mounting = 正在挂载

# Words on a row when a part has been mounted.
disks-mounted = 已挂载

# SAFETY: needs a human review before it ships
# Words on a row while a part is being unmounted.
disks-unmounting = 正在卸载

# SAFETY: needs a human review before it ships
# Words on a row when a part has been unmounted.
disks-unmounted = 已卸载

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = 正在关闭

# Words on a drive's row when it is safe to remove.
disks-safe-remove = 可以安全移除

# Words on a drive's row while it is being powered off.
disks-powering-off = 正在断电

# Words on a drive's row when it has been powered off.
disks-off-safe = 可以安全拔下

# Words on a row while a part is being renamed.
disks-renaming = 正在重命名

# Words on a row when a part has been renamed.
disks-renamed = 已重命名

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = 现在由电脑为你选择文件夹

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = 它出现在 { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = 开机时挂载到 { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = 开机时不挂载

# Words on a row while a folder or start-up choice is being saved.
disks-saving = 正在保存

# Words on a row while a part is being resized.
disks-resizing = 正在调整大小

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = 已调整为 { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Trindle 找不到这些文件，所以无法腾出空间。

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = 正在创建快照

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Trindle 之后找不到该快照。

# Red words when the snapshot, or switching off the swap file, before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } 没有调整任何大小。

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = 创建快照期间情况发生了变化，所以 Trindle 没有调整任何大小。

# Words on a row while room is being made.
disks-making-room = 正在腾出空间

# Words on a row while the swap file is switched off before room is made.
disks-swap-off-doing = 正在关闭交换文件

# Words on a row while the swap file is switched back on after room is made.
disks-swap-on-doing = 正在重新开启交换文件

# Red words when the swap file couldn't be switched back on after making room. { $done } is how making room went, { $why } why the swap file is still off, both as sentences.
disks-swap-still-off = { $done } 重启前交换文件会保持关闭。{ $why }

# Words on a row when room has been made. { $size } is how much.
disks-room-made = 已腾出 { $size } 空间

# Words on a row while a part is being repaired.
disks-repairing = 正在修复

# Words on a row when a part has been repaired.
disks-repaired = 已修复

# SAFETY: needs a human review before it ships
# Red words when some faults could not be put right.
disks-repair-failed = Trindle 无法修复一些故障。请把想保留的文件复制到另一个驱动器。

# Words on a row while a part is being checked.
disks-checking = 正在检查

# Words on a row when the check found no faults.
disks-no-faults = 未发现故障

# Red words when the check found faults.
disks-faults-found = 发现故障：点按扳手图标进行修复。

# Words on a row while a part or drive is being formatted.
disks-formatting = 正在格式化

# Words on a row when a part has been formatted.
disks-formatted = 已格式化

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = 擦除整个驱动器

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = 找不到新分区。

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = 无法连接驱动器服务。

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Trindle 无法读取新分区的标识符。

# Words on a row while a part is being deleted.
disks-deleting = 正在删除

# Words on a row when a part has been deleted.
disks-deleted = 已删除

# Words on a row while a new part is being made.
disks-making-it = 正在创建

# Words on a row when a new part has been made.
disks-made = 已创建

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = 无法保存：{ $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# SAFETY: needs a human review before it ships
# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words }（{ $why }）

# Red words under the folder name box when it is empty.
disks-name-empty = 请先给它起个名字。

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
       *[other] 名称太长。请控制在 { $max } 个字符以内。
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = 只能使用字母、数字、- 和 _（不能有空格）。

# Red words under the folder name box when it starts with a dash.
disks-name-start = 请以字母或数字开头。

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } 已被占用。

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
       *[other] 名称对这个分区来说太长。请控制在 { $max } 个字符以内。
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT 名称不能包含 * ? . , ; : / \ | + = < > [ ] 或 "。

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] 请先挂载它。这种分区只能在挂载时扩大。
       *[smaller] 请先挂载它。这种分区只能在挂载时缩小。
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] 这种类型无法扩大。
       *[smaller] 这种类型无法缩小。
    }

# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password. Said whole, as a sentence.
disks-no-password = 你没有输入密码，所以没有任何改动。

# SAFETY: needs a human review before it ships
# Red words when the password box was closed with no password, after some steps of a change were already done.
disks-no-password-part-way = 你没有输入密码，所以工作在中途停止了。

# Red words when another change is pressed while one is still being made.
disks-one-at-a-time = 另一项更改仍在进行中。请在它完成后再试。

# SAFETY: needs a human review before it ships
# Red words when it could not be made sure that the password would be asked for.
disks-not-sure = Trindle 无法确认会要求输入密码，所以没有任何改动。

# SAFETY: needs a human review before it ships
# Red words after how a change ended, when the password just given could not be taken back.
disks-standing = Trindle 无法撤销你刚才输入的密码，所以几分钟内对驱动器的另一项更改可能不会要求输入密码。Trindle 正在再次尝试。

# SAFETY: needs a human review before it ships
# Words after how a change ended, when the password just given has been taken back after all.
disks-taken-back = 你刚才输入的密码现已撤销，所以下一次更改会要求输入密码。

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Trindle 无法连接驱动器服务，所以没有任何改动。

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = 进行此项更改时，“驱动器”被关闭了。

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = “驱动器”会保持打开，直到此项更改完成，这样你可以看到结果。

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }，所以没有任何改动。

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }，所以工作在中途停止了。

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = 无法询问驱动器服务

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } 工作在中途停止了。

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }。

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = 已完成：{ $steps }。

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = 失败：{ $steps }。

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = 未执行：{ $steps }。

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = 不确定：{ $steps }。

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = 为此已卸载 { $dev }，现在没有挂载。

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = 为此已卸载 { $dev }，现在仍未挂载：请在它的行中挂载。

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } 可能已被卸载：请查看它的行。

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = 为此已卸载 { $dev }。请在它的行中重新挂载，并在询问时输入它的密码。

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = 为此已卸载 { $dev }，且无法重新挂载（{ $why }）：请在它的行中挂载。

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = 格式化已完成，但 Trindle 无法再次为它设置文件夹 { $dir }。{ $why } 请在它的行中重新选择文件夹。

# Red words when a drive is in use and cannot be unmounted.
disks-busy = 仍有东西在使用它。请关闭所有显示其文件的窗口或程序，然后再试。

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = 这个大小对其中已有的文件来说太小了。

# Step of a change. { $dev } is the part.
disks-step-unmount = 卸载 { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = 重新挂载 { $dev }

# Step of a change.
disks-step-eject = 为移除驱动器做准备

# Step of a change.
disks-step-power-off = 将驱动器断电

# Step of a change. { $dev } is the part.
disks-step-format = 格式化 { $dev }

# Step of a change.
disks-step-create = 创建分区

# Step of a change. { $dev } is the part.
disks-step-delete = 删除 { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = 调整 { $dev } 的大小

# Step of a change. { $dev } is the part.
disks-step-resize = 调整 { $dev } 上内容的大小

# Step of a change. { $dev } is the part.
disks-step-repair = 修复 { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = 检查 { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = 重命名 { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = 保存 { $dev } 的文件夹和开机设置

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = 对 { $dev } 执行步骤 { $method }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = 这个 { $kind } 分区只能在未使用时缩小，所以无法在这里从中腾出空间。

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = xfs 分区无法缩小。

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = 这个 { $kind } 分区在使用中无法缩小。

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Trindle 只能为系统自己的磁盘创建快照，所以无法在这里腾出空间。

# Red words under the Make room row.
disks-room-not-in-table = 它不是驱动器分区表中的分区，所以 Trindle 无法从中腾出空间。

# Red words under the Make room row.
disks-room-no-online-shrink = 驱动器服务无法在使用中缩小这里的 Btrfs，所以 Trindle 无法腾出空间。

# Red words under the Make room row.
disks-room-holds-parts = 它包含其他分区，所以 Trindle 无法从中腾出空间。

# Red words under the Make room row.
disks-room-not-gpt = 它所在的驱动器没有 GPT 分区表，所以 Trindle 无法从中腾出空间。

# Red words under the Make room row.
disks-room-mount-first = 请先挂载它：Btrfs 需要在挂载时缩小。

# Red words under the Make room row.
disks-room-mounts-not-one = 它没有挂载，所以 Trindle 无法腾出空间。

# Red words under the Make room row.
disks-room-read-only = 它以只读方式挂载，所以 Trindle 无法腾出空间。

# Red words under the Make room row.
disks-room-read-only-unknown = Trindle 无法确定它是否只读，所以无法腾出空间。

# Red words under the Make room row.
disks-room-many-drives = 这些文件分布在不止一个驱动器上，所以 Trindle 无法腾出空间。

# Red words under the Make room row.
disks-room-drives-unknown = Trindle 无法确定这些文件分布在几个驱动器上，所以无法腾出空间。

# Red words under the Make room row.
disks-room-swap-unknown = Trindle 无法确定上面是否有交换文件，所以无法腾出空间。

# Red words under the Make room row.
disks-room-used-unknown = Trindle 无法读取它已用了多少，所以无法腾出空间。

# Red words under the Make room row.
disks-room-area-unknown = 驱动器服务没有说明文件系统有多大，所以 Trindle 无法检查它，也无法腾出空间。

# Red words under the Make room row.
disks-room-open-missing = Trindle 找不到它已解锁的副本，所以无法腾出空间。

# Red words under the Make room row.
disks-room-encryption-sizes = 为加密给出的大小不一致，所以 Trindle 无法腾出空间。

# Red words under the Make room row.
disks-room-sizes = 为它给出的大小不一致，所以 Trindle 无法腾出空间。

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = 它保存了 { $used }，需要保留的比这更多，所以能释放的空间太少。

# Red words under the Make room row.
disks-room-sizes-unread = 无法读取它的大小。

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = 可以腾出从 { $least } 到 { $most } 的空间。

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }，所以 Trindle 已停止。

# What was found wrong while making room.
disks-room-check-unmounted = 这些文件不再处于挂载状态

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = 分区为 { $is }，本应为 { $should }

# What was found wrong while making room.
disks-room-check-open = 加密的已解锁副本大小不对

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = 文件系统为 { $is }，本应为 { $should }

# What was found wrong while making room.
disks-room-check-area-small = 文件系统比已用的内容还小

# What was found wrong while making room.
disks-room-check-fit = 文件系统已装不进分区

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [partition] 驱动器服务不再列出该分区，所以工作已停止。
        [copy] 驱动器服务不再列出已解锁的副本，所以工作已停止。
       *[files] 驱动器服务不再列出该文件系统，所以工作已停止。
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = 缩小文件系统

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = 缩小加密的已解锁副本

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = 缩小分区

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = 检查大小

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = 在 { $dev } 下腾出空间之前（{ $at }）

# Red words when the swap file is back on after making room, but Trindle met a problem doing it. { $done } is how making room went, { $why } the problem, both as sentences.
disks-swap-on-but = { $done } { $why }

# Red words when another change is pressed while one is still being made.
disks-change-going = 正在进行更改。


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = 请再点一次：{ $drive } 上的 { $name } 将从 { $from } 缩小到 { $to }。请先备份其中的文件，因为缩小过程中断电可能会丢失文件。

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = { $name } 上的 Windows 处于休眠状态或停留在快速启动状态，缩小它可能会丢失其中的文件。请启动 Windows，选择“重启”，然后再试一次。
