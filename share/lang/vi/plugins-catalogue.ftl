
# machine translated (Vietnamese), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Cập nhật plugin đang tạm dừng vì danh sách plugin bị hỏng.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = Bắt đầu lại

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle không gỡ được bản ghi: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Một tên có chứa byte null.

# Error text when there is no record name to take away.
plugins-start-no-note = Không có bản ghi nào để gỡ.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Trindle không kiểm tra được danh mục plugin vì thiếu khoá xác minh.

# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Danh mục plugin của Trindle không có chữ ký, nên Trindle không dùng nó.

# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Chữ ký trên danh mục plugin của Trindle có kích thước sai, nên Trindle không dùng danh mục này.

# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Danh mục plugin của Trindle không được ký bằng khoá của Trindle, nên Trindle không dùng nó.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Danh mục plugin của Trindle không có số phiên bản, nên Trindle không dùng nó.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle không dùng danh mục plugin này vì số phiên bản của nó, { $is }, cũ hơn số đã nhận trước đó, { $after }.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Danh mục plugin của Trindle đã được ký nhưng không đọc được, nên Trindle không dùng nó: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Danh mục tốt gần nhất đang được hiện.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Đây không phải văn bản.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Danh mục quá lớn.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Đây không phải danh mục plugin, vì { $file } không có phần [Catalogue] với Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Danh mục này dùng định dạng mới hơn định dạng mà phiên bản Trindle này đọc được. Trindle hỗ trợ Format={ $format }.

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } bị bỏ qua: { $why ->
        [id] id của nó không chỉ gồm chữ cái, chữ số, dấu chấm và dấu gạch ngang
        [needs] nó cần có Name và Version
        [archive] Archive của nó không phải tên một tệp .tar.zst nằm cạnh danh mục
        [hash] SHA256 của nó không phải 64 chữ số thập lục phân
        [permission] nó xin một quyền không tồn tại
       *[icon] Icon của nó không phải tên một tệp SVG hoặc PNG nằm cạnh danh mục
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar không khởi động được: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle không giải nén được tệp nén: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = không rõ lý do

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = Tệp nén không có { $file } ở cấp trên cùng, hoặc trong thư mục duy nhất ở cấp trên cùng.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle không tạo được thư mục tạm: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl không khởi động được: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Không tải về được: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = Tệp đã tải về không khớp mã kiểm tra (SHA-256) của danh mục, nên Trindle chưa cài gì.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = Trindle chưa cài gì vì plugin không qua được các bước kiểm tra: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = Tệp nén chứa { $found }, không phải { $wanted }, nên Trindle chưa cài gì.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle đã cài plugin nhưng không ghi lại được rằng nó đến từ danh mục của Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Plugin chưa được cài, nên không có gì để cập nhật.
