# machine translated (Japanese), not yet reviewed by a person.

# Trindle's own words, in English: the source file for every language. The
# toolkit's words (keys starting kit-) are in trindle-toolkit/share/lang/en.ftl.
# See README.md in the toolkit's share/lang folder for how to translate.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## Report a problem (the window opened from the ring, and after a crash)

# Window title when the person opens it themselves from the ring.
report-title-note = 問題を報告
# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = 問題が起きました。報告を送りますか？
# Under the title: what a report holds, and what it never holds.
report-summary = あなたが書いた内容、コンピューターの情報、今回の起動中のエラー、Trindle のログを送ります。名前とアドレス（メール、ネットワーク、Wi-Fi）は、Trindle が集めた内容から取り除きます。ログの行には、ファイル名やウェブアドレスが残ることがあります。あなたのファイルやパスワードは送りません。
# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = 何が起きましたか？
# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = どのアプリですか？
# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = わからない
# Dropdown choice: the app is not one of Trindle's own.
report-app-other = ほかのプログラム
# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = デスクトップ
# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = 送信
# Button at the bottom of the window when the person opened it themselves.
report-cancel = キャンセル
# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = 送信しない
# Button that opens the exact text of the report.
report-show-details = 詳細を表示
# Button that closes the exact text of the report.
report-hide-details = 詳細を隠す
# Button under the shown report text that copies it.
report-copy = コピー
# Button under the shown report text that saves it as a file in Downloads.
report-save = 保存
# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = まだ報告を送れません。
# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = 確認中。
# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle はベータ版です。報告は匿名です。名前、メールアドレス、ホームフォルダは取り除かれます。ユーザー名は任意です。
# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Buildmorphic のユーザー名（任意）
# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle は報告を送れませんでした。{ $why } 「送信」を押してもう一度試してください。
# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = 一度に届いた報告が多すぎます。
# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = 接続を待っています。
# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = 接続を待っています。{ $why }
# Under the typing box after the report was sent.
report-sent = 送信しました。ありがとうございます。
report-sent-as-bug = 送信しました。不具合報告の番号は { $number } です。ありがとうございます。
# Under the typing box while the report is being posted.
report-sending = 送信中。
# Under the typing box after Save. $name is the file's name.
report-saved-as = Downloads/{ $name } として保存しました。
# Red words when the Buildmorphic username holds an email address.
report-no-at = メールアドレスではなく、Buildmorphic のユーザー名を入力してください。@ は使えません。
# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = ユーザー名は2文字から31文字で、小文字、数字、ハイフンが使えます。
# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = GitHub の名前には、英字、数字、ハイフンだけが使えます。
# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle は報告を後のために保存できませんでした：{ $reason }。
# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = 保存できませんでした：{ $reason }。
# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = 許可がありません
# The reason inside "Could not save it: ...": there is no room.
report-reason-full = ドライブがいっぱいです
# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = フォルダがありません
# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = ドライブのエラーです
# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl がありません。
# Why a report could not be posted: the post could not be started.
report-not-posted = 送れませんでした。
# Why a report could not be posted: the computer is offline.
report-no-connection = 接続がありません。
# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = サーバーから { $code } が返りました。

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = 何が起きたか、もう少し書いてください。

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = ログを読み込んでいます。

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = 報告に付けるログがないため、送れません。代わりに保存してください。

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = 画像は任意です。プロジェクトの持ち主だけが見られます。
# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = PNG 画像を選ぶ
# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = スクリーンショットを撮る
# Tooltip of the round button that takes the picture off the report.
report-picture-remove = 画像を外す
# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = あなたの画像、幅 { $width }、高さ { $height }
# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = PNG 画像を選ぶ
report-picture-filter = PNG 画像
# Under the picture while it is being read.
report-picture-reading = 読み込み中。
# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = スクリーンショットを撮れませんでした。
report-picture-no-chooser = Trindle はファイル選択を開けませんでした。
# Red words under the picture, each the reason it is refused.
report-picture-not-png = PNG 画像ではありません。
report-picture-too-big = { $megabytes } MB を超えています。
report-picture-too-large = 画像が大きすぎます。幅も高さも { $side } ピクセルまでです。
report-picture-damaged = 画像が壊れています。
report-picture-unreadable = 読み取れませんでした。
