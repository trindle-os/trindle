# machine translated (Japanese), not yet reviewed by a person.

# The plugins code: what a plugin's author or the person is told when a plugin's plugin.ini is refused, when its permissions or ports cannot be put to use, and the words of its shortcuts. Names of parts and keys of plugin.ini, such as [Ring] and Label, stay as they are in every language.

## The check of a plugin.ini (src/plugins/mod.rs)

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle はフォルダの先頭にある { $file } を読み込めません（{ $why }）。
# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } に [Plugin] の部分がありません。
# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = この版の Trindle は Interface={ $known } に対応しています。Interface={ $n } には対応していません。
# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] には Interface={ $known } が必要です。
# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] には、英字、数字、ドット、ハイフンでできた Id が必要です。例: org.example.Weather。
# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] には Name が必要です。
# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] には Version が必要です。
# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] の Summary は 1 行にしてください。
# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
       *[other] [{ $group }] の { $key } は 1 行の普通の文章にし、{ $max } 文字以内にしてください。
    }
# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = 権限 { $name } が 2 回書かれています。
# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = { $home } か network のどちらか一方だけを求めてください。network はインターネットだけでなく、ローカルネットワークにも届きます。
# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = 権限 { $name } はありません。使える権限は { $list } です。
# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] には Icon が必要です。プラグインのフォルダにある SVG か PNG のファイルを指定してください。
# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] の Icon は { $icon } を指していますが、プラグインのフォルダにありません。
# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] には Label が必要です。ポインターを重ねたときに表示されます。
# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = [{ $group }] には、実行するコマンドを示す Run が必要です。
# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] には、項目名を示す Fields が必要です。
# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = 項目名 { $name } に使えるのは、英字、数字、ハイフン、アンダースコアだけです。
# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = 項目 { $name } に [{ $group }] の部分がありません。
# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] には Label が必要です。
# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
       *[other] [{ $group }] の Choices は最大 { $most } 個です。1 つにつき { $chars } 文字以内の普通の文章にしてください。
    }
# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] の ChoicesFrom には、プラグインのフォルダにあるファイルを指定してください。使えるのは英字、数字、ドット、ハイフン、アンダースコアだけです。名前をドットで始めたり、フォルダの道筋を含めたりしないでください。
# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] は選択式なので、2 つ以上の選択肢がある Choices か、ファイルを指す ChoicesFrom が必要です。
# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] の Default が Choices のどれでもありません。
# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] はメモなので、プラグインのフォルダにあるファイルを指す TextFrom が必要です。使えるのは英字、数字、ドット、ハイフン、アンダースコアだけです。名前をドットで始めたり、フォルダの道筋を含めたりしないでください。
# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] はメモなので、設定を保存しません。Default は付けられません。
# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] の Kind が { $kind } になっていますが、switch、text、choice、note のどれかにしてください。
# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] はスイッチなので、Default は true か false にしてください。
# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] を使うには、[Plugin] の Permissions に background 権限が必要です。
# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = このプラグインは何にもつながっていません。[Ring]、[Settings]、[Background]、[Shortcut]、または通知の権限のどれかを付けてください。
# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } は、プラグインのフォルダの外を指すリンクです。

## A Run line (src/plugins/mod.rs)

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] の Run をコマンドとして読み取れません。
# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] の Run には、{ $program } ではなく、プラグインのフォルダ内のプログラムを指定してください。
# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] の Run は { $program } を指していますが、プラグインにそのプログラムがないか、実行できません。

## Ring menu items (src/plugins/mod.rs)

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
       *[other] [Ring] の Menu に項目が { $n } 個あります。最大 { $max } 個までです。
    }
# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = メニュー ID { $id } に使えるのは、英字、数字、ハイフン、アンダースコアだけです。
# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] の Menu に { $id } が 2 回書かれています。
# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] の Menu に { $id } がありますが、[{ $group }] の部分がありません。
# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
       *[other] [{ $group }] の Label は、{ $max } 文字以内の短い 1 行にしてください。
    }

## The PIN part (src/plugins/mod.rs)

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] は、Trindle のカタログにある Flow だけが使えます。Flow は control 権限を求めます。
# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = accessibility 権限を使えるのは、カタログにある Trindle のスクリーンリーダーだけです。
# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] には、PIN を受け取るコマンドを示す Run が必要です。

## Settings fields shown only when another field holds a value (src/plugins/mod.rs)

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
       *[other] [{ $group }] の ShowWhen は { $max } 文字以内にしてください。
    }
# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] の ShowWhen は 項目名=値 の形で書いてください。例: ShowWhen=mode=home。別の項目と、その項目が持つべき値を指定します。
# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] の ShowWhen は 項目名=値 の形で書いてください。項目名に使えるのは、英字、数字、ハイフン、アンダースコアだけです。
# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
       *[other] [{ $group }] の ShowWhen には、{ $max } 文字以内で 1 行の普通の文章の値が必要です。
    }
# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] の ShowWhen が自分自身の項目を指しています。前にある別の項目を指してください。
# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] の ShowWhen が項目 { $other } を指していますが、このプラグインの Fields にありません。
# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] の ShowWhen が項目 { $other } を指していますが、Fields ではそれが後ろにあります。前にある項目を指してください。
# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] の ShowWhen が項目 { $other } を指していますが、それにも ShowWhen があります。項目が頼れるのは、ShowWhen を持たない項目だけです。
# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] の ShowWhen がスイッチ { $other } を指しているので、値は true か false にしてください。
# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] の ShowWhen の値 { $value } は、{ $other } の Choices にありません。
# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] の ShowWhen がメモ { $other } を指していますが、メモは設定を保存しません。スイッチ、選択式、テキストの項目を使ってください。

## Ports a plugin listens on (src/plugins/mod.rs)

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] の Listen: { $why }
# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] の Listen を使うには、[Plugin] の Permissions に network 権限が必要です。
# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] の Listen を使うには、[Plugin] の Permissions に listen 権限が必要です。
# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = listen 権限を使うには、プラグインが接続を受けられるポートを [Network] の Listen に書く必要があります。
# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = この plugin.ini は、この版の Trindle が対応していないインターフェースを使っています。
# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = 待ち受けるポートが書かれていません。
# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = この plugin.ini を読み込めません。
# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = 待ち受けは許可されていません。
# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = ポートの一覧が変わりました。もう一度、接続を許可してください。

## Permissions (src/plugins/mod.rs)

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none =
    { $n ->
       *[other] { $name } は動かす前にあなたの許可が必要です。求めている { $n } 件の権限を、まだどれも許可していません。
    }
# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = { $name } が求めている { $n } 件の権限のうち、{ $a } 件だけを許可しています。
# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] 求めている権限はありません。
       *[other] 求めている { $n } 件の権限は、すべて許可されています。
    }
# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = 許可
# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = 別のポートで接続されること
# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = インストールされていません。
# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = 別のプラグイン
# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = そのプラグインはインストールされていません。
# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = 保存できませんでした: { $why }
# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = プラグインのファイルをコピーできませんでした。
# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } プラグインはバックグラウンドで動いていません: { $not }
# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } 古い版も元に戻せませんでした: { $back } どちらもプラグインのフォルダに残してあります。
# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle は起動に必要な trindle-plugin を見つけられませんでした。
# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
       *[other] { $n } 秒以内に起動しませんでした。
    }
# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = 参加できるデスクトップのセッションがないため、起動しませんでした。先にログインしてください。
# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = どのデスクトップのセッションに参加するか判断できない（{ $sessions }）ため、起動しませんでした。WAYLAND_DISPLAY を正しいものにして、もう一度実行してください。
# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = ファイルがあるべき場所にフォルダがあります
# Error text when a plugin's id is not an id.
plugins-not-an-id = 正しい ID ではありません

## Keyboard shortcuts of plugins (src/plugins/shortcuts.rs)

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
       *[other] [Plugin] の Shortcuts にショートカットが { $n } 個あります。最大 { $max } 個までです。
    }
# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = ショートカット ID { $id } に使えるのは、英字、数字、ハイフン、アンダースコアだけです。
# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] の Shortcuts に { $id } が 2 回書かれています。
# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] の Shortcuts に { $id } がありますが、[{ $group }] の部分がありません。
# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] の Keys { $keys } は、すでに Trindle のショートカットです。別のキーを選んでください。
# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] の Keys { $keys } は、このプラグインの別のショートカットですでに使われています。
# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] には、Ctrl+Alt+S のような Keys が必要です。{ $why }
# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = そのキーの組み合わせはショートカットとして使えません。{ $why }
# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } は、すでに Trindle のショートカットです。
# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } は、すでにあなた自身のショートカットです。
# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } は、すでに { $holder } で使われています。
# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = それは、インストール済みのプラグインのショートカットではありません。
# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle はそれを使えませんでした（{ $why }）。
# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = { $why } ため、Trindle は保存できませんでした。何も変わっていません。
# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = 保存できませんでした: { $why }。

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = ポートを閉じられませんでした: { $why }。
# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle はポートを開けなかったため、待ち受けをもう一度オフにしました: { $why }。
# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = ポートを開けませんでした（{ $why }）。待ち受けをオフに戻すこともできませんでした（{ $kept }）。開いているものはありません。設定の権限でオフにしてください。
