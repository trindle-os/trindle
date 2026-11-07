
# machine translated (Vietnamese), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = vừa xong

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
       *[other] { $n } phút trước
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
       *[other] { $n } giờ trước
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
       *[other] { $n } ngày trước
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = Đang chờ mật khẩu...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Đã bật chặn

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Đã tắt chặn

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Đã lưu

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Hãy nhập tên một trang web, ví dụ example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Tính năng chặn đang tắt. Các truy vấn tên miền đi đến máy chủ riêng của mạng bạn, như trước.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Tính năng chặn đang khởi động.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Quảng cáo và trang web xấu đã bị chặn, và các truy vấn tên miền đi riêng tư đến { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Không thể kết nối máy chủ riêng tư trên mạng này, nên tạm dùng máy chủ riêng của mạng bạn. Tính năng chặn vẫn hoạt động.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Quảng cáo và trang web xấu đã bị chặn. Các truy vấn tên miền đi đến máy chủ riêng của mạng bạn, nơi có thể thấy chúng.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Quảng cáo và trang web xấu đã bị chặn. Các truy vấn tên miền đi đến máy chủ riêng của VPN, qua VPN.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Quảng cáo và trang web xấu đã bị chặn.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Đang khởi động

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Tắt

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = tính năng chặn đang khởi động

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = tính năng chặn đang tắt

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Đã chặn hôm nay

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = truy vấn đã bị chặn hôm nay

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Danh sách

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Đang tải

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Chưa có

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = các danh sách trang web

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = trang web trong danh sách, tải { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Chưa chặn gì

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = Các trang web sẽ hiện ở đây, mới nhất trước.

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
# SAFETY: needs a human review before it ships
settings-blocking-let-through = Luôn cho phép trang web này

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Vừa bị chặn

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Các trang web hiện ở đây khi bị chặn, mới nhất trước. Dấu cộng cho phép một trang từ giờ trở đi. Chỉ các trang bị chặn mới hiện ở đây. Các trang đã được cho phép không bao giờ được lưu lại.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Hãy bật tính năng chặn để xem nó chặn những gì.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = ví dụ example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Thêm một trang web

# Small line under "Add a site".
settings-blocking-add-sub = Trang đó và mọi tên miền con của nó sẽ không bao giờ bị chặn.

# Tip of the minus button on a site that is let through: blocks it again.
# SAFETY: needs a human review before it ships
settings-blocking-block-again = Chặn lại trang web này

# Heading of the card that lists the sites that are let through.
# SAFETY: needs a human review before it ships
settings-blocking-allowed = Luôn cho phép

# Help text (i button) of the Always let through card.
# SAFETY: needs a human review before it ships
settings-blocking-allowed-about = Các trang web trong danh sách này, cùng mọi tên miền con, không bao giờ bị chặn, kể cả khi chúng nằm trong các danh sách chặn. Dấu trừ sẽ chặn lại một trang.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Chặn

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = chặn quảng cáo theo dõi mã độc lừa đảo dns riêng tư truy vấn quad9 mullvad cloudflare cho phép block adverts ads trackers

# Help text at the top of the page.
settings-blocking-help = Chặn quảng cáo, trình theo dõi và các trang web xấu đã biết cho mọi trình duyệt và ứng dụng trên máy tính này, và gửi các truy vấn tên miền qua kết nối mã hóa, để mạng bạn đang dùng không thể đọc hay thay đổi chúng.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Chặn quảng cáo và trang web xấu

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Nơi các truy vấn được gửi đến

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Dùng các danh sách quảng cáo, trình theo dõi và trang web xấu của HaGeZi, tải mỗi ngày một lần. Khi tắt, không có gì chạy. Các truy vấn tên miền đi qua kết nối mã hóa đến máy chủ riêng tư được chọn ở đây. Quad9 không lưu lại ai đã hỏi. "Mạng của bạn" vẫn chặn, nhưng không riêng tư: máy chủ riêng của mạng bạn thấy mọi truy vấn.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = Danh sách của HaGeZi, tải mỗi ngày một lần.

# Heading of the line with the private server dropdown.
settings-blocking-private = Máy chủ riêng tư

# Small line under Private server.
settings-blocking-private-sub = Quad9 không lưu lại ai đã hỏi.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Mạng của bạn

