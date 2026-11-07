
# machine translated (Vietnamese), not yet reviewed by a person.

# Window title when the person opens it themselves from the ring.
report-title-note = Báo cáo sự cố

# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Đã xảy ra lỗi. Gửi báo cáo?

# Under the title: what a report holds, and what it never holds.
report-summary = Thao tác này gửi lỗi, bản tóm tắt về máy tính và các dòng cuối của nhật ký, đã loại bỏ tên, địa chỉ email, địa chỉ mạng và tên Wi-Fi. Tên tệp và địa chỉ web vẫn có thể còn trong các dòng nhật ký. Tệp và mật khẩu của bạn không được gửi.

# Typing box at the top of the window, shown quietly while empty.
report-what-doing = Bạn đang làm gì? (không bắt buộc)

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
report-not-taken = Hiện chưa nhận báo cáo.

# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Đang kiểm tra.


# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Có quá nhiều báo cáo cùng lúc.

# Written beside the Send button once the report has been sent.
report-already-sent = Đã gửi.

# Written under the typing box when Save is pressed a second time.
report-already-saved = Đã lưu.

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
report-no-at = Hãy dùng tên người dùng Buildmorphic, không phải địa chỉ email, nên không có @.

# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = Tên người dùng gồm 2 đến 31 chữ thường, chữ số hoặc dấu gạch ngang.

# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = Tên GitHub chỉ gồm chữ cái, chữ số và dấu gạch ngang.

# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Không thể giữ lại để gửi sau: { $reason }.

# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Không thể lưu: { $reason }.

# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = không có quyền

# The reason inside "Could not save it: ...": there is no room.
report-reason-full = ổ đĩa đã đầy

# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = thiếu thư mục

# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = ổ đĩa từ chối

# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = thiếu curl.

# Why a report could not be posted: the post could not be started.
report-not-posted = Không thể gửi.

# Why a report could not be posted: the computer is offline.
report-no-connection = Không có kết nối.

# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = Địa chỉ trả lời { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Hãy viết thêm vài từ để nói điều gì đã xảy ra.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Đang đọc nhật ký.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = Không có nhật ký để gửi kèm, nên không thể gửi. Hãy lưu lại thay vào đó.



report-sent-as-bug = Đã gửi. Được ghi nhận là lỗi { $number }. Cảm ơn bạn.

