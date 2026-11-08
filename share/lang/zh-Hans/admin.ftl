
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = 这不是有效的登录名。

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = 这个登录名太长了。

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = 没有名为 { $name } 的用户。

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } 是系统账户，不会被改动。

# SAFETY: needs a human review before it ships
# Red words in Settings when no password reached the helper that sets it.
admin-password-none = 没有提供密码。

# SAFETY: needs a human review before it ships
# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
admin-password-short =
    { $n ->
       *[other] 密码至少需要 { $n } 个字符。
    }

# SAFETY: needs a human review before it ships
# Red words in Settings when a new password has a line break in it.
admin-password-line-break = 密码中不能包含换行。

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } 已存在。

# SAFETY: needs a human review before it ships
# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name } 是唯一的管理员，所以仍保留管理员身份。

# SAFETY: needs a human review before it ships
# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = 你不能删除自己的账户。

# SAFETY: needs a human review before it ships
# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = { $name } 已被删除，但其登录界面上的头像仍然保留：{ $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = 这张图片太大了。

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = 这不是 Trindle 制作的图片。

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = 无法让登录界面访问头像文件夹：{ $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = 这不是有效的语言名称。

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = 无法读取 Debian 的语言列表：{ $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } 不是 Debian 已知的语言。

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = 电脑名称由字母、数字和连字符组成，最多 63 个字符，不能以连字符开头或结尾。

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = 限制目前已开启，但关机后会失效：{ $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = 目前已充满电，但下次启动时限制会恢复：{ $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# SAFETY: needs a human review before it ships
# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = 其他电脑仍然可以连接，因为开关无法关闭：{ $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = 这不是有效的插件标识。

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = 不认识这个人。

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = 不知道此人的文件夹。

# SAFETY: needs a human review before it ships
# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = 无法保存放行的插件列表：{ $why }。

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = 找不到该插件。

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = 没有更新任何内容：无法确定这台电脑的 Trindle 版本，所以无法把 Trindle 自身的更新排除在外。

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = 正在安装更新

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = 正在整理

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = 正在安装 Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = 这不是软件包的名称。

# SAFETY: needs a human review before it ships
# Red words when the package to remove is not installed.
admin-package-not-installed = 未执行：它没有安装。

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = 这不是 fwupd 已知的设备。

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }；{ $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = 无法确定是谁请求了 Tailscale。

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = 网站拦截未能及时启动。

# SAFETY: needs a human review before it ships
# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = 拦截没有完全关闭：{ $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = 无法还原 { $path }：{ $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = 这不是网站名称。

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = 没有这样的服务器。

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }：{ $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } 在收到全部内容之前就停止了，所以不能算已完成：{ $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } 失败了。

# SAFETY: needs a human review before it ships
# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = 无法删除 { $path }：{ $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = 没有改动任何内容，因为{ $why }：{ $path }

# SAFETY: needs a human review before it ships
# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } 恢复原状也没有成功：{ $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = 这不是可以用来保留早期版本的时长。

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = 找不到快照文件夹。

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = 存放快照的位置上有一个不是文件夹的东西。

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = 无法确定是谁提出的请求。

# SAFETY: needs a human review before it ships
# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = 这只能通过 pkexec 运行。

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = 这不是电脑的名称。

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = 这不是共享文件夹的名称。

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = 这不是电脑的地址。

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = 这不是共享文件夹的路径。

# SAFETY: needs a human review before it ships
# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = 在这里只能关闭你自己的共享文件夹。

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = 那里没有打开的共享文件夹。

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = { $path } 与 Trindle 留下时不一样了，所以不会在那里挂载任何内容。

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } 与 Trindle 留下时不一样了。

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }：{ $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = 打开的共享文件夹太多了。

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = 没有安装 { $tool }。它包含在 Debian 的 { $package } 软件包中。

# SAFETY: needs a human review before it ships
# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = 无法读取登录信息。

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = 无法读取这台电脑上已挂载的内容：{ $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = 仍有程序在使用其中的内容。请先关闭它。

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = 那台电脑没有响应。注销时会将其关闭。

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = 这不是用户编号。

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = 无法创建 sambashare 组。

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = 正在获取 Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = 正在设置共享

# SAFETY: needs a human review before it ships
# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = 共享尚未设置。请先共享一个文件夹。

# SAFETY: needs a human review before it ships
# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = 网络上的其他人仍然能看到这台电脑：{ $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = 这不是快照编号。

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = 在设置中创建

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = 无法确定系统位于哪块磁盘。

# SAFETY: needs a human review before it ships
# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = 系统不在独立的 Btrfs 子卷中，所以无法换成快照。

# SAFETY: needs a human review before it ships
# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = 无法从 { $url } 获取签名密钥。

# SAFETY: needs a human review before it ships
# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = 该地址返回的内容不是签名密钥。

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = 没有改动任何内容，因为无法读取 { $path }：{ $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = 正在获取它的签名密钥

# Second line of the progress while a source is added.
admin-step-adding-source = 正在添加并读取它的列表

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = 无法使用该软件源。{ $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = 无法使用该软件源，所以已将其移除。{ $why }

# SAFETY: needs a human review before it ships
# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } 还不能开关它：{ $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = 无法为 { $user } 启动它：{ $why }

# SAFETY: needs a human review before it ships
# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = 无法让 { $user } 使用虚拟机，而 Cowork 需要它：{ $why }

# SAFETY: needs a human review before it ships
# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = 无法让 { $user } 免密码使用它：{ $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } 已安装，但设置没有完成：{ $why }

# Line of the progress while a program is installed.
admin-step-installing-it = 正在安装

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = 这不是现成的软件源之一。

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } 不是为这种电脑（{ $arch }）制作的。

# Last line of the progress while a program is installed.
admin-step-setting-up = 正在设置

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = 地址必须以 https:// 开头，且不能包含其他内容。

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = 套件和组件必须是普通的词。

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = 这不是软件包名称。

# First line of the progress while a source and its program are taken away.
admin-step-removing = 正在移除

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = 正在重新读取列表

# SAFETY: needs a human review before it ships
# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = 在这里只能移除由 Trindle 添加的软件源。

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = 该软件源仍然存在：{ $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = 磁盘上没有快照 { $n }。

# SAFETY: needs a human review before it ships
# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = 没有改动任何内容：快照无法准备就绪。{ $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = 无法把当前系统暂时放到一边：{ $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = 当前系统现在名为 { $aside }，而不是 { $sub }，在改回 { $sub } 之前电脑将无法启动：{ $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = 无法迁移快照：{ $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }；{ $second }

# SAFETY: needs a human review before it ships
# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = 无法把快照放到位：{ $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = 无法更新已搁置系统的列表：{ $why }

# SAFETY: needs a human review before it ships
# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = 无法移除：{ $names }

# SAFETY: needs a human review before it ships
# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = 已不再有任何东西需要这些程序，所以已将其移除：{ $list }。

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = 部分更新未能安装。已保留的更新：{ $list }。

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature }已关闭：它的驱动（{ $driver }，DKMS）暂时无法在新内核上构建。

admin-driver-off = 一个驱动（{ $driver }，DKMS）暂时无法在新内核上构建，因此它提供的功能已关闭。

# Said once after the drivers that are off.
admin-driver-off-rest = 内核和其余更新已安装。

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = 虚拟摄像头

admin-language-in-use = { $code } 是这台电脑启动时使用的语言，所以会保留。



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = 这台电脑没有可以关闭的拆分锁减速功能。

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = 找不到该磁盘，或者它存放着系统。

# The drive could not be closed where it was open.
admin-games-drive-busy = 该磁盘正在使用。请关闭其中已打开的内容，然后重试：{ $why }

# Another software change has the read-only system open.
admin-system-busy = 另一项软件更改正在进行。请在它结束后重试。

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = 没有做任何更改：系统无法解锁：{ $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = 系统无法重新锁定。下次启动时会将其锁定：{ $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = 无法保持 /usr/local 可写：{ $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = 只有 root 才能运行，且不能通过 pkexec 运行。

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = 需要 Btrfs。


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = 快照已就位，电脑将从它启动。清理旧系统没有完成，下次启动时会再试一次：{ $why }

