
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Chào mừng đến với Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = Mở vòng tròn

# Title of the third page, where the first apps are picked.
welcome-page-apps = Chọn ứng dụng của bạn

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Còn nhiều lựa chọn khác

# Title of the fifth page, about plugins.
welcome-page-plugins = Plugin thêm nhiều hơn

# Title of the sixth page, about crash reports.
welcome-page-reports = Giúp sửa lỗi Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Các chấm

# Title of the last page, with a few things worth knowing.
welcome-page-know = Nên biết

# Sentence under the title of the first page.
welcome-sentence-hello = Mọi thứ bạn cần nằm trong một vòng tròn, gọi là vòng tròn.

# Sentence under the title of the second page.
welcome-sentence-ring = Nhấp đúp sát mép màn hình, hoặc nhấn phím Super.

# Sentence under the title of the third page.
welcome-sentence-apps = Hầu như chưa cài gì, nên hãy chọn các ứng dụng bạn muốn.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Chọn thêm, rồi cài mọi thứ đã chọn chỉ bằng một cú bấm.

# Sentence under the title of the fifth page.
# SAFETY: needs a human review before it ships
welcome-sentence-plugins = Plugin là các phần bổ sung nhỏ từ Ứng dụng, và mỗi cái chỉ làm những gì bạn cho phép.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Gửi báo cáo khi có sự cố, nếu bạn chọn.

# Sentence under the title of the dots page.
welcome-sentence-dots = Một chấm nghĩa là có chương trình đang dùng thứ gì đó của bạn.

# Sentence under the title of the last page.
welcome-sentence-know = Vài điều làm Trindle khác biệt.

# Top right of every page: where the person is in the tour.
welcome-step = Bước { $n } trên { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Bước { $n } trên { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
# SAFETY: needs a human review before it ships
welcome-done-file-note = Phần Chào mừng đã được xem. Nó nằm trong Tất cả ứng dụng với tên Chào mừng; hãy xóa tệp này để xem lại ở lần đăng nhập tiếp theo.

# Name of the round button at the foot that closes the tour.
welcome-close = Đóng

# Name of the round button at the foot that goes to the page before.
welcome-back = Quay lại

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Tiếp theo

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Cài các ứng dụng đã chọn

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Bắt đầu dùng Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = Hãy kết nối internet để cài ứng dụng.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Thử lại

# Foot while apps are going in: their names.
welcome-installing = Đang cài { $names }.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = Đang cài { $names }, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
       *[other] { $names } đã có trong vòng tròn.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = Không thể cài các ứng dụng. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak đã từ chối.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Đã chọn: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } và { $more } ứng dụng khác

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = Vòng tròn

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = Có gì trong đó

# Name of the first thing the ring holds.
welcome-held-programs = Các chương trình của bạn

# Line under it.
welcome-held-programs-line = Nằm ở vòng ngoài.

# Name of the second thing the ring holds.
welcome-held-time = Giờ và ngày

# Line under it.
welcome-held-time-line = Nằm ở giữa.

# Name of the third thing the ring holds.
welcome-held-switches = Các nút bật tắt

# Line under it.
welcome-held-switches-line = Âm thanh, màn hình, giữ máy thức và nguồn điện.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = Bằng con trỏ

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = Bằng bàn phím

# Note under the picture of the Super key.
welcome-ring-super-note = Phím Super thường có biểu tượng Windows.

# Head of a card of apps: web browsers.
welcome-kind-browser = Trình duyệt web

# Head of a card of apps: email programs.
welcome-kind-email = Thư điện tử

# Head of a card of apps: chat programs.
welcome-kind-chat = Trò chuyện

# Head of a card of apps: office programs.
welcome-kind-office = Văn phòng

# Head of a card of apps: music and video players.
welcome-kind-media = Nhạc và video

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Ảnh và vẽ

# Head of a card of apps: notes and writing.
welcome-kind-notes = Ghi chú và soạn thảo

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF và tài liệu

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Hiện chưa có ứng dụng nào trong số này trên Flathub.

# Beside an app that has been installed.
welcome-app-installed = Đã cài

# Beside an app that is going in now.
welcome-app-installing = Đang cài

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, đã chọn

# Name of the round button on an app's row that picks it.
welcome-app-pick = Chọn { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = Trong vòng tròn

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Các plugin

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = Trong Ứng dụng, mục Plugin.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Mở Plugin trong Ứng dụng

# Said in the plugins card while there are none to show.
welcome-plugins-none = Các plugin có sẵn được liệt kê trong Ứng dụng.

# Head of the card on the crash reports page.
welcome-reports-card = Báo cáo sự cố

# The one question about crash reports.
welcome-reports-question = Giúp sửa lỗi Trindle bằng cách gửi báo cáo khi có sự cố? Bạn luôn được xem báo cáo trước.

# Note under the question about crash reports.
welcome-reports-note = Không có gì được gửi nếu bạn chưa bấm. Bạn xem toàn bộ báo cáo trước, và nó không chứa tệp, mật khẩu hay tên Wi-Fi.

# Second note under the question about crash reports.
welcome-reports-settings = Bạn có thể đổi điều này bất cứ lúc nào trong Cài đặt, Báo cáo sự cố.

# The answer that turns crash reports off.
welcome-reports-no = Không

# The answer that turns crash reports on.
welcome-reports-yes = Có, hãy hỏi tôi

# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = Gửi báo cáo khi có sự cố

# Head of a card on the last page.
welcome-know-boxes = Ứng dụng sống trong hộp riêng của chúng

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-know-boxes-line = Ứng dụng sẽ hỏi trước khi truy cập tệp, camera hoặc micro của bạn.

# Head of a card on the last page.
welcome-know-updates = Cập nhật bảo mật tự cài đặt

# Line in that card.
welcome-know-updates-line = Mỗi ngày, không cần hỏi. Các bản cập nhật khác chờ bạn.

# Head of a card on the last page.
welcome-know-ring = Cài đặt và Ứng dụng nằm trong vòng tròn

# Line in that card.
welcome-know-ring-line = Ở giữa nó, phía trên giờ.

# Head of a card on the last page.
welcome-know-help = Trợ giúp nằm trong Cài đặt

# Line in that card.
welcome-know-help-line = Phần giới thiệu này nằm trong Tất cả ứng dụng với tên Chào mừng, khi nào bạn muốn xem lại.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Một màn hình nhỏ không có gì ngoài vòng tròn ở giữa: giờ ở tâm, các ứng dụng yêu thích xung quanh.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = Vòng tròn mở trên một màn hình nhỏ, với biểu tượng của vài plugin xếp một hàng ở giữa, dưới giờ, là nơi biểu tượng plugin nằm sau khi được cài.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Hai hình vẽ nhỏ về nơi các chấm hiện: trên thẻ của một cửa sổ trong vòng tròn, và ở giữa phía trên màn hình.

welcome-dots-where-tab = Trên thẻ của một cửa sổ

welcome-dots-where-corner = Giữa phía trên màn hình

welcome-picture-ring = Một màn hình nhỏ. Con trỏ di chuyển đến sát mép trên và nhấp đúp, rồi vòng tròn mở ra ở giữa.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = Phím Super đang được nhấn, mở vòng tròn ra.

# Title of the first page, which asks how much the person knows.
welcome-page-level = Bạn biết đến đâu?

# Sentence under that title.
welcome-sentence-level = Chọn mức gần nhất, và phần giới thiệu sẽ bỏ qua những gì bạn đã biết.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Mới dùng Linux

# Line in that answer's card.
welcome-level-new-line = Mọi trang, kể cả những gì máy tính Linux nào cũng làm.

# Name of the answer for someone who has used Linux.
welcome-level-linux = Đã dùng Linux

# Line in that answer's card.
welcome-level-linux-line = Chỉ phần riêng của Trindle. Bỏ qua kiến thức cơ bản về Linux.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = Đã dùng Trindle

# Line in that answer's card.
welcome-level-trindle-line = Đóng phần giới thiệu ngay và không hiện lại nữa.

# Title of the page about files and folders.
welcome-page-files = Tệp của bạn

# Sentence under that title.
welcome-sentence-files = Mọi thứ của bạn đều nằm trong thư mục cá nhân.

# Title of the page about getting software.
welcome-page-software = Cài phần mềm

# Sentence under that title.
welcome-sentence-software = Ứng dụng đến từ Ứng dụng, không phải từ tệp cài đặt tải về.

# Title of the page about the administrator password.
welcome-page-admin = Mật khẩu quản trị viên

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Thay đổi cho cả máy tính sẽ hỏi mật khẩu trước.

# Title of the page about snapshots.
welcome-page-snapshots = Bản chụp hệ thống

# Sentence under that title.
welcome-sentence-snapshots = Trindle giữ các bản sao của hệ thống, nên có thể hoàn tác một bản cập nhật.

# Title of the page about the person's own words.
welcome-page-words = Chữ của riêng bạn

# Sentence under that title.
welcome-sentence-words = Đổi bất kỳ chữ nào Trindle hiển thị, hoặc thêm ngôn ngữ của riêng bạn.

# Head of a card on the files page.
welcome-files-home = Thư mục cá nhân

# Line in that card.
welcome-files-home-line = Tài liệu, Tải về và Ảnh nằm trong đó. Thư mục mở ngay tại đó.

# Head of a card on the files page.
welcome-files-drives = Không có ký tự ổ đĩa

# Line in that card.
welcome-files-drives-line = Ổ USB hoặc đĩa khác sẽ hiện trong Thư mục khi bạn cắm vào.

# Head of a card on the files page.
welcome-files-hidden = Tên ẩn

# Line in that card.
welcome-files-hidden-line = Tên bắt đầu bằng dấu chấm, như .config, được ẩn đi. Nó chứa cài đặt của một ứng dụng.

# Head of a card on the files page.
welcome-files-bin = Thùng rác

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = Tệp đã xoá nằm trong thùng rác cho đến khi bạn dọn sạch.

# Head of a card on the software page.
welcome-software-apps = Một nơi cho ứng dụng

# Line in that card.
welcome-software-apps-line = Mở Ứng dụng từ vòng tròn để tìm, cài đặt và gỡ chương trình.

# Head of a card on the software page.
welcome-software-exe = Không có tệp cài đặt

# Line in that card.
welcome-software-exe-line = Tệp cài đặt Windows, đuôi .exe, không chạy được ở đây. Hãy tìm ứng dụng trong Ứng dụng.

# Head of a card on the software page.
welcome-software-updates = Cập nhật ở một nơi

# Line in that card.
welcome-software-updates-line = Ứng dụng và Cài đặt giữ mọi thứ luôn mới, nên ứng dụng nào cũng không cần trình cập nhật riêng.

# Head of a card on the software page.
welcome-software-remove = Gỡ bỏ sạch sẽ

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = Gỡ một ứng dụng sẽ lấy đi toàn bộ và không để lại gì.

# Head of a card on the administrator password page.
welcome-admin-why = Vì sao nó hỏi

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Cài cho mọi người hoặc thay đổi đĩa ảnh hưởng đến mọi người dùng, nên máy tính hỏi trước.

# Head of a card on the administrator password page.
welcome-admin-who = Mật khẩu của ai

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = Của một quản trị viên, thường là người bạn dùng để đăng nhập.

# Head of a card on the administrator password page.
welcome-admin-not = Khi nào không hỏi

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Ứng dụng bạn cài cho riêng mình và tệp của bạn không bao giờ cần đến nó.

# Head of a card on the snapshots page.
welcome-snap-taken = Chụp sẵn cho bạn

# Line in that card.
welcome-snap-taken-line = Trên hệ thống Btrfs, một bản được chụp trước và sau mỗi thay đổi phần mềm.

# Head of a card on the snapshots page.
welcome-snap-list = Liệt kê trong Cài đặt

# Line in that card.
welcome-snap-list-line = Cài đặt, Bản chụp hệ thống liệt kê chúng. Nút máy ảnh chụp một bản ngay bây giờ.

# Head of a card on the snapshots page.
welcome-snap-boot = Dùng khi máy không khởi động

# Line in that card.
welcome-snap-boot-line = Chọn Trindle snapshots trong menu khởi động, rồi chọn một bản chụp.

# Head of a card on the your own words page.
welcome-words-change = Đổi một chữ

# Line in that card.
welcome-words-change-line = Tìm bất kỳ chữ nào Trindle hiển thị và gõ chữ của bạn. "Open it, brew" có thể thay cho "Click here to open the file".

# Head of a card on the your own words page.
welcome-words-share = Chia sẻ một bộ chữ

# Line in that card.
welcome-words-share-line = Xuất chữ của bạn thành một tệp, hoặc nhập tệp bạn bè gửi. Tệp được kiểm tra trước khi dùng.

# Head of a card on the your own words page.
welcome-words-all = Mọi chương trình cùng lúc

# Line in that card.
welcome-words-all-line = Cả Trindle đọc chữ từ một nơi, nên một thay đổi hiện ở mọi nơi.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = Nó nằm trong Cài đặt, Vùng và ngôn ngữ. Trợ giúp giải thích từng bước.

# Name of the round button that opens that Settings page.
welcome-words-open = Mở Vùng và ngôn ngữ
