
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = Khởi động cùng đăng nhập

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay tự khởi động khởi động đăng nhập tự động chạy chờ

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = Các ứng dụng tự mở khi bạn đăng nhập. Độ trễ cho phép ứng dụng chờ mạng hoặc một ứng dụng khác trước.

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = Không có gì tự khởi động khi bạn đăng nhập.

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = { $name } tự khởi động khi bạn đăng nhập.

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
       *[other] { $n } ứng dụng tự khởi động khi bạn đăng nhập.
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
       *[other] { $n } ứng dụng khác được giữ lại nhưng đang tắt.
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = Ngay lập tức

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = Sau 1 phút

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = Sau 2 phút

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
       *[other] Sau { $n } giây
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = Độ trễ cho { $name }

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = Khởi động cùng đăng nhập

# Name of the bin button on an app's line that stops it starting at login.
# SAFETY: needs a human review before it ships
settings-startup-stop = Ngừng khởi động cùng đăng nhập

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = Không gỡ được, nên nó vẫn khởi động khi đăng nhập.

# Red words under the add card when something could not be added.
settings-startup-not-added = Không thêm được.

# Placeholder of the search box above the list of installed apps.
settings-startup-find = Tìm một ứng dụng

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = Khởi động mục này cùng đăng nhập

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = Không thêm được: ứng dụng không còn được cài.

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = ví dụ: syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = Chọn một chương trình trên ổ đĩa

# Heading of the line for adding a command.
settings-startup-command = Lệnh

# Small words under the command line.
settings-startup-command-about = Như gõ trong Terminal, hoặc một chương trình trên ổ đĩa.

# Red words under the command line when nothing was typed.
settings-startup-command-empty = Hãy gõ một lệnh, hoặc chọn một chương trình.

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = Chọn chương trình khởi động cùng đăng nhập

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = Không có gì tự khởi động

# Small words under that line.
settings-startup-none-about = Hãy thêm một ứng dụng bên cạnh.

# Title of the card listing what starts at login.
settings-startup-list-title = Khởi động khi đăng nhập

# Words behind the i of the card listing what starts at login.
settings-startup-about = Mỗi ứng dụng có thể chờ một lúc sau khi bạn đăng nhập, để mạng hoặc một ứng dụng khác sẵn sàng trước. Công tắc giữ nó ở đây mà không khởi động; thùng rác ngừng hẳn việc khởi động khi đăng nhập.

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } chạy: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = Chọn từ các ứng dụng đã cài

# Name of the button that opens the box for typing a command.
settings-startup-open-command = Gõ một lệnh, hoặc chọn một chương trình

# Heading of the line for adding an installed app.
settings-startup-app-heading = Một ứng dụng đã cài

# Small words under that line.
settings-startup-app-about = Chọn một ứng dụng từ danh sách.

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = Một lệnh hoặc một chương trình

# Small words under that line.
settings-startup-cmd-about = Gõ vào, hoặc chọn từ ổ đĩa.

# Words behind the i of the Add an app card.
settings-startup-add-about = Chọn từ các ứng dụng đã cài, hoặc đưa ra một lệnh để chạy, như bạn gõ trong Terminal, hoặc một chương trình chọn từ ổ đĩa. Nó khởi động ở lần đăng nhập tiếp theo, ngay lập tức; sau khi vào danh sách, bạn có thể chọn thời gian chờ cho nó.

# Title of the card for adding an app.
settings-startup-add-title = Thêm một ứng dụng
