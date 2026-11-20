# machine translated (Korean), not yet reviewed by a person.

# Trindle's plugin catalogue: why it was not used, why a plugin could not be fetched or put in, and the pause with its Start again button (src/plugins/catalogue.rs).

## The pause (src/plugins/catalogue.rs)

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = 플러그인 목록이 손상되어 플러그인 업데이트를 멈췄어요.
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = 다시 시작
# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle이 기록을 지우지 못했어요: { $why }
# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = 이름에 널 바이트가 있어요.
# Error text when there is no record name to take away.
plugins-start-no-note = 지울 기록이 없어요.

## Why the catalogue was not used (src/plugins/catalogue.rs)

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = 확인용 키가 없어서 Trindle이 플러그인 목록을 확인할 수 없어요.
# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Trindle의 플러그인 목록에 서명이 없어서 Trindle이 쓰지 않았어요.
# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Trindle의 플러그인 목록 서명 크기가 맞지 않아서 Trindle이 목록을 쓰지 않았어요.
# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Trindle의 플러그인 목록이 Trindle의 키로 서명되지 않아서 Trindle이 쓰지 않았어요.
# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle의 플러그인 목록에 버전 번호가 없어서 Trindle이 쓰지 않았어요.
# Red line under Trindle's own plugins when the catalogue is older than one accepted before.
plugins-refused-older = 전에 쓴 목록보다 오래된 플러그인 목록이라 Trindle이 쓰지 않았어요.
# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle의 플러그인 목록은 서명되어 있지만 읽을 수 없어서 Trindle이 쓰지 않았어요: { $why }
# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } 마지막으로 정상이었던 목록을 보여 줘요.
# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = 텍스트가 아니에요.
# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = 목록이 너무 커요.
# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = 플러그인 목록이 아니에요. { $file }에 Format이 있는 [Catalogue] 부분이 없어요.
# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = 이 목록은 이 버전의 Trindle이 읽을 수 있는 형식보다 새로운 형식이에요. Trindle은 Format={ $format }을 지원해요.
# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id }을 뺐어요: { $why ->
        [id] id는 영문자, 숫자, 점, 줄표로만 만들 수 있어요
        [needs] Name과 Version이 필요해요
        [archive] Archive는 목록 옆에 있는 .tar.zst 파일의 이름이어야 해요
        [hash] SHA256은 16진수 64자리여야 해요
        [permission] 없는 권한을 요청해요
       *[icon] Icon은 목록 옆에 있는 SVG 또는 PNG 파일의 이름이어야 해요
    }.

## Fetching and putting in a plugin (src/plugins/catalogue.rs)

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar가 시작되지 않았어요: { $why }
# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle이 압축 파일을 풀지 못했어요: { $why }
# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = 이유를 알 수 없어요
# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = 압축 파일의 맨 위 또는 맨 위의 폴더 하나 안에 { $file }이 없어요.
# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle이 임시 폴더를 만들지 못했어요: { $why }
# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl이 시작되지 않았어요: { $why }
# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = 내려받지 못했어요: { $why }
# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = 내려받은 파일이 목록의 체크섬(SHA-256)과 달라서 Trindle이 아무것도 설치하지 않았어요.
# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = 플러그인이 검사를 통과하지 못해서 Trindle이 아무것도 설치하지 않았어요: { $why }
# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = 압축 파일에 { $wanted }이 아니라 { $found }이 들어 있어서 Trindle이 아무것도 설치하지 않았어요.
# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle이 플러그인을 설치했지만 Trindle의 목록에서 왔다는 기록은 남기지 못했어요: { $why }.
# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = 설치되어 있지 않아서 업데이트할 것이 없어요.
