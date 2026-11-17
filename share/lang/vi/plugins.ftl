
# machine translated (Vietnamese), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle không đọc được tệp { $file } ở ngoài cùng thư mục ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } không có phần [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Phiên bản Trindle này hỗ trợ Interface={ $known }, không phải Interface={ $n }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] cần có Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] cần có Id gồm chữ cái, chữ số, dấu chấm và dấu gạch ngang, ví dụ org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] cần có Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] cần có Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = Summary trong [Plugin] chỉ gồm một dòng.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
       *[other] [{ $group }] { $key } phải là văn bản thuần trên một dòng, tối đa { $max } ký tự.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = Quyền { $name } không tồn tại. Các quyền hiện có là { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] cần có Icon: một tệp SVG hoặc PNG trong thư mục của plugin.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = Icon trong [Ring] nêu { $icon }, nhưng tệp này không có trong thư mục của plugin.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] cần có Label, hiện ra khi con trỏ dừng trên nó.

# Red words when the [Ring] part has no Run.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] cần có Fields, tức là tên các trường của nó.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = Tên trường { $name } chỉ được chứa chữ cái, chữ số, dấu gạch nối và dấu gạch dưới.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = Trường { $name } không có phần [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] cần có Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
       *[other] [{ $group }] có thể có tối đa { $most } Choices. Mỗi mục phải là văn bản thuần, tối đa { $chars } ký tự.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom phải nêu tên một tệp trong thư mục của plugin. Chỉ dùng chữ cái, chữ số, dấu chấm, dấu gạch ngang và dấu gạch dưới. Đừng bắt đầu tên bằng dấu chấm và đừng thêm đường dẫn thư mục.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] là một lựa chọn, nên cần Choices có từ hai mục trở lên, hoặc ChoicesFrom trỏ tới một tệp.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default không nằm trong các Choices của nó.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] là một ghi chú, nên cần TextFrom nêu tên một tệp trong thư mục của plugin. Chỉ dùng chữ cái, chữ số, dấu chấm, dấu gạch ngang và dấu gạch dưới. Đừng bắt đầu tên bằng dấu chấm và đừng thêm đường dẫn thư mục.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] là một ghi chú, nên không lưu thiết lập nào và không thể có Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind là { $kind }, nhưng phải là switch, text, choice hoặc note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] là một công tắc, nên Default của nó phải là true hoặc false.

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] cần quyền background trong Permissions của [Plugin].

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = Plugin không kết nối với gì cả. Hãy thêm [Ring], [Settings], [Background], [Shortcut] hoặc quyền thông báo.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } là một liên kết trỏ ra ngoài thư mục của plugin.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run không đọc được như một lệnh.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run phải nêu một chương trình nằm trong thư mục của plugin, không phải { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run nêu { $program }, nhưng plugin không chứa chương trình đó (thiếu, hoặc không chạy được).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
       *[other] Menu trong [Ring] có { $n } mục. Chỉ được tối đa { $max }.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = ID của mục menu { $id } chỉ được chứa chữ cái, chữ số, dấu gạch nối và dấu gạch dưới.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = Menu trong [Ring] nêu { $id } hai lần.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = Menu trong [Ring] nêu { $id }, nhưng mục này không có phần [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
       *[other] [{ $group }] Label phải là một dòng ngắn, tối đa { $max } ký tự.
    }

# Red words when a menu item has no Run. { $group } is the part.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] chỉ dành cho Flow trong danh mục của Trindle, plugin này xin quyền control.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] cần có Run, tức là lệnh nhận mã PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
       *[other] [{ $group }] ShowWhen có thể dài tối đa { $max } ký tự.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen phải có dạng field=value, ví dụ ShowWhen=mode=home. Hãy nêu một trường khác và giá trị mà trường đó phải có.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen phải viết dạng field=value, và tên trường chỉ được có chữ cái, chữ số, dấu gạch ngang và dấu gạch dưới.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
       *[other] [{ $group }] ShowWhen cần một giá trị là văn bản thuần trên một dòng, tối đa { $max } ký tự.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen nêu chính trường của nó. Nó phải nêu một trường khác, đứng trước.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen nêu trường { $other }, nhưng trường này không có trong Fields của plugin.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen nêu trường { $other }, đứng sau nó trong Fields. Phải là một trường đứng trước.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen nêu trường { $other }, trường này cũng có ShowWhen riêng. Một trường chỉ được phụ thuộc vào trường không có ShowWhen.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen nêu công tắc { $other }, nên giá trị của nó phải là true hoặc false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen có giá trị { $value }, không nằm trong các Choices của { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen nhắc tới ghi chú { $other }, vốn không lưu thiết lập nào. Hãy dùng công tắc, lựa chọn hoặc ô văn bản.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen cần quyền network trong Permissions của [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen cần quyền listen trong Permissions của [Plugin].

# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = Quyền listen cần có [Network] Listen để liệt kê các cổng mà plugin có thể nhận kết nối.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = plugin.ini của nó dùng một interface mà phiên bản Trindle này không hỗ trợ.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = Nó không liệt kê cổng nào để lắng nghe.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = Không đọc được plugin.ini của nó.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = Nó chưa được cho phép lắng nghe.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = Các cổng được liệt kê đã thay đổi. Bạn cần cho phép kết nối qua các cổng đó lại.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none =
    { $n ->
       *[other] { $name } cần sự cho phép của bạn trước khi hoạt động được. Bạn chưa cho phép quyền nào trong { $n } quyền nó xin.
    }

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = Bạn mới cho phép { $a } trong { $n } quyền mà { $name } xin.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] Nó không xin gì cả.
       *[other] Cả { $n } quyền nó xin đều đã được cho phép.
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = Cho phép

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
# SAFETY: needs a human review before it ships
plugins-other-ports = được kết nối qua các cổng khác

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Chưa được cài.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = một plugin khác

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Plugin đó chưa được cài.

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = Không lưu được: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Không chép được các tệp của plugin.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } Plugin không chạy ở chế độ nền: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle không tìm thấy trindle-plugin để khởi động nó.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
       *[other] nó không khởi động được trong { $n } giây.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = không có phiên desktop nào đang chạy để nó tham gia, nên nó chưa được khởi động. Hãy đăng nhập trước.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = không rõ nên tham gia phiên desktop nào ({ $sessions }), nên nó chưa được khởi động. Hãy chạy lại với WAYLAND_DISPLAY đặt đúng phiên.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = có một thư mục ở chỗ lẽ ra là tệp

# Error text when a plugin's id is not an id.
plugins-not-an-id = không phải ID hợp lệ

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
       *[other] Shortcuts trong [Plugin] có { $n } phím tắt. Chỉ được tối đa { $max }.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = ID của phím tắt { $id } chỉ được chứa chữ cái, chữ số, dấu gạch nối và dấu gạch dưới.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts trong [Plugin] nêu { $id } hai lần.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts trong [Plugin] nêu { $id }, nhưng phím tắt này không có phần [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys } đã là một phím tắt của Trindle. Hãy chọn phím khác.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys } đã được một phím tắt khác trong plugin này dùng.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] cần có Keys, ví dụ Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = Tổ hợp phím đó không phải phím tắt hợp lệ. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } đã là phím tắt của Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } đã là một phím tắt của riêng bạn.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } đã được { $holder } dùng.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Đó không phải phím tắt của plugin đã cài.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle không áp dụng được ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Trindle không lưu được vì { $why }. Chưa có gì thay đổi.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Không lưu được: { $why }.

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = Không đóng được các cổng của nó: { $why }.

# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle không mở được các cổng của nó, nên tính năng lắng nghe đã tắt lại: { $why }.

# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = Các cổng của nó không được mở ({ $why }), và cũng không tắt lại được tính năng lắng nghe ({ $kept }). Không có gì đang mở. Hãy tắt nó trong Cài đặt, Quyền.


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = Chỉ trình đọc màn hình của Trindle trong danh mục mới dùng được quyền trợ năng.

# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = Quyền { $name } bị nêu hai lần.

# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = Hãy xin { $home } hoặc network, đừng xin cả hai: network đã vào được mạng cục bộ lẫn internet.

# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = [{ $group }] cần có Run, tức là lệnh để chạy.

# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } Cũng không khôi phục lại được phiên bản cũ: { $back } Cả hai đều được giữ trong thư mục của plugin.
