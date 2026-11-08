
# machine translated (Vietnamese), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Tên đăng nhập này không hợp lệ.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Tên đăng nhập này quá dài.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = Không có người dùng nào tên { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } là tài khoản hệ thống và được để nguyên.

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = Chưa có mật khẩu nào được đưa ra.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
       *[other] Mật khẩu cần có ít nhất { $n } ký tự.
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = Mật khẩu không được chứa dấu xuống dòng.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } đã tồn tại.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = { $name } là quản trị viên duy nhất, nên vẫn giữ quyền này.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Bạn không thể xoá tài khoản của chính mình.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = { $name } đã bị xoá, nhưng ảnh của họ vẫn còn lưu cho màn hình đăng nhập: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Ảnh này quá lớn.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Đây không phải ảnh do Trindle tạo.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Không mở được thư mục ảnh cho màn hình đăng nhập: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Đây không phải tên một ngôn ngữ.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Không đọc được danh sách ngôn ngữ của Debian: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } không phải ngôn ngữ mà Debian biết.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Tên máy gồm chữ cái, chữ số và dấu gạch ngang, tối đa 63 ký tự, không bắt đầu hoặc kết thúc bằng dấu gạch ngang.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = Giới hạn đang bật lúc này, nhưng sẽ mất khi tắt máy: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = Hiện đang sạc đầy, nhưng giới hạn sẽ quay lại ở lần khởi động sau: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = Các máy khác vẫn có thể kết nối, vì không tắt được công tắc: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Đây không phải id của plugin.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Không biết người này.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Không biết thư mục của người này.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-plugins-list-not-kept = Không lưu được danh sách plugin được cho qua: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Không tìm thấy plugin đó.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Chưa cập nhật gì: không xác định được máy này đang dùng phiên bản Trindle nào, nên không thể loại bản cập nhật riêng của Trindle ra.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Đang cài các bản cập nhật

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Đang dọn dẹp

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Đang cài Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Đây không phải tên một gói.

# Red words when the package to remove is not installed.
admin-package-not-installed = Chưa làm: gói này chưa được cài.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Đây không phải thiết bị mà fwupd biết.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Không xác định được ai đã yêu cầu Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = Việc chặn trang không khởi động kịp.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = Việc chặn trang chưa tắt hoàn toàn: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Không đặt lại được { $path }: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Đây không phải tên một trang web.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Không có máy chủ như vậy.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } đã dừng trước khi nhận đủ dữ liệu, nên không thể coi là đã xong: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } đã thất bại.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = Không xoá được { $path }: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Chưa thay đổi gì, vì { $why }: { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Việc khôi phục lại như cũ cũng không thành: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Đây không phải khoảng thời gian có thể giữ các phiên bản cũ.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Không có thư mục chứa các bản chụp.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Có thứ không phải thư mục nằm ở chỗ lưu các bản chụp.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Không xác định được ai đã yêu cầu.

# Red words when a step that must go through the administrator's password was started another way.
# SAFETY: needs a human review before it ships
admin-pkexec-only = Việc này chỉ được chạy qua pkexec.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Đây không phải tên một máy tính.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Đây không phải tên một thư mục chia sẻ.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Đây không phải địa chỉ một máy tính.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Đây không phải đường dẫn của một thư mục chia sẻ.

# Red words in Folders when the shared folder to close is not one of the person's own.
# SAFETY: needs a human review before it ships
admin-share-not-yours = Ở đây chỉ đóng được các thư mục chia sẻ của chính bạn.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = Không có thư mục chia sẻ nào đang mở ở đó.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = { $path } không còn như Trindle để lại, nên không gắn gì vào đó.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } không còn như Trindle để lại.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Đang mở quá nhiều thư mục chia sẻ.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } chưa được cài. Nó nằm trong gói { $package } của Debian.

# Red words in Folders when the login for a share could not be read.
# SAFETY: needs a human review before it ships
admin-login-unreadable = Không đọc được thông tin đăng nhập.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Không đọc được những gì đã gắn trên máy này: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Vẫn còn một chương trình đang mở thứ gì đó trên đó. Hãy đóng nó trước.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Máy đó không trả lời. Thư mục sẽ được đóng khi bạn đăng xuất.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Đây không phải mã người dùng.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = Không tạo được nhóm sambashare.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Đang tải Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Đang thiết lập chia sẻ

# Red words in Settings when a sharing password is set before sharing is set up.
# SAFETY: needs a human review before it ships
admin-sharing-not-set-up = Chưa thiết lập chia sẻ. Hãy chia sẻ một thư mục trước.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = Máy này vẫn hiện với các máy khác trên mạng: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Đây không phải số của một bản chụp.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Chụp trong Cài đặt

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Không xác định được hệ thống nằm trên ổ đĩa nào.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = Hệ thống không nằm trong một subvolume Btrfs riêng, nên không thể đổi sang bản chụp.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Không tải được khoá ký từ { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = Địa chỉ đó trả về thứ không phải khoá ký.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Chưa thay đổi gì, vì không đọc được { $path }: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Đang tải khoá ký của nguồn

# Second line of the progress while a source is added.
admin-step-adding-source = Đang thêm nguồn và đọc danh sách của nó

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Không dùng được nguồn này. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Không dùng được nguồn này, nên nó đã được gỡ đi. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } chưa thể bật hay tắt nó: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = không khởi động được cho { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = Không thể cho { $user } dùng máy ảo, thứ mà Cowork cần: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = Không thể cho { $user } dùng nó mà không cần mật khẩu: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } đã được cài, nhưng việc thiết lập chưa xong: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Đang cài đặt

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Đây không phải một trong các nguồn làm sẵn.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } không được làm cho loại máy này ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Đang thiết lập

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Địa chỉ phải bắt đầu bằng https:// và không chứa gì khác.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = Suite và components phải là chữ thường.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Đây không phải tên một gói.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Đang gỡ bỏ

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Đang đọc lại các danh sách

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = Ở đây chỉ gỡ được các nguồn do Trindle thêm.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = Nguồn vẫn còn đó: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = Bản chụp { $n } không có trên ổ đĩa.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Chưa thay đổi gì: không chuẩn bị được bản chụp. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Không cất hệ thống hiện tại sang một bên được: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = hệ thống hiện tại giờ đang mang tên { $aside } thay vì { $sub }, và máy sẽ không khởi động được cho đến khi nó được đặt lại tên { $sub }: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Không chuyển được các bản chụp sang: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Không đặt được bản chụp vào chỗ: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Không cập nhật được danh sách những gì đã cất sang một bên: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = Không xoá được: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Không còn gì cần đến các gói này nữa, nên chúng đã được gỡ: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Không phải bản cập nhật nào cũng được cài. Còn giữ lại: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } đang tắt: trình điều khiển của nó ({ $driver }, DKMS) chưa biên dịch được trên nhân mới.

admin-driver-off = Một trình điều khiển ({ $driver }, DKMS) chưa biên dịch được trên nhân mới, nên phần nó đảm nhận đang tắt.

# Said once after the drivers that are off.
admin-driver-off-rest = Nhân và phần còn lại của bản cập nhật đã được cài.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = camera ảo

admin-language-in-use = { $code } là ngôn ngữ máy tính này khởi động, nên nó được giữ lại.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Máy tính này không có tính năng làm chậm split-lock để tắt.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Không tìm thấy ổ đĩa đó, hoặc nó chứa hệ thống.

# The drive could not be closed where it was open.
admin-games-drive-busy = Ổ đĩa đang được dùng. Hãy đóng những gì đang mở trên đó, rồi thử lại: { $why }

# Another software change has the read-only system open.
admin-system-busy = Một thay đổi phần mềm khác đang diễn ra. Hãy thử lại khi nó kết thúc.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Chưa có gì được thay đổi: không thể mở khoá hệ thống: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Không thể khoá lại hệ thống. Nó sẽ được khoá ở lần khởi động tới: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Không thể giữ /usr/local ở trạng thái có thể ghi: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Chỉ root mới chạy được lệnh này, và không qua pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Cần Btrfs.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = Bản chụp hệ thống đã vào đúng chỗ và máy tính sẽ khởi động từ đó. Việc dọn dẹp hệ thống cũ chưa xong và sẽ được thử lại ở lần khởi động sau: { $why }

