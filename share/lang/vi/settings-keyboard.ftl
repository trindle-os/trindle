
# machine translated (Vietnamese), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Bàn phím và chuột

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = bố cục ngôn ngữ gõ caps lock bàn phím ảo nút vòng tròn chuột bàn di chuột con trỏ tốc độ chạm nhấp cuộn con lăn chậm hơn nhanh hơn tự nhiên thuận tay trái phím tắt đổi phím ctrl escape super alt mac nút bên chạm giữ chương trình tạo layout language typing on-screen keyboard button ring mouse touchpad trackpad pointer speed tap click scroll wheel slower faster natural left handed plugin shortcut keys remap change swap side buttons tapped held made by programs keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Mọi thứ ở đây đổi ngay lập tức. Bạn có thể chuyển sang bố cục thứ hai bằng cách nhấn Alt và Shift cùng lúc.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Đang gõ bằng { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt và Shift chuyển sang { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock không làm gì.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock đang là { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = một phím Ctrl nữa

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = Máy tính này không có danh sách bố cục bàn phím.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Bàn phím

# Heading of the line in that card.
settings-keyboard-no-list-heading = Không có danh sách bố cục bàn phím

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Để cài, hãy chạy lệnh: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
       *[other] Bạn đã đổi { $n } phím và nút.
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = Bố cục

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = Bố cục

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = Biến thể

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = Chuẩn

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = Không có

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = Chuyển sang

# Words behind the i of the layout card.
settings-keyboard-layout-about = Phần này đặt mỗi phím gõ ra gì. Bạn có thể chọn một bố cục và một biến thể của nó. Nhấn Alt và Shift cùng lúc để chuyển sang bố cục thứ hai, và nhấn lại để chuyển về. Thay đổi có tác dụng ngay lập tức.

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = Đang dùng

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = Một phím Ctrl nữa

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = Không làm gì

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Dùng Caps Lock làm

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Nút bàn phím trong vòng tròn

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Phím khác

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock có thể làm một phím khác, ví dụ phím Ctrl thứ hai hoặc Escape. Nút bàn phím nằm ở giữa vòng tròn và mở một bàn phím trên màn hình. Bạn có thể dùng nó để gõ bằng chuột hoặc màn hình cảm ứng. Nút này hiện ra vào lần mở vòng tròn tiếp theo.

# Words under a line when a press that was not the person's own changed nothing.
# SAFETY: needs a human review before it ships
settings-keyboard-only-yours = Chương trình không đổi được điều này.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Hãy bấm phím hoặc nút chuột cần đổi.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = Bạn đã không bấm gì, nên không thêm gì.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Hãy gõ phím dùng thay thế, như Escape hoặc Ctrl+C, hoặc gõ Nothing.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle không biết đó là phím nào. Hãy chọn phím từ danh sách.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Mọi bàn phím và chuột

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Bạn đã tự đổi phím này ở bên dưới, và thay đổi đó sẽ được dùng.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = Đổi { $key } trên

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = Đổi { $key } thành

# Name of the bin button that takes a change away.
settings-keyboard-take-away = Gỡ thay đổi này

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = Dòng { $n } đã bị bỏ qua: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Không đọc được các phím đã đổi

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Các phím của bạn vẫn hoạt động bình thường cho đến khi Trindle đọc được các thay đổi. { $why }

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Chọn phím hoặc nút cần đổi

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Bấm phím hoặc nút cần đổi

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Đổi một phím hoặc nút

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = Bạn đã đổi { $key } rồi.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Phím và nút

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Phím hoặc nút chuột nào cũng có thể làm việc khác. Nó có thể thành một phím hay nút khác, hoặc các phím nhấn cùng lúc như Ctrl+C, hoặc các phím nhấn lần lượt như H, I. Nó cũng có thể không làm gì, hoặc làm một việc khi chạm nhanh và một việc khác khi giữ, như Escape khi chạm và Ctrl khi giữ. Cách này hoạt động ở mọi nơi, kể từ màn hình đăng nhập. Ctrl, Alt và một phím F luôn mở bảng điều khiển văn bản, và gõ trindle-settings --keys default ở đó sẽ đưa mọi phím về như cũ.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Đổi chỗ Ctrl và Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock: chạm là Escape, giữ là Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Đổi chỗ Escape và Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Đổi chỗ Alt và Super, như trên bàn phím Mac

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Các nút bên của chuột dùng để sao chép và dán

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = Tin cậy { $name }

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Chưa có

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Bàn phím và chuột do chương trình tạo

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = Công cụ đổi phím và trình khởi chạy game có thể tạo bàn phím hoặc chuột riêng. Phím và cú nhấp của chúng vẫn gõ và chơi được, nhưng không bấm được các nút được bảo vệ, như Tắt máy hoặc Cho phép, và không trả lời được câu hỏi dành cho bạn. Hãy bật một cái ở đây để cho phép nó được tính là bạn.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Tay cầm { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } trên { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = { $title } làm gì

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = Đổi chỗ A và B

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = Đổi chỗ X và Y

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Đảo hướng lên và xuống của cần điều khiển phải

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Phím tắt của plugin

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } không được dùng: { $why }

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Phím cho { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Đã lưu

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = Bàn di chuột cũng vậy.

# Title of the mouse card.
settings-keyboard-mouse-title = Chuột

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = Tốc độ con trỏ là quãng con trỏ di chuyển khi bạn dịch chuột. Nó áp dụng cho cả bàn di chuột. Tốc độ cuộn là quãng trang di chuyển mỗi lần xoay con lăn. Hãy chọn Chậm hơn, Bình thường hoặc Nhanh hơn, hoặc đặt bất kỳ chỗ nào trên thanh trượt. Với cuộn tự nhiên, trang di chuyển theo hướng con lăn xoay, như trên điện thoại. Thuận tay trái đổi chỗ nút trái và nút phải.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Tốc độ con trỏ

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Tốc độ cuộn

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Cuộn tự nhiên

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Thuận tay trái

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Hỗ trợ nhấp chuột

# Small words under the clicking helper line.
# SAFETY: needs a human review before it ships
settings-keyboard-clicker-about = Các cú nhấp từ bộ nhấp theo thời gian giữ hoặc công tắc được tính là của chính bạn.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Bàn di chuột

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = Tốc độ cuộn là quãng trang di chuyển khi bạn cuộn bằng hai ngón. Chạm để nhấp khiến một cú chạm nhẹ được tính là cú nhấp. Với cuộn tự nhiên, trang di chuyển theo hướng ngón tay bạn di chuyển, như trên điện thoại. Tắt khi gõ giúp lòng bàn tay không làm con trỏ di chuyển khi bạn gõ.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Chạm để nhấp

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Tắt khi gõ

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Chậm hơn

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Bình thường

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Nhanh hơn

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = Cổng USB

# Words behind the i of that card.
settings-keyboard-usb-about = Nếu một cổng USB không khởi động được, Trindle tắt rồi bật nó một lần để thử lại. Trindle không bao giờ làm vậy quá một lần cho mỗi cổng.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle cũng không hoàn tác được thay đổi: { $undo }


# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Chụp màn hình, báo cáo và phím tắt

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Chụp ảnh màn hình

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Hiện danh sách phím tắt

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Chụp ảnh màn hình ngay

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Hiện danh sách ngay

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Khởi động lại màn hình nền

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Khởi động lại vòng tròn và các phần phụ trợ

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Khởi động lại mọi thứ

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Phím cho { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Khởi động lại ngay. Các cửa sổ đang mở vẫn giữ nguyên.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Hỏi trước, rồi khởi động lại mọi thứ

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Đã khởi động lại

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = Một phím tắt khác đã dùng { $keys }.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Chưa lưu. { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Báo cáo sự cố

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Báo cáo sự cố ngay

# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = Chưa lưu. Không đọc được tệp các phím đã đổi, nên tệp được giữ nguyên.
