
# machine translated (Vietnamese), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Mở cài đặt của nó

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Chưa xong: { $why }

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = Bản cập nhật này cần một quyền mới. Nó đang chờ bạn.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Hiện là phiên bản { $mine }, bản { $new } đã sẵn sàng

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = Hiện là phiên bản { $mine }, bản { $new } đã sẵn sàng. Nó yêu cầu nhiều hơn trước: { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = Các nút ở đây sẽ trở lại khi nó làm xong.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Đang tìm các plugin…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = Chưa cài plugin nào, và cũng chưa có plugin nào để cài.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [1] Chưa cài plugin nào; có 1 plugin sẵn sàng để cài.
       *[other] Chưa cài plugin nào; có { $n } plugin sẵn sàng để cài.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [1] đã cài 1 plugin
       *[other] đã cài { $n } plugin
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [1] 1 plugin có bản cập nhật
       *[other] { $n } plugin có bản cập nhật
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [1] 1 bản cập nhật đang bị giữ lại
       *[other] { $n } bản cập nhật đang bị giữ lại
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, tất cả đều mới nhất.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, và { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, và { $have }; { $held }.

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = Nó không yêu cầu gì.

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = Nó yêu cầu: { $list }.

# Title of the Plugins page.
store-plugins-title = Plugin

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Đã cài phiên bản { $mine }; bản { $new } đã sẵn sàng.

# On a plugin's card: its installed version.
store-plugins-version-installed = Phiên bản { $version }, đã cài.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Phiên bản { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
# SAFETY: needs a human review before it ships
store-plugins-install = Cài đặt { $name }

# Tip of the round button that releases a plugin whose updates are held back.
# SAFETY: needs a human review before it ships
store-plugins-release = Bỏ giữ { $name }

# Tip of the round button that updates a plugin to a version.
# SAFETY: needs a human review before it ships
store-plugins-update-to = Cập nhật lên { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = Plugin đến từ đâu

# Information on the card that says where plugins come from.
store-plugins-come-from = Plugin chỉ đến từ danh sách riêng của Trindle, được đọc từ trindle.org mỗi khi trang này mở. Để đề xuất một plugin, hãy gửi nó: mỗi plugin được kiểm tra trước khi chính thức xuất hiện.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Đã cài, nhưng phần chạy nền của nó không hoạt động: { $why }

# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = Không thể gỡ bỏ: { $why }
