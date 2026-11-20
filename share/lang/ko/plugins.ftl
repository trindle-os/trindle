# machine translated (Korean), not yet reviewed by a person.

# The plugins code: what a plugin's author or the person is told when a plugin's plugin.ini is refused, when its permissions or ports cannot be put to use, and the words of its shortcuts. Names of parts and keys of plugin.ini, such as [Ring] and Label, stay as they are in every language.

## The check of a plugin.ini (src/plugins/mod.rs)

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Trindle이 폴더 맨 위의 { $file } 파일을 읽을 수 없어요 ({ $why }).
# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file }에 [Plugin] 부분이 없어요.
# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = 이 버전의 Trindle은 Interface={ $known }만 지원해요. Interface={ $n }은 지원하지 않아요.
# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin]에 Interface={ $known }이 필요해요.
# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin]에는 영문자, 숫자, 점, 하이픈으로 된 Id가 필요해요. 예: org.example.Weather.
# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin]에 Name이 필요해요.
# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin]에 Version이 필요해요.
# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = [Plugin] Summary는 한 줄이어야 해요.
# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
       *[other] [{ $group }] { $key }은 한 줄의 일반 텍스트여야 하고, { $max }자를 넘으면 안 돼요.
    }
# Red words when plugin.ini asks for the same permission twice. { $name } is the permission.
plugins-permission-twice = { $name } 권한이 두 번 적혀 있어요.
# Red words when plugin.ini asks for the local network and the whole network together. { $home } is the local network permission name.
plugins-home-network-or-network = { $home } 또는 network 중 하나만 요청하세요. network는 인터넷뿐 아니라 로컬 네트워크에도 연결돼요.
# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = { $name } 권한은 없어요. 쓸 수 있는 권한은 { $list }예요.
# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring]에 Icon이 필요해요. 플러그인 폴더 안의 SVG 또는 PNG 파일이에요.
# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = [Ring] Icon이 { $icon }(으)로 되어 있지만, 플러그인 폴더에 그 파일이 없어요.
# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring]에 Label이 필요해요. 포인터를 올리면 보여요.
# Red words when a part (the Ring, a menu item, a shortcut or the background part) has no Run. { $group } is the part.
plugins-needs-run = [{ $group }]에 Run이 필요해요. 실행할 명령이에요.
# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings]에 Fields가 필요해요. 항목들의 이름이에요.
# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = 항목 이름 { $name }에는 영문자, 숫자, 하이픈, 밑줄만 쓸 수 있어요.
# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = { $name } 항목에 [{ $group }] 부분이 없어요.
# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }]에 Label이 필요해요.
# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
       *[other] [{ $group }]의 Choices는 최대 { $most }개예요. 각각 한 줄의 일반 텍스트이고 { $chars }자를 넘으면 안 돼요.
    }
# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom에는 플러그인 폴더 안의 파일 이름을 적어야 해요. 영문자, 숫자, 점, 하이픈, 밑줄만 쓰세요. 이름을 점으로 시작하거나 폴더 경로를 넣지 마세요.
# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }]은 선택 항목이라서 선택지가 둘 이상인 Choices 또는 파일인 ChoicesFrom이 필요해요.
# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = [{ $group }] Default가 Choices에 없어요.
# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }]은 메모 항목이라서 TextFrom에 플러그인 폴더 안의 파일 이름을 적어야 해요. 영문자, 숫자, 점, 하이픈, 밑줄만 쓰세요. 이름을 점으로 시작하거나 폴더 경로를 넣지 마세요.
# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }]은 메모 항목이라서 설정을 저장하지 않으므로 Default를 쓸 수 없어요.
# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = [{ $group }] Kind가 { $kind }인데, switch, text, choice, note 중 하나여야 해요.
# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }]은 스위치라서 Default는 true 또는 false여야 해요.
# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background]를 쓰려면 [Plugin] Permissions에 background 권한이 있어야 해요.
# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = 이 플러그인은 어디에도 연결되지 않아요. [Ring], [Settings], [Background], [Shortcut] 또는 notifications 권한을 넣어 주세요.
# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path }은 플러그인 폴더 밖으로 이어지는 링크예요.

## A Run line (src/plugins/mod.rs)

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = [{ $group }] Run을 명령으로 읽을 수 없어요.
# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = [{ $group }] Run에는 플러그인 폴더 안의 프로그램을 적어야 해요. { $program }은 안 돼요.
# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = [{ $group }] Run에 { $program }(이)라고 적혀 있지만, 플러그인에 그 프로그램이 없어요 (없거나 실행할 수 없어요).

## Ring menu items (src/plugins/mod.rs)

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
       *[other] [Ring] Menu에 항목이 { $n }개 있어요. 최대 { $max }개까지 쓸 수 있어요.
    }
# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = 메뉴 ID { $id }에는 영문자, 숫자, 하이픈, 밑줄만 쓸 수 있어요.
# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = [Ring] Menu에 { $id }이 두 번 적혀 있어요.
# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = [Ring] Menu에 { $id }이 있지만 [{ $group }] 부분이 없어요.
# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
       *[other] [{ $group }] Label은 { $max }자 이하의 짧은 한 줄이어야 해요.
    }

## The PIN part (src/plugins/mod.rs)

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }]은 Trindle 카탈로그의 Flow 전용이고, Flow는 control 권한을 요청해요.
# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = 카탈로그에 있는 Trindle 화면 낭독기만 accessibility 권한을 쓸 수 있어요.
# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }]에 Run이 필요해요. PIN을 받는 명령이에요.

## Settings fields shown only when another field holds a value (src/plugins/mod.rs)

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
       *[other] [{ $group }] ShowWhen은 { $max }자 이하여야 해요.
    }
# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = [{ $group }] ShowWhen은 field=value 형식이어야 해요. 예: ShowWhen=mode=home. 다른 항목과 그 항목이 가져야 할 값을 적어 주세요.
# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = [{ $group }] ShowWhen은 field=value 형식이어야 하고, 항목 이름에는 영문자, 숫자, 하이픈, 밑줄만 쓸 수 있어요.
# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
       *[other] [{ $group }] ShowWhen에는 한 줄의 일반 텍스트 값이 필요해요. { $max }자를 넘으면 안 돼요.
    }
# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = [{ $group }] ShowWhen이 자기 자신의 항목을 가리켜요. 앞에 있는 다른 항목을 적어야 해요.
# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = [{ $group }] ShowWhen이 { $other } 항목을 가리키지만, 이 플러그인의 Fields에 없어요.
# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = [{ $group }] ShowWhen이 { $other } 항목을 가리키는데, Fields에서 그 항목이 뒤에 있어요. 앞에 있는 항목이어야 해요.
# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = [{ $group }] ShowWhen이 { $other } 항목을 가리키는데, 그 항목에도 ShowWhen이 있어요. ShowWhen이 없는 항목에만 기댈 수 있어요.
# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = [{ $group }] ShowWhen이 { $other } 스위치를 가리키므로 값은 true 또는 false여야 해요.
# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = [{ $group }] ShowWhen의 값 { $value }은 { $other }의 Choices에 없어요.
# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = [{ $group }] ShowWhen이 메모 { $other }을 가리키는데, 메모는 설정을 저장하지 않아요. 스위치, 선택 또는 텍스트 항목을 쓰세요.

## Ports a plugin listens on (src/plugins/mod.rs)

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Network] Listen: { $why }
# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen을 쓰려면 [Plugin] Permissions에 network 권한이 있어야 해요.
# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen을 쓰려면 [Plugin] Permissions에 listen 권한이 있어야 해요.
# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = listen 권한을 쓰려면 [Network] Listen에 플러그인이 연결을 받을 포트를 적어야 해요.
# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = 이 플러그인의 plugin.ini는 이 버전의 Trindle이 지원하지 않는 인터페이스를 써요.
# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = 연결을 받을 포트가 적혀 있지 않아요.
# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = plugin.ini를 읽을 수 없어요.
# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
plugins-ports-not-allowed = 연결을 받도록 허용되지 않았어요.
# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
plugins-ports-changed = 적힌 포트가 바뀌었어요. 이 포트의 연결을 다시 허용해야 해요.

## Permissions (src/plugins/mod.rs)

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
plugins-needs-none =
    { $n ->
       *[other] { $name }: 작동하려면 허용이 필요해요. 요청한 권한 { $n }개를 아직 하나도 허용하지 않았어요.
    }
# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
plugins-needs-some = { $name }이 요청한 권한 { $n }개 중 { $a }개만 허용했어요.
# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
plugins-settled =
    { $n ->
        [0] 요청하는 권한이 없어요.
       *[other] 요청한 권한 { $n }개가 모두 허용되어 있어요.
    }
# Button that opens the window where a plugin's permissions are allowed.
plugins-allow = 허용
# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = 다른 포트로 연결되는 것
# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = 설치되어 있지 않아요.
# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = 다른 플러그인
# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = 그 플러그인은 설치되어 있지 않아요.
# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = 저장하지 못했어요: { $why }
# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = 플러그인 파일을 복사하지 못했어요.
# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } 플러그인이 백그라운드에서 실행되고 있지 않아요: { $not }
# Red words when an update could not be put in and the old version could not be put back either. { $why } is why the update failed; { $back } is why the old version could not be put back.
plugins-swap-not-back = { $why } 이전 버전도 되돌리지 못했어요: { $back } 두 버전 모두 플러그인 폴더에 남아 있어요.
# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = Trindle이 시작에 필요한 trindle-plugin을 찾지 못했어요.
# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
       *[other] { $n }초 안에 시작되지 않았어요.
    }
# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = 참여할 데스크톱 세션이 없어서 시작하지 않았어요. 먼저 로그인하세요.
# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = 어느 데스크톱 세션에 참여할지 알 수 없어서 ({ $sessions }) 시작하지 않았어요. WAYLAND_DISPLAY를 알맞은 값으로 설정하고 다시 실행하세요.
# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = 파일이 있어야 할 곳에 폴더가 있어요
# Error text when a plugin's id is not an id.
plugins-not-an-id = 올바른 ID가 아니에요

## Keyboard shortcuts of plugins (src/plugins/shortcuts.rs)

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
       *[other] [Plugin] Shortcuts에 단축키가 { $n }개 있어요. 최대 { $max }개까지 쓸 수 있어요.
    }
# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = 단축키 ID { $id }에는 영문자, 숫자, 하이픈, 밑줄만 쓸 수 있어요.
# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = [Plugin] Shortcuts에 { $id }이 두 번 적혀 있어요.
# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = [Plugin] Shortcuts에 { $id }이 있지만 [{ $group }] 부분이 없어요.
# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = [{ $group }] Keys { $keys }은 이미 Trindle의 단축키예요. 다른 키를 고르세요.
# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = [{ $group }] Keys { $keys }은 이 플러그인의 다른 단축키에서 이미 쓰고 있어요.
# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }]에 Keys가 필요해요. 예: Ctrl+Alt+S. { $why }
# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = 올바른 단축키가 아니에요. { $why }
# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys }은 이미 Trindle의 단축키예요.
# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys }은 이미 내 단축키예요.
# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys }은 이미 { $holder }에서 쓰고 있어요.
# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = 설치된 플러그인의 단축키가 아니에요.
# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Trindle이 사용할 수 없었어요 ({ $why }).
# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = { $why } 때문에 Trindle이 저장하지 못했어요. 바뀐 것은 없어요.
# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = 저장하지 못했어요: { $why }.

# Said when a plugin's ports can't be closed. { $why } is the reason.
plugins-ports-not-closed = 포트를 닫지 못했어요: { $why }.
# Said when a plugin's ports can't be opened and listening is off again. { $why } is the reason.
plugins-ports-not-opened = Trindle이 포트를 열지 못해서 연결 받기를 다시 껐어요: { $why }.
# Said when a plugin's ports can't be opened and listening couldn't be switched off again. { $why } and { $kept } are the reasons.
plugins-ports-not-opened-kept = 포트를 열지 못했고 ({ $why }), 연결 받기도 다시 끄지 못했어요 ({ $kept }). 열려 있는 포트는 없어요. 설정의 권한에서 끄세요.
