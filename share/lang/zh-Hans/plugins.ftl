# machine translated (Simplified Chinese), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = 文件夹顶层没有可读取的 { $file }（{ $why }）。

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } 没有 [Plugin] 部分。

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } 这个 Trindle 不认识：它认识 Interface={ $known }。

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] 需要 Interface={ $known }。

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] 需要一个由字母、数字、点和短横线组成的 Id，例如 org.example.Weather。

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] 需要 Name。

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] 需要 Version。

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] 的 Summary 只能是一行。

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
       *[other] [{ $group }] 的 { $key } 必须是单行的普通文字，最多 { $max } 个字符。
    }

# SAFETY: needs a human review before it ships
# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = 权限 { $name } 不存在，现有的权限有：{ $list }。

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] 需要 Icon：插件文件夹里的一个 SVG 或 PNG 文件。

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] 的 Icon 指向 { $icon }，但它不在插件的文件夹里。

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] 需要 Label，指针停在上面时会显示。

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] 需要 Run，即点击后启动的内容。

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] 需要 Fields，即各字段的名称。

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = 字段名称 { $name } 只能包含字母、数字、短横线和下划线。

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = 字段 { $name } 没有 [{ $group }] 部分。

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] 需要 Label。

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
       *[other] [{ $group }] 的 Choices 最多 { $most } 个，每个是最多 { $chars } 个字符的普通文字。
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] 的 ChoicesFrom 是插件主目录里的一个文件名：只能含字母、数字、点、短横线和下划线，不能以点开头，不能含文件夹。

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] 是选择项，所以需要有两个或更多选项的 Choices，或者用 ChoicesFrom 指定一个文件。

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] 的 Default 不是它的 Choices 之一。

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] 是说明文字，所以需要 TextFrom，即插件主目录里的一个文件名：只能含字母、数字、点、短横线和下划线，不能以点开头，不能含文件夹。

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] 是说明文字，不保存任何设置，所以没有 Default。

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] 的 Kind 是 { $kind }：它必须是 switch、text、choice 或 note。

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] 是开关，所以它的 Default 为 true 或 false。

# SAFETY: needs a human review before it ships
# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] 需要在 [Plugin] 的 Permissions 中有 background 权限。

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = 这个插件没有挂接任何内容：请给它 [Ring]、[Settings]、[Background]、[Shortcut]，或通知权限。

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } 链接到了插件文件夹之外。

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] 的 Run 无法作为命令读取。

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] 的 Run 必须指向插件文件夹内的程序，而不是 { $program }。

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] 的 Run 指向 { $program }，但它不是插件自带的程序（不存在，或不可执行）。

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
       *[other] [Ring] 的 Menu 有 { $n } 项：最多允许 { $max } 项。
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = 菜单 id { $id } 只能包含字母、数字、短横线和下划线。

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] 的 Menu 两次列出了 { $id }。

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] 的 Menu 列出了 { $id }，但它没有 [{ $group }] 部分。

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
       *[other] [{ $group }] 的 Label 是最多 { $max } 个字符的一行短文字。
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] 需要 Run，即选择它后启动的内容。

# SAFETY: needs a human review before it ships
# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] 只用于 Trindle 目录中的 Flow，它请求 control 权限。

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] 需要 Run，即接收 PIN 码的命令。

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
       *[other] [{ $group }] 的 ShowWhen 最多 { $max } 个字符。
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] 的 ShowWhen 写成 field=value，例如 ShowWhen=mode=home：另一个字段，加上它必须持有的那个值。

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] 的 ShowWhen 写成 field=value，其中字段名称只能包含字母、数字、短横线和下划线。

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
       *[other] [{ $group }] 的 ShowWhen 需要一个单行的普通文字值，最多 { $max } 个字符。
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] 的 ShowWhen 指向了它自己的字段：它必须指向另一个更靠前的字段。

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] 的 ShowWhen 指向字段 { $other }，但它不在这个插件的 Fields 中。

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] 的 ShowWhen 指向字段 { $other }，但它在 Fields 中排在后面：必须是更靠前的字段。

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] 的 ShowWhen 指向字段 { $other }，而它自己也有 ShowWhen：字段只能依赖没有 ShowWhen 的字段。

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] 的 ShowWhen 指向开关 { $other }，所以它的值是 true 或 false。

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] 的 ShowWhen 的值是 { $value }，它不是 { $other } 的 Choices 之一。

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] 的 ShowWhen 指向说明文字 { $other }，它不保存任何设置：字段只能依赖开关、选择项或文本。

# SAFETY: needs a human review before it ships
# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [网络] 监听：{ $why }

# SAFETY: needs a human review before it ships
# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen 需要在 [Plugin] 的 Permissions 中有 network 权限。

# SAFETY: needs a human review before it ships
# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen 需要在 [Plugin] 的 Permissions 中有 listen 权限。

# SAFETY: needs a human review before it ships
# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = listen 权限需要 [Network] Listen，即允许进入的端口。

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = 它的 plugin.ini 不是针对这个 Trindle 认识的接口。

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = 它没有指定要监听的端口。

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = 无法读取它的 plugin.ini。

# SAFETY: needs a human review before it ships
# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = 它还没有被允许监听。

# SAFETY: needs a human review before it ships
# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = 它指定的端口与之前允许的不一致。必须重新允许。

# SAFETY: needs a human review before it ships
# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none = { $name } 需要您的许可才能工作：已允许 0 项，共 { $n } 项。

# SAFETY: needs a human review before it ships
# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = { $name } 只得到了它所请求权限的一部分：已允许 { $a } 项，共 { $n } 项。

# SAFETY: needs a human review before it ships
# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] 它没有请求任何权限。
       *[other] 它请求的全部 { $n } 项都已允许。
    }

# SAFETY: needs a human review before it ships
# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = 允许

# SAFETY: needs a human review before it ships
# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = 通过与以前不同的其他端口被访问

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = 未安装。

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = 另一个插件

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = 那个插件没有安装。

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = 无法保存：{ $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = 无法复制插件的文件。

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } 它的后台部分没有运行：{ $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = 找不到 trindle-plugin，无法启动它。

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
       *[other] 它没有在 { $n } 秒内启动。
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = 没有可加入的桌面会话，所以没有启动。请先登录。

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = 无法判断该加入哪个桌面会话（{ $sessions }），所以没有启动。请把 WAYLAND_DISPLAY 设为正确的会话后再运行。

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = 本应是文件的位置上有一个文件夹

# Error text when a plugin's id is not an id.
plugins-not-an-id = 不是插件的 id

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
       *[other] [Plugin] 的 Shortcuts 有 { $n } 个快捷键：最多允许 { $max } 个。
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = 快捷键 id { $id } 只能包含字母、数字、短横线和下划线。

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] 的 Shortcuts 两次列出了 { $id }。

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] 的 Shortcuts 列出了 { $id }，但它没有 [{ $group }] 部分。

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] 的 Keys { $keys } 已是 Trindle 的快捷键；请选择其他按键。

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] 的 Keys { $keys } 已被该插件的另一个快捷键使用。

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] 需要 Keys，例如 Ctrl+Alt+S。{ $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] 需要 Run，即按键启动的内容。

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = 为它输入的按键不是快捷键。{ $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } 已是 Trindle 的快捷键。

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } 已是您自己的快捷键之一。

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } 已被 { $holder } 使用。

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = 那不是已安装插件的快捷键。

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = 无法启用它（{ $why }）。

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = 无法保存：没有更改任何内容，因为{ $why }。

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = 无法保存（{ $why }）。


# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = 辅助功能权限只供目录中 Trindle 的屏幕阅读器使用。

