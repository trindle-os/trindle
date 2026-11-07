
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Debian 软件包

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = 已暂缓更新，停留在版本 { $have }；{ $waits }

# On a row held back: the one that waits.
store-upd-held-waits = 已暂缓更新；{ $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = 已暂缓更新，停留在版本 { $have }

# On a thing held back with no update waiting.
store-upd-held = 已暂缓更新

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = 已暂缓更新，停留在版本 { $have }；暂无更新版本

# On the Held back card: nothing newer waits.
store-upd-held-nothing = 已暂缓更新；暂无更新版本

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
       *[other] { $n } 个更新
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
       *[other] 有 { $n } 个更新可用
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }，共 { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
       *[other] 有 { $n } 个已暂缓
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }；{ $held }

# A line made into a sentence.
store-upd-sentence = { $text }。

# Two sentences one after the other.
store-upd-and-then = { $first }{ $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = 固件需要重启才能完成更新。

# Headline while the check runs.
store-upd-checking = 正在检查更新。

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = 正在更新 { $name }。安装期间你可以继续工作。

# Headline while all the updates go in.
store-upd-installing-all = 正在安装更新。安装期间你可以继续工作。

# Headline when the updates are in and the computer must start again.
store-upd-restart = 重启以完成更新：更新已安装，电脑重新启动后才会生效。

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = 已安装的内容需要重启才能完成。

# Added to the headline with no internet.
store-upd-no-more = 这台电脑没有连接互联网，无法再查找更多更新。

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = 没有可用的更新

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = 这台电脑已知的更新都已处理。没有连接互联网时，无法查找更多更新。

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = 应用

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle 和 Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = 固件

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = 无法检查{ $names }的更新。

# Headline when everything is up to date.
store-upd-current = 一切都是最新的。

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = 上次检查时间：{ $time }。

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = 没有互联网无法检查

# Red words under a card when a check took too long.
store-upd-too-long = 等待回应的时间太长。

# Tip of the round button that checks again.
store-upd-check-again = 再次检查

# Main button while the check runs.
store-upd-checking-main = 正在检查

# Main button while the updates go in.
store-upd-installing-main = 正在安装

# Main button that installs every update waiting.
store-upd-install-n =
    { $n ->
       *[other] 安装 { $n } 个更新
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = 立即重启

# Main button that checks for updates.
store-upd-check-for = 检查更新

# Tip of the round button that releases a thing held back.
store-upd-release = 解除暂缓 { $name }

# Tip of the round button that holds a thing's updates back.
store-upd-hold = 暂缓更新 { $name }

# A card's state while it is being checked.
store-upd-status-checking = 正在检查

# A card's state while its updates go in.
store-upd-status-installing = 正在安装

# A card's state when its check failed; the reason follows.
store-upd-status-failed = 无法检查。{ $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }，{ $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }；{ $held }已暂缓

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } 个已暂缓

# A card's state when nothing waits.
store-upd-up-to-date = 已是最新

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = 来自 { $from }，{ $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = 来自 { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = 共享组件，来自 { $from }

# Note under the Apps card.
# SAFETY: needs a human review before it ships
store-upd-yours-alone = 仅属于你自己，所以更新时不需要密码。应用会带上其新版本所需的共享组件。

# Note under the Apps card: what holding back does.
store-upd-hold-why = 已暂缓的更新会停留在当前版本，在这里和圆环中的“全部安装”时也不会安装，直到你解除暂缓。

# Heading of the Apps card.
store-upd-card-apps = 应用

# Plugins card: every plugin is up to date.
store-upd-plugins-current = 所有插件都是最新的。

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
       *[other] 有 { $n } 个插件的更新已暂缓。
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
       *[other] 有 { $n } 个插件有更新
    }

# Heading of the Plugins card.
store-upd-card-plugins = 插件

# Held back card: nothing is held back.
store-upd-nothing-held = 没有暂缓的更新。

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = 已解除暂缓

# Note under the Held back card: what releasing does.
store-upd-release-why = 解除暂缓的项目会重新和其他更新一起安装，包括这里和圆环中的“全部安装”。

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = 某设备的固件

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = 设备 { $id } 的固件

# Heading of the Held back card.
store-upd-card-held = 已暂缓

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } 到 { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
       *[other] { $n } 个更新，一起安装
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = 来自 Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = 显示 Debian 更新的 { $shown } 项，共 { $total } 项。

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = 这台电脑使用的是 Trindle { $version }。

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle 和 Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Debian 的软件包彼此配合，单独安装一个可能带入其他软件包，或使它们不同步。所以它们从一个按钮一起安装。

# Note on the Trindle and Debian card: why no row has a Hold back button.
store-upd-not-held = Trindle 和 Debian 的更新包含整台电脑的安全修复，因此不能暂缓。

# Heading of the card about the daily security updates.
store-upd-card-security = 安全更新

# On the security card: its state.
store-upd-security-on = 已开启，每天一次

# On the security card.
store-upd-security-note = Debian 的修复每天自动安装，无需询问。

# On the security card.
store-upd-security-part = 属于 Trindle 的一部分，无法关闭。

# On the security card: that holding back never stops these.
store-upd-never-held = 暂缓更新不会影响这些安全更新。

# Firmware card: the computer has no updater.
store-upd-fw-none = 这台电脑没有固件更新工具。

# Firmware card: restart to finish.
store-upd-fw-restart = 重启以完成

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = 版本 { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = 新固件会在电脑重启时安装。

# Note on the firmware card when the updater is missing.
store-upd-fw-install = 请安装 fwupd，以保持固件为最新。

# Heading of the Firmware card.
store-upd-card-firmware = 固件

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd 无法获取最新列表。

# Red words when Flathub could not be reached.
store-upd-no-flathub = 无法连接 Flathub。

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = 无法获取 Debian 的列表。

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = 无法获取 Debian 的列表。{ $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = 未能暂缓。无法保存已暂缓项目的列表：{ $why }。

# The same, for a release.
store-upd-not-released-list = 未能解除暂缓。无法保存已暂缓项目的列表：{ $why }。

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = 已在“应用”中暂缓，但无法通知 Flatpak 本身，所以其他更新应用的工具仍可能安装它。

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = 已在“应用”中解除暂缓，但无法通知 Flatpak 本身，它可能仍会被跳过。请再次暂缓并解除暂缓。

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak 提示：{ $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle



# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = 自动检查更新已关闭。点击“检查更新”即可查找。

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = 未检查

# Tip of the round button that ends an update that is running.
store-upd-stop = 停止

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = 正在停止

# Under a row whose update was stopped.
store-upd-stopped = 已停止。点击“更新”继续。

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = 已停止。点击“安装”继续。

# Note under the Apps card.
store-upd-steam-restarts = 没有游戏在运行后，Steam 会重启。

