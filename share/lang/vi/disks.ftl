
# machine translated (Vietnamese), not yet reviewed by a person.

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = Ổ đĩa

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = Về trang này

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = Mọi ổ đĩa trong hoặc cắm vào máy tính này, vẽ thành một thanh gồm các phần của nó. Mỗi phần rộng bằng dung lượng của nó và được tô đầy theo mức đã dùng, nên dễ thấy phần nào đã đầy.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = Dưới hình vẽ, mỗi ổ đĩa có một thẻ. Mở một phần, gắn hoặc bỏ gắn nó, hoặc tháo ổ đĩa ra an toàn hay tắt nguồn nó. Biểu tượng bút chì trên một phần mở ra các thay đổi: tên, thư mục riêng, có sẵn sàng khi khởi động hay không, kích thước, kiểm tra và sửa chữa, định dạng hoặc xóa nó. Dấu cộng trong vùng trống tạo một phần mới.

# Third paragraph of the longer words that open from the i beside the title.
# SAFETY: needs a human review before it ships
disks-help-3 = Ổ đĩa mà hệ thống chạy từ đó vẫn được hiển thị nhưng không bao giờ được đề nghị bất kỳ thay đổi nào làm xóa hay dời dữ liệu. Mọi thay đổi với bất kỳ ổ đĩa nào đều hỏi mật khẩu quản trị viên, mỗi lần. Biểu tượng con mắt ẩn một ổ đĩa hoặc một phần khỏi Folders và vòng tròn: nó vẫn ở đây, màu xám, và không bị khóa.

# Sentence under the title of the main card, saying what the picture of the drives shows.
disks-map-caption = Mỗi phần của mỗi ổ đĩa, được tô đầy theo mức đã dùng.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = Ổ đĩa của bạn

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = Hiện không đọc được các ổ đĩa.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = Thử lại

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = Xem lại

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = Không tìm thấy ổ đĩa nào. Hãy cắm một ổ vào và nó sẽ hiện ở đây.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = Dịch vụ ổ đĩa

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = Bus hệ thống

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what } không khả dụng. Hãy cài bằng lệnh: sudo apt install { $package }

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = Đang xem các ổ đĩa…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name } đang hỏng. Hãy sao chép ngay mọi thứ bạn muốn giữ ra khỏi nó.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name } cần được xem xét: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] Các ổ đĩa và dung lượng trên chúng.
       *[other] Các ổ đĩa và dung lượng trên chúng, cùng { $plugged } ổ đĩa đang cắm vào.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] Gần đầy: chỉ còn trống { $free } trên máy tính này. Hãy sớm xóa hoặc dời bớt tệp.
       *[other] Gần đầy: chỉ còn trống { $free } trên máy tính này, và { $plugged } ổ đĩa đang cắm vào. Hãy sớm xóa hoặc dời bớt tệp.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] Sắp đầy: còn trống { $free } trên máy tính này.
       *[other] Sắp đầy: còn trống { $free } trên máy tính này, và { $plugged } ổ đĩa đang cắm vào.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] Còn nhiều chỗ: trống { $free } trên máy tính này.
       *[other] Còn nhiều chỗ: trống { $free } trên máy tính này, và { $plugged } ổ đĩa đang cắm vào.
    }

# Title of the card with the ring for the disk the system is on.
disks-ring-system = Máy tính này

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = Tệp của bạn

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] Còn trống { $free }, nơi hệ thống nằm.
       *[files] Còn trống { $free }, cho tệp của riêng bạn.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: đầy { $full }, trống { $free } trên { $whole }

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = trên { $whole }

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = Chưa có số đo, các ổ đĩa không cho biết tình trạng

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = Khỏe mạnh, theo chính ổ đĩa báo

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
       *[other] Khỏe mạnh, theo cả { $n } ổ đĩa báo
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
       *[other] { $n } ổ cần được xem xét, xem thẻ của chúng
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
       *[other] { $n } ổ đĩa, tổng cộng { $total }. { $health }.
    }

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = trống { $size }

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = Chưa gắn

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = Vùng trống

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = Không có đĩa hay thẻ trong đó

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = Chưa có gì trên đó

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB hoặc tháo rời được

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, bên trong máy tính này

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB hoặc tháo rời được, đã ẩn

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, bên trong máy tính này, đã ẩn

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, đầy { $full }, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

# Name given to a part that has no label and no device name.
disks-part-name = Phần

# Name given to a drive that gives no maker or model.
disks-drive-name = Ổ đĩa

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = Swap

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = Đã mã hóa, đã mở khóa

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = Đã mã hóa, đã khóa

# Kind of a part that holds other parts.
disks-kind-container = Chứa các phần khác

# Kind of a part that has no filesystem on it.
disks-kind-none = Không có hệ thống tệp

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, phần mà máy tính khởi động từ đó
        [legacy] { $kind }, phần mà máy tính đời cũ khởi động từ đó
       *[loader] { $kind }, phần riêng của trình khởi động
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = mọi máy tính

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = chỉ Linux

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = thiết bị cũ

# Trouble a drive reports about its spare space.
disks-trouble-spare = sắp hết dung lượng dự phòng

# Trouble a drive reports about its heat.
disks-trouble-temperature = quá nóng

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = đang bị mòn

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = đã tự chuyển sang chỉ đọc

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = bộ nhớ sao lưu của nó đã hỏng

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = một phần của nó đã tự chuyển sang chỉ đọc

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = đã mòn quá mức nhà sản xuất dự kiến

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
       *[other] { $n } vùng hỏng
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
       *[other] đã dùng { $hours } giờ
    }

# How a drive says it is when it is failing, on its card.
# SAFETY: needs a human review before it ships
disks-health-failing = Đang hỏng: hãy sao chép ngay mọi thứ bạn muốn giữ ra khỏi nó

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = Cần chú ý: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = Khỏe mạnh

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = Không có ổ đĩa nào trả lời

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = Vỏ ổ trả lời, nhưng ổ đĩa bên trong thì không. Hãy rút ra, chờ mười giây rồi cắm vào cổng xanh hoặc cổng USB-C bằng cáp riêng của nó; nếu vẫn vậy, ổ đĩa bên trong có thể đã lỏng hoặc hỏng.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = Tháo an toàn

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = Tắt nguồn

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
# SAFETY: needs a human review before it ships
disks-erase-whole = Xóa toàn bộ ổ đĩa

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] Hiện ổ đĩa này
       *[part] Hiện phần này
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
# SAFETY: needs a human review before it ships
disks-eye-hide =
    { $what ->
        [drive] Ẩn ổ đĩa này. { $lock }
       *[part] Ẩn phần này. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Đã ẩn khỏi Folders và vòng tròn. { $lock }

# Note on the card of the drive the system runs from.
# SAFETY: needs a human review before it ships
disks-system-note = Hệ thống chạy từ đây: không thể xóa gì ở đây.

# Note on the card of a drive that has no partition table.
disks-no-table = Không có bảng phân vùng: toàn bộ ổ đĩa còn trống.

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = chưa gắn

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = gắn khi khởi động trong { $dir }

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = xuất hiện trong { $dir }

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = trong { $dir }

# Tip of the round button that opens the folder a part is mounted in.
disks-open = Mở

# Tip of the round button that mounts a part.
disks-mount = Gắn

# Tip of the round button that unmounts a part.
disks-unmount = Bỏ gắn

# Tip of the round button on a part that opens the box of changes.
disks-change-it = Thay đổi

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = Tên

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = Tên (tùy chọn)

# Tip of the button that saves a part's new name.
disks-rename-tip = Đổi tên

# Note under the Name row of a part.
disks-name-note = Tên gọi ở đây và trên các máy tính khác.

# Placeholder of the typing box for a part's folder.
disks-folder-name = Tên thư mục

# Tip of the button that saves a part's folder.
disks-use-folder = Dùng thư mục này

# Label of the row where a part's folder is chosen.
disks-folder = Thư mục

# Note under the Folder row of a part.
disks-folder-note = Nơi tệp của nó xuất hiện. Để trống để máy tính tự chọn.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = Gắn khi khởi động

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = Sẵn sàng mỗi khi máy tính khởi động. Nếu nó không có mặt, máy tính vẫn khởi động bình thường.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = Kích thước tính bằng GB

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = Từ { $least } đến { $most }.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = Từ { $least } đến { $most }. Hãy gắn nó để xem nó nhỏ được đến đâu.

# Tip of the button that resizes a part.
disks-resize-tip = Đổi kích thước

# Label of the row where a part is resized.
disks-size = Kích thước

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-erases = Việc này sẽ xóa mọi thứ trên { $words }. Không thể hoàn tác.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
# SAFETY: needs a human review before it ships
disks-resize-warning = Nhấp lần nữa: { $name } trên { $drive } sẽ đổi từ { $from } thành { $to }

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = Tạo chỗ cho một phần mới

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = Chỗ tính bằng GB

# Tip of the button that makes room under a part that is in use.
disks-room-tip = Tạo chỗ

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
# SAFETY: needs a human review before it ships
disks-room-warning = Nhấp lần nữa: { $name } trên { $drive } sẽ mất { $size }, sau khi chụp ảnh nhanh

# Note under the row that makes room.
disks-room-note = Được thu nhỏ khi đang dùng, sau khi chụp ảnh nhanh, để lại vùng trống.

# Tip of the button that checks a part for faults.
disks-check-tip = Kiểm tra lỗi

# Tip of the button that repairs a part.
disks-repair-tip = Sửa chữa

# Label of the row where a part is checked and repaired.
disks-faults = Lỗi

# Note under the Faults row.
# SAFETY: needs a human review before it ships
disks-faults-note = Tìm lỗi trong cách tệp được lưu và sửa chúng. Nó được bỏ gắn trong lúc này chạy.

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-repair-warning = Nhấp lần nữa để sửa { $name }, trên { $drive }

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = Loại

# Label of the row where a part is formatted.
# SAFETY: needs a human review before it ships
disks-format = Định dạng

# Tip of the button that formats a part.
# SAFETY: needs a human review before it ships
disks-format-tip = Định dạng

# Note under the row that erases the whole drive.
# SAFETY: needs a human review before it ships
disks-format-whole = Bắt đầu lại từ đầu như một phần duy nhất, chiếm cả ổ đĩa.

# Note under the row that formats one part.
# SAFETY: needs a human review before it ships
disks-format-part = Chỉ xóa mọi thứ trên phần này.

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
# SAFETY: needs a human review before it ships
disks-format-warning = Nhấp lần nữa: mọi thứ trên { $words } sẽ bị xóa

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $name }, { $size }, trên { $drive }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-delete-name = Xóa { $name }, { $size }, trên { $drive }

# Label of the row where a part is deleted.
# SAFETY: needs a human review before it ships
disks-delete-label = Xóa phần này

# Note under the row that deletes a part.
# SAFETY: needs a human review before it ships
disks-delete-note = Dung lượng của nó trở thành vùng trống, sẵn sàng cho một phần mới hoặc để phần trước nó mở rộng ra.

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = Tạo phần mới ở đây

# Label of the row where a new part is made.
disks-new-part = Phần mới

# Note under the row where a new part is made.
disks-new-part-note = Được tạo trong vùng trống này và định dạng sẵn để dùng.

# Tip of the button that makes a new part.
disks-make-tip = Tạo

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
# SAFETY: needs a human review before it ships
disks-make-warning = Nhấp lần nữa để tạo phần { $size } trên { $drive }

# Title of the box for erasing a whole drive. { $name } is the drive.
# SAFETY: needs a human review before it ships
disks-sheet-erase = Xóa { $name }

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $size }, trên { $drive }

# Title of the box for making a new part in free space.
disks-sheet-new = Một phần mới

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = trống { $size }, trên { $drive }

# Tip of the round button that closes the box of changes.
disks-close = Đóng

# Words on a row while a part is being mounted.
disks-mounting = Đang gắn

# Words on a row when a part has been mounted.
disks-mounted = Đã gắn

# Words on a row while a part is being unmounted.
disks-unmounting = Đang bỏ gắn

# Words on a row when a part has been unmounted.
disks-unmounted = Đã bỏ gắn

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = Đang đóng

# Words on a drive's row when it is safe to remove.
disks-safe-remove = An toàn để tháo

# Words on a drive's row while it is being powered off.
disks-powering-off = Đang tắt nguồn

# Words on a drive's row when it has been powered off.
disks-off-safe = Đã tắt: an toàn để rút ra

# Words on a row while a part is being renamed.
disks-renaming = Đang đổi tên

# Words on a row when a part has been renamed.
disks-renamed = Đã đổi tên

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = Máy tính lại tự chọn thư mục của nó

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = Nó xuất hiện trong { $dir }

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = Gắn khi khởi động trong { $dir }

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = Không động đến khi khởi động

# Words on a row while a folder or start-up choice is being saved.
disks-saving = Đang lưu

# Words on a row while a part is being resized.
disks-resizing = Đang đổi kích thước

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = Đã đổi kích thước thành { $size }

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Không tìm thấy tệp, nên không thể tạo chỗ.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = Đang chụp ảnh nhanh

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Sau đó không tìm thấy ảnh nhanh.

# Red words when the snapshot before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } Không có gì được đổi kích thước.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Mọi thứ đã thay đổi trong lúc chụp ảnh nhanh, nên không có gì được đổi kích thước.

# Words on a row while room is being made.
disks-making-room = Đang tạo chỗ

# Words on a row when room has been made. { $size } is how much.
disks-room-made = Đã tạo { $size } chỗ

# Words on a row while a part is being repaired.
disks-repairing = Đang sửa chữa

# Words on a row when a part has been repaired.
disks-repaired = Đã sửa chữa

# Red words when some faults could not be put right.
# SAFETY: needs a human review before it ships
disks-repair-failed = Một số lỗi không thể sửa được. Hãy sao chép ra mọi thứ bạn muốn giữ.

# Words on a row while a part is being checked.
disks-checking = Đang kiểm tra

# Words on a row when the check found no faults.
disks-no-faults = Không thấy lỗi

# Red words when the check found faults.
disks-faults-found = Đã thấy lỗi: nhấn biểu tượng cờ lê để sửa.

# Words on a row while a part or drive is being formatted.
# SAFETY: needs a human review before it ships
disks-formatting = Đang định dạng

# Words on a row when a part has been formatted.
# SAFETY: needs a human review before it ships
disks-formatted = Đã định dạng

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
# SAFETY: needs a human review before it ships
disks-erasing-whole = xóa toàn bộ ổ đĩa

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = Không tìm thấy phần mới.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = Không thể kết nối tới dịch vụ ổ đĩa.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Không đọc được mã nhận dạng của phần mới.

# Words on a row while a part is being deleted.
# SAFETY: needs a human review before it ships
disks-deleting = Đang xóa

# Words on a row when a part has been deleted.
# SAFETY: needs a human review before it ships
disks-deleted = Đã xóa

# Words on a row while a new part is being made.
disks-making-it = Đang tạo

# Words on a row when a new part has been made.
disks-made = Đã tạo

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = Không thể lưu thay đổi đó: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

# Red words under the folder name box when it is empty.
disks-name-empty = Hãy đặt tên trước.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
       *[other] Tên đó quá dài: tối đa { $max } ký tự.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = Chỉ dùng chữ cái, chữ số, - và _ (không có dấu cách).

# Red words under the folder name box when it starts with a dash.
disks-name-start = Hãy bắt đầu bằng một chữ cái hoặc chữ số.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir } đã được dùng.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
       *[other] Tên đó quá dài cho loại này: tối đa { $max } ký tự.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = Tên FAT không được chứa * ? . , ; : / \ | + = < > [ ] hoặc ".

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] Hãy gắn nó trước: loại này chỉ có thể được làm lớn hơn khi đang gắn.
       *[smaller] Hãy gắn nó trước: loại này chỉ có thể được làm nhỏ hơn khi đang gắn.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] Loại này không thể được làm lớn hơn.
       *[smaller] Loại này không thể được làm nhỏ hơn.
    }

# Red words when the password box was closed with no password. Said whole, as a sentence.
# SAFETY: needs a human review before it ships
disks-no-password = Chưa nhập mật khẩu, nên không có gì thay đổi.

# Red words when the password box was closed with no password, after some steps of a change were already done.
# SAFETY: needs a human review before it ships
disks-no-password-part-way = Chưa nhập mật khẩu, nên công việc đã dừng giữa chừng.

# Red words when another change is pressed while one is still being made.
disks-change-going = Một thay đổi đang được thực hiện.

disks-one-at-a-time = Một thay đổi khác vẫn đang được thực hiện. Hãy thử lại khi nó xong.

# Red words when it could not be made sure that the password would be asked for.
# SAFETY: needs a human review before it ships
disks-not-sure = Không thể bảo đảm mật khẩu sẽ được hỏi, nên không có gì thay đổi.

# Red words after how a change ended, when the password just given could not be taken back.
# SAFETY: needs a human review before it ships
disks-standing = Không thể thu hồi mật khẩu vừa nhập, nên trong vài phút một thay đổi khác với ổ đĩa có thể không hỏi mật khẩu. Đang thử lại.

# Words after how a change ended, when the password just given has been taken back after all.
# SAFETY: needs a human review before it ships
disks-taken-back = Mật khẩu đã nhập nay đã được thu hồi: thay đổi tiếp theo sẽ hỏi mật khẩu.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Không thể kết nối tới dịch vụ ổ đĩa, nên không có gì thay đổi.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = Ổ đĩa đã bị đóng trong lúc thay đổi này đang được thực hiện.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = Ổ đĩa vẫn mở cho đến khi việc này xong, để có thể báo nó kết thúc ra sao.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason }, nên không có gì thay đổi.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason }, nên công việc đã dừng giữa chừng.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = Không thể hỏi dịch vụ ổ đĩa

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } Công việc đã dừng giữa chừng.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = Đã xong: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = Thất bại: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = Chưa làm: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = Không rõ: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev } đã được bỏ gắn cho việc này và hiện chưa được gắn.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev } đã được bỏ gắn cho việc này và vẫn chưa gắn: hãy gắn nó từ hàng của nó.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev } có thể đã bị bỏ gắn: hãy xem hàng của nó.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = { $dev } đã được bỏ gắn cho việc này, và gắn lại nó cần một mật khẩu riêng: hãy gắn nó từ hàng của nó.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev } đã được bỏ gắn cho việc này và không thể gắn lại ({ $why }): hãy gắn nó từ hàng của nó.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = Đã định dạng, nhưng nó chưa được trả lại thư mục { $dir }. { $why } Hãy chọn lại thư mục từ hàng của nó.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = Có thứ gì đó vẫn đang dùng nó. Hãy đóng mọi cửa sổ hoặc chương trình đang hiện tệp của nó, rồi thử lại.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = Không đủ chỗ cho những gì đang có trên nó ở kích thước đó.

# Step of a change. { $dev } is the part.
disks-step-unmount = bỏ gắn { $dev }

# Step of a change. { $dev } is the part.
disks-step-mount = gắn lại { $dev }

# Step of a change.
disks-step-eject = đóng ổ đĩa để tháo ra

# Step of a change.
disks-step-power-off = tắt nguồn ổ đĩa

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-format = định dạng { $dev }

# Step of a change.
disks-step-create = tạo phần mới

# Step of a change. { $dev } is the part.
# SAFETY: needs a human review before it ships
disks-step-delete = xóa { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize-part = đổi kích thước phần { $dev }

# Step of a change. { $dev } is the part.
disks-step-resize = đổi kích thước nội dung trên { $dev }

# Step of a change. { $dev } is the part.
disks-step-repair = sửa chữa { $dev }

# Step of a change. { $dev } is the part.
disks-step-check = kiểm tra { $dev }

# Step of a change. { $dev } is the part.
disks-step-rename = đổi tên { $dev }

# Step of a change. { $dev } is the part.
disks-step-fstab = lưu lựa chọn thư mục và khởi động của { $dev }

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = bước { $method } trên { $dev }

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = Phần { $kind } này chỉ có thể được thu nhỏ khi không dùng, nên không thể tạo chỗ từ nó ở đây.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = Phần xfs này không thể được thu nhỏ.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = Phần { $kind } này không thể được thu nhỏ khi đang dùng.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Chỉ có thể chụp ảnh nhanh của chính ổ đĩa hệ thống, nên không thể tạo chỗ ở đây.

# Red words under the Make room row.
disks-room-not-in-table = Nó không phải một phần trong bảng của ổ đĩa, nên không thể tạo chỗ từ nó.

# Red words under the Make room row.
disks-room-no-online-shrink = Dịch vụ ổ đĩa không thể thu nhỏ Btrfs khi nó đang dùng ở đây, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-holds-parts = Nó chứa các phần khác, nên không thể tạo chỗ từ nó.

# Red words under the Make room row.
disks-room-not-gpt = Ổ đĩa của nó không có bảng GPT, nên không thể tạo chỗ từ nó.

# Red words under the Make room row.
disks-room-mount-first = Hãy gắn nó trước: Btrfs được thu nhỏ khi đang gắn.

# Red words under the Make room row.
disks-room-mounts-not-one = Nó không được gắn đúng một nơi, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-read-only = Nó được gắn ở chế độ chỉ đọc, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-read-only-unknown = Không xác định được nó có chỉ đọc hay không, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-many-drives = Các tệp trải trên nhiều hơn một ổ đĩa, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-drives-unknown = Không xác định được các tệp trải trên bao nhiêu ổ đĩa, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-swap-file = Có một tệp swap nằm trên nó, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-swap-unknown = Không xác định được có tệp swap nằm trên nó hay không, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-used-unknown = Không đọc được mức đang dùng của nó, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-area-unknown = Dịch vụ ổ đĩa không cho biết vùng chứa tệp lớn bao nhiêu, nên không thể kiểm tra và không thể tạo chỗ.

# Red words under the Make room row.
disks-room-open-missing = Không tìm thấy bản đang mở của nó, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-encryption-sizes = Các kích thước được cho cho phần mã hóa không khớp nhau, nên không thể tạo chỗ.

# Red words under the Make room row.
disks-room-sizes = Các kích thước được cho cho nó không khớp nhau, nên không thể tạo chỗ.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = Nó chứa { $used } và cần giữ nhiều hơn mức đó, nên có thể giải phóng quá ít.

# Red words under the Make room row.
disks-room-sizes-unread = Không đọc được các kích thước của nó.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = Có thể tạo chỗ từ { $least } đến { $most }.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what }, nên công việc đã bị dừng.

# What was found wrong while making room.
disks-room-check-unmounted = Các tệp không còn được gắn

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = Phần này là { $is } và lẽ ra phải là { $should }

# What was found wrong while making room.
disks-room-check-open = Bản đang mở của phần mã hóa không đúng kích thước cần có

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = Vùng chứa tệp là { $is } và lẽ ra phải là { $should }

# What was found wrong while making room.
disks-room-check-area-small = Vùng chứa tệp nhỏ hơn mức đang dùng

# What was found wrong while making room.
disks-room-check-fit = Phần chứa không còn vừa với nội dung

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [part] Dịch vụ ổ đĩa không còn liệt kê phần này, nên công việc đã bị dừng.
        [copy] Dịch vụ ổ đĩa không còn liệt kê bản đang mở, nên công việc đã bị dừng.
       *[files] Dịch vụ ổ đĩa không còn liệt kê hệ thống tệp, nên công việc đã bị dừng.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = thu nhỏ vùng chứa tệp

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = thu nhỏ bản đang mở của phần mã hóa

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = thu nhỏ phần

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = kiểm tra các kích thước

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = Trước khi tạo chỗ dưới { $dev } ({ $at })


# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = Nhấp lần nữa: { $name } trên { $drive } sẽ đi từ { $from } xuống { $to }. Hãy sao lưu tệp của nó trước, vì mất điện khi đang thu nhỏ có thể làm mất chúng.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = Windows trên { $name } đã được ngủ đông hoặc để ở chế độ khởi động nhanh, nên thu nhỏ phân vùng có thể làm mất tệp của nó. Hãy khởi động Windows, chọn Khởi động lại, rồi thử lại.

