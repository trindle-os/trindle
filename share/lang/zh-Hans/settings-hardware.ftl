# machine translated (Simplified Chinese), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = 硬件

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = 硬件 设备 驱动 驱动程序 固件 缺少 问题 芯片 显卡 声音 网络 蓝牙 存储 摄像头 键盘 鼠标 hardware devices drivers driver firmware pci usb graphics sound network bluetooth storage camera keyboard mouse missing problem chip

# Text behind the info button of the Hardware page.
settings-hardware-help = 这里列出 Trindle 能看到的这台电脑的所有部件，按用途分组。没有驱动程序、或驱动程序缺少固件文件的部件排在最前面。驱动程序随系统提供，所以这里没有需要下载的东西。

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
       *[other] 你的 { $n } 个设备都有可用的驱动程序。
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
       *[other] 有 { $n } 个设备需要处理。
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = 显卡

settings-hardware-kind-sound = 声音

settings-hardware-kind-network = 网络

settings-hardware-kind-bluetooth = 蓝牙

settings-hardware-kind-storage = 存储

settings-hardware-kind-input = 键盘和鼠标

settings-hardware-kind-camera = 摄像头

settings-hardware-kind-other = 其他

# Under a device that works; name is the driver's name.
settings-hardware-driver = 正常。驱动程序：{ $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = 正常

# Under a device that has no driver.
settings-hardware-no-driver = 没有驱动程序

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = 缺少固件

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = 未使用驱动程序

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = 未知设备 { $id }

# Card name.
settings-hardware-chip = 安全芯片

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = 启动时芯片没有解锁磁盘，所以要求输入密码。BIOS 更新可能导致这种情况。

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = 再次使用芯片

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = 磁盘密码

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = 取消

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = 请先输入磁盘密码。

# Beside the line while it works.
settings-hardware-chip-waiting = 正在设置芯片

# Once it worked.
settings-hardware-chip-done = 从下次启动起，芯片会解锁磁盘。
