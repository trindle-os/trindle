
# machine translated (Vietnamese), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Báo cáo sự cố

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Đã xảy ra lỗi. Gửi báo cáo?

# Under the title: what a report holds, and what it never holds.
report-summary = Thao tác này gửi lời bạn viết, thông tin máy tính, các lỗi của lần khởi động này và nhật ký của Trindle. Tên và địa chỉ (email, mạng, Wi-Fi) được loại khỏi những gì Trindle thu thập. Tên tệp và địa chỉ web vẫn có thể còn trong các dòng nhật ký. Thao tác này không gửi tệp hay mật khẩu của bạn.

# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = Chuyện gì đã xảy ra?

# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = Ứng dụng nào?

# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = Không chắc

# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Chương trình khác

# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = Màn hình nền

# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = Gửi

# Button at the bottom of the window when the person opened it themselves.
# SAFETY: needs a human review before it ships
report-cancel = Huỷ

# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = Không gửi

# Button that opens the exact text of the report.
report-show-details = Xem chi tiết

# Button that closes the exact text of the report.
report-hide-details = Ẩn chi tiết

# Button under the shown report text that copies it.
report-copy = Sao chép

# Button under the shown report text that saves it as a file in Downloads.
report-save = Lưu

# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Hiện chưa thể gửi báo cáo.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Đang kiểm tra.

# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Có quá nhiều báo cáo cùng lúc.

# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Đang chờ kết nối.

# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Đang chờ kết nối. { $why }

# Under the typing box after the report was sent.
report-sent = Đã gửi. Cảm ơn bạn.

# Under the typing box while the report is being posted.
report-sending = Đang gửi.

# Under the typing box after Save. $name is the file's name.
report-saved-as = Đã lưu thành Downloads/{ $name }.

# Red words when the Buildmorphic username holds an email address.
report-no-at = Hãy nhập tên người dùng Buildmorphic của bạn, không phải địa chỉ email. Đừng dùng ký tự @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Tên người dùng phải dài từ 2 đến 31 ký tự, gồm chữ thường, chữ số hoặc dấu gạch ngang.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Tên GitHub chỉ được dùng chữ cái, chữ số và dấu gạch ngang.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle không lưu được báo cáo để gửi sau: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Không lưu được: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = không có quyền

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = ổ đĩa đã đầy

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = thiếu thư mục

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = lỗi ở ổ đĩa

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = thiếu curl.

# Why a report could not be posted: the post could not be started.
report-not-posted = Không gửi được.

# Why a report could not be posted: the computer is offline.
report-no-connection = Không có kết nối.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Máy chủ trả về { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Hãy viết thêm vài từ về chuyện đã xảy ra.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Đang đọc nhật ký.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Không có nhật ký để gửi kèm báo cáo, nên không thể gửi. Hãy lưu lại thay vào đó.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = Ảnh không bắt buộc. Chỉ chủ dự án thấy được.

# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = Chọn một ảnh PNG

# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Chụp ảnh màn hình

# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Bỏ ảnh

# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Ảnh của bạn, { $width } x { $height }

# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = Chọn một ảnh PNG

report-picture-filter = Ảnh PNG

# Under the picture while it is being read.
report-picture-reading = Đang đọc.

# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = Không chụp được ảnh màn hình.

report-picture-no-chooser = Trindle không mở được trình chọn tệp.

# Red words under the picture, each the reason it is refused.
report-picture-not-png = Đây không phải ảnh PNG.

report-picture-too-big = Lớn hơn { $megabytes } MB.

report-picture-too-large = Ảnh quá lớn. Ảnh có thể rộng tối đa { $side } pixel và cao tối đa { $side } pixel.

report-picture-damaged = Ảnh bị hỏng.

report-picture-unreadable = Không đọc được.

report-sent-as-bug = Đã gửi. Số báo cáo lỗi của bạn là { $number }. Cảm ơn bạn.

# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle đang ở bản beta. Báo cáo là ẩn danh: tên, địa chỉ email và thư mục cá nhân đều được loại bỏ. Tên người dùng là tùy chọn.

# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Tên người dùng Buildmorphic (tùy chọn)

# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle không gửi được báo cáo. { $why } Nhấn Gửi để thử lại.
