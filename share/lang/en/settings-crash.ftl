# Settings: the Crash reports page (src/settings/panels/crash_reports.rs).

## The Crash reports page (src/settings/panels/crash_reports.rs)

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Crash reports
# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = crash reports errors bugs faults send sent privacy telemetry buildmorphic
# Help line at the foot of the Crash reports page.
settings-crash-help = When one of Trindle's programs crashes, the crash reports add-on keeps a short note on this computer. It sends nothing until you run its send command and say yes to each report, which shows the whole report first. Your name and GitHub name go in only if you have typed them here. Never a message, a file name, where you are or anything about other programs. Removing the add-on stops it all.
# Heading and label of the switch for sending crashes.
settings-crash-crashes = Crashes
# Under the Crashes switch.
settings-crash-crashes-short = Where in Trindle's code it crashed.
# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = Crashes sends the program, its version, the kind of crash and where in Trindle's code it happened.
# Heading and label of the switch for sending errors.
settings-crash-errors = Errors
# Under the Errors switch.
settings-crash-errors-short = The error's fixed name, and where.
# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = Errors sends the program, its version, the fixed name Trindle gives the error and where in its code.
# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = About this computer
# Under the About this computer switch.
settings-crash-computer-short = Linux, processor, memory, graphics, screens.
# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = About this computer adds to each report: the Linux it runs, the kind of processor, memory rounded down, Wayland or X11, who made the graphics, and how many screens.
# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = What is sent
# Help text of the What is sent card: { $switches } is what each switch sends.
settings-crash-what-about = { $switches } Reports carry no name unless you type your Buildmorphic username in the box below, which only gives you credit.
# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Nothing was changed, because { $why }.
# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Nothing was changed, because the file that keeps it is not as Trindle wrote it.
# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = No settings folder.
# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = The settings folder could not be made.
# Red words when a setting could not be written to its file.
settings-crash-not-written = The setting could not be written.
# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Crash ({ $signal }) in { $program }
# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Error { $code } in { $program }
# In the list of sent reports: the report was taken.
settings-crash-taken = Taken
# In the list of sent reports: the report was not sent because reports are closed.
settings-crash-tracker-closed = Not sent: reports closed
# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Not sent: too big
# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Not sent: refused
# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Not sent: too many at once
# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Not sent
# Said beside a line once what was typed or chosen in it is saved.
settings-crash-saved = Saved
# Tip of the button that clears the list of sent reports.
settings-crash-clear = Clear the list
# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = The list could not be cleared.
# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Removed
# Heading of the line that holds the clear button.
settings-crash-kept-here = Kept on this computer only
# Under the Kept on this computer only heading.
settings-crash-newest-first = The newest first, up to fifty.
# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Nothing yet
# Under the Nothing yet heading.
settings-crash-listed-here = Each report is listed here, exactly as it was sent.
# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Show exactly what was sent
# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
       *[other] { $when }, { $n } times: { $answer }
    }
# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Sent so far
# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Each report is listed exactly as it was sent, the newest first, and kept on this computer only. Taken means the report was received. Not sent: reports closed means reports are not being taken just now; refused means it was refused as wrongly made; too many means too many came at once.
# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = After a crash you are asked, with the whole report shown, whether to send it.
# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Nothing is sent, and you are not asked after a crash.
# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Nothing is sent: crashes and errors are both turned off.
# Headline of the page: what is reported (crashes, errors or both) in { $which }.
settings-crash-sum-reported =
    { $which ->
        [errors] Errors in Trindle's programs are reported anonymously.
        [both] Crashes and errors in Trindle's programs are reported anonymously.
       *[crashes] Crashes in Trindle's programs are reported anonymously.
    }
# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Ask about problems and sharing reports
# Under the Ask about problems heading.
settings-crash-ask-sub = Off hides it all and deletes waiting reports.
# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Off. No reports were waiting.
# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
settings-crash-off-deleted =
    { $n ->
        [one] Off. { $n } waiting report deleted.
       *[other] Off. { $n } waiting reports deleted.
    }
# Tip of the button that opens the community page.
settings-crash-about-community = About the community
# Heading of the line with the community button.
settings-crash-go-to-bm = Reports go to Buildmorphic
# Under the Reports go to Buildmorphic heading.
settings-crash-community = The community that helps build Trindle.
# Placeholder of the box for the Buildmorphic username.
settings-crash-username = username
# Heading of the line with the username box.
settings-crash-your-username = Your Buildmorphic username
# Under the username heading.
settings-crash-username-sub = Optional. Never an email address.
# Heading of a line about joining with an email address.
settings-crash-join = Join with just an email address
# Under the Join heading.
settings-crash-join-sub = Earn credit and levels for problems you report first.
# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = A report holds only a fixed list
# Under the fixed list heading.
settings-crash-fixed-sub = You see it first, and nothing goes without your press.
# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Reports you send
# Help text of the Reports you send card (the information button).
settings-crash-reports-about = When one of Trindle's programs crashes, a window can open asking whether to send a report. It shows the exact text first: your words, the computer's model and graphics, the error and the log from just before. Files, passwords and Wi-Fi names are never read, and names and addresses are blanked. You can also open the same window from the ring's Report a problem button at any time.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = github name
# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Your GitHub name
# Small words under that heading.
settings-crash-github-about = Optional. Shown as Reported by.
