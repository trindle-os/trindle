
# machine translated (Vietnamese), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Cập nhật plugin đã tạm dừng: danh sách plugin bị hỏng.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = Bắt đầu lại

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Không gỡ được bản ghi: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Một tên có chứa ký tự nul.

# Error text when there is no record name to take away.
plugins-start-no-note = Không có ghi chú nào để gỡ.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Trindle này không có khoá để kiểm tra danh mục plugin, nên không nhận danh mục nào.

# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Danh mục plugin của Trindle không có chữ ký, nên không được dùng.

# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Chữ ký của danh mục plugin của Trindle không đúng kích cỡ của một chữ ký, nên danh mục không được dùng.

# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Danh mục plugin của Trindle không được ký bằng khoá của Trindle, nên không được dùng.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Danh mục plugin của Trindle không cho biết số thứ tự của nó, nên không được dùng.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Danh mục plugin của Trindle cũ hơn danh mục đã nhận trước đó (số { $is }, sau số { $after }), nên không được dùng.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Danh mục plugin của Trindle đã được ký nhưng không đọc được, nên không được dùng: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Danh mục tốt gần nhất đang được hiện.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Đây không phải văn bản.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Danh mục quá lớn so với một danh mục thật.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Đây không phải danh mục plugin: { $file } không có phần [Catalogue] với Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Danh mục thuộc kiểu mới hơn kiểu mà Trindle này đọc được (Format={ $format }).

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } bị bỏ qua: { $why ->
        [id] id của nó không phải chữ cái, chữ số, dấu chấm và dấu gạch ngang
        [needs] nó cần có Name và Version
        [archive] Archive của nó không phải tên một tệp .tar.zst nằm cạnh danh mục
        [hash] SHA256 của nó không phải 64 chữ số thập lục phân
        [permission] nó xin một quyền không tồn tại
       *[icon] Icon của nó không phải tên một tệp SVG hoặc PNG nằm cạnh danh mục
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = Không khởi động được tar: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Không giải nén được tệp nén: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = tar từ chối

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = Tệp nén không có { $file } ở ngoài cùng, hay trong thư mục duy nhất ở ngoài cùng.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Không tạo được thư mục tạm: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = Không khởi động được curl: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Không tải về được: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = Tệp này không phải tệp mà danh mục liệt kê (SHA-256 khác), nên chưa cài gì.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = Plugin không qua được bước kiểm tra plugin của Trindle, nên chưa cài gì: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = Tệp nén chứa { $found }, không phải { $wanted }, nên chưa cài gì.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Plugin đã được cài, nhưng không ghi được ghi chú rằng nó đến từ danh mục của Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Plugin chưa được cài, nên không có gì để cập nhật.
