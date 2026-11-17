# Settings, Printers page, and the words of the CUPS helper (src/common/cups.rs) that the print window of the screenshot program also shows.

## Printers page (src/settings/panels/printers.rs)

# Sentence at the top of the page when there are no printers.
settings-printers-sum-none = No printers yet. Any that Trindle finds on your network are listed below.
# Sentence at the top of the page when the printer used first is paused.
settings-printers-sum-paused = { $name } is paused, so nothing will print on it for now.
# Sentence at the top of the page when the printer used first is printing.
settings-printers-sum-printing = { $name } is printing.
# Sentence at the top of the page when the printer used first is ready.
settings-printers-sum-ready = Ready to print on { $name }.
# Sentence at the top of the page for any other state of the printer used first; the state is a word CUPS gives.
settings-printers-sum-state = { $name } is { $state }.
# Sentence at the top of the page when one printer is set up and none is chosen to print on first.
settings-printers-sum-one-unchosen = { $name } is set up, but you haven't chosen a default printer.
# Sentence at the top of the page when several printers are set up and none is chosen to print on first.
settings-printers-sum-many-unchosen = { $n } printers are set up, but you haven't chosen a default printer.
# Red words under the Print jobs line when a job number is not one CUPS knows.
settings-printers-bad-job = That job no longer exists.
# Shown beside a print job after it was cancelled.
settings-printers-cancelled = Cancelled
# Red words when CUPS fails and says nothing more.
settings-printers-lpstat-failed = Check failed.
# Red words when Trindle cannot reach CUPS at all, after trying twice.
printers-unreachable = The printing service isn't running or can't be reached.
# Words in the ring while the network is searched for printers.
settings-printers-searching = Looking
# Words in the ring when the search is finished.
settings-printers-search-done = Done
# Shown beside a printer after it was made the one used first.
settings-printers-default-now = Default now
# Shown beside a printer after a test page was sent.
settings-printers-sent = Sent
# Shown beside a printer after it was removed.
settings-printers-removed = Removed
# Red words under the By its address line when nothing was typed.
settings-printers-type-address = Type the printer's address first.
# Shown under the By its address line while the printer is added.
settings-printers-adding = Adding…
# Shown beside a printer or the address line after it was added.
settings-printers-added = Added
# Shown beside a found printer while a driver is looked for.
settings-printers-finding-driver = Looking for its driver…
# Heading of a print job in the queue: its number and the printer.
settings-printers-job = Job { $number } on { $printer }
# Small line under a print job: who sent it, how big it is and when.
settings-printers-job-sub = { $user }, { $size }, { $when }
# Tip of the cross button that cancels a print job.
settings-printers-cancel-job = Cancel this job
# Heading of the card that lists waiting print jobs.
settings-printers-jobs = Print jobs
# Help text (i button) of the Print jobs card.
settings-printers-jobs-about = Every job waiting to print, on any printer. Trindle updates the list every few seconds while this page is open. Pages that have already printed can't be cancelled.
# Line in the Print jobs card when CUPS does not answer.
settings-printers-cups-silent = CUPS isn't responding
# Small line under "CUPS is not answering" in the Print jobs card.
settings-printers-queue-unreadable = Couldn't read the queue.
# Line in the Print jobs card while the queue is first read, and on the page while it is read.
settings-printers-looking = Looking…
# Line in the Print jobs card when no job is waiting.
settings-printers-nothing-waiting = Nothing waiting
# State of a printer shown in its small line when it is paused.
settings-printers-state-paused = Paused
# State of a printer shown in its small line when it is idle.
settings-printers-state-idle = Idle
# State of a printer shown in its small line when it is printing.
settings-printers-state-printing = Printing
# Small line under a printer that is the one used first: "Default" and its state.
settings-printers-default-state = Default, { $state }
# Tip of the star button that makes a printer the one used first.
settings-printers-make-default = Make this the default
# Tip of the button that prints a test page.
settings-printers-test-page = Print a test page
# Tip of the bin button that removes a printer.
settings-printers-remove = Remove this printer
# Line in the Add a printer card when the search needs a program that is missing.
settings-printers-needs-filters = Searching needs cups-filters
# Small line under it; the command is to be typed as it is.
settings-printers-install-filters = To install it, run: sudo apt install cups-filters
# Tip of the plus button that adds a printer that was found.
settings-printers-add-found = Add this printer
# Line in the Add a printer card when the search found nothing new.
settings-printers-nothing-new = Nothing new found
# Placeholder of the box where a printer's address is typed.
settings-printers-address-placeholder = 192.168.1.50
# Tip of the button that adds the printer whose address was typed.
settings-printers-add-it = Add it
# Heading of the line where a printer's address is typed.
settings-printers-by-address = Add by address
# Heading of the card for adding printers.
settings-printers-add-card = Add a printer
# Help text (i button) of the Add a printer card.
settings-printers-add-about = Trindle first looks for printers that announce themselves, then checks each device on the network in turn. The printer must be switched on and on the same network. Some printers don't announce themselves, and some networks don't pass the announcement on. If so, add the printer by its address instead, such as 192.168.1.50. You'll find the address on the printer's own screen, under network settings.
# Name of the page in the list of settings pages.
settings-printers-title = Printers
# Words the settings search looks for on this page (not shown).
settings-printers-keywords = printer print scanner
# Help text at the top of the page.
settings-printers-help = Trindle finds printers on your network by itself. To use one, press Add this printer beside it.
# Headline of the page when printing is not installed.
settings-printers-top-not-installed = Printing isn't installed on this computer.
# Headline of the page when printing has not answered yet.
settings-printers-top-looking = Looking…
# Headline of the page when the printing service does not answer.
settings-printers-top-silent = Trindle can't reach the printing service, so nothing can print right now.
# Heading of the card shown when printing is not installed.
settings-printers-printing = Printing
# Help text (i button) of that card.
settings-printers-printing-about = Printing needs CUPS, which isn't on this computer. Once it is installed, Trindle finds printers on your network by itself.
# Line in that card.
settings-printers-unavailable = Printing isn't available
# Small line under it; the command is to be typed as it is.
settings-printers-install-cups = Install it with: sudo apt install cups
# Line in that card.
settings-printers-then-found = Finding printers
# Small line under "CUPS is not answering" in the Printers card.
settings-printers-service = Printing service
# Line in the Printers card when there are none.
settings-printers-none = No printers yet
# Heading of the card that lists the printers.
settings-printers-card = Printers
# Help text (i button) of the Printers card.
settings-printers-card-about = Every printer set up on this computer. Your account needs to be in the lpadmin group to make changes.

## The CUPS helper (src/common/cups.rs)

# Red words under a page range box that cannot be used.
printers-bad-range = For example: 1-3, 5
# Red words under a copies box that cannot be used.
printers-bad-copies = 1 to 99 copies.
# Name of the choice to print on one side of the paper.
printers-sides-one = One side
# Name of the choice to print on both sides, turning the page on its long edge.
printers-sides-long = Both, long edge
# Name of the choice to print on both sides, turning the page on its short edge.
printers-sides-short = Both, short edge
# Name of the A4 paper size.
printers-paper-a4 = A4
# Name of the A3 paper size.
printers-paper-a3 = A3
# Name of the A5 paper size.
printers-paper-a5 = A5
# Name of the US Letter paper size.
printers-paper-letter = Letter
# Name of the US Legal paper size.
printers-paper-legal = Legal
# Name of the choice to print the page upright (portrait).
printers-orientation-portrait = Upright
# Name of the choice to print the page sideways (landscape).
printers-orientation-landscape = Sideways
# Red words when a printing error is a refusal because the account is not in the lpadmin group.
printers-not-allowed = You aren't allowed to do this. Your account needs to be in the lpadmin group. After that, log out and back in.
# Red words when CUPS refuses and gives no reason.
printers-refused = CUPS refused.
# Red words when the printer name chosen is not one CUPS accepts.
printers-bad-name = That printer name isn't allowed.
# Red words when printing is not installed on the computer.
printers-not-installed = Printing isn't installed on this computer.
# Red words when the printer chosen has been removed since.
printers-gone = { $name } is no longer set up.
# Red words when the printer chosen is paused.
printers-paused = { $name } is paused.
# Red words when sending the document to CUPS took too long.
printers-too-slow = The document took too long to send.
# Red words when a print job could not be cancelled.
settings-printers-not-cancelled = Trindle couldn't cancel the job.
# Red words when a printer could not be made the default.
settings-printers-not-default = Trindle couldn't make it the default printer.
# Red words when the test page could not be sent.
settings-printers-not-sent = Trindle couldn't send the test page.
# Red words when a printer could not be removed.
settings-printers-not-removed = Trindle couldn't remove the printer.
# Red words when a printer could not be added.
settings-printers-not-added = Trindle couldn't add the printer.
# Red words when the list of printers could not be read.
settings-printers-not-read = Trindle couldn't read the list of printers.
