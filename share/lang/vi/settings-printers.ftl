
# machine translated (Vietnamese), not yet reviewed by a person.

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = Chưa có máy in nào. Các máy in tìm thấy trên mạng của bạn được liệt kê bên dưới.

# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } đang tạm dừng, nên tạm thời sẽ không in gì trên nó.

# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } đang in.

# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Sẵn sàng in trên { $name }.

# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } đang ở trạng thái { $state }.

# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } đã được thiết lập; chưa chọn máy in nào để in trước.

# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = Đã thiết lập { $n } máy in; chưa chọn máy nào để in trước.

# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = CUPS không biết lệnh in đó.

# Shown beside a print job after it was cancelled.
# SAFETY: needs a human review before it ships
settings-printers-cancelled = Đã huỷ

# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = lpstat không chạy được

# Words in the ring while the network is searched for printers.
settings-printers-searching = Đang tìm

# Words in the ring when the search is finished.
settings-printers-search-done = Xong

# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Giờ là mặc định

# Shown beside a printer after a test page was sent.
settings-printers-sent = Đã gửi

# Shown beside a printer after it was removed.
# SAFETY: needs a human review before it ships
settings-printers-removed = Đã gỡ

# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Hãy nhập địa chỉ của máy in trước.

# Shown under the By its address line while the printer is added.
settings-printers-adding = Đang thêm...

# Shown beside a printer or the address line after it was added.
settings-printers-added = Đã thêm

# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Đang tìm trình điều khiển của nó...

# Heading of a print job in the queue: its number and the printer.
settings-printers-job = Lệnh in { $number } trên { $printer }

# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }

# Tip of the cross button that cancels a print job.
# SAFETY: needs a human review before it ships
settings-printers-cancel-job = Huỷ lệnh in này

# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Lệnh in

# Help text (i button) of the Print jobs card.
# SAFETY: needs a human review before it ships
settings-printers-jobs-about = Mọi lệnh in đang chờ, trên bất kỳ máy in nào, như hiện tại. Danh sách được đọc lại vài giây một lần khi trang này đang mở. Dấu chéo huỷ một lệnh in; giấy đã in rồi thì vẫn đã in.

# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS không phản hồi

# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Không thể đọc hàng đợi.

# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Đang xem…

# Small line under "Looking…" in the Print jobs card.
settings-printers-queue = Hàng đợi.

# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Không có gì đang chờ

# Small line under "Nothing waiting".
settings-printers-nothing-waiting-sub = Các lệnh in hiện ở đây khi đang in.

# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = Tạm dừng

# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Rảnh

# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Đang in

# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Mặc định, { $state }

# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = In trên máy này trước

# Tip of the button that prints a test page.
settings-printers-test-page = In một trang thử

# Tip of the bin button that removes a printer.
# SAFETY: needs a human review before it ships
settings-printers-remove = Gỡ máy in này

# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = Việc tìm kiếm cần cups-filters

# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = Hãy cài bằng lệnh: sudo apt install cups-filters

# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Thêm máy in này

# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Không tìm thấy gì mới

# Small line under "Nothing new found".
settings-printers-nothing-new-sub = Máy in đã bật và cùng mạng chưa?

# Small line under "Looking…" in the Add a printer card.
settings-printers-looking-sub = Các máy tự thông báo, rồi lần lượt từng máy.

# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50

# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Thêm

# Heading of the line where a printer's address is typed.
settings-printers-by-address = Theo địa chỉ

# Small line under "By its address".
settings-printers-by-address-sub = Như 192.168.1.50, hiện trên màn hình của chính máy in.

# Heading of the card for adding printers.
settings-printers-add-card = Thêm máy in

# Help text (i button) of the Add a printer card.
settings-printers-add-about = Trước hết tìm các máy in tự thông báo, sau đó lần lượt hỏi từng máy trên mạng. Máy in phải được bật và cùng mạng. Một số máy in không tự thông báo, và một số mạng không chuyển tiếp thông báo đó: hãy thêm theo địa chỉ, như 192.168.1.50. Bạn có thể thấy địa chỉ trên màn hình của chính máy in, trong phần cài đặt mạng.

# Name of the page in the list of settings pages.
settings-printers-title = Máy in

# Words the settings search looks for on this page (not shown).
settings-printers-keywords = máy in in máy quét printer print scanner

# Help text at the top of the page.
settings-printers-help = Máy in trên mạng của bạn tự được tìm thấy. Bấm Thêm bên cạnh một máy để dùng.

# Headline of the page when printing is not installed.
settings-printers-top-not-installed = Máy tính này chưa cài tính năng in.

# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Đang xem…

# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Dịch vụ in không phản hồi, nên lúc này chưa in được gì.

# Heading of the card shown when printing is not installed.
settings-printers-printing = In ấn

# Help text (i button) of that card.
settings-printers-printing-about = In ấn cần CUPS, mà máy tính này chưa có. Khi đã cài, máy in trên mạng của bạn sẽ tự được tìm thấy.

# Line in that card.
settings-printers-unavailable = Chưa dùng được tính năng in

# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Hãy cài bằng lệnh: sudo apt install cups

# Line in that card.
settings-printers-then-found = Sau đó sẽ tìm thấy máy in

# Small line under it.
settings-printers-then-found-sub = Những máy trên mạng của bạn, tự động.

# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Dịch vụ in.

# Line in the Printers card when there are none.
settings-printers-none = Chưa có máy in nào

# Small line under "No printers yet".
settings-printers-none-sub = Hãy thêm một máy bên cạnh.

# Heading of the card that lists the printers.
settings-printers-card = Máy in

# Help text (i button) of the Printers card.
# SAFETY: needs a human review before it ships
settings-printers-card-about = Mọi máy in đã thiết lập trên máy tính này. Ngôi sao đặt một máy làm máy in trước; biểu tượng trang in một trang thử trên máy đó; thùng rác gỡ nó đi, hỏi hai lần. Việc thay đổi cần tài khoản của bạn thuộc nhóm lpadmin.

# Red words under a page range box that cannot be used.
printers-bad-range = Các trang như 1-3, 5

# Red words under a copies box that cannot be used.
printers-bad-copies = Số bản: 1 đến 99

# Name of the choice to print on one side of the paper.
printers-sides-one = Một mặt

# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = Hai mặt, lật cạnh dài

# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = Hai mặt, lật cạnh ngắn

# Name of the A4 paper size.
printers-paper-a4 = A4

# Name of the A3 paper size.
printers-paper-a3 = A3

# Name of the A5 paper size.
printers-paper-a5 = A5

# Name of the US Letter paper size.
printers-paper-letter = Letter

# Name of the US Legal paper size.
printers-paper-legal = Legal

# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = Dọc

# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Ngang

# Red words when a printing error is a refusal because the account is not in the lpadmin group.
# SAFETY: needs a human review before it ships
printers-not-allowed = Không được phép. Tài khoản của bạn cần thuộc nhóm lpadmin; hãy đăng xuất rồi đăng nhập lại khi đã thuộc.

# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS từ chối.

# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = CUPS không chấp nhận tên máy in đó.

# Red words when printing is not installed on the computer.
printers-not-installed = Máy tính này chưa cài tính năng in.

# Red words when the printer chosen has been removed since.
printers-gone = { $name } không còn được thiết lập.

# Red words when the printer chosen is paused.
printers-paused = { $name } đang tạm dừng.

# Red words when sending the document to CUPS took too long.
printers-too-slow = Gửi tài liệu mất quá nhiều thời gian.
