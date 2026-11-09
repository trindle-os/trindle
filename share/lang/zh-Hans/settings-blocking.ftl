
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = 刚刚

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
       *[other] { $n } 分钟前
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
       *[other] { $n } 小时前
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
       *[other] { $n } 天前
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = 正在等待输入密码...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = 拦截已开启

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = 拦截已关闭

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = 已保存

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = 请输入网站名称，例如 example.com。

# Headline of the page when blocking is off.
settings-blocking-top-off = 拦截已关闭。域名查询照常发送到你所在网络自己的服务器。

# Headline of the page while blocking is starting.
settings-blocking-top-starting = 拦截正在启动。

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = 广告和恶意网站已被拦截，域名查询会私密地发送到 { $provider }。

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = 在此网络上无法连接私密服务器，暂时改用你所在网络自己的服务器。拦截仍然有效。

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = 广告和恶意网站已被拦截。域名查询发送到你所在网络自己的服务器，对方可以看到这些查询。

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = 广告和恶意网站已被拦截。域名查询通过 VPN 发送到 VPN 自己的服务器。

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = 广告和恶意网站已被拦截。

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = 启动中

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = 关闭

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = 拦截正在启动

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = 拦截已关闭

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = 今日已拦截

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = 今日已拦截的查询

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = 列表

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = 获取中

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = 暂无

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = 网站列表

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = 个网站在列表中，{ $ago }获取

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = 尚未拦截任何网站

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = 网站会显示在这里，最新的在最前。

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = 始终放行此网站

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = 最近拦截

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = 网站被拦截时会显示在这里，最新的在最前。点击加号可从现在起放行该网站。这里只显示被拦截的网站，被放行的网站不会被记录。

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = 开启拦截后，可在这里查看被拦截的内容。

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = 例如 example.com

# Heading of the line with the box for adding a site.
# SAFETY: needs a human review before it ships
settings-blocking-add = 添加网站

# Small line under "Add a site".
# SAFETY: needs a human review before it ships
settings-blocking-add-sub = 该网站及其所有子域名都不会被拦截。

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = 重新拦截此网站

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = 始终放行

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = 这里列出的网站及其所有子域名永远不会被拦截，即使它们在列表中。点击减号可重新拦截。

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = 拦截

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = 拦截 广告 追踪器 恶意软件 诈骗 dns 私密 查询 block adverts ads trackers malware scams private lookups quad9 mullvad cloudflare allow

# Help text at the top of the page.
settings-blocking-help = 为这台电脑上的所有浏览器和应用拦截广告、追踪器和已知的恶意网站，并通过加密连接发送域名查询，这样你所在的网络无法读取或更改查询内容。

# Text of the switch that turns blocking on or off.
settings-blocking-switch = 拦截广告和恶意网站

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = 域名查询发往何处

# Help text (i button) of the Blocking card.
settings-blocking-card-about = 使用 HaGeZi 的广告、追踪器和已知恶意网站列表，每天获取一次。关闭时不会运行任何程序。域名查询通过加密连接发送到此处选择的私密服务器。Quad9 不记录是谁发出的查询。“你的网络”仍然会拦截，但不私密：你所在网络自己的服务器能看到每一次查询。

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = HaGeZi 的列表，每天获取一次。

# Heading of the line with the private server dropdown.
settings-blocking-private = 私密服务器

# Small line under Private server.
settings-blocking-private-sub = Quad9 不记录是谁发出的查询。

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = 你的网络

