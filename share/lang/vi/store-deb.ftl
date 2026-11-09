
# machine translated (Vietnamese), not yet reviewed by a person.

# Window title of the window opened for a .deb file.
store-deb-title = Cài một gói

# Words at the top of the window while the package is read.
store-deb-reading = Đang đọc gói…

# Red words when the file is not a Debian package and may not have finished downloading.
store-deb-not-finished = Tệp này không phải là gói Debian có thể cài. Có thể nó chưa tải xong.

# Words at the top of the window for a package that can be installed.
store-deb-ready = { $name } sẵn sàng để cài trong thư mục riêng của nó.

# Red words at the top of the window for a package made for another kind of computer.
store-deb-elsewhere = Gói này dành cho máy tính { $arch }, nên không cài được lên máy này.

# Red words at the top of the window when the package is Trindle itself.
store-deb-trindle = Đây chính là Trindle. Nó được cập nhật từ trang Cập nhật của Ứng dụng, không phải từ một tệp.

# Red words at the top of the window for a driver or a background service.
store-deb-service = { $name } là trình điều khiển hoặc dịch vụ chạy nền, không hoạt động được từ thư mục riêng của nó, nên không thể cài ở đây.

# Words on the caution card above every package.
# SAFETY: needs a human review before it ships
store-deb-caution = Gói từ bên ngoài Debian và Trindle được cài trong thư mục riêng của nó, nơi nó bị hạn chế nhưng chưa bị cách ly hoàn toàn. Chỉ nên cài gói của những nhà phát hành bạn tin tưởng.

# Words at the top of the window once the package is installed.
store-deb-installed = { $name } đã được cài trong thư mục riêng của nó.

# In the card that says where the file came from, while it is checked.
store-deb-checking-origin = Đang kiểm tra nguồn gốc…

# Heading in the card that says where the file came from.
store-deb-where = Nguồn gốc

# Tip and spoken name of the round button that removes the package and what it saved.
# SAFETY: needs a human review before it ships
store-deb-remove = Gỡ nó và những gì nó đã lưu

# Word beside the ring while the package goes in.
store-deb-installing = Đang cài

# Main button that installs the package.
store-deb-install = Cài đặt

# Main button that installs the same version again.
store-deb-reinstall = Cài lại

# Main button that brings the installed package up to this version.
store-deb-update = Cập nhật

# Heading of the card for a package not installed yet.
# SAFETY: needs a human review before it ships
store-deb-goes-in = Nó được cài trong thư mục riêng, mặc định bị hạn chế

# Under that heading.
# SAFETY: needs a human review before it ships
store-deb-goes-in-note = Không cần mật khẩu. Nó không thể dùng internet, âm thanh hay tệp của bạn cho đến khi bạn cho phép.

# Heading of the card for a package installed at this version.
store-deb-same-version = Phiên bản { $version } đã được cài trong thư mục riêng của nó

# Under that heading.
store-deb-same-version-note = Hãy cài lại ở đó nếu nó không còn hoạt động đúng.

# Heading of the card for a package installed at another version.
store-deb-other-version = Phiên bản { $version } đã được cài trong thư mục riêng của nó

# Under that heading: the version this brings.
store-deb-other-version-note = Cài gói này ở đó sẽ nâng nó lên phiên bản { $version }, giữ nguyên những gì nó đã lưu.

# Facts line: how big the package is once installed.
store-deb-fact-size = Cỡ sau khi cài

# Facts line: the computer kind it is made for.
store-deb-fact-arch = Dành cho

# Text of the About card when the package has no description.
store-deb-no-description = Gói này không có mô tả.

# Words at the top of the window while the package goes in.
store-deb-installing-note = Đang cài trong thư mục riêng của nó. Lấy những thứ nó cần có thể mất vài phút.

# Words at the top of the window once the package is installed.
store-deb-done = Đã cài trong thư mục riêng của nó. Bạn sẽ thấy nó trong vòng tròn Trindle.

# Red words at the top of the window when parts the package needs could not be found; the parts follow.
store-deb-missing = Đã cài trong thư mục riêng của nó, nhưng không tìm thấy các phần sau, nên nó có thể không khởi động: { $parts }.

# Words at the top of the window once the package is removed.
# SAFETY: needs a human review before it ships
store-deb-removed = Đã gỡ khỏi thư mục riêng của nó, cùng mọi thứ nó đã lưu.

# Red words when a file is not a Debian package.
store-boxed-not-deb = Tệp này không phải là gói Debian có thể cài.

# Step shown while a package is unpacked.
store-boxed-unpacking = Đang giải nén { $name }

# Step shown while a part a package needs is added.
store-boxed-adding = Đang thêm { $part }

# Red words when a file is not a Debian package.
store-parts-not-deb = { $path } không phải là gói Debian có thể cài.

# Red words when a package could not be unpacked.
store-parts-not-unpacked = Không thể giải nén { $path }.

# Red words when the program is not one kept in a box.
store-loose-not-boxed = Chương trình đó không được giữ trong hộp.

# Name of Trindle's own terminal in the ring.
store-boxed-terminal = Terminal

# Title of the notice when older programs are off.
store-boxed-x-off = Chương trình cũ đang tắt

# Body of that notice: where to turn them on.
store-boxed-x-off-where = Cài đặt, Quyền

# Title of the notice when too many older programs are open.
store-boxed-x-many = Quá nhiều chương trình cũ

# Body of that notice.
store-boxed-x-many-close = Hãy đóng một cái rồi thử lại

# Body of the notice that a program could not open because its Xwayland would not start.
store-boxed-x-could-not-open = { $app } không mở được

# Body of the notice the first time a terminal opens outside its box.
# SAFETY: needs a human review before it ships
store-boxed-terminal-told = Terminal có thể truy cập nhiều phần của máy tính hơn các ứng dụng khác, vì công việc của nó cần như vậy. Chỉ gõ những lệnh bạn hiểu, từ những người bạn tin tưởng.

# Title of the notice when a plugin allowed the home network has none.
store-boxed-home-off = Mạng gia đình đang tắt

# Body of that notice: the plugin's name and why.
store-boxed-home-off-why = { $app } không có mạng: { $why }

# Why a plugin allowed the home network has none: Trindle could not find its own program.
store-boxed-home-why-self = không tìm thấy chương trình của chính nó

# The same: the unshare program is missing.
store-boxed-home-why-unshare = unshare chưa được cài

# The same: the pasta program is missing.
store-boxed-home-why-pasta = pasta chưa được cài

# The same: the network room was not made.
store-boxed-home-why-not-made = không tạo được phòng mạng của nó

# The same: the room was not a new one, so it was left alone.
store-boxed-home-why-not-new = phòng mạng của nó không phải phòng mới

# The same: the filter that keeps a box from making a room of its own could not be made.
store-boxed-home-why-filter = không tạo được bộ lọc lệnh gọi hệ thống

# The same: the room's rules would not load.
store-boxed-home-why-rules = các quy tắc của phòng mạng không nạp được

# The same: a program would not start, with its name and the reason.
store-boxed-home-why-would-not-start = { $program } không khởi động được: { $why }

# The same: pasta refused, with what it said.
store-boxed-home-why-pasta-said = pasta báo: { $why }

# The same: pasta could not join the room and said nothing.
store-boxed-home-why-pasta-no-join = pasta không vào được phòng mạng


# Title of the notice when a program was given a downloaded file and run in a box with no internet.
store-downloaded-opened = Đã mở mà không có internet

# Body of that notice: why, and what the program was given.
store-downloaded-why = Nó được tải về, nên chương trình chỉ có tệp đó, chỉ để đọc.

