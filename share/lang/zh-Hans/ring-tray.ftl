# machine translated (Simplified Chinese), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
       *[other] 有 { $n } 个插件更新需要到“应用”中查看
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = 一切都是最新的

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
       *[other] 有 { $n } 个等待安装：“应用”旁边的更新按钮可以全部装上
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = 有 { $n } 个等待安装：{ $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = 有 { $n } 个等待安装：“应用”旁边的更新按钮可以装上 { $put } 个，另外{ $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
       *[other] 查看 { $n } 个固件更新
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
       *[other] 查看 { $n } 个更新，其中有固件
    }

# SAFETY: needs a human review before it ships
# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
       *[other] 安装 { $n } 个更新
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
       *[other] 查看 { $n } 个插件更新，它们请求的权限比以前更多
    }

# SAFETY: needs a human review before it ships
# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
       *[other] 安装 { $put } 个更新；{ $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = 已是最新

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = 键盘

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = 屏幕亮度

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = 保持唤醒

# Name of the sound button when the sound is muted.
ring-tray-muted = 已静音

# Name of the sound button with the volume level.
ring-tray-volume = 音量 { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] 电量 { $pct }%，正在充电
        [onbattery] 电量 { $pct }%，使用电池
        [full] 电量 { $pct }%，已充满
        [pluggedin] 电量 { $pct }%，已接通电源
       *[other] 电量 { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = 网络已关闭

# Name of the network button when nothing is connected.
ring-tray-not-connected = 未连接

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = 通过 VPN 连接

# Name of the network button when connected by cable.
ring-tray-wired = 有线网络

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = 有线网络，连接受限

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }，信号 { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }，信号 { $strength }%，连接受限

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = 蓝牙已关闭

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = 已连接到 { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = 蓝牙已开启

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = 托盘图标

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = 所有托盘图标

# Words on the panel of every tray icon when there are none.
ring-tray-none = 没有托盘图标

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = 显示在环中

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = 从环中取出

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = 环里已有四个。请先取出一个。



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = 检查已关闭

