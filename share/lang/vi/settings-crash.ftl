
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Báo cáo sự cố

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = báo cáo sự cố lỗi hỏng gửi đã gửi riêng tư đo từ xa buildmorphic

# Help line at the foot of the Crash reports page.
# SAFETY: needs a human review before it ships
settings-crash-help = Khi một chương trình của Trindle bị lỗi, tiện ích báo cáo sự cố lưu một ghi chú ngắn trên máy tính này. Nó không gửi gì cho đến khi bạn chạy lệnh gửi và đồng ý với từng báo cáo, lệnh này hiện toàn bộ báo cáo trước. Tên của bạn và tên GitHub chỉ có trong báo cáo nếu bạn đã nhập ở đây. Không bao giờ gửi tin nhắn, tên tệp, nơi bạn ở hay bất cứ điều gì về các chương trình khác. Gỡ tiện ích sẽ dừng tất cả.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = Sự cố treo

# Under the Crashes switch.
settings-crash-crashes-short = Sự cố xảy ra ở đâu trong mã của Trindle.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = Sự cố treo gửi chương trình, phiên bản của nó, loại sự cố và nơi nó xảy ra trong mã của Trindle.

# Heading and label of the switch for sending errors.
settings-crash-errors = Lỗi

# Under the Errors switch.
settings-crash-errors-short = Tên cố định của lỗi, và nơi xảy ra.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = Lỗi gửi chương trình, phiên bản của nó, tên cố định mà Trindle đặt cho lỗi và nơi nó nằm trong mã.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = Về máy tính này

# Under the About this computer switch.
settings-crash-computer-short = Linux, bộ xử lý, bộ nhớ, đồ họa, màn hình.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = Về máy tính này thêm vào mỗi báo cáo: bản Linux đang chạy, loại bộ xử lý, bộ nhớ làm tròn xuống, Wayland hay X11, hãng sản xuất đồ họa và số màn hình.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = Những gì được gửi

# Help text of the What is sent card: { $switches } is what each switch sends.
# SAFETY: needs a human review before it ships
settings-crash-what-about = { $switches } Báo cáo không mang tên, trừ khi bạn nhập tên người dùng Buildmorphic vào ô bên dưới, việc này chỉ giúp bạn được ghi công.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Không có gì thay đổi, vì { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Không có gì thay đổi, vì tệp lưu các công tắc không đúng như Trindle đã ghi.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = Không có thư mục cài đặt.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = Không thể tạo thư mục cài đặt.

# Red words when a setting could not be written to its file.
settings-crash-not-written = Không thể ghi cài đặt.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Sự cố treo ({ $signal }) trong { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Lỗi { $code } trong { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = Đã nhận

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = Chưa gửi: trình theo dõi lỗi đã đóng

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Chưa gửi: quá lớn

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Chưa gửi: bị từ chối

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Chưa gửi: quá nhiều cùng lúc

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Chưa gửi

# Said beside the link line once it is saved.
settings-crash-saved = Đã lưu

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = Xóa danh sách

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = Không thể xóa danh sách.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Đã xóa

# Heading of the line that holds the clear button.
settings-crash-kept-here = Chỉ lưu trên máy tính này

# Under the Kept on this computer only heading.
settings-crash-newest-first = Mới nhất trước, tối đa năm mươi.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Chưa có gì

# Under the Nothing yet heading.
settings-crash-listed-here = Mỗi báo cáo được liệt kê ở đây, đúng như khi gửi.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Xem chính xác những gì đã gửi

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
       *[other] { $when }, { $n } lần: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Đã gửi đến nay

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Mỗi báo cáo được liệt kê đúng như khi gửi, mới nhất trước, và chỉ lưu trên máy tính này. Đã nhận nghĩa là Buildmorphic đã có nó. Chưa gửi: trình theo dõi lỗi đã đóng nghĩa là trình theo dõi lỗi của Trindle không nhận báo cáo; bị từ chối nghĩa là bị từ chối vì làm sai; quá nhiều cùng lúc nghĩa là quá nhiều báo cáo đến một lúc.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = Sau một sự cố bạn sẽ được hỏi, với toàn bộ báo cáo được hiển thị, có gửi hay không.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Không gửi gì, và bạn không được hỏi sau một sự cố.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Không gửi gì: sự cố treo và lỗi đều đã tắt.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Lỗi của các chương trình Trindle được báo cáo ẩn danh.
        [both] Sự cố treo và lỗi của các chương trình Trindle được báo cáo ẩn danh.
       *[crashes] Sự cố treo của các chương trình Trindle được báo cáo ẩn danh.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
# SAFETY: needs a human review before it ships
settings-crash-ask = Hỏi về sự cố và chia sẻ báo cáo

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = Tắt sẽ ẩn tất cả và xóa các báo cáo đang chờ.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Đã tắt. Không có báo cáo nào đang chờ.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
       *[other] Đã tắt. Đã xóa { $n } báo cáo đang chờ.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = Về cộng đồng

# Heading of the line with the community button.
settings-crash-go-to-bm = Báo cáo được gửi tới Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = Cộng đồng giúp xây dựng Trindle.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = tên người dùng

# Heading of the line with the username box.
settings-crash-your-username = Tên người dùng Buildmorphic của bạn

# Under the username heading.
settings-crash-username-sub = Tùy chọn. Không bao giờ là địa chỉ email.

# Heading of a line about joining with an email address.
settings-crash-join = Tham gia chỉ với một địa chỉ email

# Under the Join heading.
settings-crash-join-sub = Nhận điểm thưởng và cấp bậc cho các vấn đề bạn báo đầu tiên.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Một báo cáo chỉ chứa một danh sách cố định

# Under the fixed list heading.
settings-crash-fixed-sub = Bạn xem trước, và không gì được gửi nếu bạn chưa nhấn.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Báo cáo bạn gửi

# Help text of the Reports you send card (the information button).
# SAFETY: needs a human review before it ships
settings-crash-reports-about = Khi một chương trình của Trindle gặp sự cố, một cửa sổ có thể mở ra hỏi có gửi báo cáo không. Nó hiện nguyên văn nội dung trước: lời của bạn, kiểu máy và đồ họa của máy tính, lỗi và nhật ký ngay trước đó. Tệp, mật khẩu và tên Wi-Fi không bao giờ được đọc, còn tên và địa chỉ được che đi. Bạn cũng có thể mở cùng cửa sổ đó từ nút Báo cáo sự cố của vòng tròn bất cứ lúc nào.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = tên github

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Tên GitHub của bạn

# Small words under that heading.
settings-crash-github-about = Tùy chọn. Hiển thị là Người báo cáo.

