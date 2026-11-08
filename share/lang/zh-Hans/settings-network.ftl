
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } 已被电脑上的开关或按键关闭。请先把它打开，然后再试一次。

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } 被系统屏蔽，且无法解除屏蔽。

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = 密码未被接受。请再试一次。

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = 为此网络保存的密码未被接受。请用垃圾桶忘记该网络，然后重新加入。

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = 找不到该网络。请检查网络名称，以及它是否在范围内。

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = 网络允许这台电脑加入，但没有分配地址。请重试，或重启路由器。

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = 网络不允许这台电脑加入。请靠近一些，然后再试一次。

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Wi-Fi 网络不能使用这样的密码。请检查后再试一次。

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = 无法加入该网络。

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = 用你的密码可打开

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = 用你的密码可打开和更改

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = 家中任何人都可以打开

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = 家中任何人都可以更改

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }：{ $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = 停止共享 { $name }

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = 未托管

# State of a network device that cannot be used.
settings-network-state-unavailable = 不可用

# State of a network device that is not connected.
settings-network-state-disconnected = 未连接

# State of a network device that is getting ready to connect.
settings-network-state-preparing = 准备中

# State of a network device that is being set up.
settings-network-state-configuring = 配置中

# State of a network device that needs a password.
# SAFETY: needs a human review before it ships
settings-network-state-needs-password = 需要密码

# State of a network device that is getting an address.
settings-network-state-address = 正在获取地址

# State of a network device whose connection is being checked.
settings-network-state-checking = 检查中

# State of a network device that is waiting.
settings-network-state-waiting = 等待中

# State of a network device that is connected.
settings-network-state-connected = 已连接

# State of a network device that is disconnecting.
settings-network-state-disconnecting = 正在断开

# State of a network device that failed.
settings-network-state-failed = 失败

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = 已通过网线连接，并通过 Wi-Fi 连接到 { $name }。

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] 已连接到 { $name }，信号很强。
        [good] 已连接到 { $name }，信号良好。
        [fair] 已连接到 { $name }，信号一般。
       *[other] 已连接到 { $name }，信号较弱。
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = 已通过网线连接。

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi 已关闭，所以这台电脑没有连接。

# Headline of the page when not connected.
settings-network-top-none = 未连接。请在下方选择一个网络。

# Headline of the page while the networks are looked for.
settings-network-top-looking = 正在查找网络…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = 无法查看网络：NetworkManager 没有运行。

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = 系统总线

# Name of the page in the list of settings pages and its heading.
settings-network-title = 网络

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi 无线 互联网 以太网 有线 密码 连接 防火墙 共享 打印机 文件夹

# Help text at the top of the page.
settings-network-help = 打开或关闭 Wi-Fi，加入网络，忘记不再需要的网络，或让其他电脑连接。

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] 来自 { $name } 的信号很强。
        [good] 来自 { $name } 的信号良好。
        [fair] 来自 { $name } 的信号一般。
       *[other] 来自 { $name } 的信号较弱。
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = 它的芯片需要更新的 Linux。

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = 这台电脑没有可用的 Wi-Fi。

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = 已关闭。打开后才能加入网络。

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = 已开启，尚未加入网络。

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = 使用 Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi 和网线

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi 让这台电脑无需网线就能加入网络。关闭后，它不再查找网络，可以节省一点电量。不广播名称的网络不会出现在范围内的网络列表中：请在这里准确输入它的名称来加入，注意大小写。如果这台电脑的 Wi-Fi 芯片尚未被它的 Linux 中的任何驱动支持，就需要比 Trindle 当前所用更新的 Linux。网线有单独的一行，带有连接或断开的按钮。

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = 允许其他电脑连接

# Small line under that switch.
settings-network-share-switch-sub = 用于在家中共享文件夹和打印机。

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = 正在查找共享文件夹…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = 无法读取共享文件夹

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = 暂时无法访问

# Small line under it.
settings-network-not-reachable-sub = 开关关闭时，其他人无法打开这些文件夹。

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = 没有共享的文件夹

# Small line under it.
settings-network-no-shared-sub = 在“文件夹”中右键点击一个文件夹即可共享。

# Heading of the Sharing card.
settings-network-sharing = 共享

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = “允许其他电脑连接”关闭时，网络上的任何设备都无法访问这台电脑，除非是它自己先发起的连接，所以同一 Wi-Fi 上的人无法试探。打开它即可与家中的其他电脑共享文件夹或打印机。要共享文件夹，请在“文件夹”中右键点击它并选择“在网络上共享”。每个共享都会说明谁可以做什么。用你的密码：其他人用你的共享密码可以打开文件，或打开并更改文件。家中任何人：网络上的任何人无需密码就可以打开文件，或打开并更改文件。开关关闭时，家中的其他电脑都无法打开其中任何一个。

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = 正在停止…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = 正在等待输入密码...

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = 已允许共享

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = 已拒绝访问

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = 无法打开 Wi-Fi。

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = 无法关闭 Wi-Fi。

# Name of a wired network device that has no name of its own.
settings-network-ethernet = 以太网

# Tip of the button that disconnects a network or a cable.
# SAFETY: needs a human review before it ships
settings-network-disconnect = 断开连接

# Tip of the button that connects a network or a cable.
settings-network-connect = 连接

# Shown under a network or cable while it is being connected.
settings-network-connecting = 正在连接...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = 范围内没有网络

# Small line under it.
settings-network-none-in-range-sub = 每次打开此页面都会重新查找。

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = 范围内的网络

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = 这台电脑能收到的所有 Wi-Fi 网络。页面打开时，正在使用的排在最前，然后是这里保存过的，其余的按信号强弱排列；页面打开期间，各行位置不会变。按箭头即可加入；有加密的网络首次加入时会要求输入密码。垃圾桶会忘记已保存的网络及其密码。开放网络没有加密，附近的人可能看到通过它发送的内容。

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = 打开 Wi-Fi 即可查看范围内的网络。

# Part of a network's small line: it is saved on this computer.
settings-network-saved = 已保存

# Part of a network's small line: it needs a password.
# SAFETY: needs a human review before it ships
settings-network-secured = 已加密

# Part of a network's small line: it has no password and is not encrypted.
# SAFETY: needs a human review before it ships
settings-network-open = 开放，未加密

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = 忘记此网络

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = 无法忘记该网络。

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = 密码

# Tip of the button that joins the network.
settings-network-join = 加入

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = 取消

# Shown under a network while it is being joined.
settings-network-joining = 正在加入...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Wi-Fi 密码至少需要 8 个字符。

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = 加入隐藏网络

# Small line under it.
settings-network-hidden-sub = 请准确输入名称，注意大小写。

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = 网络名称

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = 密码（如果有）

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = 需要输入网络名称。

# Shown under a network after it was joined.
settings-network-joined = 已加入

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } 但它仍被保存了：重新加入之前，请先用垃圾桶忘记它。

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = 无法断开连接。

