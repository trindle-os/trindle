
# machine translated (Vietnamese), not yet reviewed by a person.

# Said after what the updates button will do or what waits: plugin updates that ask for more permission than before and need a look in Apps.
ring-updates-look =
    { $n ->
       *[other] { $n } bản cập nhật plugin cần được xem trong Ứng dụng
    }

# Answer to a spoken question about updates when none wait.
ring-updates-none = Mọi thứ đều đã mới nhất

# Answer to a spoken question about updates when the updates button installs them all; "it" for one, "them" for more.
ring-updates-spoken-all =
    { $n ->
       *[other] Có { $n } bản đang chờ: nút cập nhật cạnh Ứng dụng sẽ cài tất cả
    }

# Answer to a spoken question about updates when only plugin updates that need a look wait; the clause that follows says so.
ring-updates-spoken-look = Có { $n } bản đang chờ: { $look }

# Answer to a spoken question about updates when some install and some need a look; the last clause says which need a look.
ring-updates-spoken-mixed = Có { $n } bản đang chờ: nút cập nhật cạnh Ứng dụng sẽ cài { $put } bản, và { $look }

# Name and tip of the updates button when only firmware updates wait: the button takes the person to them.
ring-updates-see-firmware =
    { $n ->
       *[other] Xem { $n } bản cập nhật firmware
    }

# Name and tip of the updates button when firmware updates wait among others: the button takes the person to them.
ring-updates-see-among =
    { $n ->
       *[other] Xem { $n } bản cập nhật, trong đó có firmware
    }

# Name and tip of the updates button when updates wait that it will install.
ring-updates-install =
    { $n ->
       *[other] Cài { $n } bản cập nhật
    }

# Name and tip of the updates button when only plugin updates that ask for more than before wait.
ring-updates-see-plugin =
    { $n ->
       *[other] Xem { $n } bản cập nhật plugin, yêu cầu nhiều quyền hơn trước
    }

# Name and tip of the updates button when some updates install and some plugin updates need a look; the clause after the semicolon says which.
ring-updates-install-and-look =
    { $put ->
       *[other] Cài { $put } bản cập nhật; { $look }
    }

# Name of the updates button when nothing waits.
ring-updates-up-to-date = Đã mới nhất

# Name of the round button in the ring that shows or hides the on-screen keyboard.
ring-tray-keyboard = Bàn phím

# Name of the round button in the ring that opens the screen brightness page.
ring-tray-brightness = Độ sáng màn hình

# Name of the round button in the ring that keeps the computer awake (no dimming, locking or sleep) until pressed again.
ring-tray-keep-awake = Giữ máy thức

# Name of the sound button when the sound is muted.
ring-tray-muted = Đã tắt tiếng

# Name of the sound button with the volume level.
ring-tray-volume = Âm lượng { $pct }%

# Name of the battery button: the charge, and what the battery is doing. The state is one of charging, onbattery, full, pluggedin, or something else for none.
ring-tray-battery =
    { $state ->
        [charging] Pin { $pct }%, đang sạc
        [onbattery] Pin { $pct }%, đang dùng pin
        [full] Pin { $pct }%, đầy
        [pluggedin] Pin { $pct }%, đã cắm điện
       *[other] Pin { $pct }%
    }

# Name of the network button when networking is switched off.
ring-tray-net-off = Mạng đang tắt

# Name of the network button when nothing is connected.
ring-tray-not-connected = Chưa kết nối

# Name of the network button when the connection is through a VPN.
ring-tray-vpn = Đã kết nối qua VPN

# Name of the network button when connected by cable.
ring-tray-wired = Có dây

# Name of the network button when connected by cable but the internet is not reachable.
ring-tray-wired-limited = Có dây, kết nối hạn chế

# Name of the network button on Wi-Fi: the network's name and its signal strength.
ring-tray-wifi = { $name }, tín hiệu { $strength }%

# Name of the network button on Wi-Fi when the internet is not reachable.
ring-tray-wifi-limited = { $name }, tín hiệu { $strength }%, kết nối hạn chế

# Name of the Bluetooth button when Bluetooth is off.
ring-tray-bt-off = Bluetooth đang tắt

# Name of the Bluetooth button when devices are connected; their names follow, separated by commas.
ring-tray-bt-connected = Đã kết nối với { $names }

# Name of the Bluetooth button when Bluetooth is on and nothing is connected.
ring-tray-bt-on = Bluetooth đang bật

# Screen reader name of another program's tray icon that has no name of its own.
ring-tray-icon = Biểu tượng khay

# Tip of the round button at the end of the tray row that opens every tray icon.
ring-tray-all = Tất cả biểu tượng khay

# Words on the panel of every tray icon when there are none.
ring-tray-none = Không có biểu tượng khay

# Line of a tray icon's menu: put the icon in the ring's row.
ring-tray-show = Hiện trong vòng

# Line of a tray icon's menu: take the icon out of the ring's row.
ring-tray-take-out = Bỏ khỏi vòng

# Quiet words in a tray icon's menu in place of Show in the ring, when the row is full.
ring-tray-full = Vòng đã có bốn biểu tượng. Hãy bỏ bớt một cái trước.



# Name of the updates button when nothing is known to wait and Settings has looking for updates by itself switched off.
ring-updates-off = Việc kiểm tra đang tắt

