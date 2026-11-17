
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = 权限

# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = 权限 允许 隐私 flatpak flathub 沙盒 互联网 网络 文件 声音 麦克风 摄像头 蓝牙 通知 插件 deb x11 xwayland 旧程序 steam wine java 屏幕 终端 电脑 thunderbolt usb4 扩展坞 设备

# Help line at the foot of the Permissions page.
# SAFETY: needs a human review before it ships
settings-permissions-help = 这一页显示每个应用可以使用什么。来自 Flathub 的应用、安装在各自文件夹中的程序和插件，都在各自的隔离环境中运行，默认设有限制。对 Flathub 应用的更改会在它下次启动时生效。你已允许的 Thunderbolt 和 USB4 设备每次都会直接连接而不再询问。如果想再次被询问，请忘记该设备。

# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = 无法更改。

# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = 已保存，但 Trindle 无法重新启动该插件。更改会在它下次启动时生效。

# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = 已保存

# Help text of the AppImages card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-images = 每个 AppImage 都在各自的隔离环境中运行，拥有私有的主目录。更改会在该 AppImage 下次启动时生效。

# Help text of the Started from Folders card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-loose-programs = 你从 Folders 启动的每个程序都在各自的隔离环境中运行，拥有私有的主目录。更改会在该程序下次启动时生效。

# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }：{ $words }

# Under the Allow older programs switch.
settings-permissions-older-why = 每个程序单独运行。关闭后，它们无法打开。

# Heading of the card with the switch for older programs.
settings-permissions-older = 旧程序

# Label and tip of the switch that lets older programs open.
# SAFETY: needs a human review before it ships
settings-permissions-allow-older = 允许旧程序

# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = 这些端口只用于屏幕，所以没有设备能访问这台电脑的内存。无需询问。

# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = 这些端口只用于 USB 和屏幕，所以没有设备能访问这台电脑的内存。无需询问。

# Help text on the Thunderbolt and USB4 card when the ports let every device in.
# SAFETY: needs a human review before it ships
settings-permissions-lets-in-all = 这些端口允许所有设备连接，所以无需询问。这是在电脑的固件中设置的。

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = 这些端口只供屏幕使用。

# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = 这些端口只供 USB 和屏幕使用。

# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
# SAFETY: needs a human review before it ships
settings-permissions-short-firmware = 固件允许所有设备连接。

# Under a package's name: its version number is { $v }.
settings-permissions-version = 版本 { $v }

# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = 版本 { $v }。它没有请求任何权限。

# Under a plugin's name: its version number is { $v }, and { $notes } lists what it may do that matters most, such as "it can act as you and it can type into your windows".
settings-permissions-version-notes = 版本 { $v }：{ $notes }。

# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }。{ $detail }

# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = 无法忘记。

# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt 和 USB4

# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
# SAFETY: needs a human review before it ships
settings-permissions-thunder-about = 你已允许的 Thunderbolt 和 USB4 设备每次都会直接连接而不再询问。如果想再次被询问，请忘记该设备。{ $about }

# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-ports = 端口

# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = 正在检查端口…

# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = 已允许的设备

# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = 正在加载…

# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = 内存保护

# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = 没有 Thunderbolt 或 USB4 端口

# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = 所以无需询问。

# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = 没有设备以这种方式连接。

# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = 无需询问

# Under Devices allowed when the ports let every device in themselves.
# SAFETY: needs a human review before it ships
settings-permissions-every-one = 全部。由端口自行决定。

# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
settings-permissions-iommu = 这台电脑还会阻止设备访问不属于它们的内存，所以风险较小。如果你忘记某个设备，它会保持连接，直到你拔出它。

# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
settings-permissions-no-iommu = 如果你忘记某个设备，它会保持连接，直到你拔出它。下次再插入时，会重新询问你。

# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
settings-permissions-asked-before = 新设备会先询问你

# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Trindle 无法读取已允许设备的列表：{ $why }

# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = 目前还没有设备可以不经询问直接连接。

# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
# SAFETY: needs a human review before it ships
settings-permissions-forget-device = 忘记此设备

# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = 一个 Thunderbolt 设备

# Under an allowed device's name when its maker is not known.
# SAFETY: needs a human review before it ships
settings-permissions-may-always = 可始终连接

# Under an allowed device's name: the maker is { $vendor }.
# SAFETY: needs a human review before it ships
settings-permissions-made-by = 制造商：{ $vendor }。可始终连接。

# Headline while the list of apps is being read.
settings-permissions-reading-apps = 正在检查应用可以使用什么…

# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
       *[other] 正在检查 { $n } 个应用可以使用什么…
    }

# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = 你还没有安装任何在各自隔离环境中运行的应用或插件。

# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = 有一个应用在自己的隔离环境中运行，只能使用你在这里允许的内容。

# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
       *[other] 有 { $n } 个应用和插件各自在自己的隔离环境中运行，只能使用你在这里允许的内容。
    }

# Heading of the card for apps from Flathub.
settings-permissions-flathub-apps = Flathub 应用

# Heading of the card for programs installed in their own folder.
settings-permissions-own-folder = 在各自的文件夹中

# Heading of the card for plugins.
settings-permissions-plugins = 插件

# Heading of the card for AppImages.
settings-permissions-appimages = AppImage

# Heading of the card for programs started from Folders.
settings-permissions-from-folders = 从 Folders 启动

# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = 没有已安装的。

# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = 暂时没有。

# Help text of the Flathub apps card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-flathub-about = 来自 Flathub 的每个应用都在各自的隔离环境中运行，只能使用这里允许的内容。更改会在应用下次启动时生效。

# Help text of the In their own folder card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-debs-about = 随 Trindle 附带的程序（例如终端），以及你从 .deb 文件安装的程序。每个程序都在各自的文件夹中运行，默认设有限制。

# Help text of the Plugins card (the information button).
# SAFETY: needs a human review before it ships
settings-permissions-plugins-about = 插件一开始就受到限制，你只能允许插件所请求的内容。在后台运行的插件会在你更改后立即重新启动。

# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = 输入代码

settings-permissions-codes-about = Trindle 在你输入时会把一些短代码换成文字。每个代码都标明它来自哪个插件。

settings-permissions-codes-none = 没有启用的代码

# Button on a plugin's row: stops that plugin's codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-off = 关闭 { $name } 的代码

settings-permissions-codes-count =
    { $n ->
       *[other] { $n } 个代码，{ $id }
    }

# A code's second line: what it types and who owns it.
settings-permissions-code-types = 为 { $name } 输入“{ $words }”

settings-permissions-codes-more =
    { $n ->
       *[other] 另有 { $n } 个代码已启用
    }

# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread-line = 无法读取插件文件夹

# Red words under that line.
# SAFETY: needs a human review before it ships
settings-permissions-codes-unread = Trindle 无法读取插件文件夹，因此部分代码可能缺失。

settings-permissions-not-added = 从 Folders 打开，不在 Apps 中

settings-permissions-kind-tip = 应用类型


# Heading of the card for programs that are not in a box.
settings-outside-heading = 沙盒之外的程序

# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle 每隔几秒检查一次，并告诉你检查到了什么。它无法阻止这些程序，也可能漏掉只运行一瞬间的程序。隔离环境中的程序只能使用其隔离环境允许的内容。

# Label of the switch for the notice about the network.
settings-outside-network = 程序使用网络时告诉我

# Label of the switch for the notice about the person's files.
settings-outside-files = 程序打开我的文件时告诉我

# Under those two switches.
settings-outside-once = 每个程序只告诉一次。

# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = 不再忽略 { $name }

# Under a program marked fine for the network.
settings-outside-fine-network = 使用网络没问题。

# Under a program marked fine for the person's files.
settings-outside-fine-files = 打开你的文件没问题。

# The line in the card when no program is marked fine.
settings-outside-no-fine = 没有被标记为没问题的程序。

# Heading of the card for downloaded files.
settings-downloads-heading = 下载的文件

# Help text of that card (the information button).
settings-downloads-about = 在“下载”中的文件，以及浏览器标记为已下载的文件，会在隔离环境中打开。打开它的程序只能得到那个文件，只能读取它，不能上网，也看不到你的文件夹。这不包括浏览器自己的“打开”按钮，它会直接启动程序。

# Label of the switch that opens downloaded files in a box.
settings-downloads-box = 在沙盒中打开下载的文件

# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = 正常打开 { $kind } 文件

# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = { $kind } 文件

# Under a file type's switch.
settings-downloads-normal-sub = 它们正常打开，可以上网。

# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = 还没有文件类型。下载的文件在隔离环境中打开后，类型会显示在这里。
