
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the installer's window (what the window list and a screen reader call it).
installer-window-title = Cài đặt { $name }

# Name of the first stop on the line the mark rolls along, the welcome page.
installer-stop-welcome = Chào mừng

# Name of the second stop on that line, the page where the disk is chosen.
installer-stop-disk = Ổ đĩa

# Name of the third stop on that line, the page about encrypting the disk.
installer-stop-privacy = Riêng tư

# Name of the fourth stop on that line, the page for the disk password.
installer-stop-password = Mật khẩu

# Name of the fifth stop on that line, the page about the person.
installer-stop-you = Bạn

# Name of the sixth stop on that line, the page that says what is about to happen.
installer-stop-ready = Sẵn sàng

# Name of the seventh stop on that line, shown while the install runs.
installer-stop-installing = Đang cài đặt

# Name of the last stop on that line, shown when the install is done.
installer-stop-done = Xong

# Foot of the install page, before the helper has said what it is doing.
installer-getting-ready = Đang chuẩn bị

# Red line at the foot when the installer's helper could not be started.
installer-no-start = Không thể khởi động trình cài đặt: { $why }

# Written to the error output when the plan sent to the helper could not be read.
installer-plan-unreadable = Không thể đọc kế hoạch cài đặt.

# Written to the error output when the plan sent to the helper could not be read, with the reason.
installer-plan-unreadable-why = Không thể đọc kế hoạch cài đặt: { $why }

# Red line at the foot when the helper stopped without saying why.
installer-stopped = Quá trình cài đặt đã dừng. Chưa có thêm gì được ghi.

# Name of the round button at the foot that goes back a page.
installer-back = Quay lại

# Name of the main button at the foot that goes to the next page.
installer-next = Tiếp theo

# Name of the main button at the foot of the Ready page that starts the install.
installer-install = Cài đặt

# Name of the main button at the foot of the last page that restarts the computer.
installer-restart = Khởi động lại ngay

# Beside the keyboard test box on the first page.
installer-try-keyboard = Thử bàn phím

# Name of the box on the first page where the person types to try the keyboard.
installer-try-keyboard-box = Gõ vào đây để thử bàn phím

# Note beside Install after one press, when the second press writes beside another system.
installer-arm = Bấm lần nữa để cài đặt

# Note beside Install after one press, when the second press erases a whole disk.
# SAFETY: needs a human review before it ships
installer-arm-erase = Bấm lần nữa để xóa sạch và cài đặt

# Note beside Next on the first page while no language is chosen.
installer-missing-language = Hãy chọn ngôn ngữ

# Note beside Next on the first page while no keyboard is chosen.
installer-missing-keyboard = Hãy chọn bàn phím

# Note beside Next on the first page while no time zone is chosen.
installer-missing-time = Hãy chọn múi giờ

# Note beside Next on the disk page while no disk is chosen.
installer-missing-disk = Hãy chọn nơi cài Trindle

# Note beside Next on the disk password page while no password has been typed twice.
installer-missing-disk-password = Gõ mật khẩu ổ đĩa hai lần, giống nhau cả hai lần

# Note beside Next on the account page while the sign-in name is not good.
installer-missing-username = Cần có tên đăng nhập

# Note beside Next on the account page while the computer's name is not good.
installer-missing-hostname = Hãy đặt tên cho máy tính này

# Note beside Next on the account page while no password has been typed twice.
installer-missing-password = Gõ mật khẩu của bạn hai lần, giống nhau cả hai lần

# Title of the first page.
installer-welcome-title = Chào mừng bạn đến với { $name }

# Sentence under the title of the first page.
installer-welcome-help = Hãy chọn ngôn ngữ; bàn phím và giờ sẽ theo ngôn ngữ đó, và bạn có thể đổi cả hai.

# Head of the card for choosing the language, and the name of its search box.
installer-language = Ngôn ngữ

# Head of the card for choosing the keyboard, and the name of its search box.
installer-keyboard = Bàn phím

# Head of the card for choosing the time zone, and the name of its search box.
installer-time = Múi giờ

# Under a card's head while nothing has been chosen in its list.
installer-select = Chọn

# Title of the disk page.
installer-disks-title = Nơi cài { $name }

# Sentence under the title of the disk page.
# SAFETY: needs a human review before it ships
installer-disks-help = Mọi thứ trên ổ đĩa bạn chọn sẽ bị thay thế, trừ khi bạn chừa chỗ bên cạnh những gì đang có.

# Shown on the disk page when no disk is big enough.
installer-disks-none = Không tìm thấy ổ đĩa nào đủ lớn. Trindle cần 20 GB.

# Name and words of the choice to erase a whole disk.
# SAFETY: needs a human review before it ships
installer-disks-whole = Dùng cả ổ đĩa, thay thế mọi thứ trên đó

# Name and words of the choice to keep a partition and share the disk; it says which one.
installer-disks-share = Giữ { $what } và chia sẻ ổ đĩa

# Under the slider that decides how much of a shared partition is kept: the size each side gets.
installer-disks-split = { $kept } dành cho dữ liệu hiện có, { $mine } dành cho Trindle

# A partition or disk with its size, such as the name of a disk and how big it is.
installer-named-size = { $name }, { $size }

# Name of a partition with nothing on it.
installer-empty-space = Vùng trống

# Names run together in a list, the last one after the word and.
installer-list-and = { $rest } và { $last }

# Under a disk's name: what is on it.
installer-disk-has = Có { $list } trên đó

# Under a disk's name: what is on it, when it is encrypted.
installer-disk-has-encrypted = Có { $list } trên đó, đã mã hóa

# Under a disk's name: it holds an encrypted system Trindle cannot read.
installer-disk-has-encrypted-system = Có một hệ thống đã mã hóa trên đó

# Under a disk's name: there is nothing on it.
installer-disk-empty = Trống

# Under a disk's name: it holds files but no system Trindle knows.
installer-disk-has-files = Có tệp trên đó, nhưng không có hệ thống nào Trindle nhận ra

# Head of the card with the choice about a swap file, with the size of this computer's memory.
installer-swap-title = Tệp hoán đổi { $gb } GB

# Name of the switch that makes a swap file.
installer-swap-switch = Tạo tệp hoán đổi

# Under the swap file's head when Secure Boot is on.
installer-swap-secure-boot = Secure Boot đang bật, nên máy tính này không thể ngủ đông, nhưng tệp hoán đổi vẫn giúp ích khi bộ nhớ sắp hết. Tệp chỉ được tạo nếu ổ đĩa còn dư nhiều chỗ.

# Under the swap file's head when the computer can hibernate.
installer-swap-note = Chừa chỗ để ngủ đông, giữ nguyên mọi thứ đang mở khi tắt nguồn, và là nơi dự phòng khi bộ nhớ sắp hết. Không có nó, máy tính không thể ngủ đông. Tệp chỉ được tạo nếu ổ đĩa còn dư nhiều chỗ.

# Title of the page about encrypting the disk.
installer-privacy-title = Mã hóa ổ đĩa

# Sentence under the title of that page.
installer-privacy-help = Ổ đĩa đã mã hóa không thể bị đọc bởi bất kỳ ai tháo nó ra khỏi máy tính này.

# Head of the card with the three choices.
installer-privacy-locked = Cách khóa ổ đĩa

# Name and words of the choice not to encrypt the disk.
installer-privacy-none = Không mã hóa ổ đĩa

# Name and words of the choice to encrypt the disk with a password typed at start-up.
installer-privacy-password = Mã hóa và hỏi mật khẩu khi máy tính khởi động

# Name and words of the choice to encrypt the disk and let the computer's TPM chip unlock it.
installer-privacy-chip-choice = Mã hóa và để chip TPM của máy tính này mở khóa

# Head of the card about the TPM chip.
installer-privacy-about = Về chip

# In the card about the chip, on a computer that has one.
installer-privacy-chip = Máy tính này có chip TPM. Lần khởi động đầu tiên sau khi cài sẽ hỏi mật khẩu một lần, trong lúc chip làm quen với máy tính này; sau đó chip tự mở khóa ổ đĩa và bạn không cần gõ gì. Hãy giữ mật khẩu: đó là cách duy nhất để vào lại nếu chip bị thay hoặc ổ đĩa được chuyển sang máy tính khác.

# In the card about the chip, on a computer that has none Trindle can use.
installer-privacy-no-chip = Máy tính này không có chip TPM mà Trindle dùng được, nên sẽ hỏi mật khẩu mỗi lần khởi động. Trên máy tính có chip, chip có thể tự mở khóa ổ đĩa.

# Title of the disk password page.
installer-disk-title = Mật khẩu ổ đĩa

# Sentence under the title of the disk password page.
installer-disk-help = Mật khẩu được gõ trước mọi thứ khác khi máy tính khởi động.

# Head of the card with the disk password boxes, and the name of the first box.
installer-disk-password = Mật khẩu ổ đĩa

# Name of the round button that makes up a passphrase.
installer-disk-suggest = Gợi ý một cụm mật khẩu

# Label and name of the second box of a password, where it is typed again.
installer-same-again = Nhập lại

# Red line under the password boxes when the two do not match.
installer-mismatch = Hai mật khẩu không khớp nhau.

# Head of the card that tells the person to write the password down.
installer-disk-write-down = Hãy ghi lại

# Words in that card.
installer-disk-write-down-text = Hãy ghi mật khẩu này ra giấy và cất ở nơi an toàn, xa máy tính. Không có nó, không ai mở được ổ đĩa, kể cả bạn, mãi mãi. Không có cách nào vào lại. Bốn từ thông thường tạo thành một cụm mật khẩu dễ gõ mà khó đoán.

# Title of the account page.
installer-you-title = Về bạn

# Sentence under the title of the account page.
installer-you-help = Đây là tài khoản bạn dùng để đăng nhập.

# Head of the card with the name boxes.
installer-account = Tài khoản của bạn

# Label and name of the box for the person's name.
installer-your-name = Tên của bạn

# Label and name of the box for the sign-in name.
installer-user-name = Tên đăng nhập

# Label and name of the box for the computer's name.
installer-host-name = Tên của máy tính này

# Head of the card with the password boxes.
installer-your-password = Mật khẩu của bạn

# Label and name of the first password box.
installer-password = Mật khẩu

# Label of the strength meter and the name a screen reader gives it.
installer-strength = Độ khó đoán

# Red line under the account boxes: the sign-in name is empty.
installer-user-needed = Cần có tên đăng nhập.

# Red line under the account boxes: a name is too long.
installer-name-too-long = Tên này quá dài.

# Red line under the account boxes: the sign-in name does not start with a small letter.
installer-user-start = Hãy bắt đầu bằng một chữ cái thường.

# Red line under the account boxes: the sign-in name uses letters it may not.
installer-user-letters = Chỉ dùng chữ cái thường, chữ số, - và _.

# Red line under the account boxes: the sign-in name belongs to the system.
installer-user-taken = Tên này đã được hệ thống sử dụng.

# Red line under the account boxes: the computer's name is empty.
installer-host-needed = Hãy đặt tên cho máy tính này.

# Red line under the account boxes: the computer's name uses letters it may not.
installer-host-letters = Chỉ dùng chữ cái, chữ số và -.

# Under the strength meter: how long a computer would take to guess the password, when it is at once.
installer-guess-now = Máy tính có thể đoán ra ngay lập tức.

# Under the strength meter: how long a computer would take to guess the password, in minutes.
installer-guess-minutes =
    { $n ->
       *[other] Máy tính có thể đoán ra trong khoảng { $n } phút.
    }

# Under the strength meter: how long a computer would take to guess the password, in hours.
installer-guess-hours =
    { $n ->
       *[other] Máy tính có thể đoán ra trong khoảng { $n } giờ.
    }

# Under the strength meter: how long a computer would take to guess the password, in days.
installer-guess-days =
    { $n ->
       *[other] Máy tính có thể đoán ra trong khoảng { $n } ngày.
    }

# Under the strength meter: how long a computer would take to guess the password, in years.
installer-guess-years =
    { $n ->
       *[other] Máy tính cần khoảng { $n } năm để đoán ra.
    }

# Under the strength meter: the password is too strong to guess in any time worth saying.
installer-guess-never = Máy tính cần nhiều thời gian hơn cả lịch sử loài người để đoán ra.

# Title of the Ready page.
installer-ready-title = Sẵn sàng cài đặt

# Head of the card that lists what will happen.
installer-ready-nothing-changed = Chưa có gì bị thay đổi

# Head of the card about security updates.
installer-ready-security = Cập nhật bảo mật

# In the card about security updates.
installer-ready-security-text = { $name } tự cài các bản vá bảo mật của Debian mỗi ngày. Không thể tắt việc này.

# Sentence under the title when a whole disk will be replaced.
# SAFETY: needs a human review before it ships
installer-ready-replaced = Mọi thứ trên { $disk } sẽ bị thay thế.

# Sentence under the title when the install goes beside what is there.
installer-ready-kept = Mọi thứ trên { $part } được giữ nguyên, nhưng hãy sao lưu trước: thu nhỏ phân vùng có thể làm mất tệp nếu mất điện.

# Sentence under the title when no disk has been chosen.
installer-ready-none = Chưa chọn ổ đĩa nào.

# Name of a row in the list: the whole disk will be used.
installer-ready-whole = Toàn bộ { $disk }

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-whole-text = Mọi thứ trên đó sẽ bị { $name } thay thế.

# Name of a row in the list: the install goes beside a partition.
installer-ready-beside = Bên cạnh { $part }

# Words of that row.
# SAFETY: needs a human review before it ships
installer-ready-beside-text = { $part } sẽ được thu nhỏ xuống còn { $size }, và { $name } được cài vào vùng trống giải phóng ra.

# Name of a row in the list when no disk is chosen.
installer-ready-no-disk = Không có ổ đĩa

# Words of that row.
installer-ready-no-disk-text = Chưa chọn ổ đĩa.

# Name of a row in the list: the disk will not be encrypted.
installer-ready-plain = Không mã hóa

# Words of that row.
installer-ready-plain-text = Ai tháo ổ đĩa ra cũng đọc được.

# Name of a row in the list: the disk will be encrypted.
installer-ready-locked = Đã mã hóa

# Words of that row when a password is typed at each start.
installer-ready-locked-password = Bạn gõ mật khẩu của ổ đĩa mỗi lần máy tính khởi động.

# Words of that row when the TPM chip unlocks the disk.
installer-ready-locked-chip = Gõ mật khẩu một lần, sau đó chip TPM của máy tính này sẽ mở khóa.

# Words of the language row: the keyboard and time zone chosen.
installer-ready-language = Bàn phím { $keyboard }, múi giờ { $zone }.

# Title of the page shown while the install runs.
installer-installing-title = Đang cài đặt { $name }

# Head of the card that holds the short tour.
installer-tour = Giới thiệu nhanh

# Title of the last page.
installer-done-title = Đã cài đặt { $name }

# Sentence under the title of the last page.
installer-done-help = Hãy rút USB ra, rồi khởi động lại.

# Head of the card that repeats the warning about the disk password.
installer-done-keep = Hãy giữ mật khẩu ổ đĩa an toàn

# In that card when the TPM chip unlocks the disk.
installer-keep-chip = Ổ đĩa đã được mã hóa. Lần khởi động đầu tiên sẽ hỏi mật khẩu ổ đĩa một lần, trong lúc chip làm quen với máy tính này; sau đó ổ đĩa tự mở. Hãy ghi mật khẩu đó ra giấy và cất ở nơi an toàn, xa máy tính này. Không có nó, không ai mở được ổ đĩa, kể cả bạn, mãi mãi.

# In that card when the password is typed at each start.
installer-keep-password = Ổ đĩa đã được mã hóa. Nếu chưa ghi, hãy ghi mật khẩu ổ đĩa ra giấy ngay bây giờ và cất ở nơi an toàn, xa máy tính này. Không có nó, không ai mở được ổ đĩa, kể cả bạn, mãi mãi.

# Title of the first slide.
installer-slide-ring = Mọi thứ trong một vòng tròn

# Words of the first slide.
installer-slide-ring-text = Di con trỏ đến mép màn hình rồi nhấp đúp, vòng tròn sẽ mở ra: các chương trình của bạn ở vòng ngoài, giờ và các nút bật tắt ở giữa.

# Title of the second slide.
installer-slide-snapshots = Khôi phục những gì bản cập nhật làm hỏng

# Words of the second slide.
installer-slide-snapshots-text = Một bản chụp hệ thống được tạo trước và sau mỗi thay đổi phần mềm. Trong Cài đặt, bạn có thể đưa cả hệ thống về bất kỳ bản chụp nào, và menu khởi động cũng làm được việc này khi máy không khởi động.

# Title of the third slide.
installer-slide-apps = Ứng dụng không làm phiền bạn

# Words of the third slide.
installer-slide-apps-text = Ứng dụng Flathub, ứng dụng của chính Debian, AppImage và nguồn phần mềm ở cùng một nơi, với bản sao lưu bạn có thể mang sang máy tính khác.

# Title of the fourth slide.
installer-slide-yours = Của riêng bạn, thật nhanh

# Words of the fourth slide.
installer-slide-yours-text = Giao diện sáng hoặc tối, âm thanh riêng của bạn, và bảng nhớ tạm ghi nhớ những gì bạn đã chép.

# Title of the fifth slide.
installer-slide-small = Nhỏ gọn và êm

# Words of the fifth slide.
installer-slide-small-text = Màn hình nền dùng ít bộ nhớ và khởi động trong vài giây, nên máy tính xách tay cũ như mới trở lại.

# Title of the tour slide about the ring.
installer-tour-ring = Vòng tròn là cả màn hình nền

# Note on the ring picture, at the corner of the screen.
installer-tour-ring-corner = Đặt con trỏ vào góc này để mở vòng tròn

# Note on the ring picture, at the time.
installer-tour-ring-time = Giờ và ngày, ở giữa

# Note on the ring picture, at the programs.
installer-tour-ring-programs = Các chương trình của bạn nằm ở vòng ngoài

# Note on the ring picture, at the switches.
installer-tour-ring-switches = Âm thanh, màn hình, tất cả ứng dụng, giữ máy thức, và tắt máy

# Title of the tour slide about Settings.
installer-tour-settings = Cài đặt, bằng lời dễ hiểu

# Note on the Settings picture, at the search box.
installer-tour-settings-search = Gõ vào đây để tìm một cài đặt mà không phải lục tìm

# Note on the Settings picture, at Network.
installer-tour-settings-network = Mạng là nơi bạn kết nối mạng Wi-Fi

# Note on the Settings picture, at the light or dark choice.
installer-tour-settings-dark = Sáng hoặc tối, tùy bạn muốn lúc nào

# Note on the Settings picture, at the colours.
installer-tour-settings-colours = Hai mươi màu, hoặc tự nhập màu của bạn

# Title of the tour slide about Apps.
installer-tour-apps = Ứng dụng, cho mọi nhu cầu khác

# Note on the Apps picture, at the search box.
installer-tour-apps-search = Tìm chương trình theo tên hoặc theo công dụng

# Note on the Apps picture, at the list.
installer-tour-apps-many = Hàng nghìn chương trình miễn phí, từ Flathub và Debian

# Note on the Apps picture, at updates.
installer-tour-apps-updates = Cập nhật cho mọi thứ, kể cả chính Trindle

# Title of the tour slide about voice control.
installer-tour-voice = Bạn có thể nói chuyện với máy thay vì gõ

# Note on the voice picture, at where voice control lives.
installer-tour-voice-where = Điều khiển bằng giọng nói nằm ở đây: Cài đặt, rồi Vòng tròn

# Note on the voice picture, at the button that adds voice.
installer-tour-voice-add = Bấm Thêm giọng nói một lần. Dung lượng khoảng 150 MB và mất vài phút

# Note on the voice picture, about what to say.
installer-tour-voice-say = Sau đó chạm vào giữa vòng tròn và nói tên một ứng dụng, “to hơn”, hoặc “cài đặt Wi-Fi”

# Note on the voice picture, about where speech is understood.
installer-tour-voice-private = Lời bạn nói được hiểu ngay trên máy tính này và không đi đâu khác

# Title of the tour slide about snapshots.
installer-tour-snapshots = Đường lui, trước khi có gì thay đổi

# Note on the snapshots picture, about when a snapshot is taken.
installer-tour-snapshots-before = Máy tính tạo một bản chụp trước mỗi lần cập nhật

# Note on the snapshots picture, about going back.
installer-tour-snapshots-back = Nếu có gì hỏng, hãy quay về như cũ từ menu khởi động

# Name of the picture of the ring, and the head of its card.
installer-live-caption = Vòng tròn, nơi mọi thứ có mặt

# Sentence under the title of the welcome from a stick.
installer-live-help = Bạn đang chạy từ USB, nên chưa có gì trên máy tính này bị thay đổi.

# Head of the card with the three first things to do.
installer-live-begin = Để bắt đầu

# Name of the first thing to do.
installer-live-open = Mở vòng tròn

# Line under it.
installer-live-open-line = Nhấp đúp ở mép màn hình, hoặc nhấn Super.

# Name of the second thing to do.
installer-live-speak = Nói với máy

# Line under it when voice control is there.
installer-live-speak-line = Chạm vào giữa vòng tròn và nói điều bạn muốn.

# Line under it when voice control has to be added.
installer-live-speak-add = Thêm giọng nói trong Cài đặt, Vòng tròn, rồi chạm vào giữa và nói.

# Name of the third thing to do.
installer-live-tab = Tất cả trong vòng tròn

# Line under it.
installer-live-tab-line = Những gì đang mở, thông báo và bảng nhớ tạm của bạn đều ở đó.

# The choice of the dark look.
installer-live-dark = Tối

# The choice of the light look.
installer-live-light = Sáng

# Name the screen reader gives to the choice between dark and light.
installer-live-look = Sáng hoặc tối, cho cả màn hình nền

# Name of the round button that closes this window and opens the ring.
installer-live-around = Xem qua trước

# Name of the main button that opens the installer.
installer-live-install = Cài đặt { $name }

# Foot of the install page while room is made on the disk.
installer-step-room = Đang tạo chỗ trống

# Foot of the install page while the disk is prepared.
installer-step-prepare = Đang chuẩn bị ổ đĩa

# Foot of the install page while a place is made beside another system.
installer-step-place = Đang tạo chỗ cho Trindle

# Foot of the install page while the disk is looked at.
installer-step-check = Đang kiểm tra những gì có sẵn trên ổ đĩa

# Foot of the install page while another system is made smaller.
installer-step-smaller = Đang thu nhỏ hệ thống kia

# Foot of the install page while the disk is encrypted.
installer-step-lock = Đang khóa ổ đĩa

# Foot of the install page while the TPM chip is set up.
installer-step-chip = Đang thiết lập chip của máy tính

# Foot of the install page while the system is copied.
installer-step-copy = Đang chép Trindle vào ổ đĩa

# Foot of the install page while settings are written.
installer-step-settings = Đang ghi các cài đặt

# Foot of the install page while the person's account is made.
installer-step-account = Đang tạo tài khoản của bạn

# Foot of the install page while the stick's own parts are taken out.
installer-step-live-parts = Đang gỡ các phần chạy trực tiếp

# Foot of the install page while the start-up files are made.
installer-step-startup = Đang tạo các tệp khởi động

# Foot of the install page while the boot menu is set up.
installer-step-boot = Đang thiết lập menu khởi động

# Foot of the install page while snapshots are set up.
installer-step-snapshots = Đang thiết lập các bản chụp hệ thống

# Foot of the install page at the very end.
installer-step-finishing = Đang hoàn tất

# Red line at the foot: a program the installer runs could not start.
installer-run-no-start = Không thể khởi động { $program }: { $why }

# Red line at the foot: a program the installer runs did not finish.
installer-run-failed = { $program } chưa hoàn thành. { $why }

# Red line at the foot: a file could not be written.
installer-write-failed = Không thể ghi { $path }: { $why }

# Red line at the foot: something given as a disk is not one.
installer-not-a-disk = { $dev } không phải là ổ đĩa.

# Red line at the foot: something given as a disk is not on this computer.
installer-not-a-disk-here = { $dev } không phải là ổ đĩa trên máy tính này.

# Red line at the foot: no disk was chosen.
installer-no-disk-chosen = Chưa chọn ổ đĩa nào.

# Red line at the foot: the sign-in name cannot be used.
installer-bad-username = Trindle không dùng được tên đăng nhập đó.

# Red line at the foot: the computer's name cannot be used.
installer-bad-hostname = Trindle không dùng được tên máy tính đó.

# Red line at the foot: the language cannot be used.
installer-bad-language = Trindle không dùng được ngôn ngữ đó.

# Red line at the foot: the keyboard cannot be used.
installer-bad-keyboard = Trindle không dùng được bàn phím đó.

# Red line at the foot: the time zone cannot be used.
installer-bad-timezone = Trindle không dùng được múi giờ đó.

# Red line at the foot: the person's name has something that cannot be written.
installer-bad-fullname = Tên đó có ký tự mà Trindle không ghi được.

# Red line at the foot: a password has a line break in it.
installer-bad-password = Mật khẩu không được chứa dấu xuống dòng.

# Red line at the foot: the second half of the installer was not started as root.
installer-needs-root = Nửa sau của trình cài đặt phải chạy với quyền root.

# Red line at the foot: there is no boot partition to share.
installer-no-boot-partition = Ổ đĩa này không có phân vùng khởi động để dùng chung, nên Trindle không thể cài bên cạnh những gì đang có.

# Red line at the foot: the partition just made cannot be found.
installer-new-part-missing = Không tìm thấy phần mới của ổ đĩa.

# Red line at the foot: the size of a partition cannot be read.
installer-size-unreadable = Không thể đọc kích thước của { $part }.

# Red line at the foot: a partition of this kind cannot be made smaller.
installer-cannot-shrink = Trindle không thể thu nhỏ phân vùng { $fs }.

# Red line at the foot: a partition of a kind that is not known cannot be made smaller.
installer-cannot-shrink-unknown = Trindle không thể thu nhỏ một phân vùng không rõ loại.

# Red line at the foot: encrypting needs a password and there is none.
installer-disk-password-needed = Cần có mật khẩu để mã hóa ổ đĩa.

# Red line at the foot: the copy of the system could not start.
installer-copy-no-start = Không thể chép hệ thống: { $why }

# Red line at the foot: the copy of the system failed.
installer-copy-failed = Không thể chép hệ thống. { $why }

# Red line at the foot: there is not enough room for a swap file.
installer-swap-no-room = Không đủ chỗ cho tệp hoán đổi.

# Red line at the foot: the swap file's place on the disk cannot be read.
installer-swap-unreadable = Không thể đọc vị trí của tệp hoán đổi trên ổ đĩa.

# Red line at the foot: the new partitions could not be named in the table of disks.
installer-fstab-unidentified = Không thể nhận diện các phân vùng mới.

# Red line at the foot: the time zone could not be set.
installer-timezone-failed = Không thể đặt múi giờ: { $why }

# Red line at the foot: the boot menu could not be installed.
installer-boot-failed = Không thể cài menu khởi động trên máy tính này.

# Name of that button after one press, when a second press goes ahead.
installer-sure = Chắc chứ?



# Note on the ring picture, at the tab at the edge.
installer-tour-ring-foot = Những gì đang mở, bảng nhớ tạm, và một nút xoá sạch màn hình

# Red line at the foot: the chosen disk is the stick Trindle started from.
installer-disk-is-stick = { $dev } là thanh USB mà Trindle đang chạy từ đó.

# Red line at the foot: something on the chosen disk is mounted or swapped to.
installer-disk-in-use = { $dev } đang được dùng. Hãy đóng thứ đang dùng nó, rồi thử lại.

# Said beside how far the install has got, once it has started.
installer-started = Đã bắt đầu cài đặt

# The button that ends an install under way.
installer-halt = Dừng

# Beside Stop after its first press.
installer-halt-sure = Dừng cài đặt? Nội dung cũ của ổ đĩa đã mất rồi.

# While an install is being stopped.
installer-halting = Đang dừng

# When Stop could not let go of the disk.
installer-halt-stuck = Không thể nhả ổ đĩa ra. Hãy khởi động lại máy tính để thử lại.

# The copy is done and the person is still answering.
installer-step-waiting = Đã sao chép xong. Đang chờ câu trả lời của bạn.



installer-already-running = Một trình cài đặt khác đang chạy trên máy này.


# Shown beside the disabled Next button on the first page, and as the reason the install will not start, when the computer has under 1 GB of memory.
installer-memory-short = Trindle cần ít nhất 1 GB bộ nhớ, và máy tính này có ít hơn.

# Shown beside the disabled Next button on the privacy page, and as the reason the install will not start, when the disk is to be encrypted on a computer that did not start by UEFI.
installer-lock-needs-uefi = Ổ đĩa chỉ có thể được mã hóa trên máy tính khởi động bằng UEFI, và máy này thì không.

# Red words when the Windows partition chosen to share the disk was hibernated or left in Windows' fast start-up; { $part } is the partition.
installer-windows-asleep = Windows trên { $part } đã được ngủ đông hoặc để ở chế độ khởi động nhanh, nên thu nhỏ phân vùng có thể làm mất tệp của nó. Hãy khởi động Windows, chọn Khởi động lại, rồi thử lại.

# Red words on the last page when the snapshots could not be set up; { $why } is the reason.
installer-snapshots-failed = Trindle đã được cài đặt, nhưng không thể thiết lập các bản chụp hệ thống: { $why }

# Red words on the last page when the boot menu could not be made after setting up snapshots; { $why } is the reason.
installer-snapshots-menu-failed = Trindle đã được cài đặt, nhưng không thể làm cho menu khởi động liệt kê các bản chụp hệ thống: { $why }

# Head of the card on the last page that lists what could not be done.
installer-done-problems = Chưa thiết lập xong mọi thứ

