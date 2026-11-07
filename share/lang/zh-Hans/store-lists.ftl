
# machine translated (Simplified Chinese), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Flatpak 文件

# On an installed thing's line: it came from a .deb file.
store-from-deb = .deb 文件

# On an installed thing's line: it is a plugin.
store-from-plugin = 插件

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = 候选发布版

# Tip of the round button on an app's tile while it is being removed.
store-list-removing = 正在移除 { $name }

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = 正在安装 { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = 打开 { $name }

# Tip of the round button on an app's tile that installs the app.
store-list-install = 安装 { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = 已安装

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = 显示

# How long is left when it is a minute or less.
store-left-minute = 还剩约一分钟

# How long is left, in minutes.
store-left-minutes = 还剩约 { $n } 分钟

# How long is left, in hours.
store-left-hours =
    { $n ->
       *[other] 还剩约 { $n } 小时
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = 已对照 HaGeZi 的威胁情报列表检查

# Where a download came from: the site and the page that led to it.
store-origin-both = 下载自 { $site }，经由 { $referrer } 到达。

# Where a download came from: the site.
store-origin-site = 下载自 { $site }。

# Where a download came from: the page that led to it.
store-origin-referrer = 经由 { $referrer } 到达。

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle 无法判断它来自何处。

# Warning when the site is on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-listed = { $from } { $name } 在 HaGeZi 的威胁情报列表上，该列表收录以有害下载和诈骗著称的网站。只有在确定它的来源可靠时才安装。

# Said when the site is not on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-clear = { $from } 该网站不在 HaGeZi 的威胁情报列表上，但该列表并不完整，所以这并不能证明它是安全的。

# Said when the list could not be fetched; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-not-checked = { $from } 无法检查该网站，因为无法获取 HaGeZi 的威胁情报列表。

# Name shown for a review whose writer gave none.
store-review-someone = 某人

# Red words when the keyring could not be reached.
store-key-unreached = 无法访问密钥环：{ $why }。

# Red words when the key kept in the keyring is not text.
store-key-not-text = 保存的密钥不是文本。

# Red words when a file that held the key could not be removed.
store-key-not-removed = 无法移除 { $path }：{ $why }

# Red words when a file that held the key could not be read.
store-key-not-read = 无法读取 { $path }：{ $why }

