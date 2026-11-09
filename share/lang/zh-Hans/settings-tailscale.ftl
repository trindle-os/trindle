
# machine translated (Simplified Chinese), not yet reviewed by a person.

# What a device runs, shown in a device's small line, when it is an iPhone or iPad.
settings-tailscale-system-ios = iPhone 或 iPad

# In a device's small line: the device is on the network now.
settings-tailscale-online = 当前在线

# In a device's small line: the device has never been seen.
settings-tailscale-not-seen = 尚未出现过

# In a device's small line: when the device was last online; the date is already written.
settings-tailscale-last-online = 上次在线：{ $day }

# Shown beside the Use Tailscale switch while the password is asked for, when Tailscale is already installed.
# SAFETY: needs a human review before it ships
settings-tailscale-waiting = 正在等待输入密码...

# Shown beside the Use Tailscale switch while the password is asked for, before Tailscale is installed.
# SAFETY: needs a human review before it ships
settings-tailscale-waiting-install = 正在等待输入密码，然后安装 Tailscale...

# Shown beside the switch after Tailscale is turned on.
settings-tailscale-done-on = Tailscale 已开启

# Shown beside the switch after Tailscale is turned off.
settings-tailscale-done-off = Tailscale 已关闭

# Shown under the Sign in line once the browser is being opened.
settings-tailscale-opening = 正在浏览器中打开登录页面...

# Red words under the Sign in line when Tailscale could not be started.
settings-tailscale-not-started = 无法启动 Tailscale。

# Red words under the Sign out line when signing out did not work.
settings-tailscale-sign-out-failed = Tailscale 无法退出登录。

# Headline of the page while Tailscale is being asked how it is.
settings-tailscale-top-asking = 正在询问 Tailscale…

# Headline of the page when Tailscale is off and not installed.
settings-tailscale-top-not-installed = 未安装 Tailscale。请在下方打开开关来安装它。

# Headline of the page when Tailscale is installed but off.
settings-tailscale-top-off = Tailscale 已关闭，所以不占用内存。

# Headline of the page when Tailscale is on but does not answer.
settings-tailscale-top-silent = Tailscale 没有响应。它可能仍在启动。

# Headline of the page when connected: the name of the private network and of this computer.
settings-tailscale-top-connected = 已连接到 { $network }，本机名称为 { $name }。

# Name used in the headline for the private network when it has no name of its own.
settings-tailscale-network-fallback = Tailscale

# Headline of the page when Tailscale is on and has to be signed in to.
settings-tailscale-top-sign-in = Tailscale 已开启。请在下方登录，加入你的私人网络。

# Headline of the page for any other state; the state is already written in lower case.
settings-tailscale-top-state = Tailscale 状态：{ $state }。

# Name of the page in the list of settings pages.
settings-tailscale-title = Tailscale

# Words the settings search looks for on this page (not shown).
settings-tailscale-keywords = tailscale vpn 私人网络 设备 远程 tailnet

# Help text at the top of the page.
settings-tailscale-help = Tailscale 把你自己的电脑和手机连成一个私人网络，无论它们身在何处。打开开关之前不会安装其中任何内容，关闭时它也不占用内存。

# Small line under the Use Tailscale switch before Tailscale is installed.
settings-tailscale-switch-sub-first = 从 Tailscale 自己的软件源安装。

# Small line under the Use Tailscale switch once Tailscale is installed.
settings-tailscale-switch-sub = 关闭：已停止，不随电脑启动。

# Text of the switch that turns Tailscale on or off, and heading of its line.
settings-tailscale-switch = 使用 Tailscale

# Help text (i button) of the Tailscale card when Tailscale is off or has not answered.
# SAFETY: needs a human review before it ships
settings-tailscale-about-plain = 第一次打开时，会从 Tailscale 自己的软件源安装 Tailscale，然后启动它。它的后台服务在开启时会占用内存，所以只有在此开关打开时才运行。此开关关闭时，Tailscale 会停止，也不会随电脑启动。防火墙会允许你自己的 Tailscale 设备进入，其他一切仍和以前一样被挡在外面。

# Help text (i button) of the Tailscale card when Tailscale is on but not yet connected.
settings-tailscale-about-sign-in = { settings-tailscale-about-plain } 登录会在浏览器中打开 Tailscale 的登录页面。请在你的每台设备上使用同一个账户。

# Help text (i button) of the Tailscale card when connected.
settings-tailscale-about-connected = { settings-tailscale-about-plain } 退出登录会使这台电脑离开你的私人网络，直到你再次登录。

# Line in the devices card when there are no other devices.
settings-tailscale-no-devices = 还没有其他设备

# Small line under "No other devices yet".
settings-tailscale-no-devices-sub = 在另一台设备上用同一个账户登录。

# Shown on the devices card while Tailscale is off.
settings-tailscale-waits-off = 打开 Tailscale 即可查看你的设备。

# Shown on the devices card while Tailscale has not answered.
settings-tailscale-waits-looking = 正在查找你的设备…

# Shown on the devices card when Tailscale needs signing in to.
settings-tailscale-waits-sign-in = 登录后即可查看你的设备。

# Shown on the devices card when Tailscale is on but not connected.
settings-tailscale-waits-connect = 连接后即可查看你的设备。

# Small line under this computer when it has no address yet.
settings-tailscale-no-address = 还没有地址。

# Small line under this computer: its address on the private network.
settings-tailscale-address = 在你的私人网络中的地址：{ $address }。

# Name of this computer's line when Tailscale gives it no name.
settings-tailscale-this-computer = 这台电脑

# Heading of the line that signs in to Tailscale, and tip of its round button.
settings-tailscale-sign-in = 登录

# Small line under the Sign in line.
settings-tailscale-sign-in-sub = 在浏览器中打开登录页面。

# Heading of the line that joins the private network again, and tip of its round button.
settings-tailscale-connect = 连接

# Small line under the Connect line.
settings-tailscale-connect-sub = 重新加入你的私人网络。

# Heading of the line that says the firewall lets the person's own devices in.
# SAFETY: needs a human review before it ships
settings-tailscale-firewall = 你自己的设备可以进入

# Small line under that heading.
# SAFETY: needs a human review before it ships
settings-tailscale-firewall-sub = 其他一切仍和以前一样被挡在外面。

# Heading of the line that signs out, and tip of its button.
# SAFETY: needs a human review before it ships
settings-tailscale-sign-out = 退出登录

# Small line under the Sign out line.
settings-tailscale-sign-out-sub = 离开你的私人网络。

# Heading of the card with the Tailscale switch.
settings-tailscale-card = Tailscale

# Heading of the card that lists the person's devices.
settings-tailscale-devices = 你的设备

# Help text (i button) of the Your devices card.
settings-tailscale-devices-about = 所有登录到你的私人网络的设备，包括它在网络中的地址、运行的系统以及上次在线的时间。要添加设备，请在你的手机或另一台电脑上安装 Tailscale，并用同一个账户登录。

