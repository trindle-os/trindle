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
report-summary = This sends the error, a summary of the computer and the log's last lines, with names, email addresses, network addresses and Wi-Fi names taken out. File names and web addresses can still be in the log lines. It does not send your files or passwords.
# Typing box at the top of the window, shown quietly while empty.
report-what-doing = What were you doing? (optional)
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
report-not-taken = Reports are not being taken yet.
# Written where the Send button would be, while Buildmorphic is being asked whether it is taking reports.
report-checking = Checking.
# Under the title, beta only: why a report matters and what it holds.
report-beta = Trindle is in beta, and every report helps us make it run perfectly. A report is anonymous and holds nothing private or personal. No email is needed, and a username is optional. Nothing is sent until you press Send.
# Typing box under the first one: the optional Buildmorphic username that credits the report.
report-username-box = Buildmorphic username (optional)
# Red words when a report posted by hand could not get through. $why is the reason, such as "No connection."
report-try-again = Not sent. { $why } Press Send to try again.
# Why a report is waiting: Buildmorphic had too many reports at once and the report is tried again later.
report-too-many = Too many reports at once.
# Written beside the Send button once the report has been sent.
report-already-sent = Already sent.
# Written under the typing box when Save is pressed a second time.
report-already-saved = Already saved.
# Written beside the buttons while a report that could not be sent waits for the computer to be online.
report-waiting = Waiting for a connection.
# Under the typing box when the report is waiting. $why is the reason it could not be posted, such as "No connection."
report-waiting-why = Waiting for a connection. { $why }
# Under the typing box after the report was sent.
report-sent = Sent. Thank you.
report-sent-as-bug = Sent. Seen as bug { $number }. Thank you.
# Under the typing box while the report is being posted.
report-sending = Sending.
# Under the typing box after Save. $name is the file's name.
report-saved-as = Saved as Downloads/{ $name }.
# Red words when the Buildmorphic username holds an email address.
report-no-at = Use your Buildmorphic username, not an email address, so no @.
# Red words when the Buildmorphic username holds letters that are not allowed.
report-name-chars = A username is 2 to 31 small letters, numbers or dashes.
# Red words in Settings, Crash reports, when the GitHub name holds letters that are not allowed.
report-github-chars = A GitHub name holds letters, numbers and dashes only.
# Red words when a report could not be kept for later. $reason is one of the four reasons below.
report-keep-failed = Could not keep it for later: { $reason }.
# Red words when a report could not be saved. $reason is one of the four reasons below.
report-save-failed = Could not save it: { $reason }.
# The reason inside "Could not save it: ...": the person may not write there.
report-reason-permission = no permission
# The reason inside "Could not save it: ...": there is no room.
report-reason-full = the drive is full
# The reason inside "Could not save it: ...": the Downloads folder is gone.
report-reason-missing = the folder is missing
# The reason inside "Could not save it: ...": any other refusal.
report-reason-refused = the drive refused
# Why a report could not be posted: the program that posts it is not installed.
report-curl-missing = curl is missing.
# Why a report could not be posted: the post could not be started.
report-not-posted = It could not be posted.
# Why a report could not be posted: the computer is offline.
report-no-connection = No connection.
# Why a report could not be posted: the address replied with a fault. $code is the number it replied with.
report-answered = The address answered { $code }.

# Beside the Send button, when the note is too short: Buildmorphic needs a few words with the log.
report-say-more = Write a few more words to say what happened.

# Beside the Send button, while Trindle's log is still being read.
report-reading-log = Reading the log.

# Beside the Send button, when no log lines were found: a report with no log is not sent.
report-no-log = There is no log to send with it, so it cannot be sent. Save it instead.
