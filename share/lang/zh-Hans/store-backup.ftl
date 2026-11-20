
# machine translated (Simplified Chinese), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
# SAFETY: needs a human review before it ships
store-refusal-nothing = 没有恢复任何内容，因为{ $why }。

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Trindle 只恢复了备份的一部分，因为{ $why }。

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
# SAFETY: needs a human review before it ships
store-because-outside = 备份中包含一个链接或其他内容，会指向它所要放入的文件夹之外

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = 这台电脑上已有东西占着备份中某一部分要放的位置

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = 没有可以解包备份的地方

# The reason when the file is not a Trindle backup.
store-because-not-trindle = 该文件不是 Trindle 备份

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = 备份已损坏，Trindle 无法读取其中的清单

# The reason when the file is not a Flat backup.
store-because-not-flat = 该文件看起来不是 Flat 备份

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = 该文件中没有 Flat 应用清单

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = 它由比 Trindle 能读取的更新的 Flat 版本制作

# The reason when the file holds no apps.
store-because-no-apps = 该文件中没有应用

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = 无法读取备份：{ $why }

# The reason when the program gave none.
store-because-none = 未给出原因

# The reason when the system said permission was denied.
# SAFETY: needs a human review before it ships
store-because-permission = Trindle 无权写入那里

# The reason when the disk is full.
store-because-space = 这台电脑上没有剩余空间了

# The reason when a part of the computer is read-only.
store-because-read-only = 这台电脑的那一部分是只读的

# The reason when the person's disk quota is used up.
store-because-quota = 这个账户允许使用的空间已用完

# The reason when a file or program is missing.
store-because-missing = 缺少它所需的文件或程序

# The reason when no reason could be read from the program's last line.
store-because-failed = 失败了

# The reason when the backup has no name to be written under.
store-because-no-name = 它没有名称

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = 备份中它的设置指向了它自己的文件夹之外

# The reason when something is too big to be read whole.
store-because-too-big = 它太大，无法读取

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = 备份中它的设置已损坏或被更改

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = 除了你的账户头像，其他内容都已恢复。{ $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Trindle 也没有恢复你的账户头像。{ $why }

# Said when the pkexec program could not be started.
# SAFETY: needs a human review before it ships
store-refusal-pkexec = 无法请求输入密码：{ $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Trindle 无法从备份中读取它：{ $why }。

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Trindle 没有制作备份，因为{ $why }。

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = 暂时没有

# Beside the size of a part of the backup: about how big it is.
store-backup-about = 约 { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = 正在恢复 Trindle 的设置和 AppImage……

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Trindle 已恢复你的设置和数据，但 Flatpak 无法安装这些应用：{ $failed }。

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Trindle 已恢复你的设置和数据，但没有安装这些应用：{ $refused }。

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
       *[other] Trindle 跳过了 { $n } 个应用，因为它们的名称无效。
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = { $name } 软件源没有加密，所以 Trindle 没有使用它

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = { $name } 软件源不在这台电脑上，备份中也没有说明它在哪里

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = { $name } 的设置（因为备份中的副本没有通过检查）

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name }（因为它的来源名称无效）

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what }（因为{ $why }）

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name }（Flatpak 无法安装它：{ $why }）

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = 全部内容（因为没有可以解包备份的地方）

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = 每个应用的权限

# Said among what could not be done: one app's settings.
store-flat-app-settings = { $name } 的设置

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }。原来的内容保留为 { $path }：{ $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = 正在安装 { $name }（第 { $at } 个，共 { $total } 个）……

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = 正在恢复 { $name } 的设置（第 { $at } 个，共 { $total } 个）……

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
       *[other] 已安装 { $n } 个应用。
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings =
    { $n ->
       *[other] 已恢复 { $n } 个应用的设置。
    }

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
       *[other] 已有 { $n } 个应用安装过。
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = 遗漏：{ $list }


# Said among what could not be done when a backup's permissions asked for more than the app had: only ones that take access away are put back.
store-flat-grants-left-out = { $what }：Trindle 只恢复了会收回访问权限的权限。
