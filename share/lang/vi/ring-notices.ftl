
# machine translated (Vietnamese), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = Pin còn { $pct }%

# Line under the low battery warning's title.
ring-battery-plug-in = Hãy cắm sạc sớm.

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = Đang sạc

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = Đã rút sạc

# Line under the title when the charger comes out.
ring-battery-running = Đang chạy bằng pin, còn { $pct }%

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 phút

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 giờ

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 giờ

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = Sáng hôm sau

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = Báo lại sau 10 phút

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = Báo lại sau 1 giờ

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = Báo lại sau 6 giờ

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = Báo lại vào sáng hôm sau

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = Quay lại tin nhắn

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = Bỏ ghim

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = Ghim

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = Báo lại sau

# Tip of the button on the stack of notification circles that clears them all.
# SAFETY: needs a human review before it ships
ring-act-clear-all = Xóa tất cả

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = Đã sao chép

# Tip of the button on a notification circle that copies its message.
ring-act-copy = Sao chép tin nhắn

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = Nhấp để chuyển cho trợ lý của bạn.

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = Màn hình nền của Trindle

# Name of the login screen in the crash notice.
ring-crash-greeter = Màn hình đăng nhập của Trindle

# Name of the lock screen in the crash notice.
ring-crash-lock = Màn hình khóa của Trindle

# Name of Settings in the crash notice.
ring-crash-settings = Cài đặt Trindle

# Name of Apps in the crash notice.
ring-crash-apps = Ứng dụng của Trindle

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = { $name } của Trindle

# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } đã dừng bất ngờ

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = Ổ đó không còn nữa

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = Không thể mở { $name }

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = Ngủ

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = Ngủ đông

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = Khóa màn hình

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = Không làm gì

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = Tắt máy

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = Đóng nắp sẽ làm theo mục Cài đặt, Nguồn

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = Giữ ở bất kỳ đâu để phóng to

# Screen reader name of the magnifier's glass.
ring-magnify-name = Kính lúp

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = Dừng phóng to

# Tip of the switch in the ring that turns the microphone off for every program.
# SAFETY: needs a human review before it ships
ring-mic-off-tip = Tắt micrô cho mọi chương trình

# Tip of the switch in the ring that turns the microphone back on.
# SAFETY: needs a human review before it ships
ring-mic-on-tip = Bật lại micrô

# Line in the ring when something other than the person pressed the microphone switch on.
# SAFETY: needs a human review before it ships
ring-mic-own-press = Chỉ chính bạn nhấn mới bật lại được micrô.

# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = Không thể đổi công tắc micrô: { $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = Không tìm thấy chương trình

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = Do một chương trình khác gửi

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = Không thể khởi động

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = Đã sao chép cho trợ lý của bạn

# Screen reader name of a notification circle.
ring-notes-name = Thông báo

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = Đang di chuyển. Thả ra để đặt xuống, Escape để trả về.

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = Đã di chuyển

# Said when Escape put a held notification back.
ring-notes-put-back = Đã trả về chỗ cũ

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = Hãy ghim để di chuyển

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
       *[other] { $n } giây
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = Đến khi bạn đóng

# Time under a notification from yesterday.
ring-notify-yesterday = Hôm qua { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = Dưới bên phải

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = Dưới bên trái

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = Trên bên trái

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = Trên bên phải

# Title of the notice when a plugin that asks for permissions has been given none.
# SAFETY: needs a human review before it ships
ring-plugin-needs = { $name } cần quyền của bạn

# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
# SAFETY: needs a human review before it ships
ring-plugin-cannot-work = Chưa thể hoạt động: được cho phép 0 trên { $asks }.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = Đã sao chép. { $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = Đã kết nối mạng

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = Đang mở tệp của bạn

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
       *[other] { $names } và { $more } mục khác
    }

