
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = 打开它的设置

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = 未完成：{ $why }

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = 这次更新需要新的权限，正在等你决定。

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = 当前版本 { $mine }，{ $new } 已就绪

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = 当前版本 { $mine }，{ $new } 已就绪。它要求的权限比以前多：{ $more }。{ $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = 它完成后，这里的按钮会重新出现。

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = 正在查找插件……

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = 没有安装任何插件，目前也没有可安装的插件。

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
       *[other] 还没有安装任何插件；有 { $n } 个可以安装。
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
       *[other] 已安装 { $n } 个插件
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
       *[other] 有 { $n } 个可更新
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
       *[other] 有 { $n } 个更新被暂缓
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }；{ $held }。

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }。

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }，全部为最新。

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }，{ $have }。

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }，{ $have }；{ $held }。

# On a plugin's card when it asks for nothing.
# SAFETY: needs a human review before it ships
store-plugins-asks-nothing = 它不要求任何权限。

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = 它要求的权限：{ $list }。

# Title of the Plugins page.
store-plugins-title = 插件

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = 已安装版本 { $mine }；{ $new } 已就绪。

# On a plugin's card: its installed version.
store-plugins-version-installed = 版本 { $version }，已安装。

# On a plugin's card: the version on offer.
store-plugins-version-offered = 版本 { $version }。

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }。

# Tip of the round button that installs a plugin.
# SAFETY: needs a human review before it ships
store-plugins-install = 安装 { $name }

# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = 解除 { $name } 的更新暂缓

# Tip of the round button that updates a plugin to a version.
# SAFETY: needs a human review before it ships
store-plugins-update-to = 更新到 { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = 插件的来源

# Information on the card that says where plugins come from.
store-plugins-come-from = 插件只来自 Trindle 自己的列表，每次打开这个页面时都会从 trindle.org 读取。如果想提供插件，请提交给我们：每个插件上线前都会经过测试。

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = 它已安装，但它的后台部分没有运行：{ $why }

# Red words under a plugin that could not be removed; the reason follows.
# SAFETY: needs a human review before it ships
store-plugins-not-removed = 无法删除它：{ $why }

