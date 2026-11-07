
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = 键盘和鼠标

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = 键盘 布局 语言 输入 大写锁定 屏幕键盘 按钮 圆环 鼠标 触摸板 指针 速度 轻点 点击 滚动 滚轮 慢 快 自然 左手 插件 快捷键 重映射 更改 交换 ctrl escape super alt mac 侧键 程序创建 keyd layout language typing caps lock on-screen keyboard button ring mouse touchpad trackpad pointer speed tap click scroll wheel natural left handed plugin shortcut remap swap side buttons

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = 这里的所有更改都会立即生效。同时按 Alt 和 Shift 可切换到第二种布局。

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = 当前输入布局：{ $layout }。

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = 按 Alt 和 Shift 切换到 { $layout }。

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock 不起作用。

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock 当作 { $key } 使用。

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = 另一个 Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = 这台电脑上没有键盘布局列表。

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = 键盘

# Heading of the line in that card.
settings-keyboard-no-list-heading = 无法使用键盘布局列表

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = 请用以下命令安装：sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
       *[other] 已更改 { $n } 个按键和按钮。
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = 布局

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = 布局

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = 变体

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = 标准

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = 无

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = 切换到

# Small words under the second layout line.
settings-keyboard-switch-to-about = 同时按 Alt 和 Shift 可在两者之间切换。

# Words behind the i of the layout card.
settings-keyboard-layout-about = 每个键输入什么由布局决定，也可以选择它的变体，例如没有死键的变体。同时按 Alt 和 Shift 可切换到第二种布局，再按一次则切换回来。所有更改立即生效。

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = 正在使用

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = 另一个 Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = 无

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Caps Lock 键的作用

# Small words under that line.
settings-keyboard-caps-acts-as-about = 另一个 Ctrl、Escape 等。

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = 圆环中的键盘按钮

# Small words under that line.
settings-keyboard-ring-button-about = 在屏幕上显示键盘。

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = 其他按键

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock 可以改作其他按键，例如另一个 Ctrl 或 Escape。键盘按钮位于圆环中央，点击后会在屏幕上显示键盘，方便用鼠标或触摸屏输入。圆环下次打开时它才会出现。

# Words under a line when a press that was not the person's own changed nothing.
# SAFETY: needs a human review before it ships
settings-keyboard-only-yours = 只有您本人的按键才能更改这一项。

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = 请按下要更改的按键或鼠标按钮。

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = 没有按下任何键，因此没有添加任何内容。

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = 输入它要变成什么，例如 Escape、Ctrl+C 或 Nothing。

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = 无法区分这次按键。请改为从列表中选择该键。

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = 所有键盘和鼠标

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = 下方也有更改，将采用下方的更改。

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = { $key } 在哪个键盘或鼠标上更改

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = { $key } 变成什么

# Name of the bin button that takes a change away.
# SAFETY: needs a human review before it ships
settings-keyboard-take-away = 撤销这项更改

# Small words under a changed key's line when it changes on every keyboard and mouse.
settings-keyboard-every-device-about = 适用于所有键盘和鼠标。

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = 第 { $n } 行未被使用：{ $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = 无法读取已更改的按键

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = 在能读取之前，所有按键保持原样：{ $why }。

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = 选择要更改的按键或按钮

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = 按下要更改的按键或按钮

# Small words under the change a key line.
settings-keyboard-keys-add-about = 直接按下它，或从列表中选择。

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = 更改按键或按钮

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key } 已在下方更改过。

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = 按键和按钮

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = 任何按键或鼠标按钮都可以改作他用：另一个按键或按钮、同时按住的组合键（如 Ctrl+C）、依次按下的一串键（如 H、I）、完全不起作用，或者轻点是一种作用、按住是另一种作用（如轻点为 Escape，按住为 Ctrl）。从登录界面起就在所有地方生效。Ctrl、Alt 加一个 F 键始终可以进入文本控制台，在那里输入 trindle-settings --keys default 即可恢复所有按键。

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = 交换 Ctrl 和 Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock：轻点为 Escape，按住为 Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = 交换 Escape 和 Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = 交换 Alt 和 Super（与 Mac 键盘一致）

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = 鼠标侧键用于复制和粘贴

# Small words under a keyboard or mouse a program made, when it counts as the person.
# SAFETY: needs a human review before it ships
settings-keyboard-made-yes = 视同您本人，就像插上的实体键盘一样。

# Small words under a keyboard or mouse a program made, when it does not count as the person.
# SAFETY: needs a human review before it ships
settings-keyboard-made-no = 可以输入和操作，但不能按受保护的按钮。

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = 将 { $name } 视同您本人

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = 目前没有

# Small words under that line.
settings-keyboard-made-none-about = 程序创建的键盘或鼠标会显示在这里，您可以选择是否将其视同您本人。

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = 程序创建的键盘和鼠标

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = 按键重映射工具和游戏启动器可以创建自己的键盘或鼠标。它们的按键和点击仍可输入和操作，但不能按受保护的按钮（如“关机”或“允许”），也不能在需要您本人回答的提问中输入。在这里打开某一项，即可将其视同您本人。

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = 手柄 { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $title } 上的“{ $name }”

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = { $title } 的作用

# Small words under a controller's changed button; the words Button and what it becomes stand for the two sides of the line typed.
settings-keyboard-pad-type = 按“按钮 = 变成什么”的格式输入。

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = 交换 A 和 B

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = 交换 X 和 Y

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = 反转右摇杆的上下方向

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = 插件快捷键

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin }。未使用：{ $why }

# Small words under a plugin shortcut; { $plugin } is the plugin. The example is keys a person types.
settings-keyboard-plugin-about = { $plugin }。请输入按键，例如 Ctrl+Alt+S；留空表示不设置。

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = { $label } 的按键

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = 已保存

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = 触摸板同样适用。

# Title of the mouse card.
settings-keyboard-mouse-title = 鼠标

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = 指针速度是鼠标移动一段距离时指针走多远，触摸板也一样。滚动速度是滚轮每转一格页面移动多远：较慢、正常或较快，也可以在滑块上任意选择。开启自然滚动后，页面随滚轮的方向移动，就像在手机上一样。左手模式会对调左右按钮。

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = 指针速度

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = 滚动速度

# Small words under the wheel's scroll speed.
settings-keyboard-scroll-wheel-about = 滚轮每转一格页面移动多远。

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = 自然滚动

# Small words under natural scrolling for the mouse wheel.
settings-keyboard-natural-wheel-about = 页面随滚轮移动，就像在手机上一样。

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = 左手模式

# Small words under that switch.
settings-keyboard-left-handed-about = 对调左右按钮。

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = 点击辅助

# Small words under the clicking helper line.
settings-keyboard-clicker-about = 悬停点击器或开关点击器发出的点击视同您本人的点击。

# Title of the touchpad card.
settings-keyboard-touchpad-title = 触摸板

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = 滚动速度是双指滑动时页面移动多远。轻点即点击会把轻轻一点当作一次点击。开启自然滚动后，页面随手指的方向移动，就像在手机上一样。打字时关闭可避免手掌擦过触摸板时在您打字期间移动指针。

# Small words under the touchpad's scroll speed.
settings-keyboard-scroll-touchpad-about = 双指滑动时页面移动多远。

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = 轻点即点击

# Small words under that switch.
settings-keyboard-tap-about = 轻轻一点就算一次点击。

# Small words under natural scrolling for the touchpad.
settings-keyboard-natural-touchpad-about = 页面随手指移动，就像在手机上一样。

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = 打字时关闭

# Small words under that switch.
settings-keyboard-quiet-about = 手掌擦过时指针保持不动。

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = 较慢

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = 正常

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = 较快

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB 端口

# Words behind the i of that card.
settings-keyboard-usb-about = 当某个 USB 端口无法启动时，Trindle 会将其关闭再打开一次重试，每个端口最多重试一次。

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error }；并且无法撤销它的标记：{ $undo }



# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = 截图和快捷键列表

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = 截图

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = 显示快捷键列表

# Small words under each of the two shortcuts.
settings-keyboard-shortcuts-about = 输入按键，留空则表示没有。

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = 立即截图

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = 立即显示列表

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = 重启桌面

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = 重启圆环和辅助部分

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = 全部重启

# Small words under each of the two restart shortcuts. The example is keys a person types.
settings-keyboard-restart-about = 输入按键，例如 Ctrl+Alt+Escape，留空则表示没有。

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = { $label }的按键

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = 立即重启。已打开的窗口保持原样。

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = 全部重启。会先询问。

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = 已重启

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } 已被另一个快捷键使用。

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = 未保存：{ $why }

