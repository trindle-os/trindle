
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Về máy tính này

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = tên máy tên máy tính tên mạng giới thiệu hệ thống phiên bản debian trindle bộ nhớ ram bộ xử lý cpu ổ đĩa đồ họa màn hình mẫu máy nhân kernel hoạt động chương trình đang chạy bận quản lý tác vụ giám sát hệ thống ổ đĩa usb dung lượng

# Text behind the info button of the About page.
settings-about-help = Trang này cho biết tên máy tính này như các máy khác trên mạng thấy, nó được tạo nên từ những gì, và mất bao lâu để khởi động. Hoạt động cho biết máy đang làm gì lúc này và chương trình nào dùng bộ nhớ và bộ xử lý. Ổ đĩa cho biết các ổ đĩa của bạn và dung lượng của chúng. Mỗi mục mở trong một cửa sổ riêng. Hãy dùng các mũi tên trong Chi tiết hơn để mở chúng.

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
settings-about-name-bad = Chỉ dùng chữ cái, chữ số và dấu gạch nối. Đừng bắt đầu hoặc kết thúc bằng dấu gạch nối.

# Words under the computer name box while the password window is open.
settings-about-name-waiting = Đang chờ mật khẩu của bạn…

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

# Heading of the card with the computer's name.
settings-about-name-group = Tên

# Text behind the info button of the Name card.
settings-about-name-about = Đây là tên của máy tính trên mạng và ở dấu nhắc trong Terminal. Chỉ dùng chữ cái, chữ số và dấu gạch nối, đừng bắt đầu hoặc kết thúc bằng dấu gạch nối. Bạn cần nhập mật khẩu để đổi tên.

# Heading of the card about the computer's hardware.
settings-about-hardware = Máy tính

# Text behind the info button of the hardware card.
settings-about-hardware-about = Phần này cho biết hãng và mẫu máy, bộ xử lý và đồ họa của máy tính. Nó cũng cho biết bộ xử lý chạy được bao nhiêu luồng cùng lúc, và từng màn hình đang dùng với kích thước và tốc độ làm mới.

# Heading of the card about the software.
settings-about-software = Phần mềm

# Text behind the info button of the software card.
settings-about-software-about = Trindle là một màn hình nền chạy trên Debian, nơi lo các chương trình bên dưới. Nhân (kernel) là phần làm việc với phần cứng. Trình tổng hợp riêng của Trindle vẽ màn hình nền, dùng Wayland.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trình tổng hợp riêng của Trindle

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
       *[other] { $n } ứng dụng
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Chi tiết hơn

# Text behind the info button of the More detail card.
settings-about-more-about = Hoạt động cho biết máy đang làm gì lúc này và chương trình nào dùng bộ nhớ và bộ xử lý. Ổ đĩa cho biết mọi ổ đĩa, mức đầy và tình trạng của chúng, cùng các USB.

# Bold line of the setting that opens Activity.
settings-about-activity = Xem những gì đang chạy

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Hoạt động

# Bold line of the setting that opens Drives.
settings-about-drives = Ổ đĩa và dung lượng

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Ổ đĩa

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle không lưu được thời gian khởi động vào Downloads.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Ngày giờ,Giây

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Thời gian khởi động Trindle.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Trong Downloads, với tên { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Lưu

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Lưu mọi thời gian khởi động vào Downloads

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Lưu thời gian của mỗi lần khởi động

# Quiet line under that setting.
settings-about-times-keep-sub = Lưu vào Downloads dưới dạng bảng tính.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Mỗi cột là một lần khởi động. Mới nhất ở bên phải.

# Heading of the card with the last ten start-up times.
settings-about-times = Mười lần khởi động gần nhất

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle tính giờ mỗi lần khởi động từ nút nguồn đến màn hình đăng nhập. Thời gian gõ mật khẩu ổ đĩa không được tính. Mới nhất ở trên cùng.

# Heading of the memory tile.
settings-about-memory = Bộ nhớ

# Heading of the disk tile.
settings-about-disk = Ổ đĩa

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Báo cáo sự cố

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Báo cáo được gửi tới Buildmorphic, cộng đồng giúp xây dựng Trindle. Bạn có thể tham gia chỉ với một địa chỉ email. Bạn nhận điểm và cấp bậc cho những sự cố mình báo đầu tiên, dù không sửa được. Một báo cáo chỉ chứa một danh sách chi tiết cố định, và không có gì được gửi cho đến khi bạn nhấn Gửi. Tên người dùng hoặc tên GitHub là tuỳ chọn, và tên người dùng của bạn không bao giờ là địa chỉ email.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Đề nghị gửi báo cáo khi có sự cố

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Tên người dùng Buildmorphic

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = Tên GitHub

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = Về cộng đồng

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
       *[other] Đã tắt. Đã xoá { $n } báo cáo đang chờ.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
       *[other] Đã tắt. Không xoá được { $n } báo cáo đang chờ.
    }



# Heading of the card with the switch for looking for updates.
settings-about-updates = Cập nhật

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle kiểm tra phiên bản mới của ứng dụng, plugin, firmware và của chính nó ngay sau khi bạn đăng nhập, rồi cứ vài giờ một lần. Nếu bạn tắt mục này, Trindle chỉ kiểm tra khi bạn nhấn Kiểm tra cập nhật trong Ứng dụng, hoặc nút cập nhật trên vòng tròn. Các bản vá bảo mật của Debian được cài mỗi ngày và có nút bật tắt riêng. Nếu bạn tắt nút đó, chúng không còn tự cài và các lỗ hổng bảo mật đã biết vẫn chưa được vá cho đến khi bạn cập nhật. Đổi một trong hai nút này cần mật khẩu quản trị viên của bạn, để cả máy tính dùng lựa chọn của bạn.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Kiểm tra cập nhật

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Đang chờ mật khẩu của bạn…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Cập nhật bảo mật

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Các bản vá bảo mật không còn tự cài.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Đã tắt, nhưng các báo cáo đang chờ của tiện ích Reporting chưa được xoá.
