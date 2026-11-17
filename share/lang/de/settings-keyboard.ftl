
# machine translated (German), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Tastatur und Maus

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = layout sprache tippen feststelltaste bildschirmtastatur schaltfläche ring maus touchpad trackpad zeiger geschwindigkeit tippen klicken scrollen rad langsamer schneller natürlich linkshändig plugin tastenkürzel tasten umbelegen ändern tauschen ctrl escape super alt mac seitentasten angetippt gehalten von programmen erstellt keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Alles hier ändert sich sofort. Mit Alt und Umschalt zusammen kannst du zu einem zweiten Layout wechseln.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Du tippst in { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt und Umschalt wechseln zu { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Die Feststelltaste tut nichts.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Die Feststelltaste wirkt als { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = ein zweites Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = Die Liste der Tastaturlayouts ist auf diesem Computer nicht vorhanden.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Tastatur

# Heading of the line in that card.
settings-keyboard-no-list-heading = Die Liste der Tastaturlayouts ist nicht verfügbar

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Installiere sie mit: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] Du hast eine Taste oder Schaltfläche geändert.
       *[other] Du hast { $n } Tasten und Schaltflächen geändert.
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = Layout

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = Layout

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = Variante

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = Standard

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = Keine

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = Wechseln zu

# Words behind the i of the layout card.
settings-keyboard-layout-about = Hier legst du fest, was jede Taste tippt. Du kannst ein Layout und eine Variante davon wählen. Drücke Alt und Umschalt zusammen, um zu einem zweiten Layout zu wechseln, und noch einmal, um zurückzuwechseln. Änderungen gelten sofort.

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = Gerade in Benutzung

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Feststelltaste

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = Ein zweites Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = Nichts

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Feststelltaste verwenden als

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Tastaturknopf im Ring

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Andere Tasten

# Words behind the i of that card.
settings-keyboard-other-about = Die Feststelltaste kann stattdessen als andere Taste dienen, zum Beispiel als zweites Ctrl oder als Escape. Der Tastaturknopf sitzt in der Mitte des Rings und öffnet eine Tastatur auf dem Bildschirm. Damit kannst du mit der Maus oder einem Touchscreen tippen. Er erscheint, wenn sich der Ring das nächste Mal öffnet.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Ein Programm kann das nicht ändern.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Drücke die Taste oder Maustaste, die geändert werden soll.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = Du hast nichts gedrückt, darum wurde nichts hinzugefügt.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Gib die Taste ein, die stattdessen gelten soll, zum Beispiel Escape oder Ctrl+C, oder gib Nothing ein.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Trindle konnte nicht erkennen, welche Taste das war. Wähle die Taste stattdessen aus der Liste.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Jede Tastatur und Maus

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Du hast diese Taste unten selbst geändert, und diese Änderung gilt.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = { $key } ändern auf

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = { $key } ändern in

# Name of the bin button that takes a change away.
settings-keyboard-take-away = Diese Änderung entfernen

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = Zeile { $n } wurde ignoriert: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Die geänderten Tasten konnten nicht gelesen werden

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Deine Tasten funktionieren wie gewohnt, bis Trindle die Änderungen lesen kann. { $why }

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Wähle die Taste oder Schaltfläche, die geändert werden soll

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Drücke die Taste oder Schaltfläche, die geändert werden soll

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Eine Taste oder Schaltfläche ändern

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = Du hast { $key } schon geändert.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Tasten und Schaltflächen

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Jede Taste und jede Maustaste kann etwas anderes tun. Sie kann zu einer anderen Taste oder Schaltfläche werden, zu zusammen gedrückten Tasten wie Ctrl+C oder zu Tasten, die nacheinander gedrückt werden, wie H, I. Sie kann auch gar nichts tun oder beim Antippen etwas anderes tun als beim Halten, zum Beispiel Escape beim Antippen und Ctrl beim Halten. Das funktioniert überall, schon ab dem Anmeldebildschirm. Mit Ctrl, Alt und einer F-Taste öffnest du immer eine Textkonsole, und wenn du dort trindle-settings --keys default eingibst, ist jede Taste wieder wie vorher.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Ctrl und Feststelltaste tauschen

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Feststelltaste: antippen für Escape, halten für Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Escape und Feststelltaste tauschen

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Alt und Super tauschen, wie bei einer Mac-Tastatur

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Die Seitentasten der Maus kopieren und einfügen

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = { $name } vertrauen

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Bisher keine

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Von Programmen erstellte Tastaturen und Mäuse

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = Programme zum Umbelegen und Spielestarter können eine eigene Tastatur oder Maus erstellen. Ihre Tasten und Klicks können weiterhin tippen und spielen, aber sie können keine geschützten Schaltflächen drücken, wie Herunterfahren oder Erlauben, und keine Frage beantworten, die für dich gedacht ist. Schalte eine hier ein, damit sie wie du zählt.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Controller { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } auf { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = Was { $title } tut

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = A und B tauschen

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = X und Y tauschen

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Oben und unten am rechten Stick vertauschen

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Plugin-Tastenkürzel

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } wird nicht verwendet: { $why }

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Tasten für { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Gespeichert

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = Gilt auch für das Touchpad.

# Title of the mouse card.
settings-keyboard-mouse-title = Maus

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = Die Zeigergeschwindigkeit bestimmt, wie weit sich der Zeiger bewegt, wenn du die Maus bewegst. Sie gilt auch für das Touchpad. Die Scrollgeschwindigkeit bestimmt, wie weit sich eine Seite bei jeder Drehung des Rads bewegt. Wähle Langsamer, Normal oder Schneller oder stelle sie irgendwo auf dem Regler ein. Beim natürlichen Scrollen bewegt sich die Seite in die Richtung, in die das Rad dreht, wie bei einem Handy. Linkshändig tauscht die linke und die rechte Maustaste.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Zeigergeschwindigkeit

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Scrollgeschwindigkeit

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Natürliches Scrollen

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Linkshändig

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Klickhilfe

# Small words under the clicking helper line.
settings-keyboard-clicker-about = Die Klicks eines Verweil- oder Schalterklickers zählen als deine.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Touchpad

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = Die Scrollgeschwindigkeit bestimmt, wie weit sich eine Seite bewegt, wenn du mit zwei Fingern scrollst. Tippen zum Klicken lässt ein leichtes Antippen als Klick zählen. Beim natürlichen Scrollen bewegt sich die Seite in die Richtung, in die sich deine Finger bewegen, wie bei einem Handy. Aus beim Tippen verhindert, dass deine Handfläche den Zeiger bewegt, während du tippst.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Tippen zum Klicken

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Aus beim Tippen

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Langsamer

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Schneller

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB-Anschlüsse

# Words behind the i of that card.
settings-keyboard-usb-about = Wenn ein USB-Anschluss nicht startet, schaltet Trindle ihn einmal aus und wieder ein, um es erneut zu versuchen. Das passiert nie öfter als einmal pro Anschluss.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } Trindle konnte auch die Änderung nicht rückgängig machen: { $undo }



# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Bildschirmfotos, Berichte und Tastenkürzel

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Bildschirmfoto aufnehmen

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Die Liste der Tastenkürzel zeigen

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Jetzt ein Bildschirmfoto aufnehmen

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Die Liste jetzt zeigen

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Die Arbeitsoberfläche neu starten

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Den Ring und die Hilfsprogramme neu starten

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Alles neu starten

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Tasten für { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Jetzt neu starten. Offene Fenster bleiben, wie sie sind.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Erst nachfragen, dann alles neu starten

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Neu gestartet

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = Ein anderes Tastenkürzel verwendet { $keys } schon.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Nicht gespeichert. { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Ein Problem melden

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Jetzt ein Problem melden

# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = Nicht gespeichert. Die Datei mit den geänderten Tasten konnte nicht gelesen werden, darum wurde sie so gelassen, wie sie ist.
