
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = 安装软件包

# Words at the top of the window while the package is read.
store-deb-reading = 正在读取软件包…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = 这个文件不是可以安装的 Debian 软件包。它可能没有下载完整。

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } 可以安装到它自己的文件夹中。

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = 这个软件包适用于 { $arch } 电脑，因此无法安装在这台电脑上。

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = 这是 Trindle 本身。它通过“应用”的“更新”页面更新，而不是通过文件。

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } 是驱动程序或后台服务，无法在它自己的文件夹中运行，因此不能安装在这里。

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = 来自 Debian 和 Trindle 之外的软件包会装入它自己的文件夹，受到限制，但并未被完全隔离。请只安装你信任的制作者提供的软件包。

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } 已安装在它自己的文件夹中。

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = 正在检查它的来源…

# Heading in the card that says where the file came from.
store-deb-where = 来源

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = 删除它及其保存的内容

# Word beside the ring while the package goes in.
store-deb-installing = 正在安装

# Main button that installs the package.
# SAFETY: needs a human review before it ships
store-deb-install = 安装

# Main button that installs the same version again.
# SAFETY: needs a human review before it ships
store-deb-reinstall = 重新安装

# Main button that brings the installed package up to this version.
# SAFETY: needs a human review before it ships
store-deb-update = 更新

# Heading of the card for a package not installed yet.
store-deb-goes-in = 它会装入自己的文件夹，默认受到限制

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = 不需要密码。在你允许之前，它无法使用互联网、声音或你的文件。

# Heading of the card for a package installed at this version.
store-deb-same-version = 版本 { $version } 已安装在它自己的文件夹中

# Under that heading.
store-deb-same-version-note = 如果它已无法正常工作，请在那里重新安装。

# Heading of the card for a package installed at another version.
store-deb-other-version = 版本 { $version } 已安装在它自己的文件夹中

# Under that heading: the version this brings.
store-deb-other-version-note = 在那里安装这个软件包会将它升级到版本 { $version }，并保留它已保存的内容。

# Facts line: how big the package is once installed.
store-deb-fact-size = 安装后的大小

# Facts line: the computer kind it is made for.
store-deb-fact-arch = 适用于

# Text of the About card when the package has no description.
store-deb-no-description = 这个软件包没有说明。

# Words at the top of the window while the package goes in.
store-deb-installing-note = 正在安装到它自己的文件夹。获取所需内容可能需要几分钟。

# Words at the top of the window once the package is installed.
store-deb-done = 已安装到它自己的文件夹。你可以在圆环中找到它。

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = 已安装到它自己的文件夹，但找不到这些部分，所以它可能无法启动：{ $parts }。

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = 已从它自己的文件夹中删除，连同它保存的所有内容。

# Red words when a file is not a Debian package.
store-boxed-not-deb = 这个文件不是可以安装的 Debian 软件包。

# Step shown while a package is unpacked.
store-boxed-unpacking = 正在解包 { $name }

# Step shown while a part a package needs is added.
store-boxed-adding = 正在添加 { $part }

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } 不是可以安装的 Debian 软件包。

# Red words when a package could not be unpacked.
store-parts-not-unpacked = 无法解包 { $path }。

# Red words when the program is not one kept in a box.
store-loose-not-boxed = 这个程序不是放在盒子里的。

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = 终端

# Title of the notice when older programs are off.
store-boxed-x-off = 旧版程序已关闭

# Body of that notice: where to turn them on.
store-boxed-x-off-where = 设置，权限

# Title of the notice when too many older programs are open.
store-boxed-x-many = 旧版程序太多

# Body of that notice.
store-boxed-x-many-close = 请关闭一个后再试

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } 无法打开

# Body of the notice the first time a terminal opens outside its box.
# SAFETY: needs a human review before it ships
store-boxed-terminal-told = 终端能访问的电脑内容比其他应用更多，因为它的工作需要这样。请只输入你理解的命令，并且只输入你信任的人给你的命令。

# Title of the notice when a plugin allowed the home network has none.
store-boxed-home-off = 家庭网络已关闭

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } 没有网络：{ $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = 找不到它自己的程序

# The same: the unshare program is missing.
store-boxed-home-why-unshare = 没有安装 unshare

# The same: the pasta program is missing.
store-boxed-home-why-pasta = 没有安装 pasta

# The same: the network room was not made.
store-boxed-home-why-not-made = 没有创建它的网络空间

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = 它的网络空间不是新的

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = 无法创建系统调用过滤器

# The same: the room's rules would not load.
store-boxed-home-why-rules = 网络空间的规则无法加载

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } 无法启动：{ $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta 报告：{ $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta 无法加入该网络空间



# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = 在没有网络的情况下打开

# Body of that notice: why, and what the program was given.
store-downloaded-why = 它是下载来的，所以程序只能得到那个文件，且是只读的。

