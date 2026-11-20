# Trindle's own words, in English: the source file for every language. The
# toolkit's words (keys starting kit-) are in trindle-toolkit/share/lang/en.ftl.
# See README.md in the toolkit's share/lang folder for how to translate.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## Report a problem (the window opened from the ring, and after a crash)

# Window title when the person opens it themselves from the ring.
report-title-note = Report a problem
# Window title when Trindle's ring opens it after one of Trindle's programs crashed.
report-title-crash = Something went wrong. Send a report?
# Under the title: what a report holds, and what it never holds.
report-summary = This sends your words, computer details, this boot's errors and Trindle's log. Names and addresses (email, network, Wi-Fi) are taken out of what Trindle gathers. File names and web addresses can still appear in log lines. It doesn't send your files or passwords.
# Big typing box in the middle of the window, shown quietly while empty.
report-what-doing = What happened?
# Beside the dropdown at the top of the window that says which app the report is about; also its name for the screen reader.
report-app-box = Which app?
# Dropdown choice: the person doesn't know which app it was.
report-app-unsure = Not sure
# Dropdown choice: the app is not one of Trindle's own.
report-app-other = Another program
# Dropdown choice: the desktop itself (the panels, windows and session), not a single app.
report-app-desktop = The desktop
# Button at the bottom of the window that sends the report; it is also the word beside it when it is switched off.
report-send = Send
# Button at the bottom of the window when the person opened it themselves.
report-cancel = Cancel
# Button at the bottom of the window when it opened by itself (after a crash, or on the live image).
report-dont-send = Don't send
# Button that opens the exact text of the report.
report-show-details = Show details
# Button that closes the exact text of the report.
report-hide-details = Hide details
# Button under the shown report text that copies it.
report-copy = Copy
# Button under the shown report text that saves it as a file in Downloads.
report-save = Save
# Written where the Send button would be, when Buildmorphic says it is not taking reports or cannot be reached.
report-not-taken = Reports can't be sent yet.
# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Checking.
# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle is in beta. Reports are anonymous: names, email addresses and home folders are removed. A username is optional.
# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Buildmorphic username (optional)
# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Trindle couldn't send the report. { $why } Press Send to try again.
# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Too many reports at once.
# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Waiting for a connection.
# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Waiting for a connection. { $why }
# Under the typing box after the report was sent.
report-sent = Sent. Thank you.
report-sent-as-bug = Sent. Your bug report number is { $number }. Thank you.
# Under the typing box while the report is being posted.
report-sending = Sending.
# Under the typing box after Save. $name is the file's name.
report-saved-as = Saved as Downloads/{ $name }.
# Red words when the Buildmorphic username holds an email address.
report-no-at = Enter your Buildmorphic username, not an email address. Don't use an @.
# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = A username must be 2 to 31 characters long, using lowercase letters, numbers or dashes.
# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = A GitHub name can only use letters, numbers and dashes.
# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Trindle couldn't save the report for later: { $reason }.
# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Couldn't save it: { $reason }.
# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = no permission
# The reason inside "Could not save it: ...": there is no room.
report-reason-full = the drive is full
# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = the folder is missing
# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = a drive error
# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl is missing.
# Why a report could not be posted: the post could not be started.
report-not-posted = Couldn't send it.
# Why a report could not be posted: the computer is offline.
report-no-connection = No connection.
# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = The server sent { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Write a few more words about what happened.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Reading the log.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = There is no log to send with the report, so it can't be sent. Save it instead.

# Beside the picture buttons, and beside the picture once one is chosen: who can see it.
report-picture-line = Optional picture. Only the project owner sees it.
# Tooltip of the round button that opens the file chooser for a picture.
report-picture-choose = Choose a PNG picture
# Tooltip of the round button that takes a picture of the screen now.
report-picture-shot = Take a screenshot
# Tooltip of the round button that takes the picture off the report.
report-picture-remove = Remove the picture
# Name of the small picture for a screen reader. $width and $height are in pixels.
report-picture-name = Your picture, { $width } by { $height }
# Title of the file chooser, and the name of its one filter.
report-picture-choose-title = Choose a PNG picture
report-picture-filter = PNG pictures
# Under the picture while it is being read.
report-picture-reading = Reading.
# Red words under the picture when the screenshot could not be taken, or no file chooser answers.
report-picture-no-shot = Couldn't take a screenshot.
report-picture-no-chooser = Trindle couldn't open the file chooser.
# Red words under the picture, each the reason it is refused.
report-picture-not-png = It isn't a PNG picture.
report-picture-too-big = It's over { $megabytes } MB.
report-picture-too-large = The picture is too large. It can be up to { $side } pixels wide and { $side } pixels high.
report-picture-damaged = The picture is damaged.
report-picture-unreadable = Couldn't read it.
