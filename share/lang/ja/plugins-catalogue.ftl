# machine translated (Japanese), not yet reviewed by a person.

# Trindle's plugin catalogue: why it was not used, why a plugin could not be fetched or put in, and the pause with its Start again button (src/plugins/catalogue.rs).

## The pause (src/plugins/catalogue.rs)

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = プラグインのリストが壊れているため、プラグインの更新を止めています。
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = やり直す
# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle は記録を削除できませんでした: { $why }
# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = 名前にヌルバイトが含まれています。
# Error text when there is no record name to take away.
plugins-start-no-note = 削除する記録がありません。

## Why the catalogue was not used (src/plugins/catalogue.rs)

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = 確認用の鍵がないため、Trindle はプラグインカタログを確認できません。
# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Trindle のプラグインカタログに署名がないため、Trindle は使いませんでした。
# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = Trindle のプラグインカタログの署名のサイズが正しくないため、Trindle はカタログを使いませんでした。
# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Trindle のプラグインカタログが Trindle の鍵で署名されていないため、Trindle は使いませんでした。
# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Trindle のプラグインカタログにバージョン番号がないため、Trindle は使いませんでした。
# Red line under Trindle's own plugins when the catalogue is older than one accepted before.
plugins-refused-older = このプラグインカタログは以前使ったものより古いため、Trindle は使いませんでした。
# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Trindle のプラグインカタログは署名されていますが読み取れないため、Trindle は使いませんでした: { $why }
# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } 最後に使えたカタログを表示しています。
# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = テキストではありません。
# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = カタログが大きすぎます。
# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = プラグインカタログではありません。{ $file } に Format のある [Catalogue] の部分がありません。
# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = このカタログは、この版の Trindle が読めるものより新しい形式です。この Trindle は Format={ $format } に対応しています。
# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } は除外されました: { $why ->
        [id] id が英数字、ドット、ダッシュでできていません
        [needs] Name と Version が必要です
        [archive] Archive がカタログと同じ場所にある .tar.zst ファイルの名前ではありません
        [hash] SHA256 が 16 進数 64 桁ではありません
        [permission] 存在しない権限を求めています
       *[icon] Icon がカタログと同じ場所にある SVG か PNG ファイルの名前ではありません
    }。

## Fetching and putting in a plugin (src/plugins/catalogue.rs)

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar を起動できませんでした: { $why }
# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle はアーカイブを展開できませんでした: { $why }
# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = 理由は不明です
# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = アーカイブの最上位、または最上位にある 1 つのフォルダーの中に { $file } がありません。
# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle は一時フォルダーを作れませんでした: { $why }
# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl を起動できませんでした: { $why }
# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = ダウンロードできませんでした: { $why }
# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = ダウンロードしたファイルがカタログのチェックサム (SHA-256) と一致しないため、Trindle は何もインストールしませんでした。
# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = プラグインが検査に通らなかったため、Trindle は何もインストールしませんでした: { $why }
# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = アーカイブの中身は { $wanted } ではなく { $found } だったため、Trindle は何もインストールしませんでした。
# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle はプラグインをインストールしましたが、Trindle のカタログから来たという記録を書けませんでした: { $why }
# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = インストールされていないため、更新するものがありません。
