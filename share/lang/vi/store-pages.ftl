
# machine translated (Vietnamese), not yet reviewed by a person.

# How many apps there are, as part of a line under a page title.
store-grid-apps-count =
    { $n ->
       *[other] { $count } ứng dụng
    }

# How many updates wait, as part of the line under the front page's title.
store-grid-updates-waiting =
    { $n ->
       *[other] { $count } bản cập nhật đang chờ
    }

# Line under the title of the front page; says how many apps there are and what waits.
store-grid-home-line = { $apps } để chọn trên Flathub; { $more }.

# Part of the front page's line when nothing waits to be updated.
store-grid-home-uptodate = mọi thứ của bạn đều đã mới nhất

# Line under the title of a page of apps; says how many there are.
store-grid-line = { $apps }.

# Line under the title of a search that found nothing.
store-grid-none-found = Không có ứng dụng nào khớp “{ $text }”.

# Line under the title of an empty page of apps.
store-grid-none-here = Chưa có ứng dụng nào ở đây.

# Advice in the empty page of a search.
store-grid-try-shorter = Hãy thử từ ngắn hơn, hoặc xem qua một loại ở bên trái.

# Advice in the empty page of a kind of apps.
store-grid-search-or-look = Tìm theo tên ở trên, hoặc xem qua một loại khác ở bên trái.

# Under a page of apps: how many are shown of how many there are.
store-grid-showing = Đang hiện { $shown } trên { $apps }.

# Link under a page of apps that shows more of them.
store-grid-show-more = Hiện thêm { $next }

# Under the title of the page that waits for Flathub's list of apps.
store-wait-fetching = Đang tải danh sách ứng dụng từ Flathub. Lần đầu mất khoảng một phút.

# Under the title of the waiting page when Flathub could not be reached though the computer is online.
store-wait-no-reach = Ứng dụng không kết nối được với Flathub.

# Under the title of the waiting page when the computer is not connected.
store-wait-offline = Máy tính này chưa kết nối internet. Ứng dụng sẽ tự thử lại ngay khi có kết nối.

# In the waiting page when the computer is not connected.
store-wait-comes-from = Các ứng dụng để chọn đến từ Flathub, qua internet.

# In the waiting page: which pages work meanwhile; the pages are listed first.
store-still-work-it = { $pages } và phần cài đặt cho ứng dụng, ở bên trái, vẫn dùng được khi không có nó.

# In the waiting page when Flathub did not answer: which pages work meanwhile.
store-still-work-flathub = { $pages } và phần cài đặt cho ứng dụng, ở bên trái, vẫn dùng được khi không có Flathub.

# In the waiting page when the computer is not connected: which pages work meanwhile.
store-still-work-internet = { $pages } và phần cài đặt cho ứng dụng, ở bên trái, vẫn dùng được khi không có internet.

# Main button of the waiting page that tries again to fetch Flathub's list.
store-wait-try-again = Thử lại

# Name of the way back from an app's page that was opened from the results of a search.
store-back-results = Kết quả

# Red words under the search box when no web browser opens a link.
store-no-browser = Không có trình duyệt web nào để mở { $url }. Hãy cài một trình duyệt trong Ứng dụng trước.

# Said in the card or under the title while the lists are being read.
store-looking = Đang tìm…

# Said under an app's row on the Install page when the app was put in.
store-done-installed-note = Giờ nó đã nằm trong mục Đã cài, và trong vòng tròn

# Said under an app's row when it was taken off, and the word on its dial.
store-done-removed = Đã gỡ

# The word on an app's dial when installing or removing did not work.
store-done-not = Chưa xong

# The word on an app's dial when it was installed.
store-done-installed = Đã cài

# Red words under an app when Flatpak gave no reason it would not work.
store-flatpak-refused = Flatpak từ chối.

# Tab of the Installed page that shows everything.
store-tab-all = Tất cả

# Tab of the Installed page that shows apps from Flatpak files.
store-tab-flatpak-files = Tệp Flatpak

# Tab of the Installed page that shows AppImages.
store-tab-appimages = AppImage

# Tab of the Installed page that shows programs from .deb files.
store-tab-deb-files = Tệp deb

# Tab of the Installed page that shows plugins.
store-tab-plugins = Plugin

# Line under the title of the Installed page when nothing is installed.
store-installed-nothing = Chưa cài gì cả. Mục Khám phá có rất nhiều thứ để thử.

# Line under the title of the Installed page when only plugins are installed.
store-installed-only-plugins =
    { $n ->
        [1] Đã cài 1 plugin
       *[other] Đã cài { $n } plugin
    }

# Start of the Installed page's line when programs are installed.
store-installed-programs =
    { $n ->
        [1] Đã cài 1 chương trình
       *[other] Đã cài { $n } chương trình
    }

# Start of the Installed page's line when plugins are installed with other things.
store-installed-things = Đã cài { $n } mục

# Installed line when every program is from Flathub.
store-installed-all-flathub =
    { $n ->
        [1] { $start }, từ Flathub
       *[other] { $start }, tất cả từ Flathub
    }

# Installed line when every program is from a Flatpak file.
store-installed-all-files =
    { $n ->
        [1] { $start }, từ một tệp Flatpak
       *[other] { $start }, tất cả từ các tệp Flatpak
    }

# Installed line when every program is an AppImage.
store-installed-all-appimages =
    { $n ->
        [1] { $start }, là một AppImage
       *[other] { $start }, tất cả là AppImage
    }

# Installed line when every program is from a .deb file.
store-installed-all-debs =
    { $n ->
        [1] { $start }, từ một tệp .deb
       *[other] { $start }, tất cả từ các tệp .deb
    }

# Installed line when programs come from several places; the places are listed after the comma.
store-installed-mixed-list = { $start }, { $parts }

# Installed line when plugins or .deb programs are among the apps; the kinds are listed after the colon.
store-installed-mixed-kinds = { $start }: { $parts }

# Part of the Installed line: how many apps are from Flathub.
store-installed-part-flathub = { $n } từ Flathub

# Part of the Installed line: how many apps are from Flatpak files.
store-installed-part-files =
    { $n ->
        [1] 1 từ một tệp Flatpak
       *[other] { $n } từ các tệp Flatpak
    }

# Part of the Installed line: how many AppImages there are.
store-installed-part-appimages =
    { $n ->
        [1] 1 AppImage
       *[other] { $n } AppImage
    }

# Part of the Installed line: how many apps there are.
store-installed-part-apps =
    { $n ->
        [1] 1 ứng dụng
       *[other] { $n } ứng dụng
    }

# Part of the Installed line: how many programs are from .deb files.
store-installed-part-debs =
    { $n ->
        [1] 1 từ một tệp .deb
       *[other] { $n } từ các tệp .deb
    }

# Part of the Installed line: how many plugins there are.
store-installed-part-plugins =
    { $n ->
        [1] 1 plugin
       *[other] { $n } plugin
    }

# The whole Installed line: what is installed, then how up to date it is.
store-installed-line = { $said }. { $state }

# End of the Installed line when the one thing installed is up to date.
store-installed-state-one = Nó đã mới nhất.

# End of the Installed line when everything is up to date.
store-installed-state-all = Tất cả đều đã mới nhất.

# End of the Installed line when some have updates waiting.
store-installed-state-waiting =
    { $n ->
        [1] 1 mục có bản cập nhật đang chờ.
       *[other] { $n } mục có bản cập nhật đang chờ.
    }

# Line under the Installed title when the list is narrowed by a search, for programs.
store-installed-match-programs = { $shown } trên { $total } chương trình khớp “{ $find }”.

# The same when plugins are among the things.
store-installed-match-things = { $shown } trên { $total } mục khớp “{ $find }”.

# Line under the Installed title while updates go in elsewhere: who is updating, then when the buttons come back.
store-installed-other-busy = { $who } { $back }

# Button at the top of the Installed page that adds an AppImage.
store-installed-add-appimage = Thêm AppImage

# Placeholder of the typing box on the Installed page.
store-installed-search = Tìm trong những gì đã cài

# Line under the Installed title when an AppImage could not be added.
store-installed-appimage-failed = Không thể thêm AppImage.

# In the card of the Installed page when nothing is installed.
store-installed-empty = Ứng dụng bạn cài từ Khám phá, từ GitHub hoặc dưới dạng AppImage, các chương trình từ tệp .deb và plugin sẽ hiện ở đây.

# In the card of the Installed page when the search matches nothing.
store-installed-no-match = Không có mục nào đã cài khớp “{ $find }”.

# Note at the foot of the Installed list.
# SAFETY: needs a human review before it ships
store-installed-remove-twice = Gỡ cài đặt hỏi hai lần: hãy bấm lại trong vòng ba giây.

# On an installed thing's line: its version.
store-installed-version = phiên bản { $version }

# On an installed thing's line: its updates are held back.
store-installed-held = đang giữ lại

# On an installed thing's line: its updates are held back and a newer one waits.
store-installed-held-waits = đang giữ lại; { $waits }

# Tip and spoken name of the button while an app is updating.
store-installed-updating = Đang cập nhật { $name }

# Tip and spoken name of the button that updates an app or plugin.
store-installed-update = Cập nhật { $name }

# Tip of the round button that shows an installed app's page.
store-installed-about = Giới thiệu

# Under an installed thing's row while it is being taken off.
# SAFETY: needs a human review before it ships
store-installed-removing = Đang gỡ...

# Red words in the Installed card when the session bus cannot be asked for a file.
store-add-no-bus = Không có session bus để hỏi chọn tệp nào.

# Title of the window that asks which AppImage to add.
store-add-choose-title = Chọn một AppImage

# Name of the filter in the window that asks which AppImage to add.
store-add-filter = AppImage

# Red words in the Installed card when the file chooser could not be asked.
store-add-no-chooser = Không thể mở trình chọn tệp.

# Words in the Installed card while an AppImage is being added.
store-add-adding = Đang thêm...

# Words under the search box when an AppImage let go on the window is already there.
store-add-already = { $name } đã có sẵn: xem mục Đã cài.

# Words once an AppImage is in the ring.
store-add-in-ring = Đã thêm. Giờ nó đã có trong danh sách của bạn.

# Red words under the search box when a dropped file is not a Flatpak reference this computer can use.
store-add-bad-ref = Đây không phải tham chiếu Flatpak mà máy tính này dùng được.

# Words under the search box while a Flatpak reference is installed; the source is given.
store-add-installing-ref = Đang cài { $name } từ { $host }...

# Words under the search box once a Flatpak reference is installed.
store-add-installed-ref = Đã cài { $name }.

# Red words when Flatpak gave no reason it would not install a reference.
store-add-ref-refused = Flatpak không chịu cài.

# Red words under the search box when a Flatpak reference could not be installed; the reason follows.
store-add-ref-failed = Không thể cài { $name }. { $why }

# Words in the Installed card while an AppImage replaces another.
# SAFETY: needs a human review before it ships
store-add-replacing = Đang thay thế...

# Heading of the question when an AppImage has the name of one already there.
# SAFETY: needs a human review before it ships
store-replace-head = { $name } đã có sẵn

# Small heading over the AppImage being brought.
store-replace-new = Bản đang được đưa vào

# Small heading over the AppImage already there.
store-replace-old = Bản đã có sẵn

# Tip of the round button that keeps the AppImage already there.
# SAFETY: needs a human review before it ships
store-replace-skip = Bỏ qua: giữ bản đã có sẵn

# Button that replaces the AppImage already there with the new one.
# SAFETY: needs a human review before it ships
store-replace-do = Thay thế

# Heading of the card that asks what to do with a Flatpak being removed.
# SAFETY: needs a human review before it ships
store-flatpak-ask-head = Gỡ { $name }?

# Under that heading: how much saved data the app has.
# SAFETY: needs a human review before it ships
store-flatpak-ask-size = Dữ liệu đã lưu của nó chiếm { $size }.

# Under that heading when the app has saved no data.
store-flatpak-ask-none = Nó không có dữ liệu đã lưu.

# Under that heading while the size of the saved data is being counted.
store-flatpak-ask-measuring = Đang tính dữ liệu đã lưu của nó...

# Under the choices: the two that delete data cannot be undone.
# SAFETY: needs a human review before it ships
store-flatpak-ask-undone = Dữ liệu đã xóa không thể khôi phục.

# Button: remove the app and leave the data it saved.
# SAFETY: needs a human review before it ships
store-flatpak-keep = Gỡ, giữ lại dữ liệu

# Button (red): remove the app and delete its saved data.
# SAFETY: needs a human review before it ships
store-flatpak-wipe = Gỡ và xóa dữ liệu của nó

# Button (red): delete only the saved data and keep the app.
# SAFETY: needs a human review before it ships
store-flatpak-data-only = Chỉ xóa dữ liệu của nó

# Why Delete its data only is greyed.
store-flatpak-data-only-none = Nó không có dữ liệu đã lưu để xóa.

# Tip of the round button that shuts the card with nothing done.
# SAFETY: needs a human review before it ships
store-flatpak-cancel = Hủy: giữ cả hai

# Said under the card or the row once only the data has been deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-gone = Dữ liệu đã lưu của nó đã bị xóa.

# Words while only the data is being deleted.
# SAFETY: needs a human review before it ships
store-flatpak-data-going = Đang xóa dữ liệu đã lưu của nó...

# Red words under a button when a job ended and gave no reason.
store-job-no-reason = Việc này chưa hoàn tất và không nêu lý do.

# Tip of the round i button beside a refusal that shows what is behind it.
store-show-behind = Cho xem vấn đề nằm ở đâu

# Placeholder of the box on the GitHub page.
store-gh-search-placeholder = Tìm trên GitHub: nhập rồi nhấn Enter

# Heading of the card of matches from AppImageHub.
store-gh-hub = Trên AppImageHub ({ $n })

# Note on the GitHub page before GitHub has been asked.
store-gh-press-enter = Nhấn Enter để tìm cả trên toàn bộ GitHub.

# Note on the GitHub page when the computer has no internet.
store-gh-offline-note = Chưa hỏi GitHub: không có internet để hỏi.

# Note on the GitHub page while GitHub is being asked.
store-gh-asking = Đang hỏi GitHub...

# Note on the GitHub page when GitHub knows nothing more.
store-gh-nothing-more = GitHub không còn gì thêm.

# Note on the GitHub page while the projects found are looked through.
store-gh-looking =
    { $n ->
        [1] Đang tìm tệp AppImage và Flatpak trong 1 dự án...
       *[other] Đang tìm tệp AppImage và Flatpak trong { $n } dự án...
    }

# Note on the GitHub page when no match has a file for this computer.
store-gh-none-fit = Không kết quả nào trên GitHub có tệp AppImage hoặc Flatpak cho máy tính này.

# Heading of the card of matches from GitHub.
store-gh-on = Trên GitHub

# Heading of the card that holds the search box.
store-gh-search-card = Tìm trên GitHub

# Line under the title of the GitHub page.
# SAFETY: needs a human review before it ships
store-gh-line = Ứng dụng trên GitHub dưới dạng tệp AppImage hoặc Flatpak. Không ai kiểm tra các tệp này: chỉ cài từ những người bạn tin tưởng.

# Line under the title of the GitHub page when the computer has no internet.
store-gh-offline = Không thể tìm trên GitHub khi máy tính này chưa kết nối internet.

# On a GitHub project's line: it has a Flatpak file and an AppImage.
store-gh-both-kinds = Flatpak và AppImage

# Tip of the round button that shows or hides a project's files.
store-gh-see-files = Xem các tệp

# Button that installs a GitHub project or file.
# SAFETY: needs a human review before it ships
store-gh-install = Cài đặt

# Link to a project's page on GitHub.
store-gh-open = Mở trên GitHub

# Under a file of a GitHub project: its kind, its size and its release.
store-gh-file-line = { $kind }, { $size }, bản phát hành { $tag }

# Name of the GitHub key's button and of its box.
# SAFETY: needs a human review before it ships
store-gh-key = Khóa GitHub

# Name of the GitHub key's button when a key is stored.
# SAFETY: needs a human review before it ships
store-gh-key-stored = Khóa GitHub: đã lưu một khóa

# In the GitHub key's box: what the key is for.
# SAFETY: needs a human review before it ships
store-gh-key-words = Gần như chắc chắn bạn không cần khóa này. Khi không có khóa, GitHub trả lời 60 yêu cầu mỗi giờ từ máy tính này; khi có khóa, là 5.000. Khóa chỉ nâng giới hạn đó. Nó chỉ dùng để xem các tệp công khai và không làm gì khác: nó không cho Trindle đường vào tài khoản của bạn, và bạn không đánh dấu quyền nào khi tạo nó. Khóa được giữ trên máy tính này, trong chùm khóa của bạn, và chỉ gửi cho GitHub.

# In the GitHub key's box: how to make a key.
# SAFETY: needs a human review before it ships
store-gh-key-how = Hãy tạo khóa trên GitHub, đặt tên bất kỳ, không đụng đến quyền nào, bấm Generate token, rồi dán những gì nó hiện ra vào đây.

# Red words under the GitHub key's box when the key could not be removed.
# SAFETY: needs a human review before it ships
store-gh-key-not-removed = Không thể xóa khóa: { $why }

# Red words under the GitHub key's box when the key could not be kept.
# SAFETY: needs a human review before it ships
store-gh-key-not-kept = Không thể lưu khóa: { $why }

# Placeholder of the GitHub key's box when a key is stored already.
# SAFETY: needs a human review before it ships
store-gh-key-keep-empty = Để trống để giữ khóa hiện có

# Placeholder of the GitHub key's box.
# SAFETY: needs a human review before it ships
store-gh-key-paste = Dán khóa vào

# Tip of the round button that removes the stored GitHub key.
# SAFETY: needs a human review before it ships
store-gh-key-remove = Xóa khóa

# Link in the GitHub key's box to the page where a key is made.
# SAFETY: needs a human review before it ships
store-gh-key-make = Tạo khóa trên GitHub

# Tip of the round button that keeps the GitHub key typed.
# SAFETY: needs a human review before it ships
store-gh-key-keep = Lưu khóa

# Red words when a project's newest release has nothing for this computer.
store-gh-nothing-installs = Không có gì trong bản phát hành mới nhất của nó cài được ở đây.

# Word on the file's dial and under a GitHub file when it was added.
store-added = Đã thêm

# Line under the title of the Move your apps page.
store-bk-line = Ứng dụng và cài đặt của bạn, trong một tệp để mang đi bất cứ đâu.

# Heading of the card that makes a backup.
store-bk-card = Sao lưu

# Tip of the round button while the backup is being made.
store-bk-making-tip = Đang tạo bản sao lưu

# Main button that makes the backup.
store-bk-make = Tạo bản sao lưu

# Tip of the round button that shows the backup in Folders.
store-bk-show = Hiện nó trong Thư mục

# Name of the switch for the list of apps in a backup.
store-bk-apps = Ứng dụng

# Under the Apps switch: how many Flatpak apps are listed.
store-bk-apps-list =
    { $n ->
        [1] Danh sách 1 ứng dụng Flatpak của bạn.
       *[other] Danh sách { $n } ứng dụng Flatpak của bạn.
    }

# Under the Apps switch: the list, then why the file stays small.
store-bk-apps-sub = { $list } Máy tính kia sẽ cài chúng mới từ Flathub, nên tệp vẫn nhỏ.

# Name of the switch for the apps' own data in a backup.
store-bk-data = Dữ liệu riêng của ứng dụng

# Under the Apps' own data switch.
store-bk-data-sub = Cài đặt và công việc đã lưu của từng ứng dụng, không kèm bộ nhớ đệm.

# Name of the switch for AppImages in a backup.
store-bk-appimages = AppImage

# Under the AppImages switch.
store-bk-appimages-sub = Được giữ nguyên vẹn vì mỗi cái là một tệp đơn.

# Name of the switch for Trindle's settings in a backup.
store-bk-settings = Cài đặt Trindle

# Under the Trindle settings switch.
store-bk-settings-sub = Vòng tròn, hình nền đã chọn, màu sắc, bàn phím, nguồn điện, ứng dụng mặc định và ảnh của bạn.

# Under a switch: what it holds, then how big it is now.
store-bk-now = { $sub } Hiện là { $size }.

# Name of the row that says where the backup goes.
store-bk-where = Nơi lưu

# Under that row: the folder and the file's name.
store-bk-where-sub = Thư mục chính của bạn, với tên { $name }

# Heading of the card that puts a backup back.
store-bk-back-card = Khôi phục bản sao lưu

# Tip of the round button, and name of the row, that choose a backup file.
store-bk-choose = Chọn tệp sao lưu

# Under the choose row: which backups can be used; the name of Flat's backups is given.
store-bk-choose-sub = Bản sao lưu của Trindle, hoặc bản do { $flat } tạo.

# Note in the card that puts a backup back.
store-bk-back-note = Ứng dụng của nó được cài đặt, còn dữ liệu và cài đặt được khôi phục. Tài liệu và ảnh của bạn không có trong đó: hãy tự sao chép chúng sang.

# Said while a backup is being made.
store-bk-making = Đang sao lưu... việc này có thể mất vài phút.

# Said when a backup is made: the file and its size.
store-bk-made = Đó là { $path } ({ $size }). Hãy sao chép nó vào USB hoặc kho lưu trữ đám mây để mang theo.

# Red words when no connection to the file chooser exists.
store-bk-no-bus = Chưa chọn bản sao lưu nào, vì Trindle không có kết nối tới trình chọn tệp.

# Title of the window that asks which backup to use.
store-bk-choose-title = Chọn một bản sao lưu

# Name of the filter in the window that asks which backup to use.
store-bk-filter = Bản sao lưu của Trindle và Flat

# Red words when the file chooser gave no answer.
store-bk-no-answer = Chưa chọn bản sao lưu nào, vì trình chọn tệp không phản hồi.

# Said while a backup file is read.
store-bk-reading = Đang đọc bản sao lưu...

# Said when a backup was put back.
store-bk-logout = Hãy đăng xuất rồi đăng nhập lại để thấy mọi cài đặt.

# Said when a backup was put back, after what was done; the first part is given.
store-bk-logout-after = { $last } Hãy đăng xuất rồi đăng nhập lại để thấy mọi cài đặt.

# Said once a Flat backup is read: how many apps and how many have settings.
store-bk-found =
    { $apps ->
        [1] 1 ứng dụng, { $with } có cài đặt. Đang bắt đầu...
       *[other] { $apps } ứng dụng, { $with } có cài đặt. Đang bắt đầu...
    }

# Line under the title of the Software sources page: how many makers' sources are added.
store-src-line =
    { $n ->
        [0] Chưa thêm nguồn riêng nào của nhà sản xuất.
        [1] Đã thêm 1 nguồn của nhà sản xuất, được cập nhật cùng các bản cập nhật của Debian.
       *[other] Đã thêm { $n } nguồn của nhà sản xuất, được cập nhật cùng các bản cập nhật của Debian.
    }

# Under a maker's source that is added and installed.
store-src-added = { $what }. Đã thêm, và luôn được cập nhật.

# Under a maker's source that is added but whose program is not installed.
store-src-no-program = { $what }. Nguồn đã được thêm, nhưng chương trình chưa được cài.

# Under a maker's source whose program is installed another way.
store-src-other-way = { $what }. Đã cài bằng cách khác.

# Tip of the round button that removes a maker's source and its program.
# SAFETY: needs a human review before it ships
store-src-remove-both = Gỡ nó và nguồn của nó

# Link to a maker's page, naming the program.
store-src-open = Mở { $name }

# Tip of the round button that signs in to Tailscale.
store-src-tailscale-sign-in = Đăng nhập Tailscale

# Tip of the round button that adds a maker's source, naming the program.
# SAFETY: needs a human review before it ships
store-src-add = Thêm { $name }

# Heading of the card of makers' sources that can be added.
store-src-ready = Sẵn sàng để thêm

# Name of the field for the address of a source.
store-src-address = Địa chỉ

# Under the address field.
store-src-address-sub = Địa chỉ của kho, lấy từ hướng dẫn cài đặt của nhà sản xuất.

# Name of the field for the suite of a source.
store-src-suite = Suite

# Under the suite field.
store-src-suite-sub = Thường là trixie hoặc stable.

# Name of the field for the components of a source.
store-src-components = Thành phần

# Under the components field.
store-src-components-sub = Thường là main.

# Name of the field for a source's signing key.
# SAFETY: needs a human review before it ships
store-src-key = Khóa ký

# Under the signing key field.
# SAFETY: needs a human review before it ships
store-src-key-sub = Địa chỉ của khóa ký của nó.

# Name of the field for the program to install from a source.
store-src-program = Chương trình cần cài

# Under the program field.
store-src-program-sub = Tên gói của nó, nếu bạn muốn cài ngay bây giờ.

# Placeholder of the program field.
store-src-program-hint = không bắt buộc

# Tip of the round button that adds the source typed.
# SAFETY: needs a human review before it ships
store-src-add-this = Thêm nguồn này

# Heading of the card that adds another source.
store-src-another = Thêm một nguồn khác

# Note in that card.
# SAFETY: needs a human review before it ships
store-src-another-note = Một nguồn có thể cài bản cập nhật cho cả máy tính, nên chỉ thêm những nhà sản xuất bạn tin tưởng. Mỗi nguồn sẽ yêu cầu mật khẩu quản trị viên.

# Tip of the round button that removes a source that is already on the computer.
# SAFETY: needs a human review before it ships
store-src-remove-this = Gỡ nguồn này

# Heading of the card of sources already on the computer.
store-src-already = Đã có trên máy tính này

# Red words when the addresses of a new source are not secure.
store-src-bad-address = Cả hai địa chỉ phải bắt đầu bằng https://

# Red words when the suite or components are not plain words.
store-src-bad-words = Suite và thành phần phải là các từ thuần túy.

# Red words when the program is not a package name.
store-src-bad-package = Đây không phải tên gói.

# Word on a source's dial while its job starts.
store-src-starting = Đang bắt đầu

# Said under Tailscale's row while its sign-in page is opened.
store-src-opening = Đang mở trang đăng nhập của Tailscale...

# Said under Tailscale's row once its sign-in page has opened.
store-src-opened = Hãy đăng nhập trên trang vừa mở; việc này sẽ tự hoàn tất.

# Said under Tailscale's row when it is connected.
store-src-connected = Đã kết nối

# Said under the backup row when its apps come from an http address on the home network: the link is not encrypted, and the restore goes on.
# SAFETY: needs a human review before it ships
store-bk-http-line = Liên kết tới các ứng dụng không được mã hóa.

# Title of the card shown before a restore from an http address over the internet.
# SAFETY: needs a human review before it ships
store-bk-http-title = Không được mã hóa

# Words of that card.
# SAFETY: needs a human review before it ships
store-bk-http-card = Địa chỉ này không được mã hóa. Người nào đó trên đường truyền có thể thấy những gì được gửi, và có thể đánh tráo bản sao lưu bằng một bản khác.

# Label of the tick box on that card; the restore button works only once it is on.
# SAFETY: needs a human review before it ships
store-bk-http-tick = Tôi hiểu và chấp nhận rủi ro này

# Button on that card that restores the backup.
# SAFETY: needs a human review before it ships
store-bk-http-go = Khôi phục

# Button on that card that gives the restore up.
# SAFETY: needs a human review before it ships
store-bk-http-cancel = Hủy

# Under that heading when the folder of the saved data cannot be read, so its size is not known.
# SAFETY: needs a human review before it ships
store-flatpak-ask-uncounted = Không thể đếm được dữ liệu đã lưu của ứng dụng.

store-hub-warning = AppImageHub chỉ liệt kê các dự án: không ai xem xét hay quét chúng, và chúng không nhất thiết được ký. Trindle chạy mỗi AppImage trong một hộp có thư mục chính riêng: lúc đầu nó có thể dùng internet và âm thanh, và không thấy tệp nào của bạn.

# Said once where AppImages are listed: the line saying where each comes from is not checked.
store-from-hint = Nguồn gốc của một AppImage chỉ là gợi ý, không phải đã kiểm tra.

# Where an AppImage comes from: a project on GitHub, by its owner and name.
store-from-github = Trên GitHub: { $repo }

# The same, with how many stars the project has.
store-from-github-stars = Trên GitHub: { $repo }, { $stars } sao

# Where an AppImage comes from: a website, by its name.
store-from-site = Không ở GitHub: từ một trang web, { $site }

# Where an AppImage comes from, for one added by hand.
store-from-unknown = Không rõ nguồn

# Round button that opens the website an AppImage is fetched from.
store-from-site-open = Mở trang web của nó

# When an app was last updated.
store-age-updated = Cập nhật lần cuối: { $date }

# The same, said in red, when that is more than six months ago.
store-age-updated-old = Cập nhật lần cuối: { $date }. Hơn sáu tháng chưa cập nhật, nên có thể còn lỗi bảo mật chưa sửa.

# When the source does not say when an app was last updated.
store-age-updated-unknown = Cập nhật lần cuối: không rõ

# When an app's project was made.
store-age-created = Tạo ngày { $date }

# When the source does not say when an app's project was made.
store-age-created-unknown = Ngày tạo: không rõ

# A date the source does not give, on a line of Facts.
store-age-not-known = Không rõ

# Title of the Browse AppImages page's first card, with the plain warning.
store-browse-about = Trước khi thêm một cái

# Title of the card that lists the AppImages.
store-browse-list = AppImage

# One sentence under the Browse AppImages title.
store-browse-line = { $n ->
       *[other] AppImageHub liệt kê { $count } AppImage.
    }

# The same while AppImageHub's list is being fetched.
store-browse-fetching = Đang lấy danh sách của AppImageHub…

# The same when the list could not be fetched.
store-browse-failed = Không lấy được danh sách của AppImageHub.

# The same with no internet and no copy kept.
store-browse-offline = Không thể lấy danh sách của AppImageHub khi máy tính này chưa kết nối internet.

# In the list when the kind chosen has none.
store-browse-none = AppImageHub không liệt kê cái nào thuộc loại này.

# Under the rows: how many are shown of how many.
store-browse-showing = Đang hiện { $shown } trên { $n }.

# Round button that fetches an AppImage and adds it to Trindle's apps.
store-browse-add = Thêm vào ứng dụng của Trindle

# Round button that asks GitHub for a project's dates, stars and files.
store-browse-details = Xem chi tiết và tệp



# Part of the front page's line when nothing is known to wait and looking for updates by itself is switched off in Settings.
store-grid-home-unchecked = việc kiểm tra cập nhật đang tắt

# One line at the foot of Apps' results when the search asks for ChatGPT, OpenAI or Codex.
store-chatgpt-returns = ChatGPT sẽ trở lại khi OpenAI ký bản tải về của nó.



# Line under the front page's title when it lists Flathub's apps and AppImageHub's; says how many there are and what waits.
store-grid-home-both = Có { $apps } để chọn trên Flathub và AppImageHub; { $more }.

# Line under the front page's title when it lists only AppImageHub's apps.
store-grid-home-images = Có { $apps } để chọn trên AppImageHub.

# Selector by the title of Discover, a kind and results: Flatpaks and AppImages together.
store-show-both = Cả hai

# The same: only Flatpaks.
store-show-flatpaks = Flatpak

# The same: only AppImages.
store-show-appimages = AppImage

# Line under the name of an AppImage on a tile, with what it does.
store-appimage-line = AppImage: { $summary }

# The same when the list says nothing of what it does.
store-appimage-line-bare = AppImage

# Head of the card shown before any AppImage is installed.
store-appimage-warn-head = AppImage không được kiểm tra. Không ai xem xét hay quét chúng.

# Under it: how Trindle boxes an AppImage.
store-appimage-warn-box = Trindle chạy mỗi AppImage trong một hộp có thư mục chính riêng: lúc đầu nó có thể dùng internet và âm thanh, và không thấy tệp nào của bạn.

# Label of the small tick in that card's corner; once it is set the card never comes again.
store-appimage-warn-hide = Không hiện lại

# Button on that card that goes on with the install.
store-appimage-warn-ok = OK

# Button on that card that gives the install up.
store-appimage-warn-cancel = Hủy

