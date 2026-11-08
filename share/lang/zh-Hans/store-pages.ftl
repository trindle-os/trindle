
# machine translated (Simplified Chinese), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
       *[other] { $count } 个应用
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
       *[other] 有 { $count } 个更新等待安装
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = Flathub 上有 { $apps } 可供选择；{ $more }。

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = 你的所有应用都是最新的

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }。

# Line under the title of a search that found nothing.
store-grid-none-found = 没有与“{ $text }”匹配的应用。

# Line under the title of an empty page of apps.
store-grid-none-here = 这里还没有应用。

# Advice in the empty page of a search.
store-grid-try-shorter = 请试试更短的词，或在左边浏览某一类应用。

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = 请在上方按名称搜索，或在左边浏览其他类别。

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = 显示 { $shown } 个，共 { $apps }。

# Link under a page of apps that shows more of them.
store-grid-show-more = 再显示 { $next } 个

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = 正在从 Flathub 获取应用列表。第一次需要一分钟左右。

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = “应用”无法连接到 Flathub。

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = 这台电脑没有连接互联网。一旦连上，“应用”会自动重试。

# In the waiting page when the computer is not connected.
store-wait-comes-from = 可供选择的应用通过互联网来自 Flathub。

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = 左边的 { $pages } 和应用设置不需要它也能使用。

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = 左边的 { $pages } 和应用设置不需要 Flathub 也能使用。

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = 左边的 { $pages } 和应用设置不需要互联网也能使用。

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = 重试

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = 结果

# Red words under the search box when no web browser opens a link.
store-no-browser = 没有可以打开 { $url } 的网页浏览器。请先在“应用”中获取一个。

# Said in the card or under the title while the lists are being read.
store-looking = 正在查找……

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = 它现在已在“已安装”中，也在圆环里

# Said under an app's row when it was taken off, and the word on its dial.
# SAFETY: needs a human review before it ships
store-done-removed = 已删除

# The word on an app's dial when installing or removing did not work.
store-done-not = 未完成

# The word on an app's dial when it was installed.
store-done-installed = 已安装

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak 拒绝了。

# Tab of the Installed page that shows everything.
store-tab-all = 全部

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Flatpak 文件

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Deb 文件

# Tab of the Installed page that shows plugins.
store-tab-plugins = 插件

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = 还没有安装任何东西。“发现”里有很多可以试试的内容。

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
       *[other] 已安装 { $n } 个插件
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
       *[other] 已安装 { $n } 个程序
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = 已安装 { $n } 项内容

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
       *[other] { $start }，全部来自 Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
       *[other] { $start }，全部来自 Flatpak 文件
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
       *[other] { $start }，全部是 AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
       *[other] { $start }，全部来自 .deb 文件
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }，{ $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }：{ $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } 个来自 Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
       *[other] { $n } 个来自 Flatpak 文件
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
       *[other] { $n } 个 AppImage
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
       *[other] { $n } 个应用
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
       *[other] { $n } 个来自 .deb 文件
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
       *[other] { $n } 个插件
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }。{ $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = 它是最新的。

# End of the Installed line when everything is up to date.
store-installed-state-all = 全部都是最新的。

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
       *[other] 有 { $n } 个更新在等待安装。
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = 共 { $total } 个程序，其中 { $shown } 个与“{ $find }”匹配。

# The same when plugins are among the things.
store-installed-match-things = 共 { $total } 项内容，其中 { $shown } 项与“{ $find }”匹配。

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = 添加 AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = 搜索已安装的内容

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = 无法添加该 AppImage。

# In the card of the Installed page when nothing is installed.
store-installed-empty = 你从“发现”、从 GitHub 或以 AppImage 安装的应用、来自 .deb 文件的程序以及插件，都会显示在这里。

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = 没有与“{ $find }”匹配的已安装内容。

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = 删除需要确认两次：请在三秒内再按一次。

# On an installed thing's line: its version.
store-installed-version = 版本 { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = 已暂缓

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = 已暂缓；{ $waits }

# Tip and spoken name of the button while an app is updating.
# SAFETY: needs a human review before it ships
store-installed-updating = 正在更新 { $name }

# Tip and spoken name of the button that updates an app or plugin.
# SAFETY: needs a human review before it ships
store-installed-update = 更新 { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = 关于

# Under an installed thing's row while it is being taken off.
# SAFETY: needs a human review before it ships
store-installed-removing = 正在删除……

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = 没有会话总线，无法询问要选哪个文件。

# Title of the window that asks which AppImage to add.
store-add-choose-title = 选择 AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = 无法联系文件选择器。

# Words in the Installed card while an AppImage is being added.
store-add-adding = 正在添加……

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } 已经在这里了：请看“已安装”。

# Words once an AppImage is in the ring.
store-add-in-ring = 已添加。现在它在你的列表中了。

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = 这不是这台电脑可以使用的 Flatpak 引用。

# Words under the search box while a Flatpak reference is installed; the source is given.
# SAFETY: needs a human review before it ships
store-add-installing-ref = 正在从 { $host } 安装 { $name }……

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = { $name } 已安装。

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak 不肯安装它。

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = 无法安装 { $name }。{ $why }

# Words in the Installed card while an AppImage replaces another.
store-add-replacing = 正在替换……

# Heading of the question when an AppImage has the name of one already there.
store-replace-head = { $name } 已经在这里了

# Small heading over the AppImage being brought.
store-replace-new = 要装入的这个

# Small heading over the AppImage already there.
store-replace-old = 已经在这里的这个

# Tip of the round button that keeps the AppImage already there.
# SAFETY: needs a human review before it ships
store-replace-skip = 跳过：保留已经在这里的

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = 替换

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = 它没有完成，也没有给出原因。

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = 显示问题出在哪里

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = 搜索 GitHub：输入后按回车键

# Heading of the card of matches from AppImageHub.
store-gh-hub = 在 AppImageHub 上（{ $n }）

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = 按回车键，也可以搜索整个 GitHub。

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = 没有询问 GitHub：没有可用的互联网。

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = 正在询问 GitHub……

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = GitHub 上没有更多了。

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
       *[other] 正在 { $n } 个项目中查找 AppImage 和 Flatpak 文件……
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = GitHub 上的匹配项中，没有一个带有适用于这台电脑的 AppImage 或 Flatpak 文件。

# Heading of the card of matches from GitHub.
store-gh-on = 在 GitHub 上

# Heading of the card that holds the search box.
store-gh-search-card = 搜索 GitHub

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = GitHub 上以 AppImage 或 Flatpak 文件提供的应用。没有人检查这些文件：请只安装你信任的人提供的。

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = 这台电脑没有连接互联网时，无法搜索 GitHub。

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak 和 AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = 查看文件

# Button that installs a GitHub project or file.
# SAFETY: needs a human review before it ships
store-gh-install = 安装

# Link to a project's page on GitHub.
store-gh-open = 在 GitHub 上打开

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }，{ $size }，发布版本 { $tag }

# Name of the GitHub key's button and of its box.
store-gh-key = GitHub 密钥

# Name of the GitHub key's button when a key is stored.
store-gh-key-stored = GitHub 密钥：已保存一个

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = 你几乎肯定不需要它。没有密钥时，GitHub 每小时从这台电脑回答 60 次提问；有密钥时回答 5,000 次。密钥只是提高这个上限。它只用来查看公开文件，别无他用：它不会让 Trindle 进入你的账户，创建时你也不用勾选任何权限。它保存在这台电脑的密钥环中，只会发送给 GitHub。

# In the GitHub key's box: how to make a key.
store-gh-key-how = 在 GitHub 上创建一个密钥，随便取个名字，不要改动任何权限，按“Generate token”，然后把显示的内容粘贴到这里。

# Red words under the GitHub key's box when the key could not be removed.
# SAFETY: needs a human review before it ships
store-gh-key-not-removed = 无法删除它：{ $why }

# Red words under the GitHub key's box when the key could not be kept.
store-gh-key-not-kept = 无法保存它：{ $why }

# Placeholder of the GitHub key's box when a key is stored already.
store-gh-key-keep-empty = 留空即保留原密钥

# Placeholder of the GitHub key's box.
# SAFETY: needs a human review before it ships
store-gh-key-paste = 粘贴密钥

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = 删除密钥

# Link in the GitHub key's box to the page where a key is made.
store-gh-key-make = 在 GitHub 上创建密钥

# Tip of the round button that keeps the GitHub key typed.
# SAFETY: needs a human review before it ships
store-gh-key-keep = 保留密钥

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = 它最新的发布版本中，没有任何内容可以安装在这里。

# Word on the file's dial and under a GitHub file when it was added.
store-added = 已添加

# Line under the title of the Move your apps page.
store-bk-line = 你的应用和设置，放在一个可以带到任何地方的文件里。

# Heading of the card that makes a backup.
store-bk-card = 备份

# Tip of the round button while the backup is being made.
store-bk-making-tip = 正在制作备份

# Main button that makes the backup.
store-bk-make = 制作备份

# Tip of the round button that shows the backup in Folders.
store-bk-show = 在“文件夹”中显示

# Name of the switch for the list of apps in a backup.
store-bk-apps = 应用

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
       *[other] 你的 { $n } 个 Flatpak 应用的列表。
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } 另一台电脑会从 Flathub 重新安装它们，所以文件很小。

# Name of the switch for the apps' own data in a backup.
store-bk-data = 应用自己的数据

# Under the Apps' own data switch.
store-bk-data-sub = 每个应用的设置和已保存的工作，不含缓存。

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = 整体保留，因为它们都是单个文件。

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Trindle 设置

# Under the Trindle settings switch.
store-bk-settings-sub = 圆环、壁纸选择、颜色、键盘、电源、默认应用和你的头像。

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } 目前 { $size }。

# Name of the row that says where the backup goes.
store-bk-where = 保存位置

# Under that row: the folder and the file's name.
store-bk-where-sub = 你的主文件夹，名为 { $name }

# Heading of the card that puts a backup back.
store-bk-back-card = 恢复备份

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = 选择备份文件

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Trindle 备份，或由 { $flat } 制作的备份。

# Note in the card that puts a backup back.
# SAFETY: needs a human review before it ships
store-bk-back-note = 备份中的应用会被安装，数据和设置会被恢复。你的文档和图片不在其中：请自己把它们复制过去。

# Said while a backup is being made.
store-bk-making = 正在备份……这可能需要几分钟。

# Said when a backup is made: the file and its size.
store-bk-made = 文件是 { $path }（{ $size }）。请把它复制到 U 盘或云存储中带走。

# Red words when no connection to the file chooser exists.
store-bk-no-bus = 没有选择备份，因为 Trindle 没有连上文件选择器。

# Title of the window that asks which backup to use.
store-bk-choose-title = 选择备份

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Trindle 和 Flat 备份

# Red words when the file chooser gave no answer.
store-bk-no-answer = 没有选择备份，因为文件选择器没有响应。

# Said while a backup file is read.
store-bk-reading = 正在读取备份……

# Said when a backup was put back.
store-bk-logout = 请退出登录后重新登录，以查看所有设置。

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } 请退出登录后重新登录，以查看所有设置。

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
       *[other] { $apps } 个应用，其中 { $with } 个带有设置。正在开始……
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] 还没有添加制作者自己的软件源。
       *[other] 已添加 { $n } 个制作者的软件源，并随 Debian 的更新保持最新。
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }。已添加，并保持最新。

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }。软件源已添加，但程序没有安装。

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }。已通过其他方式安装。

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = 删除它及其软件源

# Link to a maker's page, naming the program.
store-src-open = 打开 { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = 登录 Tailscale

# Tip of the round button that adds a maker's source, naming the program.
# SAFETY: needs a human review before it ships
store-src-add = 添加 { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = 可以添加

# Name of the field for the address of a source.
store-src-address = 地址

# Under the address field.
store-src-address-sub = 软件仓库的地址，见其制作者的安装说明。

# Name of the field for the suite of a source.
store-src-suite = 套件

# Under the suite field.
store-src-suite-sub = 通常是 trixie 或 stable。

# Name of the field for the components of a source.
store-src-components = 组件

# Under the components field.
store-src-components-sub = 通常是 main。

# Name of the field for a source's signing key.
store-src-key = 签名密钥

# Under the signing key field.
store-src-key-sub = 其签名密钥的地址。

# Name of the field for the program to install from a source.
store-src-program = 要安装的程序

# Under the program field.
store-src-program-sub = 它的软件包名称，如果你想现在就安装它。

# Placeholder of the program field.
store-src-program-hint = 可选

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = 添加此软件源

# Heading of the card that adds another source.
store-src-another = 添加另一个软件源

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = 软件源可以为整台电脑安装更新，所以只添加你信任的制作者。每个都需要管理员密码。

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = 删除此软件源

# Heading of the card of sources already on the computer.
store-src-already = 已在这台电脑上

# Red words when the addresses of a new source are not secure.
store-src-bad-address = 两个地址都必须以 https:// 开头

# Red words when the suite or components are not plain words.
store-src-bad-words = 套件和组件必须是普通词语。

# Red words when the program is not a package name.
store-src-bad-package = 这不是软件包名称。

# Word on a source's dial while its job starts.
store-src-starting = 正在启动

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = 正在打开 Tailscale 的登录页面……

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = 请在已打开的页面上登录；之后会自动完成。

# Said under Tailscale's row when it is connected.
store-src-connected = 已连接

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
store-bk-http-line = 通往应用的链接没有加密。

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = 未加密

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = 这个地址没有加密。途中的人可能看到所发送的内容，还可能把备份换成另一个。

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = 我明白并接受这个风险

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = 恢复

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = 取消

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = 要移除 { $name } 吗？

# Under that heading: how much saved data the app has.
store-flatpak-ask-size = 它保存的数据占用 { $size }。

# Under that heading when the app has saved no data.
store-flatpak-ask-none = 它没有保存任何数据。

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = 正在计算它保存的数据...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = 删除的数据无法恢复。

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = 移除，保留其数据

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = 移除并删除其数据

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = 仅删除其数据

# Why Delete its data only is greyed.
store-flatpak-data-only-none = 它没有可删除的已保存数据。

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = 取消：两者都保留

# Said under the card or the row once only the data has been deleted.
store-flatpak-data-gone = 其已保存的数据已删除。

# Words while only the data is being deleted.
store-flatpak-data-going = 正在删除其已保存的数据...

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = 无法统计其已保存的数据。

store-hub-warning = AppImageHub 只列出项目：没有人审查或扫描它们，也不一定有签名。Trindle 在有私有主文件夹的盒子里运行每个 AppImage：起初它可以使用互联网和声音，看不到你的任何文件。

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = AppImage 的来源只是提示，不是检查结果。

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = 在 GitHub：{ $repo }

# The same, with how many stars the project has.
store-from-github-stars = 在 GitHub：{ $repo }，{ $stars } 个星标

# Where an AppImage comes from: a website, by its name.
store-from-site = 不在 GitHub：来自网站 { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = 来源不明

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = 打开其网站

# When an app was last updated.
store-age-updated = 最后更新：{ $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = 最后更新：{ $date }。已超过六个月未更新，可能有未修复的安全漏洞。

# When the source does not say when an app was last updated.
store-age-updated-unknown = 最后更新：不详

# When an app's project was made.
store-age-created = 创建于 { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = 创建时间：不详

# A date the source does not give, on a line of Facts.
store-age-not-known = 不详

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = 添加之前

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
       *[other] AppImageHub 列出了 { $count } 个 AppImage。
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = 正在获取 AppImageHub 的列表…

# The same when the list could not be fetched.
store-browse-failed = 无法获取 AppImageHub 的列表。

# The same with no internet and no copy kept.
store-browse-offline = 这台电脑未连接互联网时，无法获取 AppImageHub 的列表。

# In the list when the kind chosen has none.
store-browse-none = AppImageHub 没有列出这一类。

# Under the rows: how many are shown of how many.
store-browse-showing = 显示 { $n } 个中的 { $shown } 个。

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = 添加到 Trindle 的应用

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = 查看详情和文件



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = 检查更新已关闭

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = OpenAI 签署其下载包后，ChatGPT 才会回来。



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = 在 Flathub 和 AppImageHub 上有 { $apps } 可供选择；{ $more }。

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = 在 AppImageHub 上有 { $apps } 可供选择。

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = 全部

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage：{ $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage 未经检查，没有人审核或扫描它们。

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle 会在一个带有私有主文件夹的隔离环境中运行每个 AppImage：一开始它可能使用网络和声音，但看不到你的任何文件。

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = 不再显示

# Button on that card that goes on with the install.
store-appimage-warn-ok = 确定

# Button on that card that gives the install up.
store-appimage-warn-cancel = 取消

