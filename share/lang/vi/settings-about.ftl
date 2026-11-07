
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Về máy tính này

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = tên máy tên máy tính tên mạng giới thiệu hệ thống phiên bản debian trindle bộ nhớ ram bộ xử lý cpu ổ đĩa đồ họa màn hình mẫu máy nhân kernel hoạt động chương trình đang chạy bận quản lý tác vụ giám sát hệ thống ổ đĩa usb dung lượng

# Text behind the info button of the About page.
settings-about-help = Tên máy tính này như các máy khác trên mạng thấy, nó được tạo nên từ những gì, và mất bao lâu để khởi động. Việc nó đang làm lúc này, và chương trình nào đang dùng bộ nhớ và bộ xử lý, nằm trong Hoạt động; các ổ đĩa và dung lượng trên đó nằm trong Ổ đĩa, mỗi mục là một cửa sổ riêng: các mũi tên trong Chi tiết hơn sẽ mở chúng.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Không rõ

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
       *[other] { $model }, { $cores } luồng
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Đồ họa Intel

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = Đồ họa AMD

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = Đồ họa NVIDIA

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Đồ họa máy ảo

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Đồ họa

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } ở { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = Đang dùng { $amount }

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = Còn trống { $amount }

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = trên tổng { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, đã bật được { $on }.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, đã bật được { $on }. Ổ đĩa gần đầy.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } giây

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } phút

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
       *[other] { $n } phút
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
       *[other] { $n } giờ
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
       *[other] { $n } ngày
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Chỉ dùng chữ cái, chữ số và dấu gạch nối, không bắt đầu hoặc kết thúc bằng dấu gạch nối.

# Words under the computer name box while the password window is open.
settings-about-name-waiting = Đang chờ mật khẩu...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Không thể đổi tên máy tính.

# Words under the computer name box after renaming worked.
settings-about-name-done = Các chương trình đang mở sẽ thấy tên mới sau lần đăng nhập tiếp theo.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Tên máy tính

# Tip of the save button beside the computer name box.
settings-about-rename = Đổi tên máy tính này

# Bold line of the computer name setting.
settings-about-name = Tên máy tính

# Quiet line under the computer name setting.
settings-about-name-sub = Chữ cái, chữ số và dấu gạch nối.

# Heading of the card with the computer's name.
settings-about-name-group = Tên

# Text behind the info button of the Name card.
settings-about-name-about = Cách máy tính này hiện trên mạng và ở dấu nhắc trong Terminal. Chữ cái, chữ số và dấu gạch nối, không bắt đầu hoặc kết thúc bằng dấu gạch nối. Việc đổi tên sẽ hỏi mật khẩu của bạn.

# Heading of the card about the computer's hardware.
settings-about-hardware = Máy tính

# Text behind the info button of the hardware card.
settings-about-hardware-about = Hãng và mẫu máy, bộ xử lý và số luồng nó chạy cùng lúc, đồ họa, và từng màn hình đang dùng với kích thước và tốc độ làm mới.

# Quiet line under the maker and model.
settings-about-hw-computer = Máy tính

# Quiet line under the processor.
settings-about-hw-processor = Bộ xử lý

# Quiet line under the graphics.
settings-about-hw-graphics = Vẽ các hình ảnh

# Quiet line under the screens.
settings-about-hw-screen = Màn hình

# Heading of the card about the software.
settings-about-software = Phần mềm

# Text behind the info button of the software card.
settings-about-software-about = Trindle là một màn hình nền chạy trên Debian, nơi lo các chương trình bên dưới. Nhân (kernel) là phần giao tiếp với phần cứng, và trình tổng hợp riêng của Trindle vẽ màn hình nền, trên Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = Chạy bên dưới

# Quiet line under the kernel version.
settings-about-sw-kernel = Nhân (kernel)

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trình tổng hợp riêng của Trindle

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Vẽ màn hình nền, trên Wayland

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
       *[other] { $n } ứng dụng
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Đã cài, sẵn sàng trong vòng tròn

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Chi tiết hơn

# Text behind the info button of the More detail card.
settings-about-more-about = Việc nó đang làm lúc này, và chương trình nào đang dùng bộ nhớ và bộ xử lý, nằm trong Hoạt động. Mọi ổ đĩa, mức đầy và tình trạng của chúng, cùng USB, nằm trong Ổ đĩa.

# Bold line of the setting that opens Activity.
settings-about-activity = Xem những gì đang chạy

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Chương trình đang dùng bộ nhớ và bộ xử lý.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Hoạt động

# Bold line of the setting that opens Drives.
settings-about-drives = Ổ đĩa và dung lượng

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Mọi ổ đĩa, mức đầy và tình trạng.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Ổ đĩa

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Không thể lưu thời gian khởi động vào Downloads.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Trong Downloads, với tên { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Lưu

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Lưu mọi thời gian khởi động vào Downloads

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Mọi lần khởi động, để lưu giữ

# Quiet line under that setting.
settings-about-times-keep-sub = Lưu vào Downloads dưới dạng bảng tính.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Mỗi cột là một lần khởi động, mới nhất ở bên phải.

# Heading of the card with the last ten start-up times.
settings-about-times = Mười lần khởi động gần nhất

# Text behind the info button of the start-up times card.
settings-about-times-about = Mỗi lần khởi động được tính từ nút nguồn đến màn hình đăng nhập, không tính thời gian gõ mật khẩu ổ đĩa. Mới nhất ở trên cùng.

# Heading of the memory tile.
settings-about-memory = Bộ nhớ

# Heading of the disk tile.
settings-about-disk = Ổ đĩa



# Heading of the card with the switch for looking for updates.
settings-about-updates = Cập nhật

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle tìm phiên bản mới của ứng dụng, plugin, firmware và của chính nó ngay sau khi bạn đăng nhập và cứ vài giờ một lần, còn các bản vá bảo mật của Debian được cài mỗi ngày. Khi tắt, máy chỉ tìm khi bạn nhấn Kiểm tra cập nhật trong Ứng dụng, hoặc nút cập nhật trên vòng tròn. Các bản vá bảo mật của Debian có nút bật tắt riêng; khi tắt, chúng không còn tự cài và các lỗi đã biết vẫn còn cho đến khi bạn cập nhật. Đổi một trong hai nút này cần mật khẩu quản trị viên của bạn, để các dịch vụ của chính máy cũng thấy lựa chọn đó.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Kiểm tra cập nhật

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Đang chờ mật khẩu...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Đang tắt, sẽ không tìm gì trừ khi bạn yêu cầu.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Cập nhật bảo mật

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Các bản vá bảo mật tự cài mỗi ngày.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Các bản vá bảo mật không còn tự cài.

