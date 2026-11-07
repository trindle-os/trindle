//! Nothing a person or the computer would miss is written over where it lies.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const ALLOWED: [(&str, usize, &str); 63] = [
    ("toolkit/paint/service.rs", 1, "the drawing service's own lock file in the person's runtime folder: not a file a person keeps"),
    ("admin/read_only.rs", 2, "root's own record in /run of which helper has the system open, a few figures cleared and written again under its lock; nothing in it to lose"),
    ("toolkit/sys/gbm.rs", 1, "the graphics card's render node in /dev/dri, opened to draw with: a device, not a file on a drive, with nothing in it to lose"),
    ("controller/uinput.rs", 1, "the kernel's own /dev/uinput, opened to make a controller: a device, not a file on a drive, with nothing in it to lose"),
    ("common/restart.rs", 1, "the mark that the parts were restarted, in the session's own runtime folder, taken away once the new ring has said so; never a file of the person's"),
    ("session/restart_ask.rs", 2, "the mark that a full restart was asked for and the lock that lets one question be out at a time, both in the session's own runtime folder; never a file of the person's"),
    ("common/bt_audio.rs", 1, "a few words in the session's own runtime folder, Trindle's own and made again each time; never a file of the person's"),
    ("admin/mod.rs", 1, "the administrator's helper writes a system file through a spare one, with the mode it is told, and renames it"),
    ("common/held.rs", 1, "its own spare file, written out to the drive and renamed, under the lock the list is changed under"),
    ("common/in_use.rs", 2, "a spare file and a rename, so the ring never reads half, and the small mark that says every microphone is muted; both are in the session's own folder and made again at each login"),
    ("dns/list.rs", 2, "the fetched lists go to spare files and are renamed; they are fetched again when missing"),
    ("dns/mod.rs", 1, "the resolver's file goes to a spare one and is renamed; it is written again at every change of network"),
    ("keyring/pam.rs", 6, "the key handed over at login, in root's own folder, made new, read once, put back when it did not fit, emptied and removed"),
    ("keyring/store.rs", 1, "the keyring's own spare file: private from the first, never through a link, written out and renamed"),
    ("screenshot/portal_cast.rs", 1, "a spare file and a rename, in the session's own folder; gone at logout"),
    ("store/copied_guard.rs", 1, "a spare file and a rename, in the session's own folder; gone at logout"),
    ("common/boot.rs", 2, "how long this start took, noted once in /run, and a line added to the list of starts, which is only ever added to"),
    ("common/locked.rs", 1, "an empty mark that the screen is locked, in the session's own folder"),
    ("common/ring_said.rs", 1, "one word the ring leaves for the compositor, in the session's own folder"),
    ("plugins/background.rs", 2, "the number of a plugin's running program, a note for as long as it runs, and its lock file, which is empty"),
    ("ring/view/mod.rs", 1, "an empty mark that the ring is closed, in the session's own folder"),
    ("session/idle.rs", 5, "marks that something keeps the screen awake, the brightness to go back to and this program's number, all gone at logout, and a line added to its own log"),
    ("session/screensaver.rs", 2, "this program's number and when it began, gone at logout"),
    ("session/start.rs", 2, "an empty mark that plain drawing is wanted, and lines added to the session's own log"),
    ("session/wallpaper.rs", 2, "which picture shows now and this program's number, gone at logout"),
    ("store/asking.rs", 1, "a lock file, empty"),
    ("unlock/fifo.rs", 2, "an empty mark in the early start's own memory, before any drive is open, and the pipe the unlock is told through, which is no file"),
    ("usbretry/mod.rs", 2, "the kernel's own switch for a USB port, in /sys, which is not a file on a drive and can only be written where it is, and lines added to Trindle's own log of the ports tried"),
    ("usbretry/quiet.rs", 1, "the kernel's own switches for a USB hub and its port, in /sys, written just before a restart: they are not files on a drive, hold nothing to lose, and can only be written where they are"),
    ("unlock/main.rs", 1, "how long the unlock waited, noted in /run"),
    ("unlock/spin.rs", 2, "this program's number, in /run, and a line added to its own log there"),
    ("welcome/mod.rs", 1, "the mark that the Welcome has been seen: it is the file being there that counts, not its words"),
    ("common/crash.rs", 3, "lines added to Trindle's own crash log, which is only ever added to, one of them from inside a signal's handler by the kernel's own calls"),
    ("common/keyring_said.rs", 1, "a lock file in the session's own folder, opened and never written"),
    ("common/screen_scale.rs", 3, "the scale of each screen, kept for the login screen in a shared folder, opened without following a link and only if it is this person's own; the login screen works one out when it is missing or half written"),
    ("common/updating.rs", 2, "the note of an update under way, in the session's own folder, written under its own lock and emptied when the update ends"),
    ("plugins/log.rs", 1, "lines added to a plugin's own log, never through a link"),
    ("common/charge.rs", 4, "the kernel's switches for the battery, in /sys, and the laptop's own device that sets the charge limit"),
    ("thunder/sysfs.rs", 1, "the kernel's switch for a Thunderbolt device, in /sys"),
    ("common/rfkill.rs", 1, "the kernel's switch for the radios, /dev/rfkill, which takes an order and keeps nothing"),
    ("screenshot/gbm.rs", 1, "the graphics card's device, opened to draw with"),
    ("unlock/fb.rs", 1, "the early screen's own device, /dev/fb0"),
    ("unlock/keys.rs", 2, "the console, opened to read the keys typed"),
    ("common/clipboard.rs", 1, "a copied picture, under a name made for it and never used before"),
    ("common/sounds.rs", 1, "a sound made from numbers, kept so it need not be made twice"),
    ("icons/appimages.rs", 2, "an AppImage's icon and launcher, made again from the AppImage"),
    ("icons/mod.rs", 2, "an icon drawn from its app, drawn again when it is not as it should be, and the lock one run holds while it reads a picture off this computer, which is empty"),
    ("toolkit/common/file_work/gentle/mod.rs", 1, "copy_named, which copies gently into a folder its caller made for that alone: a part of a package put together in a folder of its own"),
    ("ring/tray.rs", 2, "an icon of Trindle's own, written into the session's own folder when it is not there"),
    ("screenshot/mod.rs", 1, "the whole screen copied onto an empty file this very call made new under a name never taken, from a picture still there"),
    ("screenshot/portal_notify.rs", 1, "a notice's picture, under a name of its own, for as long as the notice shows"),
    ("settings/panels/audio.rs", 1, "the test sound, made from numbers each time it is played"),
    ("voice/setup.rs", 1, "the version of what was just built, in a folder made for the build, which the installer then copies"),
    ("admin/snapshots.rs", 3, "into a snapshot just made, which is removed again if the restore fails"),
    ("installer/steps.rs", 6, "into the new system while it is installed; an install that stops is begun again, and the lock file that keeps a second installer out, which holds nothing"),
    ("mcp/tools.rs", 4, "a copy of the settings into a new folder of its own, before a change; nothing is there to write over"),
    ("store/backup.rs", 4, "into the folder a backup is put together in, removed when the backup is made or fails"),
    ("store/boxed/launchers.rs", 1, "a launcher, into an app's own box while it is put together"),
    ("store/boxed/install.rs", 1, "an empty mark in an app's own box, noting that the person added it"),
    ("store/boxed/first_apps.rs", 1, "an empty mark noting that a box's first words, told once, were said"),
    ("store/boxed/run.rs", 1, "ALSA's settings, into the box's own folder for this run"),
    ("store/parts.rs", 1, "into a package put together in a folder of its own"),
    ("text/store.rs", 5, "what is no file (a device or a pipe), a link to a file not made yet, and a document that must be written where it lies, its old words kept whole first and put back if the writing fails; see Saving never loses the old words"),
];

const TAKES: [(&str, usize, &str); 98] = [
    ("toolkit/paint/service.rs", 2, "the drawing service's own socket in the person's runtime folder, left by an earlier run or taken away when it ends"),
    ("common/games.rs", 1, "Trindle's own PipeWire drop-in for the Games page's larger sound buffer, named by Trindle, taken away when the person turns that switch off; never a file the person made"),
    ("toolkit/config.rs", 1, "a program's own setting, one file per key in its own settings folder, taken away when the program forgets that key; never a file the person made"),
    ("toolkit/crash.rs", 1, "the crash report the program itself left in its own folder, taken away once it has been shown; never a file of the person's"),
    ("toolkit/entry.rs", 1, "the program's own launcher in the person's autostart folder, named by its own id, taken away when the person turns starting at login off for that program"),
    ("common/restart.rs", 1, "the mark that a full restart was asked for, in the session's own runtime folder, taken away once the session has read it; never a file of the person's"),
    ("session/restart_ask.rs", 1, "the same mark, taken away when the compositor could not be found to end; never a file of the person's"),
    ("settings/wordsheet.rs", 2, "the person's own earlier word-sheet parts (zz-sheet-NNN.ftl) once a new sheet has been written whole, and the copy of them kept at the last import (zz-sheet-NNN.ftl.bak) just before a new copy is made, both in Trindle's own settings folder; a file of any other name there is never touched"),
    ("common/removal.rs", 1, "the saved data folder of a removed Flatpak, only under ~/.var/app/<id>, never through a link, and only when the person chose to delete it on the card that says how much there is"),
    ("plugins/launcher.rs", 1, "a launcher the ring itself made for a plugin that has since gone, found by the mark in its first lines; one of another program's or the person's is never touched"),
    ("common/bt_audio.rs", 1, "the note it left in the session's own runtime folder, removed once the restart has worked; never a file of the person's"),
    ("screenshot/portal_email.rs", 4, "the copies the mail portal made of a program's attachments, in a folder of Trindle's own that only the person can read: tidied after a while and when a copy fails; never the person's own file"),
    ("report/mod.rs", 2, "a waiting bug report file, which is the program's own copy in its own state folder: removed once it has been sent, or when the person turns reports off; never a file of the person's"),
    ("store/fence.rs", 7, "the AI programs' own small settings files (a switch's mark, a share used once, the links that start them) and the broker's run folder, all Trindle's own and made again; never a thing of the person's"),
    ("store/fence_broker.rs", 1, "the broker's own socket in the session's run folder, cleared before a new one is made"),
    ("admin/mod.rs", 2, "the administrator's helper: a system file's spare one renamed into place, and Trindle's own marks in /etc and /run taken away"),
    ("admin/dns.rs", 4, "the administrator's helper: Trindle's own marks and links in /etc and /run, and the kept /etc/resolv.conf put back under its name when blocking goes off"),
    ("admin/shares.rs", 6, "root's own file of a share's password in /run, cleared before it is made new and removed when done, and the empty folder a share (Windows or NFS) was opened on, which goes only if it is empty"),
    ("admin/snapshots.rs", 5, "an empty folder in /run and the empty one a new copy of the system comes with, and the system's own folders moved by name or traded in one step in a restore the person asked for, the present one set aside whole, and the mark that says the restore is finished"),
    ("admin/sources.rs", 2, "a second copy of a source's list that its own package added, and a leaving of a program the person has just had removed"),
    ("common/update_asked.rs", 2, "the ring's one-time note that its update button was pressed by the person, in the session's own folder: cleared before a new one is made and removed as Apps reads it, never a thing of the person's"),
    ("common/autostart.rs", 1,"a launcher in the person's own autostart folder and nowhere else, taken away only when they press to stop it starting and press again to say they mean it"),
    ("common/clipboard.rs", 1, "a copied picture in the clipboard's own private folder that no item in the list points to any more"),
    ("common/copies.rs", 4, "the shared copiers themselves: their own spare file removed after a failure, or renamed into the place of one of Trindle's own files"),
    ("toolkit/common/files.rs", 11, "the shared writers themselves: their own spare file removed after a failure or renamed into place, and the second way of the rename that takes no name already taken, which removes only a name of the very thing just moved"),
    ("toolkit/sys/os.rs", 1, "the rename that takes no name already taken, which common::files::rename_new calls"),
    ("common/grants.rs", 2, "an empty mark of a leave just given, in the session's own private folder: an old one goes before it is made new, and whoever removes it is the one that takes the leave"),
    ("common/held.rs", 2, "its own spare file, renamed over the list it was written for, or removed when the writing failed"),
    ("common/in_use.rs", 4, "its own spare file renamed over the note the ring reads, the microphone mute mark taken away when the switch is turned off, and a running box's own note taken away when the box ends or found left by a program that was killed, all in the session's own folder and made again at each login"),
    ("store/home_net.rs", 2, "pasta's own process note in the box's own run folder: cleared before a new one is made and removed when the box ends; never a thing of the person's"),
    ("common/locked.rs", 1, "the empty mark that the screen is locked, in the session's own folder, taken away at unlock"),
    ("common/outputs.rs", 2, "the old file of the screens' arrangement, in the move across and when saved, once all it held has been written whole to the new one"),
    ("common/packs.rs", 5, "the folders an archive of fonts is unpacked in and a font waiting for an answer is kept in, of names of Trindle's own, cleared before and after; a font that waits is moved to the folder kept for it, and taken away when the answer is no, or the yes has put it in place; the fonts found are copied out and never over another unasked"),
    ("common/pin.rs", 1, "the kept hash of the PIN, in Trindle's own settings, removed when the person stops using a PIN"),
    ("common/ring_said.rs", 1, "one word the ring left for the compositor, in the session's own folder, once the person has been shown it"),
    ("common/themes.rs", 3, "a theme's own spare file, cleared, made new and renamed into place, and one of the person's own themes, removed only when they ask and say again that they mean it"),
    ("compose/rest.rs", 1, "an empty switch for one session, in Trindle's own settings, used up as it is read"),
    ("disks/job.rs", 1, "a note of a run of changes to a drive, in Trindle's own folder of such notes: one left by a run that was cut short, once it has been read and told"),
    ("disks/steps.rs", 1, "a note of a run of changes to a drive, in Trindle's own folder of such notes: this run's own when it ends"),
    ("dns/list.rs", 3, "the fetched lists: each part once it has been read, and the new lists' spare files renamed over the old; they are fetched again when missing"),
    ("dns/mod.rs", 2, "the resolver's own files in /run: a spare one renamed over the list of servers, and the note for the system's resolver taken away when blocking stops; both are written again when it starts"),
    ("toolkit/common/file_work/proof.rs", 1, "the half-made copy this work made itself, while its name still leads to it: the third of the rule's own three, whose other two are in toolkit/sys/disk.rs"),
    ("toolkit/sys/disk.rs", 2, "the rule's own three: a thing given a name of this work's own within its opened folder, one name removed within an opened folder once it is proved to be the thing copied or asked about, and a half-made copy while its name still leads to it"),
    ("toolkit/common/file_work/trash.rs", 3, "the Trash's own note of an item: one made for a name that turned out taken or for a move that failed, and one whose item has left the Trash"),
    ("toolkit/common/file_work/work/finish.rs", 3, "folders a move has emptied, by the call that cannot remove one with anything in it, and what this work made under a spare name of its own and could not put in place; Delete itself goes by proof::Asked"),
    ("toolkit/common/file_work/work/leave.rs", 1, "a folder a move has emptied, by the call that cannot remove one with anything in it"),
    ("toolkit/common/file_work/clip.rs", 1, "the program's own note of what it last cut, in the session's own folder, once the cut is pasted or something else is copied"),
    ("icons/appimages.rs", 3, "the folder an AppImage is looked into, a launcher Trindle made (it bears its mark) for an AppImage that has gone, and an AppImage in its own store removed when the person asks and says again"),
    ("icons/mod.rs", 2, "icons Trindle drew, known by its mark inside them, for apps that have gone; they are drawn again when wanted"),
    ("installer/look.rs", 2, "the empty folders made in /run to look into each part of a drive, which go only if they are empty"),
    ("installer/steps.rs", 5, "in the new system while it is installed: the time zone's link before it is made again, and what belongs to the stick alone, its machine's number, its login keys, its installer's launchers and its welcome"),
    ("keyring/pam.rs", 1, "the key handed over at login, in root's own folder, removed when no password was typed, so no old key is mistaken for this one's"),
    ("keyring/migrate.rs", 1, "the keyring and its spare copy renamed aside under a name with the time in it, so both are kept"),
    ("keyring/store.rs", 2, "the keyring's own spare file renamed into place, and the copy of the keyring before that, replaced by a second name for the one about to be replaced"),
    ("mcp/tools.rs", 3, "the oldest of Trindle's own copies of the settings past the few it keeps, and, in an undo the person approved, the settings file and apps' permissions that were not there when the copy was taken"),
    ("plugins/catalogue.rs", 5, "the folder made for one install, a link lying where the kept catalogue goes, removed so it is never written through, a fetched icon of a plugin that has been removed, fetched again if wanted, a fetched picture that does not match the hash the signed index gives, and the damaged record of the catalogue's number, taken away by name inside the asking person's own folder at their request, behind the administrator's password"),
    ("plugins/log.rs", 3, "a plugin's own log: the full one renamed to be the one before, over the one before that, and both removed when the plugin goes"),
    ("plugins/background.rs", 3, "a plugin's running part: its lock, the number of its running program and the folder made for its box"),
    ("plugins/install.rs", 10, "a plugin's own folder: its new copy (taken away again if the swap fails), the old program it replaces, which is set aside, removed only once the new one is in and put back if that fails, and the whole of it when the person removes it"),
    ("plugins/notes.rs", 4, "what a plugin planted where Trindle writes in its home, a link or anything else that is no plain file, and the spare files renamed into place"),
    ("plugins/permissions.rs", 2, "the mark of what the person has been shown and allowed, taken away with the allowance it records; and Trindle's own first-time marker, removed only when a grant failed so the first-time grant can try again"),
    ("screenshot/mod.rs", 2, "the private folder made to photograph the screen in, once the picture has been kept or could not be taken"),
    ("screenshot/portal.rs", 1, "the oldest of the screenshots handed to other programs, in Trindle's own cache, past the few that are kept"),
    ("screenshot/portal_cast.rs", 2, "the note of who is sharing the screen, in the session's own folder: its spare file renamed into place, and the note removed when nobody is"),
    ("screenshot/portal_notify.rs", 3, "a notice's picture, under a name of its own, removed when the notice goes or could not be shown"),
    ("session/idle.rs", 7, "marks in the session's own folder, gone at logout: that the screen is kept awake, by the person or by a program since ended, the brightness to go back to, this program's number, and its full log renamed to the old one"),
    ("session/screensaver.rs", 4, "this program's number and when it began, in the session's own folder, removed when it stops"),
    ("session/start.rs", 2, "the session's and the login screen's own logs, each renamed to be the one before as a new one is begun"),
    ("session/wallpaper.rs", 1, "this program's number, in the session's own folder, when it could not start"),
    ("session/wsdd.rs", 1, "the socket of an earlier one that no longer answers, in the session's own folder, before a new one is made under the name"),
    ("settings/panels/crash_reports.rs", 1, "Trindle's own list of reports sent, cleared when the person asks and says again"),
    ("settings/panels/sounds.rs", 1, "a sound the person added to their own sounds, removed when they press to remove it and press again to say they mean it"),
    ("store/backup.rs", 4, "the private folders a backup is put together in and unpacked in, the backup put together under a spare name and renamed into place only when whole, and that spare file when it could not be made, so no half backup is left to be trusted and a good one of the same name is never lost"),
    ("store/boxed/install.rs", 9, "an app's own box: files built anew from kept parts and swapped for the old, notes and downloads of an install, and the whole of it when the person removes it"),
    ("store/boxed/launchers.rs", 1, "an app's own box: launchers bearing its mark"),
    ("store/boxed/run.rs", 2, "an app's own box: its folder for this run"),
    ("store/copied_guard.rs", 6, "in the session's own folder, gone at logout: the socket copied files are handed over by, and the ring's row saying a plugin is reading them, its spare file renamed into place"),
    ("store/flat.rs", 5, "in a restore the person asked for: the data folder the install of an app has just made, or the app's own from before, set aside and removed only once the one in the backup is in place (given its name back if that fails), the part of the backup unpacked to do it, and the folder that was unpacked in"),
    ("store/github.rs", 5, "an old plain file that held a key, when the key is given up, answers kept in the cache for a day, and a download: its part file renamed to its name or removed, and a bundle once it is installed"),
    ("store/key.rs", 2, "plain files an older version kept a key in: the second copies, and the file itself only once the keyring holds the key"),
    ("store/hub.rs", 1, "the fetched list's part file, renamed over the old list once whole; it is fetched again"),
    ("store/came_from.rs", 1, "the note of where an AppImage came from, removed when it is no longer known; a note, never the person's file"),
    ("settings/wordsheet/own.rs", 3, "the person's own language folder, kept whole as <tag>.bak when they remove it; an older .bak goes first, and the person asked for both"),
    ("store/origin.rs", 2, "the fetched list's part file, renamed over the old list once whole or removed; it is fetched again when missing"),
    ("store/pages/pictures.rs", 2, "a screenshot of an app fetched to a part file in the cache, renamed to its name once whole or removed; it is fetched again"),
    ("store/pages/add_file.rs", 1, "an AppImage's fetched copy let go once it is in place"),
    ("store/parts.rs", 7, "the store of unpacked packages: a part's folder made under a name of its own and renamed into place, an earlier part's file a later one replaces in a box being built, and parts no box lists any more"),
    ("store/patch.rs", 2, "a fetched package once it has been unpacked into the store, and the folder the fixes were fetched into"),
    ("store/put_back.rs", 2, "the folder made for one putting back of a backup, removed with what was not moved out of it, and each thing that is put back moved by its one name from that opened folder into the opened one it goes to"),
    ("store/secctx.rs", 1, "the socket of an earlier run in the folder made for this box's run, before a new one is made under the name"),
    ("text/store.rs", 6, "the kept words of a tab that is closed, which asks first when they were never saved, and the notes of a save once the document is whole again or they turn out to be no save's"),
    ("thunder/allowed.rs", 2, "the list's own spare file, readable by root alone, cleared, made new and renamed over the old list"),
    ("compose/xrooms.rs", 1, "the socket of an earlier run, in the session's own folder, before a new one is made under the name"),
    ("thunder/mod.rs", 1, "the service's own socket, left by an earlier run, before a new one is made under the name"),
    ("unlock/fifo.rs", 1, "the pipe the unlock is told through, in the early start's own memory, removed when done"),
    ("unlock/spin.rs", 1, "this program's number, in /run, when it is still this program's as it stops"),
    ("voice/mod.rs", 2, "the recording just made to be turned into words, in the session's own folder, once it has been read or was too short to hold any"),
    ("voice/setup.rs", 4, "the private folders the speech parts are built and fetched in, once they are installed or could not be, and the note of build tools left behind, taken away when they go or when voice does"),
];

fn ways() -> [String; 8] {
    [["fs::", "write("].concat(), ["File::", "create("].concat(), [".truncate", "(true)"].concat(), ["fs::", "copy("].concat(),
     ["set_", "len("].concat(), [".append", "(true)"].concat(), ["libc::O_", "WRONLY"].concat(), ["libc::O_", "RDWR"].concat()]
}

fn opened_to_write() -> String {
    [".write", "(true)"].concat()
}

fn made_new() -> String {
    ["create_", "new("].concat()
}

const NEAR: usize = 2;

fn takes() -> [String; 6] {
    [["fs::", "rename("].concat(), ["remove_", "file("].concat(), ["remove_", "dir_all("].concat(), ["remove_", "dir("].concat(),
     ["libc::", "rename"].concat(), ["libc::", "unlink"].concat()]
}

fn for_tests() -> String {
    ["#[cfg(", "test)]"].concat()
}

fn for_tests_too() -> String {
    ["#[cfg(any(", "test, feature = \"testing\"))]"].concat()
}

pub(crate) fn code(text: &str) -> Vec<&str> {
    let lines: Vec<&str> = text.lines().collect();
    let (mark, too) = (for_tests(), for_tests_too());
    let depth = |l: &str| l.len() - l.trim_start().len();
    let mut out = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        if !lines[i].trim_start().starts_with(&mark) && !lines[i].trim_start().starts_with(&too) {
            out.push(lines[i]);
            i += 1;
            continue;
        }
        let deep = depth(lines[i]);
        let mut first = i + 1;
        while lines.get(first).is_some_and(|l| l.trim_start().starts_with("#[")) {
            first += 1;
        }
        let begun = lines.get(first).map_or("", |l| l.trim_end());
        let whole = begun.ends_with(';') || begun.ends_with(',') || (begun.ends_with('}') && begun.contains('{'));
        let block = begun.ends_with('{');
        let ends = |l: &str| l.trim_start().starts_with('}') || (!block && (l.trim_end().ends_with(';') || l.trim_end().ends_with(',')));
        let end = if whole { first } else { (first + 1..lines.len()).find(|&k| depth(lines[k]) == deep && ends(lines[k])).unwrap_or(lines.len() - 1) };
        let end = end.min(lines.len() - 1);
        out.extend(std::iter::repeat_n("", end + 1 - i));
        i = end + 1;
    }
    out
}

fn count(text: &str, ways: &[String]) -> usize {
    let lines = code(text);
    let (opened, new) = (opened_to_write(), made_new());
    let said = |l: &str| !l.trim_start().starts_with("//");
    (0..lines.len())
        .filter(|&i| said(lines[i]))
        .filter(|&i| {
            let near = &lines[i.saturating_sub(NEAR)..(i + NEAR + 1).min(lines.len())];
            ways.iter().any(|w| lines[i].contains(w.as_str())) || (lines[i].contains(&opened) && !near.iter().any(|l| l.contains(&new)))
        })
        .count()
}

fn taken(text: &str, takes: &[String]) -> usize {
    code(text).iter().filter(|l| !l.trim_start().starts_with("//") && takes.iter().any(|w| l.contains(w.as_str()))).count()
}

pub(crate) fn all_code() -> Vec<(String, String)> {
    let mut files = Vec::new();
    let mut names = std::collections::HashMap::new();
    for (src, prefix) in crate::common::source::trees() {
        let before = files.len();
        sources(&src, &mut files);
        for f in &files[before..] {
            names.insert(f.clone(), format!("{prefix}{}", f.strip_prefix(&src).unwrap_or(f).to_string_lossy()));
        }
    }
    let texts: Vec<(PathBuf, String)> = files.into_iter().filter_map(|p| std::fs::read_to_string(&p).ok().map(|t| (p, t))).collect();
    let mut tests = BTreeSet::new();
    for (path, text) in &texts {
        only_for_tests(path, text, &mut tests);
        if path.file_name().is_some_and(|n| n.to_string_lossy().ends_with("_test.rs")) {
            tests.insert(path.clone());
        }
    }
    texts.into_iter()
        .filter(|(path, _)| !path.ancestors().any(|a| tests.contains(a)))
        .map(|(path, text)| (names[&path].clone(), text))
        .collect()
}

fn only_for_tests(path: &Path, text: &str, out: &mut BTreeSet<PathBuf>) {
    let lines: Vec<&str> = text.lines().collect();
    let own = matches!(path.file_name().and_then(|n| n.to_str()), Some("mod.rs" | "main.rs" | "lib.rs"));
    let Some(dir) = path.parent() else { return };
    let dir = if own { dir.to_path_buf() } else { dir.join(path.file_stem().unwrap_or_default()) };
    for (i, line) in lines.iter().enumerate() {
        let name = line.trim().trim_start_matches("pub(crate) ").trim_start_matches("pub ").strip_prefix("mod ").and_then(|m| m.strip_suffix(';'));
        let marked = lines[..i].iter().rev().take_while(|l| l.trim().starts_with("#[")).any(|l| l.trim().starts_with("#[cfg(test)]") || l.trim().starts_with(&for_tests_too()));
        if let (true, Some(name)) = (marked, name) {
            out.insert(dir.join(format!("{name}.rs")));
            out.insert(dir.join(name));
        }
    }
}

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(list) = std::fs::read_dir(dir) else { return };
    for entry in list.flatten() {
        let path = entry.path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|x| x == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn a_write_is_counted_outside_comments_and_the_files_own_tests() {
    let ways = ways();
    let [write, create, truncate, copy, length, append, ..] = &ways;
    let text = format!("fn a() {{ std::{write}p, b) }}\n// std::{write}p, b)\nlet f = {create}p);\nopen{truncate}.open(p);\n{copy}a, b);\n\
                        fs::read(p);\n#[cfg(test)]\nmod tests {{\n    std::{write}p, b);\n}}\n");
    assert_eq!(count(&text, &ways), 4);
    let (opened, new) = (opened_to_write(), made_new());
    let unseen = format!("let f = OpenOptions::new(){opened}.open(p)?;\nf.{length}0)?;\nlet g = OpenOptions::new(){append}.open(q)?;\n\
                          let h = File::options().read(true){opened}.create(true).truncate(false).open(r)?;\nfs::read(p);\n");
    assert_eq!(count(&unseen, &ways), 4, "a file opened to be written, given a length or added to is written where it lies");
    let fresh = format!("let f = OpenOptions::new(){opened}.{new}true).open(p)?;\nlet g = OpenOptions::new()\n    {opened}\n    .{new}true)\n    .open(q)?;\n");
    assert_eq!(count(&fresh, &ways), 0);
    let far = format!("let f = OpenOptions::new(){opened}.open(p)?;\n\n\n\nlet g = OpenOptions::new().{new}true).open(q)?;\n");
    assert_eq!(count(&far, &ways), 1, "one made new some lines off excuses nothing");
    let [rename, file, all, dir, ..] = &takes();
    let gone = format!("{rename}a, b)?;\nstd::fs::{file}a)?;\n// fs::{file}a)\nfs::{all}d)?;\nfs::{dir}d)?;\nfs::read(p);\n#[cfg(test)]\nmod tests {{\n    fs::{file}a);\n}}\n");
    assert_eq!(taken(&gone, &takes()), 4);
    let mark = for_tests();
    let amid = format!("{rename}a, b)?;\n{mark}\nmod tests {{\n    fs::{file}a);\n    fn f() {{\n    }}\n}}\n\nfn later() {{\n    fs::{file}a)?;\n    {mark}\n    fs::{file}t);\n    \
                        {mark}\n    if testing() {{\n        fs::{all}d);\n    }}\n    fs::{dir}d)?;\n}}\n{mark}\npub fn only_tests() {{\n    fs::{file}a);\n}}\n{mark}\n#[allow(dead_code)]\n\
                        fn long(\n    a: u8,\n) {{\n    fs::{file}a);\n}}\nfn last() {{ fs::{all}d); }}\n{mark}\nmod at_the_end {{\n    fs::{file}a);\n}}\n");
    assert_eq!(taken(&amid, &takes()), 4, "the code after a module of tests is counted, and the tests are not");
    assert_eq!(code(&amid).len(), amid.lines().count(), "each line stays where it was");
    let raw = format!("let fd = libc::open(p, {} | libc::O_APPEND, 0o600);\n", ways[6]);
    assert_eq!(count(&raw, &ways), 1, "a file opened to be written by the kernel's own flag is written where it lies");
    let mut found = BTreeSet::new();
    only_for_tests(Path::new("src/files/mod.rs"), "mod act;\n#[cfg(test)]\nmod shots;\n#[cfg(test)]\npub mod rig;\n", &mut found);
    assert!(found.contains(Path::new("src/files/shots.rs")) && found.contains(Path::new("src/files/rig")) && !found.contains(Path::new("src/files/act.rs")));
}

#[test]
fn nothing_is_written_over_where_it_lies_but_what_is_listed() {
    let ways = ways();
    let found: BTreeMap<String, usize> = all_code().into_iter().map(|(file, text)| (file, count(&text, &ways))).filter(|(_, n)| *n > 0).collect();
    let listed: BTreeMap<String, usize> = ALLOWED.iter().map(|(file, n, _)| (file.to_string(), *n)).collect();
    assert_eq!(listed.len(), ALLOWED.len(), "a file is listed twice");
    assert!(ALLOWED.iter().all(|(_, _, why)| !why.is_empty()), "each one listed says why");
    let wrong: Vec<String> = found.iter().filter(|(file, n)| listed.get(*file) != Some(n))
        .map(|(file, n)| format!("{file} writes a file where it lies {n} times, and {} are listed", listed.get(file).copied().unwrap_or(0)))
        .chain(listed.keys().filter(|file| !found.contains_key(*file)).map(|file| format!("{file} is listed and no longer writes any")))
        .collect();
    assert!(wrong.is_empty(), "use common::files::write_whole, or list it in ALLOWED with its reason: {wrong:#?}");
}

#[test]
fn nothing_is_taken_away_or_named_anew_but_where_it_is_counted() {
    let takes = takes();
    let found: BTreeMap<String, usize> = all_code().into_iter().map(|(file, text)| (file, taken(&text, &takes))).filter(|(_, n)| *n > 0).collect();
    let listed: BTreeMap<String, usize> = TAKES.iter().map(|(file, n, _)| (file.to_string(), *n)).collect();
    assert_eq!(listed.len(), TAKES.len(), "a file is listed twice");
    assert!(TAKES.iter().all(|(_, _, why)| !why.is_empty()), "each one counted says why");
    let wrong: Vec<String> = found.iter().filter(|(file, n)| listed.get(*file) != Some(n))
        .map(|(file, n)| format!("{file} takes a thing away or names it anew {n} times, and {} are counted", listed.get(file).copied().unwrap_or(0)))
        .chain(listed.keys().filter(|file| !found.contains_key(*file)).map(|file| format!("{file} is counted and no longer takes any away")))
        .collect();
    assert!(wrong.is_empty(), "a person's own thing goes only by its proof or into the Trash, and is named only by common::files::rename_new; \
                               when the new one has been looked at, count it in TAKES and say there why it may stay: {wrong:#?}");
}
