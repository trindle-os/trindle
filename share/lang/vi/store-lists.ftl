
# machine translated (Vietnamese), not yet reviewed by a person.

# On an installed thing's line: it came from a Flatpak file.
store-from-flatpak-file = Một tệp Flatpak

# On an installed thing's line: it came from a .deb file.
store-from-deb = Một tệp .deb

# On an installed thing's line: it is a plugin.
store-from-plugin = Plugin

# A pre-release version as people read it: the main number, the kind and its number.
store-version-pre = { $main } { $kind } { $number }

# The kind of a pre-release version written rc.
store-version-rc = bản ứng viên phát hành

# Tip of the round button on an app's tile while it is being removed.
# SAFETY: needs a human review before it ships
store-list-removing = Đang gỡ { $name }

# Tip of the round button on an app's tile while it is being installed.
store-list-installing = Đang cài { $name }

# Tip of the round button on an app's tile that opens the app.
store-list-open = Mở { $name }

# Tip of the round button on an app's tile that installs the app.
store-list-install = Cài { $name }

# Line on an installed app's tile when the app has no line of its own.
store-list-installed = Đã cài

# One tab of a strip: its name, then how many things are under it.
store-tab-count = { $name } { $n }

# Spoken name of a strip of tabs on the Installed page.
store-tabs-show = Hiển thị

# How long is left when it is a minute or less.
store-left-minute = còn khoảng một phút

# How long is left, in minutes.
store-left-minutes = còn khoảng { $n } phút

# How long is left, in hours.
store-left-hours =
    { $n ->
       *[other] còn khoảng { $n } giờ
    }

# Credit under a package's page for the list its site was checked against.
store-origin-credit = Đã đối chiếu với danh sách Threat Intelligence của HaGeZi

# Where a download came from: the site and the page that led to it.
store-origin-both = Tải từ { $site }, truy cập từ { $referrer }.

# Where a download came from: the site.
store-origin-site = Tải từ { $site }.

# Where a download came from: the page that led to it.
store-origin-referrer = Truy cập từ { $referrer }.

# Said when nothing tells where a download came from.
store-origin-unknown = Trindle không biết thứ này đến từ đâu.

# Warning when the site is on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-listed = { $from } { $name } nằm trong danh sách Threat Intelligence của HaGeZi về các trang có tệp tải độc hại và lừa đảo. Chỉ cài đặt nếu bạn chắc chắn về nguồn gốc của nó.

# Said when the site is not on the list; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-clear = { $from } Trang này không nằm trong danh sách Threat Intelligence của HaGeZi, nhưng danh sách này chưa đầy đủ, nên điều đó không chứng minh là an toàn.

# Said when the list could not be fetched; the first part says where the download came from.
# SAFETY: needs a human review before it ships
store-origin-not-checked = { $from } Không thể kiểm tra trang này vì không tải được danh sách Threat Intelligence của HaGeZi.

# Name shown for a review whose writer gave none.
store-review-someone = Một người nào đó

# Red words when the keyring could not be reached.
store-key-unreached = Không thể truy cập chùm khóa: { $why }.

# Red words when the key kept in the keyring is not text.
store-key-not-text = Khóa được lưu không phải là văn bản.

# Red words when a file that held the key could not be removed.
store-key-not-removed = Không thể gỡ { $path }: { $why }

# Red words when a file that held the key could not be read.
store-key-not-read = Không thể đọc { $path }: { $why }

