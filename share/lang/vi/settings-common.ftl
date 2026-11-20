
# machine translated (Vietnamese), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Đã lưu

# Said beside a button that worked, when nothing more is known.
settings-common-done = Xong

# Words that only say "that worked", in lower case and separated by bars. A line saying one of them under a control that has already moved is left out, as the glow says it. Translate each word, and keep the bars.
settings-common-quiet-words = đã lưu|xong|đang dùng|đã đổi|đã đặt|đã lấy|đã thêm|đã giải phóng|đã gỡ|đã lưu trữ|đã cài|đã cập nhật|đã tham gia|đã sao chép|đã thành mặc định|đã phát|đã gửi|đã kết nối|đang kết nối|bật|tắt|đã quên|dừng ở 80%|sạc đầy|bắt đầu khi đăng nhập|được giữ nhưng không khởi động|đã cập nhật mới nhất

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Chưa xong

# Tip of the information button at the top of every Settings page.
settings-common-about-page = Giới thiệu trang này

# Tip of the information button on a card: { $name } is the card's heading as it is written.
settings-common-about-card = Giới thiệu về { $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } đã có sẵn

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
       *[other] Làm tương tự cho phần còn lại (còn { $n })
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = Làm tương tự cho phần còn lại

# Tip of the button that keeps the one already there.
# SAFETY: needs a human review before it ships
settings-common-skip = Bỏ qua và giữ mục đã có

# Button that replaces the one already there.
# SAFETY: needs a human review before it ships
settings-common-replace = Thay thế

# Spoken name of the Replace button.
# SAFETY: needs a human review before it ships
settings-common-replace-spoken = Thay mục đã có bằng mục mới

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = Mục mới

# Heading of the line that shows the one already there.
settings-common-already-line = Mục đã có sẵn

# Heading of the line with the Skip and Replace buttons.
# SAFETY: needs a human review before it ships
settings-common-replace-question = Thay thế mục đã có sẵn?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
       *[other] Hoàn tác sau { $left } giây.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
       *[other] Hoàn tác sau { $left } giây. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Hoàn tác ngay

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Giữ

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } không có sẵn

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Để cài, hãy chạy lệnh: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = Trindle không mở được trang web, vì không khởi động được trình duyệt web.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = Không mở được { $name }.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = Không lưu được.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Trindle không làm được việc đó vì một phần của hệ thống không phản hồi. Hãy khởi động lại máy rồi thử lại.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Trindle không làm được việc đó vì một phần của phiên đăng nhập không phản hồi. Hãy đăng xuất rồi đăng nhập lại, sau đó thử lại.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Trindle không mở được cửa sổ để chọn tệp hoặc thư mục. Hãy đăng xuất rồi đăng nhập lại, sau đó thử lại.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = Ổ đĩa đã đầy.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Trindle không ghi được vào ổ đĩa lúc này.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = Phần của hệ thống làm việc này không chạy.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = Phản hồi quá lâu. Hãy thử lại.

# Cause given in red words after an action failed: the password was not given.
# SAFETY: needs a human review before it ships
settings-common-why-password = Chưa nhập mật khẩu.

# Cause given in red words after an action failed: the account is not allowed.
# SAFETY: needs a human review before it ships
settings-common-why-not-allowed = Tài khoản này không làm được việc đó.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Máy này thiếu một thứ cần thiết.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } và { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Word drawn in the middle of that picture.
settings-common-ring-name = Vòng tròn

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Trindle không kết nối được với phần điều khiển sạc của máy tính xách tay: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = Máy tính xách tay không nhận giới hạn sạc.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Máy này không cho bạn dừng sạc sớm.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = Không đặt được { $path }: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Dùng thử Trindle trong máy ảo

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle đang chạy trong { $machine }. Nó chạy tốt ở đó, nhưng máy ảo không bao giờ nhanh và mượt bằng máy thật. Hãy đánh giá tốc độ trên máy thật.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle đang chạy trong { $machine } mà không có đồ hoạ 3D. Bộ xử lý phải tự vẽ mọi thứ mà chip đồ hoạ thường vẽ. Cửa sổ, video và trang web sẽ chậm hơn trên máy thật, nên Trindle bỏ các hiệu ứng chuyển động. Nếu máy thật đang chạy máy ảo này cho phép, hãy bật tăng tốc 3D trong cài đặt màn hình của máy ảo. Việc đó tạo khác biệt rất lớn. Nếu bạn cài Trindle trên máy thật, không có điều nào ở trên xảy ra.

# A list of choices ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-or = { $rest } hoặc { $last }
