# machine translated (Simplified Chinese), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = 电量 { $pct }%

# Line under the low battery warning's title.
ring-battery-plug-in = 请尽快接上充电器。

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = 正在充电

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = 充电器已拔出

# Line under the title when the charger comes out.
ring-battery-running = 正在使用电池，电量 { $pct }%

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 分钟

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 小时

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 小时

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = 明早

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = 推迟 10 分钟

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = 推迟 1 小时

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = 推迟 6 小时

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = 推迟到明早

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = 返回消息

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = 取消固定

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = 固定

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = 稍后提醒

# SAFETY: needs a human review before it ships
# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = 全部清除

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = 已复制

# Tip of the button on a notification circle that copies its message.
ring-act-copy = 复制消息

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = 点击即可交给您的助手。

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = Trindle 桌面

# Name of the login screen in the crash notice.
ring-crash-greeter = Trindle 登录界面

# Name of the lock screen in the crash notice.
ring-crash-lock = Trindle 锁屏界面

# Name of Settings in the crash notice.
ring-crash-settings = Trindle 设置

# Name of Apps in the crash notice.
ring-crash-apps = Trindle 应用

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = Trindle 的 { $name }

# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } 意外停止了

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = 该驱动器已不在了

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = 无法打开 { $name }

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = 睡眠

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = 休眠

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = 锁定屏幕

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = 什么也不做

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = 关机

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = 合上盖子会按“设置”中“电源”的选择执行

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = 在任意位置按住即可放大

# Screen reader name of the magnifier's glass.
ring-magnify-name = 放大镜

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = 停止放大

# SAFETY: needs a human review before it ships
# Tip of the switch in the ring that turns the microphone off for every program.
ring-mic-off-tip = 为所有程序关闭麦克风

# SAFETY: needs a human review before it ships
# Tip of the switch in the ring that turns the microphone back on.
ring-mic-on-tip = 重新打开麦克风

# SAFETY: needs a human review before it ships
# Line in the ring when something other than the person pressed the microphone switch on.
ring-mic-own-press = 只有您自己亲手按下才能重新打开麦克风。

# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = 无法更改麦克风开关：{ $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = 找不到程序

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = 由其他程序发送

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = 无法启动

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = 已为您的助手复制

# Screen reader name of a notification circle.
ring-notes-name = 通知

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = 正在移动。松开即放下，按 Escape 放回原处。

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = 已移动

# Said when Escape put a held notification back.
ring-notes-put-back = 已放回

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = 先固定才能移动

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
       *[other] { $n } 秒
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = 直到您关闭

# Time under a notification from yesterday.
ring-notify-yesterday = 昨天 { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = 右下角

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = 左下角

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = 左上角

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = 右上角

# SAFETY: needs a human review before it ships
# Title of the notice when a plugin that asks for permissions has been given none.
ring-plugin-needs = { $name } 需要您的许可

# SAFETY: needs a human review before it ships
# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
ring-plugin-cannot-work = 它暂时无法工作：已允许 0 项，共 { $asks } 项。

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = 已复制。{ $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = 已连接到网络

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = 已打开你的文件

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
       *[other] { $names } 以及另外 { $more } 个
    }

