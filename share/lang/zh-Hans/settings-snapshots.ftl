
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = 快照

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = 快照 还原 恢复 撤销 回滚 备份 更新 出错 btrfs snapper 启动菜单 grub 早期版本 旧版本 以前的版本 已删除的文件 找回 主目录 副本

# Help line at the foot of the Snapshots page.
# SAFETY: needs a human review before it ships
settings-snap-help = 快照是系统在每次软件更改前后，以及每当你创建快照时的一份副本。还原快照会让整个系统在你下次重启时回到当时的状态。如果主目录在磁盘上有自己独立的分区（Trindle 的安装程序就是这样设置的），你自己的文件会保持不变。如果电脑无法启动，请在启动菜单中选择 Trindle 快照。“早期版本”会每天为每个人自己的文件保留一份副本，与系统快照分开。

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = 磁盘几乎已满，Trindle 没有创建副本。

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = 今天 { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = 昨天 { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = 电脑无法自行重启。请改从圆环中重启。

# Said beside the Take a snapshot line while it works.
settings-snap-taking = 正在创建快照

# Said beside the set aside line while the space is freed.
settings-snap-freeing = 正在释放……

# Said beside the set aside line once the space is freed.
settings-snap-freed = 已释放

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = 正在还原

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = 你下次重启时，系统将回到快照 { $n } 的状态。此后所做的任何更改都会被另外保存，不会丢失。

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = 正在删除

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = 已删除

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = 正在等待输入你的密码……

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = 每天创建一份副本

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = 不再创建副本

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = 已保留

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = 正在清除

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = 已清除

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] 还没有副本
       *[other] { $n } 份副本
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = 第一份会在一天之内创建。

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = 它们占用 { $size }，测量时间：{ $when }。

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = 它们占用的空间每天测量一次。

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = 立即全部清除

# Heading of the card about earlier versions of files.
settings-snap-earlier = 早期版本

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle 可以每天保存一份所有人文件的副本。这样你就可以在 Folders 中打开文件的旧版本，或找回被误删的文件。在副本中，每个人只能看到当天自己本来就能看到的内容，所以私人主目录依然是私人的。当时对他人开放的文件，在该副本被清除之前，在副本中仍然对他们开放。这些副本与系统快照分开，互不影响。它们确实会占用磁盘空间。你删除或更改的文件，在保存它的最后一份副本被清除之前，仍会占用空间。清除副本会立即为所有人释放空间，并且无法撤销。磁盘几乎已满时不会创建副本。更改其中任何设置都需要管理员密码。

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = 保留你的文件的早期版本

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = 已删除的文件会保留 { $time }，之后释放空间。

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = 每天的副本保留多久

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = 每天的副本保留多久

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = 正在检查快照……

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = 无法读取快照。

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = 你是从快照 { $n } 启动的，所以现在所做的任何操作都不会保留。

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
       *[other] 你可以回到 { $n } 个快照。最新的创建于 { $when }。
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = 还没有快照。每次软件更改前都会创建一个。

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }，保留到被删除为止

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = 还原此快照

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = 删除此快照

# Tip of the button that restarts the computer after a restore.
# SAFETY: needs a human review before it ships
settings-snap-restart-now = 立即重启

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = 尚未设置。

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = 尚未设置快照。重新安装 trindle 软件包即可设置：sudo apt install --reinstall trindle。

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = 其他类型

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = 快照需要 Btrfs。这台电脑使用的是 { $fs }。

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = 快照需要系统位于 Btrfs 上，而这台电脑使用的是 { $fs }。要使用快照，请在安装 Debian 时为系统选择 Btrfs。

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = 这台电脑上没有快照。

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = 没有快照

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = 这台电脑上没有快照

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = 加载中……

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = 无法读取快照

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = 暂无

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = 每个快照都可以让整个系统在你下次重启时回到当时的状态。还原或删除快照都需要管理员密码。

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = 立即创建快照

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = 请正常重启后再进行此操作。

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
       *[other] 还原后留下了 { $n } 个旧系统
    }

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = 释放它们占用的空间

# Heading of the card for taking a snapshot.
settings-snap-take-one = 创建一个

# Help text of the Take one card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-take-about = 在尝试可能想撤销的操作之前，先创建一个快照。你创建的快照会保留到你删除为止。如果你是从快照启动的，请先正常重启，否则你新建的快照不会保留。每次还原都会把还原前的系统另外保存，所以不会丢失任何内容。系统文件锁定后，只有安装和更新才能更改它们，而且每次都会在前后各创建一个快照。更改锁定设置需要管理员密码。

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = 从快照 { $n } 启动

# Under the Started from snapshot heading.
settings-snap-nothing-kept = 更改不会保留。

# Heading of a line on the Before restoring card.
settings-snap-files-too = 你的文件也在快照里

# Under the Your files are in the snapshots too heading.
settings-snap-files-back = 还原时也会把你的文件恢复。

# Heading of the card about what to know before restoring.
settings-snap-before = 还原之前

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = 因为你是从快照启动的，所以现在所做的任何操作都不会保留。如果想让系统保持现在这样，请在列表中按此快照旁边的“还原此快照”，然后重启。在这台电脑上，主目录可能属于系统的一部分。如果是这样，还原快照也会把你自己的文件恢复到当时的样子，所以请先把新的内容复制到安全的地方。


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = 系统文件已锁定

# Under the System files locked switch.
settings-snap-locked-sub = 只有安装和更新才会解锁。

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = 已锁定

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = 已解锁
