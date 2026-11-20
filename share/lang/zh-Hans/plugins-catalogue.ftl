# machine translated (Simplified Chinese), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = 插件更新已暂停，因为插件列表已损坏。

# SAFETY: needs a human review before it ships
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = 重新开始

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle 无法清除该记录：{ $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = 名称中含有空字节。

# Error text when there is no record name to take away.
plugins-start-no-note = 没有可清除的记录。

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Trindle 缺少验证密钥，所以无法检查插件目录。

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Trindle 的插件目录没有签名，所以 Trindle 没有使用它。

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Trindle 插件目录上的签名长度不对，所以 Trindle 没有使用该目录。

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Trindle 的插件目录不是用 Trindle 的密钥签名的，所以 Trindle 没有使用它。

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle 的插件目录没有版本号，所以 Trindle 没有使用它。

# Red line under Trindle's own plugins when the catalogue is older than one accepted before.
# SAFETY: needs a human review before it ships
plugins-refused-older = Trindle 没有使用这份插件目录，因为它比之前用过的目录更旧。

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle 的插件目录已签名但无法读取，所以 Trindle 没有使用它：{ $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } 当前显示的是上一份完好的目录。

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = 它不是文本。

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = 这份目录太大。

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = 它不是插件目录，因为 { $file } 没有带 Format 的 [Catalogue] 部分。

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = 这份目录使用的格式比这个版本的 Trindle 能读取的更新。Trindle 支持 Format={ $format }。

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } 被略过：{ $why ->
        [id] 它的 id 不是由字母、数字、点和短横线组成的
        [needs] 它需要 Name 和 Version
        [archive] 它的 Archive 不是目录旁边某个 .tar.zst 文件的名称
        [hash] 它的 SHA256 不是 64 位十六进制数字
        [permission] 它请求了一个不存在的权限
       *[icon] 它的 Icon 不是目录旁边某个 SVG 或 PNG 文件的名称
    }。

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar 没有启动：{ $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle 无法解压归档：{ $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = 原因不明

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = 归档的顶层，或顶层唯一的文件夹里，没有 { $file }。

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle 无法创建临时文件夹：{ $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl 没有启动：{ $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = 无法下载：{ $why }

# SAFETY: needs a human review before it ships
# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = 下载的文件与目录中的校验值（SHA-256）不一致，所以 Trindle 没有安装任何内容。

# SAFETY: needs a human review before it ships
# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = 由于插件没有通过检查，Trindle 没有安装任何内容：{ $why }

# SAFETY: needs a human review before it ships
# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = 归档里是 { $found }，不是 { $wanted }，所以 Trindle 没有安装任何内容。

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle 已安装该插件，但无法记录它来自 Trindle 目录：{ $why }。

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = 它没有安装，所以没有可更新的内容。
