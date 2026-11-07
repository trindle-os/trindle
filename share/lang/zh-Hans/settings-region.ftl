
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Region and language page, in the page list and as its heading.
settings-region-title = 地区和语言

# Words people may search for to find the Region and language page, separated by spaces.
settings-region-keywords = 语言 区域 国家 格式 日期 货币

# Help line at the foot of the Region and language page.
settings-region-help = 语言和格式在你下次登录时生效。

# Said beside the Language and Formats lists once a choice is saved.
settings-region-later = 注销后重新登录即可看到。

# Shown while the page reads the computer.
settings-region-looking = 查看中…

# Under Money and paper: the paper and measures the chosen formats use (paper is a4, letter or other; measures is imperial or metric).
settings-region-paper-measures =
    { $paper ->
        [a4] A4 纸
        [letter] Letter 纸
       *[other] 纸张
    }，{ $measures ->
        [imperial] 英寸和磅
       *[other] 公制单位
    }

# Under Money and paper: { $money } is the money sign and { $paper } the paper and measures.
settings-region-money-and-paper = { $money }，{ $paper }

# Under A date and a number: a date and a number written the way the formats chosen write them.
settings-region-date-and-number = { $date } 和 { $number }

# Said beside the Add a language line once a language is installed.
settings-region-installed-done = 已安装

# Headline when the language and the formats are the same: { $lang } is the language's name.
settings-region-everything-in = 一切都使用{ $lang }。

# Headline when the formats differ from the language: the names are { $lang } and { $fmt }.
settings-region-menus-in = 菜单使用{ $lang }；日期和数字按{ $fmt }的习惯书写。

# Headline when a change is saved but waits for the next login: { $words } is the sentence so far.
settings-region-with-wait = { $words } 注销后重新登录即可看到更改。

# Said beside the Install another line while a language is made.
settings-region-installing = 正在安装

# Name of the list of languages for menus and messages, heading of its line and of the card.
settings-region-language = 语言

# Name of the list of formats and heading of its line.
settings-region-formats = 格式

# Help text of the Language card (the information button).
settings-region-language-about = 语言决定菜单和消息使用的文字。格式决定日期、时间、数字、货币、度量单位和纸张尺寸的写法，可以采用另一个国家的习惯。两者都在你下次登录时生效。

# Under the Language heading.
settings-region-language-sub = 菜单和消息，下次登录后生效。

# Under the Formats heading.
settings-region-formats-sub = 日期、数字、货币和纸张尺寸。

# Heading of the line that shows a date and a number in the chosen formats.
settings-region-date-number = 日期和数字

# Heading of the line that shows the money and paper of the chosen formats.
settings-region-money-paper = 货币和纸张

# Heading of the line that shows the language now in use.
settings-region-in-use = 当前使用

# Heading of the card for installing another language.
settings-region-add = 添加语言

# Help text of the Add a language card (the information button).
# SAFETY: needs a human review before it ships
settings-region-add-about = 安装另一种语言，这样就可以将它用于菜单或格式。生成语言需要一点时间，并需要管理员密码。

# Heading of the line shown when every language is already installed.
settings-region-all-installed = 所有语言都已安装

# Heading of the line saying how many languages are installed.
settings-region-installed = 已安装

# Under the Installed heading: how many languages are installed.
settings-region-count =
    { $n ->
       *[other] 已安装 { $n } 种语言。
    }

# Name of the list of languages that can be installed, and heading of its line.
settings-region-install-another = 安装其他语言

# Tip of the button that installs the chosen language.
# SAFETY: needs a human review before it ships
settings-region-install-this = 安装此语言（需要输入你的密码）

# Under the Install another heading.
# SAFETY: needs a human review before it ships
settings-region-needs-password = 需要管理员密码。

# Heading of the line saying how many more languages can be made.
settings-region-ready = 可添加

# Under the Ready to add heading: how many more languages can be made.
settings-region-more =
    { $n ->
       *[other] 这台电脑还可以生成 { $n } 种语言。
    }

settings-region-removing = 正在移除

settings-region-removed-done = 已移除

settings-region-remove-language = 移除此语言

settings-region-kept = 这台电脑上的语言

settings-region-kept-about = 这里安装的所有语言。可移除你不用的语言；你正在使用的语言会保留。需要管理员密码。

settings-region-in-use-stays = 正在使用，所以会保留

