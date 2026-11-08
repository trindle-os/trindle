# machine translated (Vietnamese), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = Chưa khôi phục gì cả, vì { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Một phần bản sao lưu chưa được khôi phục, vì { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = bản sao lưu có một liên kết hoặc thứ khác trỏ ra ngoài thư mục mà nó được khôi phục vào

# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = trên máy này đã có thứ gì đó ở chỗ mà một phần bản sao lưu cần đặt vào

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = không có chỗ để giải nén bản sao lưu

# The reason when the backup file cannot be read.
store-because-unreadable = không đọc được tệp sao lưu

# The reason when the file is not a Trindle backup.
store-because-not-trindle = tệp đó không phải bản sao lưu của Trindle

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = bản sao lưu bị hỏng và không đọc được danh sách của nó

# The reason when the file is not a Flat backup.
store-because-not-flat = tệp đó có vẻ không phải bản sao lưu của Flat

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = tệp đó không chứa danh sách ứng dụng của Flat

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = bản này do một bản Flat mới hơn tạo ra mà Trindle chưa đọc được

# The reason when the file holds no apps.
store-because-no-apps = tệp đó không có ứng dụng nào

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = không đọc được bản sao lưu: { $why }

# The reason when the program gave none.
store-because-none = chương trình không nêu lý do

# The reason when the system said permission was denied.
store-because-permission = máy này không cho Trindle ghi vào đó

# The reason when the disk is full.
store-because-space = máy này không còn chỗ trống

# The reason when a part of the computer is read-only.
store-because-read-only = phần đó của máy này chỉ cho phép đọc

# The reason when the person's disk quota is used up.
store-because-quota = dung lượng cho phép của tài khoản này đã dùng hết

# The reason when a file or program is missing.
store-because-missing = thiếu một tệp hoặc chương trình mà nó cần

# The reason when no reason could be read from the program's last line.
store-because-failed = thất bại

# The reason when the backup has no name to be written under.
store-because-no-name = nó không có tên

# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = cài đặt của nó trong bản sao lưu trỏ ra ngoài thư mục riêng của nó

# The reason when something is too big to be read whole.
store-because-too-big = nó quá lớn để đọc trọn vẹn

# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = cài đặt của nó trong bản sao lưu không khớp với mã kiểm tra

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Đã khôi phục, trừ ảnh tài khoản của bạn. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Ảnh tài khoản của bạn cũng chưa được khôi phục. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = Không khởi động được pkexec: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Không đọc được từ bản sao lưu: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Chưa tạo được bản sao lưu, vì { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = chưa có gì

# Beside the size of a part of the backup: about how big it is.
store-backup-about = khoảng { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Đang khôi phục cài đặt và AppImage của Trindle...

# Step shown while the settings and data are put back.
store-backup-step-data = Đang khôi phục cài đặt và dữ liệu...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
       *[other] Đang cài { $count } ứng dụng từ Flathub... việc này có thể mất một lúc.
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Đã khôi phục cài đặt và dữ liệu, nhưng các ứng dụng này chưa được cài vì Flatpak từ chối: { $failed }.

# The same, when other apps were also left out for another reason.
store-not-installed-both = Đã khôi phục cài đặt và dữ liệu, nhưng các ứng dụng này chưa được cài vì Flatpak từ chối: { $failed }. Cũng chưa cài: { $refused }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Đã khôi phục cài đặt và dữ liệu, nhưng các ứng dụng này chưa được cài: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [1] Đã bỏ qua 1 ứng dụng: tên của nó không hợp lệ.
       *[other] Đã bỏ qua { $n } ứng dụng: tên của chúng không hợp lệ.
    }

# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = nguồn { $name } không được mã hóa và đã bị từ chối, nên không được dùng

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = nguồn { $name } chưa có trên máy này và bản sao lưu không cho biết nó ở đâu

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = cài đặt của { $name } (vì bản sao của chúng trong bản sao lưu không qua được bước kiểm tra)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (vì tên nguồn của nó không hợp lệ)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (vì { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (vì Flatpak từ chối: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = tất cả (vì không có chỗ để giải nén bản sao lưu)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = quyền đã cấp cho mọi ứng dụng

# Said among what could not be done: one app's settings.
store-flat-app-settings = cài đặt của { $name }

# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. Những gì có trước đó được giữ lại với tên { $path }: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = Đang cài { $name } ({ $at }/{ $total })...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Đang khôi phục cài đặt của { $name } ({ $at }/{ $total })...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
       *[other] Đã cài { $n } ứng dụng.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = Đã khôi phục cài đặt cho { $n }.

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
       *[other] { $n } đã có sẵn, giữ nguyên.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Chưa khôi phục: { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })

