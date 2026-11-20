# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the AI programs page, in the page list, as its heading and on the card listing the programs.
settings-ai-title = AI 程序

# Words people may search for to find the AI programs page, separated by spaces.
settings-ai-keywords = ai 程序 人工智能 claude codex cursor gemini aider 代理 围栏 沙盒 私有 文件夹 共享 ssh 密钥 令牌 sudo

# Help line at the foot of the AI programs page.
settings-ai-help = AI 编程程序运行在一道围栏里，看不到你的个人文件。只共享它们需要的文件夹。

# SAFETY: needs a human review before it ships
# Headline of the AI programs page.
settings-ai-note = 日常使用时，这能让 AI 程序远离你的个人文件。比如，它能阻止一个被骗的程序收集你的银行对账单并发送出去。这种保护有限，并不是保证：如果程序执意突破、电脑软件有漏洞，或者你批准了某个请求、共享了某个文件夹，东西仍有可能漏出去。

# SAFETY: needs a human review before it ships
# Note under the headline of the AI programs page.
settings-ai-inside = AI 程序启动的任何东西，例如已安装的程序和脚本，都和它一起留在围栏里。

# Red words when a change to the fence could not be made.
settings-ai-not-changed = 无法更改。

# Said beside a setting on the AI programs page once it is saved.
settings-ai-saved = 已保存

# SAFETY: needs a human review before it ships
# Title of the window for choosing a folder to share with AI programs.
settings-ai-choose-folder = 选择要与 AI 程序共享的文件夹

# SAFETY: needs a human review before it ships
# Red words when the key for AI programs to reach computers by ssh could not be made.
settings-ai-key-not-made = 无法生成密钥。

# SAFETY: needs a human review before it ships
# Label of the switch that limits the network of AI programs, and heading of its line.
settings-ai-limit-net = 只允许访问下面的网站

# SAFETY: needs a human review before it ships
# Tip of the button that removes a site from the list AI programs may reach.
settings-ai-stop-site = 不再让 AI 程序访问这个网站

# Example in the box for typing a site AI programs may reach.
settings-ai-eg-site = api.anthropic.com

# SAFETY: needs a human review before it ships
# Tip of the button that adds a site AI programs may reach.
settings-ai-let-site = 让 AI 程序访问这个网站

# SAFETY: needs a human review before it ships
# Heading of the line for adding a site.
settings-ai-add-site = 添加网站

# Under the Add a site heading.
settings-ai-add-site-sub = 填它的名称，或用 *.example.com 表示其下的所有网站。

# SAFETY: needs a human review before it ships
# Heading of the card that limits the network of AI programs.
settings-ai-network = 网络

# SAFETY: needs a human review before it ships
# Help text of the Network card (the information button).
settings-ai-network-about = 关闭时，AI 程序和你使用网络的方式相同。开启时，它们只能通过安全（https）连接访问下面的网站，围栏本身没有网络。程序的工具必须使用提供给它们的代理设置。请加入 AI 服务的网站，以及安装时需要的软件包来源的网站。

# SAFETY: needs a human review before it ships
# Label of the switch that keeps AI programs inside the fence, and heading of its line.
settings-ai-keep-inside = 把 AI 程序留在围栏里

# SAFETY: needs a human review before it ships
# Under the fence switch when it is off.
settings-ai-fence-off = 围栏已关闭。AI 程序现在能看到你能看到的一切。

# SAFETY: needs a human review before it ships
# Heading of the card with the fence switch.
settings-ai-the-fence = 围栏

# Help text of The fence card (the information button).
# SAFETY: needs a human review before it ships
settings-ai-fence-about = AI 程序仍能看到并使用系统。它们得到的不是你的主文件夹，而是一个空的私有文件夹，多次运行之间会保留。里面只有你共享的文件夹。它们看不到你的其他程序、会话、密钥或密码。不是通过此列表中的名称启动的程序，不受围栏限制。

# SAFETY: needs a human review before it ships
# Label of the switch that fences one AI program: its name is { $name }.
settings-ai-fence-name = 围住 { $name }

# Tip of the button that takes one program off the list: its name is { $name }.
settings-ai-take-off = 把 { $name } 从列表中移除

# Label of the switch for using a program's existing settings folder: its name is { $name }.
settings-ai-own-for = 为 { $name } 使用我现有的设置文件夹

# Heading of the line with the switch for using a program's existing settings folder.
settings-ai-own = 使用我现有的设置文件夹

# Under the existing settings folder heading.
settings-ai-own-sub = 里面保存着这个程序过去的对话。

# Example in the box for typing the name of an AI program.
settings-ai-eg-program = claude

# Tip of the button that adds an AI program, and heading of its line.
settings-ai-add-program = 添加程序

# SAFETY: needs a human review before it ships
# Help text of the AI programs card (the information button).
settings-ai-programs-about = 这些程序会在围栏内启动。“使用我现有的设置文件夹”关闭时，程序会用全新的私有文件夹启动。每个程序都在围栏内自行登录。你自己的登录不会被共享。

# Heading of the line shown when no folder is shared with AI programs.
settings-ai-nothing-shared = 没有共享任何内容

# SAFETY: needs a human review before it ships
# Label of the switch that lets AI programs only read a shared folder.
settings-ai-only-read = 只允许它读取这个文件夹

# SAFETY: needs a human review before it ships
# Tip of the button that stops sharing a folder with AI programs.
settings-ai-make-private = 把这个文件夹重新设为私有

# Under a shared folder's path, beside the switch that makes it read only.
settings-ai-only-read-short = 只允许读取

# SAFETY: needs a human review before it ships
# Tip of the button that shares a folder, and heading of its line.
settings-ai-share-folder = 共享文件夹

# Under the Share a folder heading.
settings-ai-share-folder-sub = 在同一位置可见，可读可写。

# SAFETY: needs a human review before it ships
# Heading of the card listing folders shared with AI programs.
settings-ai-shared = 与 AI 程序共享的内容

# SAFETY: needs a human review before it ships
# Help text of the shared folders card (the information button).
settings-ai-shared-about = 你主文件夹中的其他所有文件夹，对 AI 程序保持私有。隐藏文件夹，以及存放密钥、密码或邮件的文件夹，永远无法共享。在此共享的文件夹，程序下次启动时就能使用。

# SAFETY: needs a human review before it ships
# Tip of the button that removes a computer from the list AI programs may reach by ssh.
settings-ai-stop-computer = 不再让 AI 程序访问这台电脑

# Example in the box for typing a computer AI programs may reach by ssh.
settings-ai-eg-computer = nas.local

# SAFETY: needs a human review before it ships
# Tip of the button that adds a computer AI programs may reach by ssh.
settings-ai-let-computer = 让 AI 程序访问这台电脑

# SAFETY: needs a human review before it ships
# Heading of the line for adding a computer.
settings-ai-add-computer = 添加电脑

# SAFETY: needs a human review before it ships
# Note showing the public key to put on each computer: the key is { $key }.
settings-ai-put-key = 请把这个密钥放到每台电脑上：{ $key }

# SAFETY: needs a human review before it ships
# Tip of the button that makes a key for AI programs.
settings-ai-make-key = 生成密钥

# SAFETY: needs a human review before it ships
# Heading of the line for making a key for AI programs.
settings-ai-make-key-for = 为 AI 程序生成密钥

# Under the Make a key heading.
settings-ai-make-key-sub = 一个单独的密钥，存放在 AI 程序看不到的文件夹之外。

# SAFETY: needs a human review before it ships
# Heading of the card listing computers AI programs may reach by ssh.
settings-ai-ssh = 它们可以通过 ssh 访问的电脑

# SAFETY: needs a human review before it ships
# Help text of the ssh card (the information button).
settings-ai-ssh-about = 在围栏内，ssh 通过 Trindle 连接。Trindle 使用自己的密钥，并且只用于这里列出的电脑。AI 程序永远看不到你自己的密钥。

# SAFETY: needs a human review before it ships
# Tip of the button that stops giving AI programs a key.
settings-ai-stop-key = 不再把这个密钥给 AI 程序

# Example in the box for typing a key to give AI programs: a name, an equals sign and the value.
settings-ai-eg-key = 名称=值

# SAFETY: needs a human review before it ships
# Tip of the button that gives AI programs a key.
settings-ai-give-key = 把这个密钥给 AI 程序

# SAFETY: needs a human review before it ships
# Heading of the line for adding a key.
settings-ai-add-key = 添加密钥

# SAFETY: needs a human review before it ships
# Heading of the card listing keys given to AI programs.
settings-ai-keys = 提供给 AI 程序的密钥

# SAFETY: needs a human review before it ships
# Help text of the keys card (the information button).
settings-ai-keys-about = 你会话中的密钥和令牌永远不会交给 AI 程序。它们只会得到你在这里输入的密钥。
# Under the AI programs card, greyed, when none of the listed AI programs is on this computer.
settings-ai-none-installed = 这台电脑上没有 AI 程序。
# Under the box, when the program typed in to add is not on this computer.
settings-ai-not-installed = 这台电脑上没有该程序。
