
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = 你的插件

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = 插件 附加组件 扩展 已安装 移除 设置

# Help line at the foot of the Your plugins page.
# SAFETY: needs a human review before it ships
settings-plugins-help = 插件为 Trindle 增加功能：圆环中的图标、此处的设置、通知，或登录时就运行的内容。每个插件都在隔离环境中运行，在你于“权限”中允许之前，什么都不被允许；本页标题末尾的按钮可打开“权限”。每个插件的卡片里有它自己的设置；卡片上的垃圾桶按两次，即可移除该插件。

# Red words when the window that asks what a plugin may use could not be opened.
# SAFETY: needs a human review before it ships
settings-plugins-window-failed = 无法打开询问插件可使用哪些内容的窗口。

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = 版本 { $v }。

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = 版本 { $v }。{ $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } 没有提出任何请求，所以它在隔离环境中运行，不被允许任何内容。

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
       *[other] { $name } 请求：{ $list }；已允许 { $allowed } 项，共 { $asks } 项。它在隔离环境中运行，只能使用你允许的内容。
    }

# Under the line that tells a plugin needs more allowed, beside its Allow button.
# SAFETY: needs a human review before it ships
settings-plugins-needs-more = “允许”会列出它请求的每一项，每项都有开关。

# Heading of the line about a plugin on its card.
settings-plugins-about-it = 关于它

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = 更改它可使用的内容

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = 移除此插件

# Red words when a plugin could not be removed.
# SAFETY: needs a human review before it ships
settings-plugins-not-removed = 无法移除。

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = 在此输入

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } 没有自己的设置。

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = 按目前情况，{ $name } 没有可显示的设置。

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] 没有安装插件。
       *[other] 已安装 { $n } 个插件：{ $list }。
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = 插件

# Heading of the line shown when no plugin is installed.
settings-plugins-none = 没有已安装的插件

# Under the None installed heading.
settings-plugins-none-sub = “应用”里有可添加的插件。

# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = 打开权限

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = 正在询问 { $name }。

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } 没有回应。它的服务可能已停止。

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = 已保存。

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } 不接受这个更改。

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } 无法完成。

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } 无法完成：{ $why }。

