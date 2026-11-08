# machine translated (Simplified Chinese), not yet reviewed by a person.

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = 关闭

# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = 打开

# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = 设置

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = 用法：

# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [选项…]

# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = 选项：

# Help for the ring's --say option, printed in a terminal.
ring-cli-say = 按这些话的要求去做，就像对圆环说出来一样（无需麦克风即可试用语音命令）

# Help for the ring's --background option, printed in a terminal.
ring-cli-background = 启动但不显示，这样第一次打开是瞬间完成的

# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = 收起所有窗口以显示桌面，或把它们恢复（Super 加 D 会发出此请求）

# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = 打开圆环并立即开始聆听，就像键盘快捷键那样

# Help for the ring's --all option, printed in a terminal.
ring-cli-all = 打开圆环，并在四周显示所有应用

# Help for the ring's --power option, printed in a terminal.
ring-cli-power = 在注销、重启和关机时打开圆环

# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = 打开“设置”中的收藏页面

# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = 立即采用“设置”中的新外观（浅色或深色，或新的强调色）

# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = 重新创建顶部角落的触控点

# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = 把键盘移到屏幕上的通知圆圈中（Super 加 N 会发出此请求）

# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = 未知选项 { $option }

# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = --say 缺少参数

# Line in the ring's results when what was typed matches no app or favourite.
ring-find-nothing = 没有叫这个名字的应用或收藏

# SAFETY: needs a human review before it ships
# Tip of the round button on an app's menu that removes the app.
ring-uninstall = 卸载

# SAFETY: needs a human review before it ships
# Line that replaces the uninstall button while the app is being removed.
ring-uninstalling = 正在卸载

# SAFETY: needs a human review before it ships
# Line that replaces the uninstall button once the app is removed.
ring-uninstalled = 已卸载

# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = 从圆环中取下

# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = 正在取下

# Line that replaces that button once the app is off the ring.
ring-taken-off = 已取下

# Name of the ring's round button that opens Apps.
ring-tool-apps = 应用

# Name of the ring's round button that opens Activity.
ring-tool-activity = 活动

# Name of the ring's round button that opens Drives.
ring-tool-drives = 驱动器

# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = 截图

# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = 剪贴板

# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = 通知

# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = 放大镜

# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = 取色器

# Name of the ring's round button that opens the report a problem window.
ring-tool-report = 报告问题

# Line in the ring when the item that was pressed is not there any more.
ring-gone = 它已经不在了。

# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = 没有程序能打开它。

# Line in the ring while it listens to a spoken command.
ring-listening = 正在聆听……请说话，您停下它也会停

# Line in the ring while it works out what was said.
ring-working-out = 正在理解……

# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = 没有可关闭的窗口

# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = 现在是 { $date } { $time }

# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = 这块屏幕的亮度无法在此处调节

# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = 屏幕键盘没有安装

# Screen reader name of the ring of open windows.
ring-open-windows = 已打开的窗口

# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = 还原

# Tip of the button on a window's menu that minimises it.
ring-window-minimise = 最小化

# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = 还原大小

# Tip of the button on a window's menu that maximises it.
ring-window-maximise = 最大化

# SAFETY: needs a human review before it ships
# Tip of the button on a window's menu that stops a program that does not answer.
ring-window-force-close = 强制关闭

# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = 所有应用

# Name of the ring's round button that opens the power page.
ring-power = 电源

# SAFETY: needs a human review before it ships
# Screen reader description of the ring's power button.
ring-power-spoken = 注销、重启或关机

# Screen reader name of the ring's results while something is typed.
ring-finding = 正在查找 { $typed }

# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = 日期和时间

# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = 新窗口

# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
       *[other] 关闭全部 { $n } 个窗口
    }

# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = 从圆环中取消固定

# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = 从收藏中移除

# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = 添加到收藏

# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = 未完成

# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    中间最多放 { $n } 个。
    请先移除一个再腾出位置。

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: lock the screen.
ring-power-lock = 锁定

# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = 屏幕保护程序

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = 睡眠

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: hibernate.
ring-power-hibernate = 休眠

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: log out.
ring-power-log-out = 注销

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: restart the computer.
ring-power-restart = 重启

# SAFETY: needs a human review before it ships
# Choice on the ring's power page: turn the computer off.
ring-power-shut-down = 关机

# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = 点击“所有应用”，再右键点击任一应用，即可把它留在这里

# SAFETY: needs a human review before it ships
# Line in the ring when a spoken or typed power choice needs a click to confirm.
ring-power-only-click = 只有点击才能确认此操作。

# SAFETY: needs a human review before it ships
# Line in the ring when something other than the person tried to shut the computer down.
ring-power-only-own = 只有您本人才能关闭这台电脑。

# SAFETY: needs a human review before it ships
# Line in the ring when the power page closed because a program pressed a key.
ring-power-closed = 电源页面已关闭：是某个程序按了键，不是您。

# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = 没有可关闭的内容

ring-startup-add = 登录时启动

ring-startup-remove = 不再在登录时启动



# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = 以列表显示所有键盘快捷键，或收起列表（Super 和 K，除非在“设置”中更改）

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = 已重启

# Under the notice above.
ring-restarted-why = 圆环和辅助部分已重新启动。

