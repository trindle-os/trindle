# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = Phần cứng

# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = phần cứng thiết bị trình điều khiển driver firmware pci usb đồ họa âm thanh mạng bluetooth lưu trữ camera bàn phím chuột thiếu sự cố chip

# Text behind the info button of the Hardware page.
settings-hardware-help = Mọi bộ phận của máy tính này mà Trindle nhìn thấy, xếp theo chức năng. Bộ phận chưa có trình điều khiển, hoặc có trình điều khiển nhưng thiếu tệp firmware, được xếp lên đầu. Trình điều khiển đi kèm hệ thống nên ở đây không có gì cần tải.

# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
       *[other] Cả { $n } thiết bị của bạn đều có trình điều khiển hoạt động.
    }

# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
       *[other] { $n } thiết bị cần chú ý.
    }

# Card names, one for each kind of device.
settings-hardware-kind-graphics = Đồ họa

settings-hardware-kind-sound = Âm thanh

settings-hardware-kind-network = Mạng

settings-hardware-kind-bluetooth = Bluetooth

settings-hardware-kind-storage = Lưu trữ

settings-hardware-kind-input = Bàn phím và chuột

settings-hardware-kind-camera = Camera

settings-hardware-kind-other = Khác

# Under a device that works; name is the driver's name.
settings-hardware-driver = Đang hoạt động. Trình điều khiển: { $name }

# Under a device that works when the driver's name is not known.
settings-hardware-working = Đang hoạt động

# Under a device that has no driver.
settings-hardware-no-driver = Chưa có trình điều khiển

# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = Thiếu firmware

# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = Không dùng trình điều khiển

# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = Thiết bị không rõ { $id }

# Card name.
settings-hardware-chip = Chip bảo mật

# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = Chip không mở khóa ổ đĩa khi khởi động, nên máy hỏi mật khẩu. Cập nhật BIOS có thể gây ra.

# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = Dùng lại chip

# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = Mật khẩu ổ đĩa

# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = Hủy

# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = Hãy nhập mật khẩu ổ đĩa trước.

# Beside the line while it works.
settings-hardware-chip-waiting = Đang thiết lập chip

# Once it worked.
settings-hardware-chip-done = Chip sẽ mở khóa ổ đĩa từ lần khởi động sau.
