
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of Debian's group of updates, on its row and in the headline.
store-upd-debian-name = Các gói của Debian

# On a row held back: the version it is held at, and the one that waits.
store-upd-held-version-waits = Đang giữ lại ở phiên bản { $have }; { $waits }

# On a row held back: the one that waits.
store-upd-held-waits = Đang giữ lại; { $waits }

# On a thing held back with no update waiting: the version it is held at.
store-upd-held-version = Đang giữ lại ở phiên bản { $have }

# On a thing held back with no update waiting.
store-upd-held = Đang giữ lại

# On the Held back card: the version a thing is held at, and nothing newer waits.
store-upd-held-version-nothing = Đang giữ lại ở phiên bản { $have }; chưa có bản mới hơn

# On the Held back card: nothing newer waits.
store-upd-held-nothing = Đang giữ lại; chưa có bản mới hơn

# How many updates there are, as part of a line.
store-upd-count =
    { $n ->
        [1] 1 bản cập nhật
       *[other] { $n } bản cập nhật
    }

# How many updates are ready, as the start of the headline.
store-upd-ready-count =
    { $n ->
        [1] 1 bản cập nhật đã sẵn sàng
       *[other] { $n } bản cập nhật đã sẵn sàng
    }

# The headline: how many updates are ready, then how big they are in all.
store-upd-in-all = { $count }, tổng cộng { $size }

# How many updates are held back, as part of a line.
store-upd-held-clause =
    { $n ->
        [1] 1 bản đang bị giữ lại
       *[other] { $n } bản đang bị giữ lại
    }

# A line with how many are held back added after a semicolon.
store-upd-with-held = { $text }; { $held }

# A line made into a sentence.
store-upd-sentence = { $text }.

# Two sentences one after the other.
store-upd-and-then = { $first } { $second }

# Added to the headline when firmware is among the updates.
store-upd-firmware-restart = Firmware cần khởi động lại để hoàn tất.

# Headline while the check runs.
store-upd-checking = Đang kiểm tra bản cập nhật.

# Headline while one update goes in on its own; its name is given.
store-upd-updating-one = Đang cập nhật { $name }. Bạn có thể tiếp tục làm việc trong lúc cài.

# Headline while all the updates go in.
store-upd-installing-all = Đang cài các bản cập nhật. Bạn có thể tiếp tục làm việc trong lúc cài.

# Headline when the updates are in and the computer must start again.
# SAFETY: needs a human review before it ships
store-upd-restart = Khởi động lại để hoàn tất: các bản cập nhật đã được cài, và sẽ có hiệu lực khi máy tính khởi động lại.

# Added to the headline when what went in needs a restart.
store-upd-gone-in-restart = Những gì vừa cài cần khởi động lại để hoàn tất.

# Added to the headline with no internet.
store-upd-no-more = Máy tính này chưa kết nối internet nên không thể tìm thêm.

# Headline when nothing is ready (updates held back are added after it).
store-upd-none-ready = Không có bản cập nhật nào sẵn sàng

# Headline with nothing waiting and no internet to look with.
store-upd-nothing-known = Theo máy tính này biết, không có bản cập nhật nào đang chờ. Máy không thể tìm thêm khi chưa kết nối internet.

# Name of the apps in the headline's list of what could not be checked.
store-upd-name-apps = các ứng dụng

# Name of Trindle and Debian in the headline's list of what could not be checked.
store-upd-name-system = Trindle và Debian

# Name of the firmware in the headline's list of what could not be checked.
store-upd-name-firmware = firmware

# Headline when some checks failed; the list of what was not checked is given.
store-upd-check-failed = Không thể kiểm tra bản cập nhật cho { $names }.

# Headline when everything is up to date.
store-upd-current = Mọi thứ đều đã mới nhất.

# Added to the headline when everything is up to date: when the last check was.
store-upd-last-checked = Kiểm tra lần cuối lúc { $time }.

# How a card stands with nothing known to wait and no internet.
store-upd-not-checked = Không thể kiểm tra khi không có internet

# Red words under a card when a check took too long.
store-upd-too-long = Phản hồi mất quá nhiều thời gian.

# Tip of the round button that checks again.
store-upd-check-again = Kiểm tra lại

# Main button while the check runs.
store-upd-checking-main = Đang kiểm tra

# Main button while the updates go in.
store-upd-installing-main = Đang cài đặt

# Main button that installs every update waiting.
# SAFETY: needs a human review before it ships
store-upd-install-n =
    { $n ->
        [1] Cài 1 bản cập nhật
       *[other] Cài { $n } bản cập nhật
    }

# Main button that restarts the computer.
# SAFETY: needs a human review before it ships
store-upd-restart-now = Khởi động lại ngay

# Main button that checks for updates.
store-upd-check-for = Kiểm tra bản cập nhật

# Tip of the round button that releases a thing held back.
# SAFETY: needs a human review before it ships
store-upd-release = Bỏ giữ { $name }

# Tip of the round button that holds a thing's updates back.
# SAFETY: needs a human review before it ships
store-upd-hold = Giữ lại { $name }

# A card's state while it is being checked.
store-upd-status-checking = Đang kiểm tra

# A card's state while its updates go in.
store-upd-status-installing = Đang cài đặt

# A card's state when its check failed; the reason follows.
store-upd-status-failed = Không thể kiểm tra. { $why }

# A card's state: how many updates, and how big.
store-upd-status-size = { $count }, { $size }

# A card's state: how many updates, with some held back.
store-upd-status-held = { $text }; { $held } đang giữ lại

# A card's state when only held back updates are left.
store-upd-held-back-n = { $n } đang giữ lại

# A card's state when nothing waits.
store-upd-up-to-date = Đã mới nhất

# On a row of the Updates page: where an app comes from and what it changes to.
store-upd-from-change = Từ { $from }, { $change }

# On a row of the Updates page: where an app comes from.
store-upd-from = Từ { $from }

# On a row: a part that apps share, and where it comes from.
store-upd-shared-part = Một phần dùng chung, từ { $from }

# Note under the Apps card.
# SAFETY: needs a human review before it ships
store-upd-yours-alone = Chỉ của riêng bạn, nên được cập nhật mà không cần mật khẩu. Ứng dụng mang theo các phần dùng chung mà phiên bản mới của nó cần.

# Note under the Apps card: what holding back does.
# SAFETY: needs a human review before it ships
store-upd-hold-why = Bản cập nhật bị giữ lại sẽ ở nguyên phiên bản hiện có, và không nằm trong Cài tất cả ở đây và trên vòng tròn, cho đến khi bạn bỏ giữ.

# Heading of the Apps card.
store-upd-card-apps = Ứng dụng

# Plugins card: every plugin is up to date.
store-upd-plugins-current = Mọi plugin đều đã mới nhất.

# Plugins card: how many plugins have updates held back.
store-upd-plugins-held =
    { $n ->
        [1] Bản cập nhật của 1 plugin đang bị giữ lại.
       *[other] Bản cập nhật của { $n } plugin đang bị giữ lại.
    }

# Plugins card: how many plugins have an update.
store-upd-plugins-have =
    { $n ->
        [1] 1 plugin có bản cập nhật
       *[other] { $n } plugin có bản cập nhật
    }

# Heading of the Plugins card.
store-upd-card-plugins = Plugin

# Held back card: nothing is held back.
store-upd-nothing-held = Không có gì bị giữ lại.

# On a row of the Held back card for a thing released while the page is open.
store-upd-released = Đã bỏ giữ

# Note under the Held back card: what releasing does.
# SAFETY: needs a human review before it ships
store-upd-release-why = Mục được bỏ giữ sẽ được cài cùng các mục khác trở lại, từ Cài tất cả ở đây và trên vòng tròn.

# Name on the Held back card of a device whose firmware is not named now.
store-upd-device-firmware = Firmware của một thiết bị

# Tip name of the round button for a device's firmware that is not named now.
store-upd-device-called = firmware của thiết bị { $id }

# Heading of the Held back card.
store-upd-card-held = Đang giữ lại

# On a row: Trindle's version now, and the one it goes to.
store-upd-version-to = { $from } lên { $to }

# On a row: Debian's updates, how many, which go in by themselves or together.
store-upd-debian-count =
    { $n ->
        [1] 1 bản cập nhật, được cài riêng
       *[other] { $n } bản cập nhật, được cài cùng nhau
    }

# On a row of Debian's updates: where it comes from.
store-upd-from-debian = Từ Debian

# Under Debian's rows: how many of them show.
store-upd-debian-showing = Đang hiện { $shown } trên { $total } bản cập nhật của Debian.

# Note on the Trindle and Debian card: which version of Trindle the computer has.
store-upd-has-trindle = Máy tính này đang dùng Trindle { $version }.

# Heading of the Trindle and Debian card.
store-upd-card-system = Trindle và Debian

# Note on the Trindle and Debian card: why Debian's packages have one button.
store-upd-debian-why = Các gói của Debian được làm để khớp với nhau, và cài riêng một gói có thể kéo theo các gói khác hoặc làm chúng lệch nhau. Vì vậy chúng được cài cùng nhau, từ một nút.

# Note on the Trindle and Debian card: why no row has a Hold back button.
# SAFETY: needs a human review before it ships
store-upd-not-held = Các bản cập nhật của Trindle và Debian mang theo bản vá bảo mật cho cả máy tính, nên không thể giữ lại.

# Heading of the card about the daily security updates.
# SAFETY: needs a human review before it ships
store-upd-card-security = Cập nhật bảo mật

# On the security card: its state.
# SAFETY: needs a human review before it ships
store-upd-security-on = Bật, mỗi ngày

# On the security card.
# SAFETY: needs a human review before it ships
store-upd-security-note = Các bản vá của Debian được cài mỗi ngày mà không hỏi.

# On the security card.
# SAFETY: needs a human review before it ships
store-upd-security-part = Là một phần của Trindle; không thể tắt.

# On the security card: that holding back never stops these.
# SAFETY: needs a human review before it ships
store-upd-never-held = Việc giữ lại một bản cập nhật không bao giờ chặn các bản này.

# Firmware card: the computer has no updater.
store-upd-fw-none = Máy tính này không có trình cập nhật firmware.

# Firmware card: restart to finish.
# SAFETY: needs a human review before it ships
store-upd-fw-restart = Khởi động lại để hoàn tất

# On a firmware row with no version installed: the version it goes to.
store-upd-fw-version = Phiên bản { $version }

# Note on the firmware card when the updater is there.
store-upd-fw-note = Firmware mới được cài khi máy tính khởi động lại.

# Note on the firmware card when the updater is missing.
store-upd-fw-install = Hãy cài fwupd để giữ firmware luôn mới nhất.

# Heading of the Firmware card.
store-upd-card-firmware = Firmware

# Red words when fwupd could not fetch its list.
store-upd-fw-no-list = fwupd không tải được danh sách mới nhất.

# Red words when Flathub could not be reached.
store-upd-no-flathub = Không kết nối được với Flathub.

# Red words when Debian's lists could not be fetched.
store-upd-no-lists = Không tải được danh sách của Debian.

# Red words when Debian's lists could not be fetched; the reason follows.
store-upd-no-lists-why = Không tải được danh sách của Debian. { $why }

# Red words under a row when its updates could not be held back or released: the list of what is held could not be kept.
store-upd-not-held-list = Chưa giữ lại được. Không thể lưu danh sách những gì đang giữ lại: { $why }.

# The same, for a release.
store-upd-not-released-list = Chưa bỏ giữ được. Không thể lưu danh sách những gì đang giữ lại: { $why }.

# Red words when Flatpak could not be told a thing is held back.
store-upd-held-not-told = Đã giữ lại trong Ứng dụng, nhưng không báo được cho chính Flatpak, nên công cụ khác cập nhật ứng dụng vẫn có thể cài nó.

# Red words when Flatpak could not be told a thing is released.
store-upd-released-not-told = Đã bỏ giữ trong Ứng dụng, nhưng không báo được cho chính Flatpak và nó vẫn có thể bỏ qua mục này. Hãy giữ lại rồi bỏ giữ lần nữa.

# Red words: the above, then what Flatpak itself said.
store-upd-flatpak-said = { $words } Flatpak báo: { $why }

# Name of Trindle itself on its row of updates.
store-upd-trindle = Trindle


# Headline of Updates with looking for updates by itself switched off in Settings and no check pressed yet.
store-upd-off = Việc tự kiểm tra cập nhật đang tắt. Nhấn Kiểm tra cập nhật để xem.

# How a card stands with nothing known to wait, looking by itself switched off and no check pressed yet.
store-upd-not-looked = Chưa kiểm tra

# Tip of the round button that ends an update that is running.
store-upd-stop = Dừng

# Tip of that button once it has been pressed, while the update ends.
store-upd-stopping = Đang dừng

# Under a row whose update was stopped.
store-upd-stopped = Đã dừng. Nhấn Cập nhật để tiếp tục.

# Under the Apps card when Install all was stopped.
store-upd-stopped-all = Đã dừng. Nhấn Cài đặt để tiếp tục.

# Note under the Apps card.
store-upd-steam-restarts = Steam sẽ khởi động lại khi không còn trò chơi nào đang chạy.

