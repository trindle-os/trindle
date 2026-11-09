
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Ảnh chụp hệ thống

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = ảnh chụp hệ thống khôi phục hoàn tác sao lưu cập nhật hỏng phiên bản cũ tệp đã xoá snapshots snapshot restore undo rollback backup update broke btrfs snapper boot menu grub earlier versions older version previous deleted file put back recover home copies

# Help line at the foot of the Snapshots page.
settings-snap-help = Hệ thống như trước và sau mỗi lần thay đổi phần mềm, và bất cứ khi nào bạn chụp một ảnh. Khôi phục một ảnh sẽ đưa toàn bộ hệ thống về trạng thái đó từ lần khởi động sau; tệp của riêng bạn được giữ nguyên khi thư mục nhà có một phần đĩa riêng, như trình cài đặt của Trindle tạo ra. Nếu máy tính không khởi động được, hãy chọn Trindle snapshots trong menu khởi động. Phiên bản cũ giữ bản sao tệp của mọi người mỗi ngày, tách biệt với ảnh chụp hệ thống.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Chưa tạo bản sao: đĩa gần đầy.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = hôm nay lúc { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = hôm qua lúc { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = vào { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = Máy tính không tự khởi động lại được. Hãy khởi động lại từ vòng tròn Trindle.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Đang chụp ảnh hệ thống

# Said beside the set aside line while the space is freed.
settings-snap-freeing = Đang giải phóng...

# Said beside the set aside line once the space is freed.
settings-snap-freed = Đã giải phóng

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = Đang khôi phục

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = Hệ thống sẽ là ảnh chụp { $n } từ lần khởi động sau. Mọi thứ đã đổi từ lúc đó được để riêng ra, không bị mất.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = Đang xoá

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = Đã xoá

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = Đang chờ mật khẩu...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Mỗi ngày tạo một bản sao

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Không tạo thêm nữa

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Đã lưu

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = Đang xoá sạch

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = Đã xoá sạch

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Chưa có bản sao nào
       *[other] { $n } bản sao
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = Bản đầu tiên sẽ được tạo trong vòng một ngày.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = Chúng chiếm { $size }, đo lúc { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Dung lượng của chúng được đo mỗi ngày một lần.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = Xoá sạch tất cả ngay

# Heading of the card about earlier versions of files.
settings-snap-earlier = Phiên bản cũ

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle có thể giữ bản sao tệp của mọi người như lúc từng ngày, để mở hoặc khôi phục từ Thư mục một phiên bản cũ của tệp, hoặc tệp lỡ xoá. Mỗi người chỉ đọc được trong bản sao những gì họ đọc được vào ngày đó, nên thư mục nhà riêng tư vẫn riêng tư; tệp từng mở cho người khác thì vẫn mở cho họ trong bản sao đó cho đến khi nó biến mất. Các bản sao tách biệt với ảnh chụp hệ thống: khôi phục hệ thống không bao giờ đổi tệp của bạn, và các bản sao này không bao giờ đổi hệ thống. Cái giá là dung lượng: tệp bạn xoá hoặc sửa vẫn chiếm chỗ cho đến khi bản sao cuối cùng giữ nó biến mất, nên xoá tệp không giải phóng đĩa ngay. Xoá sạch các bản sao sẽ giải phóng ngay, cho mọi người trên máy tính này, và không thể hoàn tác. Sẽ không tạo bản sao khi đĩa gần đầy. Đổi bất kỳ điều nào ở đây cần mật khẩu quản trị viên.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Giữ các phiên bản cũ của tệp của bạn

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Tệp đã xoá được giữ { $time } trước khi giải phóng chỗ của chúng.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Giữ bản sao mỗi ngày trong

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Thời gian giữ bản sao mỗi ngày

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Đang xem các ảnh chụp hệ thống…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Không thể đọc các ảnh chụp hệ thống.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Đã khởi động từ ảnh chụp { $n }, nên mọi thứ làm bây giờ sẽ không được giữ lại.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
       *[other] Có { $n } ảnh chụp để quay lại, mới nhất từ { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Chưa có ảnh chụp nào. Một ảnh được chụp trước mỗi lần thay đổi phần mềm.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, giữ cho đến khi xoá

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = Khôi phục hệ thống về ảnh chụp này

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = Xoá ảnh chụp này

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Khởi động lại ngay

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Chúng chưa được thiết lập.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = Hãy cài lại gói trindle.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Ảnh chụp hệ thống chưa được thiết lập. Cài lại gói trindle sẽ thiết lập chúng: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = loại khác

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Chúng cần Btrfs; máy tính này dùng { $fs }.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Hãy chọn Btrfs khi cài Debian.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Ảnh chụp hệ thống cần hệ thống nằm trên Btrfs, mà máy tính này dùng { $fs }. Hãy chọn Btrfs cho hệ thống khi cài Debian để có chúng.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Máy tính này không có ảnh chụp hệ thống nào.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Không có ảnh chụp hệ thống

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Máy tính này không có ảnh chụp hệ thống nào

# Heading of the line saying how to have snapshots.
settings-snap-to-have = Để có chúng

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Đang xem…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Không thể đọc các ảnh chụp hệ thống

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Chưa có

# Under the None yet heading.
settings-snap-none-line-sub = Một ảnh được chụp trước mỗi lần thay đổi phần mềm.

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = Mỗi ảnh chụp có thể khôi phục toàn bộ hệ thống về như lúc đó, từ lần khởi động sau, hoặc bị xoá. Cả hai đều phải bấm hai lần và cần mật khẩu quản trị viên. Sau khi khôi phục, một nút khởi động lại hiện ra bên cạnh.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Chụp ảnh hệ thống ngay

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Không làm được khi đã khởi động từ một ảnh chụp.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Giữ cho đến khi bạn xoá.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
       *[other] { $n } hệ thống được để riêng ra do khôi phục
    }

# Under the set aside heading.
settings-snap-free-space = Hãy giải phóng chỗ khi mọi thứ đã ổn.

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = Giải phóng chỗ chúng đang chiếm

# Heading of the card for taking a snapshot.
settings-snap-take-one = Chụp một ảnh

# Help text of the Take one card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-take-about = Hãy chụp một ảnh trước khi thử điều bạn có thể muốn hoàn tác; ảnh bạn chụp được giữ cho đến khi bạn xoá. Khi máy tính đang khởi động từ một ảnh chụp, không gì được chụp sẽ được giữ, nên hãy khởi động lại bình thường trước. Mỗi lần khôi phục để riêng hệ thống ngay trước đó, nên không gì bị mất; hãy giải phóng chỗ chúng chiếm khi mọi thứ đã ổn.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Đã khởi động từ ảnh chụp { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Mọi thứ làm bây giờ sẽ không được giữ lại.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Tệp của bạn cũng nằm trong các ảnh chụp

# Under the Your files are in the snapshots too heading.
# SAFETY: needs a human review before it ships
settings-snap-files-back = Khôi phục cũng đưa tệp của bạn về như cũ.

# Heading of the card about what to know before restoring.
settings-snap-before = Trước khi khôi phục

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = Khi đã khởi động từ một ảnh chụp, mọi thứ làm bây giờ sẽ không được giữ lại: nếu hệ thống nên như thế này, hãy bấm khôi phục trên nó trong danh sách, rồi khởi động lại. Trên máy tính này, thư mục nhà có thể là một phần của hệ thống, và khi đó khôi phục một ảnh chụp cũng đưa tệp của riêng bạn về như lúc đó, nên hãy chép mọi thứ mới sang nơi an toàn trước.


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Tệp hệ thống đã khoá

# Under the System files locked switch.
settings-snap-locked-sub = Chỉ việc cài đặt và cập nhật mới mở khoá chúng.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Đã khoá

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Đã mở khoá

