
# machine translated (Vietnamese), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } đang bị tắt bằng công tắc hoặc phím trên chính máy tính. Hãy bật lên, rồi thử lại.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = Hệ thống đang chặn { $radio }, và Trindle không bỏ chặn được.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Mật khẩu đó không được chấp nhận. Hãy thử lại.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Mật khẩu đã lưu cho mạng này không được chấp nhận. Hãy dùng nút thùng rác để quên mạng, rồi kết nối lại.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle không tìm thấy mạng đó. Hãy kiểm tra tên mạng và xem mạng có trong tầm phủ sóng không.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Máy tính này đã vào được mạng nhưng không được cấp địa chỉ. Hãy thử lại, hoặc khởi động lại bộ định tuyến.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Máy tính này không kết nối được vào mạng. Hãy lại gần bộ định tuyến hơn, rồi thử lại.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Đó không phải mật khẩu Wi-Fi hợp lệ. Hãy kiểm tra, rồi thử lại.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Không thể kết nối vào mạng.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = mở bằng mật khẩu của bạn

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = mở và sửa bằng mật khẩu của bạn

# Small line of a shared folder: anyone on the local network may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = ai trong mạng cục bộ cũng có thể mở

# Small line of a shared folder: anyone on the local network may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = ai trong mạng cục bộ cũng có thể sửa

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = Ngừng chia sẻ { $name }

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Không được quản lý

# State of a network device that cannot be used.
settings-network-state-unavailable = Không khả dụng

# State of a network device that is not connected.
settings-network-state-disconnected = Đã ngắt kết nối

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Đang chuẩn bị

# State of a network device that is being set up.
settings-network-state-configuring = Đang thiết lập

# State of a network device that needs a password.
# SAFETY: needs a human review before it ships
settings-network-state-needs-password = Cần mật khẩu

# State of a network device that is getting an address.
settings-network-state-address = Đang nhận địa chỉ

# State of a network device whose connection is being checked.
settings-network-state-checking = Đang kiểm tra

# State of a network device that is waiting.
settings-network-state-waiting = Đang chờ

# State of a network device that is connected.
settings-network-state-connected = Đã kết nối

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Đang ngắt kết nối

# State of a network device that failed.
settings-network-state-failed = Thất bại

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Đã kết nối bằng dây cáp, và với { $name } qua Wi-Fi.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Đã kết nối với { $name }. Tín hiệu mạnh.
        [good] Đã kết nối với { $name }. Tín hiệu tốt.
        [fair] Đã kết nối với { $name }. Tín hiệu khá.
       *[other] Đã kết nối với { $name }. Tín hiệu yếu.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Đã kết nối bằng dây cáp.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi đang tắt, nên máy tính này chưa kết nối.

# Headline of the page when not connected.
settings-network-top-none = Chưa kết nối. Hãy chọn một mạng bên dưới.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Đang tìm mạng…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = Trindle không hiện được các mạng vì NetworkManager không chạy.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Bus hệ thống

# Name of the page in the list of settings pages and its heading.
settings-network-title = Mạng

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi không dây internet ethernet có dây mật khẩu kết nối tường lửa chia sẻ máy in thư mục mạng

# Help text at the top of the page.
settings-network-help = Bật hoặc tắt Wi-Fi, kết nối một mạng, quên mạng bạn không còn cần, hoặc cho phép các máy tính khác kết nối.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Tín hiệu mạnh từ { $name }.
        [good] Tín hiệu tốt từ { $name }.
        [fair] Tín hiệu khá từ { $name }.
       *[other] Tín hiệu yếu từ { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Wi-Fi cần một bản Linux mới hơn.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Máy tính này không có Wi-Fi hoạt động được.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Đang tắt. Hãy bật để kết nối một mạng.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Đang bật, nhưng chưa vào mạng nào.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Dùng Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi và dây cáp

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi nối máy tính này với một mạng mà không cần dây cáp. Nếu tắt, máy ngừng tìm mạng và tiết kiệm một chút pin. Mạng ẩn tên sẽ không bao giờ hiện trong danh sách mạng trong tầm. Để kết nối với mạng đó, hãy gõ đúng tên của nó, kể cả chữ hoa. Một số chip Wi-Fi cần trình điều khiển mà bản Linux Trindle đang dùng chưa có. Với những chip đó, bạn cần một bản Linux mới hơn. Dây cáp mạng có một dòng riêng, với nút để kết nối hoặc ngắt kết nối.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Cho phép các máy tính khác kết nối

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Đang tìm các thư mục được chia sẻ…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Không đọc được các thư mục được chia sẻ

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Chưa truy cập được

# Small line under it.
settings-network-not-reachable-sub = Các máy tính khác không thể mở chúng khi chia sẻ đang tắt.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Không có thư mục nào được chia sẻ

# Heading of the Sharing card.
settings-network-sharing = Chia sẻ

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Khi Cho phép các máy tính khác kết nối đang tắt, các máy tính khác trên mạng chỉ truy cập được máy này nếu máy này liên lạc với chúng trước. Nhờ vậy không ai dùng chung Wi-Fi có thể thử xâm nhập. Hãy bật lên để chia sẻ thư mục hoặc máy in với các máy tính khác ở nhà. Để chia sẻ một thư mục, bấm chuột phải vào nó trong Folders và chọn Chia sẻ trên mạng. Mỗi thư mục được chia sẻ cho biết ai được làm gì. Bằng mật khẩu của bạn: người khác có thể mở các tệp, hoặc mở và sửa chúng, nếu họ có mật khẩu chia sẻ của bạn. Ai ở nhà cũng được: bất kỳ ai trên mạng có thể mở các tệp, hoặc mở và sửa chúng, mà không cần mật khẩu. Khi công tắc đang tắt, các máy tính khác ở nhà không thể mở bất kỳ mục nào trong số đó.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Đang ngừng…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Đang chờ mật khẩu của bạn…

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Đang bật chia sẻ

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Đã chặn

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Không thể bật Wi-Fi.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Không thể tắt Wi-Fi.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Ngắt kết nối

# Tip of the button that connects a network or a cable.
settings-network-connect = Kết nối

# Shown under a network or cable while it is being connected.
settings-network-connecting = Đang kết nối…

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Không có mạng nào trong tầm

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Mạng trong tầm

# Help text (i button) of the Networks in range card.
settings-network-in-range-about = Mọi mạng Wi-Fi mà máy tính này bắt được. Khi bạn mở trang, mạng đang dùng đứng đầu, rồi đến các mạng đã lưu, rồi những mạng còn lại theo độ mạnh tín hiệu. Thứ tự không đổi khi trang đang mở. Bấm mũi tên để kết nối một mạng. Mạng có bảo mật sẽ hỏi mật khẩu lần đầu. Nút thùng rác sẽ quên một mạng đã lưu cùng mật khẩu của nó. Mạng mở không được mã hóa, nên người ở gần có thể thấy những gì bạn gửi qua nó.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Hãy bật Wi-Fi để xem các mạng trong tầm.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Đã lưu

# Part of a network's small line: it needs a password.
settings-network-secured = Có bảo mật

# Part of a network's small line: it has no password and is not encrypted.
# SAFETY: needs a human review before it ships
settings-network-open = Mở, không mã hóa

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = Quên mạng này

# Said in red when a saved network could not be forgotten.
# SAFETY: needs a human review before it ships
settings-network-forget-failed = Không thể quên mạng.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = Mật khẩu

# Tip of the button that joins the network.
settings-network-join = Kết nối

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = Hủy

# Shown under a network while it is being joined.
settings-network-joining = Đang kết nối…

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Mật khẩu Wi-Fi dài ít nhất 8 ký tự.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Kết nối mạng ẩn

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Tên mạng

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Mật khẩu, nếu có

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Hãy gõ tên mạng.

# Shown under a network after it was joined.
settings-network-joined = Đã kết nối

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
# SAFETY: needs a human review before it ships
settings-network-kept-anyway = { $words } Mạng vẫn được lưu. Hãy quên nó bằng nút thùng rác trước khi kết nối lại.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Không ngắt kết nối được.


# Heading of the card that lists the ports opened by hand.
settings-network-ports = Cổng đang mở

# Help text (i button) of the Open ports card.
settings-network-ports-about = Mọi cổng đều đóng trừ khi máy tính này liên lạc với máy kia trước. Chương trình chờ người khác kết nối, như máy chủ trò chơi hoặc công cụ đồng bộ, không thể truy cập được cho đến khi bạn mở cổng của nó ở đây. Cổng mở cho mọi máy tính trong cùng mạng cục bộ với máy này vào, nhưng không bao giờ cho internet rộng lớn. Trên mạng quán cà phê, khách sạn hay văn phòng, điều đó gồm cả mọi người khác ở đó. Để mở hoặc đóng một cổng, bạn cần mật khẩu của quản trị viên.

# Line in the Open ports card when no port is open.
settings-network-ports-none = Chưa mở cổng nào

# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = Đóng { $port }

# Heading of the line that opens a port.
settings-network-port-add = Mở một cổng

# Placeholder in the box where the port number is typed.
settings-network-port-number = Số cổng

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = Giao thức

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = Cả hai

# Button that opens the typed port.
settings-network-port-open = Mở

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = Hãy nhập một số từ 1 đến 65535.

# Red words when the port is already open.
settings-network-port-already = Đã mở rồi.

# Said under the line once the port is open.
settings-network-port-opened = Đã mở

# Part of a network's small line: it signs in with a username as well as a password, as work and university networks do.
settings-network-username = Cần tên người dùng

# Red words when someone tries to join a network that signs in with a username.
settings-network-username-unsupported = Mạng này cần tên người dùng cùng với mật khẩu. Trindle chưa kết nối được những mạng như vậy.
