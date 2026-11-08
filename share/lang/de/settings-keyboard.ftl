
# machine translated (German), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Tastatur und Maus

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = layout sprache tippen feststelltaste bildschirmtastatur schaltfläche ring maus touchpad trackpad zeiger geschwindigkeit tippen klicken scrollen rad langsamer schneller natürlich linkshändig plugin tastenkürzel tasten umbelegen ändern tauschen ctrl escape super alt mac seitentasten angetippt gehalten von programmen erstellt keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Alles hier ändert sich sofort. Ein zweites Layout wird mit Alt und Umschalt zusammen umgeschaltet.

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
        [one] Eine Taste ist geändert.
       *[other] { $n } Tasten und Schaltflächen sind geändert.
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

# Small words under the second layout line.
settings-keyboard-switch-to-about = Alt und Umschalt zusammen wechseln zwischen den beiden.

# Words behind the i of the layout card.
settings-keyboard-layout-about = Was jede Taste tippt: das Layout und eine Variante davon, zum Beispiel eine ohne Tottasten. Ein zweites Layout wird mit Alt und Umschalt zusammen eingeschaltet und auf dieselbe Weise zurückgeschaltet. Alles ändert sich sofort.

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
settings-keyboard-caps-acts-as = Die Feststelltaste wirkt als

# Small words under that line.
settings-keyboard-caps-acts-as-about = Ein zweites Ctrl, Escape und so weiter.

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Tastaturknopf im Ring

# Small words under that line.
settings-keyboard-ring-button-about = Blendet eine Tastatur auf dem Bildschirm ein.

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Andere Tasten

# Words behind the i of that card.
settings-keyboard-other-about = Die Feststelltaste kann stattdessen als andere Taste wirken, zum Beispiel als zweites Ctrl oder als Escape. Der Tastaturknopf ist eine Tastatur in der Mitte des Rings, die eine Tastatur auf dem Bildschirm einblendet, zum Tippen mit der Maus oder einem Touchscreen. Er erscheint, wenn sich der Ring das nächste Mal öffnet.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Das kann nur dein eigener Tastendruck ändern.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Drücke die Taste oder Maustaste, die geändert werden soll.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = Es wurde nichts gedrückt, darum wurde nichts hinzugefügt.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Gib ein, was daraus wird, zum Beispiel Escape, Ctrl+C oder Nothing.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Dieser Druck ließ sich nicht erkennen. Wähle die Taste stattdessen aus der Liste.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Jede Tastatur und Maus

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Auch unten geändert; diese Änderung gilt.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = Auf welcher Tastatur oder Maus { $key } geändert wird

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = Was aus { $key } wird

# Name of the bin button that takes a change away.
settings-keyboard-take-away = Diese Änderung entfernen

# Small words under a changed key's line when it changes on every keyboard and mouse.
settings-keyboard-every-device-about = Auf jeder Tastatur und Maus.

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = Zeile { $n } wird nicht verwendet: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Die geänderten Tasten konnten nicht gelesen werden

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Jede Taste ist wie aufgedruckt, bis sie gelesen werden können: { $why }.

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Wähle die Taste oder Schaltfläche, die geändert werden soll

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Drücke die Taste oder Schaltfläche, die geändert werden soll

# Small words under the change a key line.
settings-keyboard-keys-add-about = Drücke sie oder wähle sie aus der Liste.

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Eine Taste oder Schaltfläche ändern

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key } ist unten schon geändert.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Tasten und Schaltflächen

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Jede Taste und jede Maustaste kann etwas anderes tun: eine andere Taste oder Schaltfläche, zusammen gehaltene Tasten wie Ctrl+C, eine Folge von Tasten nacheinander wie H, I, gar nichts, oder eines beim Antippen und etwas anderes beim Halten, zum Beispiel Escape beim Antippen, Ctrl beim Halten. Es wirkt überall, ab dem Anmeldebildschirm. Ctrl, Alt und eine F-Taste erreichen immer eine Textkonsole, und trindle-settings --keys default, dort eingegeben, setzt jede Taste zurück.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Ctrl und Feststelltaste tauschen

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Die Feststelltaste ist Escape beim Antippen, Ctrl beim Halten

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Escape und Feststelltaste tauschen

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Alt und Super tauschen, wie bei einer Mac-Tastatur

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Die Seitentasten der Maus kopieren und einfügen

# Small words under a keyboard or mouse a program made, when it counts as the person.
# SAFETY: needs a human review before it ships
settings-keyboard-made-yes = Zählt als du, wie eine angesteckte Tastatur.

# Small words under a keyboard or mouse a program made, when it does not count as the person.
# SAFETY: needs a human review before it ships
settings-keyboard-made-no = Tippt und spielt. Kann keine geschützten Schaltflächen drücken.

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = { $name } zählt als du

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Bisher keine

# Small words under that line.
settings-keyboard-made-none-about = Eine, die ein Programm erstellt, wird hier angezeigt, damit du sie als dich zählen lassen kannst.

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Von Programmen erstellte Tastaturen und Mäuse

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = Programme zum Umbelegen und Spielestarter können eine eigene Tastatur oder Maus erstellen. Ihre Tasten und Klicks tippen und spielen weiterhin, können aber keine geschützten Schaltflächen drücken, wie Herunterfahren oder Erlauben, und nichts in eine Frage tippen, die für dich gedacht ist. Schalte eine hier ein, um sie als dich zählen zu lassen.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Controller { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } auf { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = Was { $title } tut

# Small words under a controller's changed button; the words Button and what it becomes stand for the two sides of the line typed.
settings-keyboard-pad-type = Gib es als Taste = was daraus wird ein.

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = A und B tauschen

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = X und Y tauschen

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Oben und unten des rechten Sticks umdrehen

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Plugin-Tastenkürzel

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin }. Nicht in Benutzung: { $why }

# Small words under a plugin shortcut; { $plugin } is the plugin. The example is keys a person types.
settings-keyboard-plugin-about = { $plugin }. Gib die Tasten ein, zum Beispiel Ctrl+Alt+S, oder nichts für keine.

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Tasten für { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Gespeichert

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = Auch das Touchpad.

# Title of the mouse card.
settings-keyboard-mouse-title = Maus

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = Die Zeigergeschwindigkeit bestimmt, wie weit der Zeiger bei einer Mausbewegung wandert, und gilt auch für das Touchpad. Die Scrollgeschwindigkeit bestimmt, wie weit sich eine Seite bei jeder Drehung des Rads bewegt: Langsamer, Normal oder Schneller, oder irgendwo auf dem Regler. Beim natürlichen Scrollen bewegt sich die Seite in die Richtung, in die das Rad dreht, wie bei einem Handy. Linkshändig tauscht die linke und die rechte Maustaste.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Zeigergeschwindigkeit

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Scrollgeschwindigkeit

# Small words under the wheel's scroll speed.
settings-keyboard-scroll-wheel-about = Wie weit sich eine Seite pro Radumdrehung bewegt.

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Natürliches Scrollen

# Small words under natural scrolling for the mouse wheel.
settings-keyboard-natural-wheel-about = Die Seite folgt dem Rad, wie bei einem Handy.

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Linkshändig

# Small words under that switch.
settings-keyboard-left-handed-about = Tauscht die linke und die rechte Maustaste.

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Klickhilfe

# Small words under the clicking helper line.
settings-keyboard-clicker-about = Die Klicks eines Verweil- oder Schalterklickers zählen als deine.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Touchpad

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = Die Scrollgeschwindigkeit bestimmt, wie weit sich eine Seite bei zwei Fingern bewegt. Tippen zum Klicken lässt ein leichtes Antippen als Klick zählen. Beim natürlichen Scrollen bewegt sich die Seite in die Richtung, in die sich deine Finger bewegen, wie bei einem Handy. Aus beim Tippen verhindert, dass eine Handfläche, die das Touchpad streift, den Zeiger bewegt, während du tippst.

# Small words under the touchpad's scroll speed.
settings-keyboard-scroll-touchpad-about = Wie weit sich eine Seite bei zwei Fingern bewegt.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Tippen zum Klicken

# Small words under that switch.
settings-keyboard-tap-about = Ein leichtes Antippen zählt als Klick.

# Small words under natural scrolling for the touchpad.
settings-keyboard-natural-touchpad-about = Die Seite folgt deinen Fingern, wie bei einem Handy.

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Aus beim Tippen

# Small words under that switch.
settings-keyboard-quiet-about = Eine streifende Handfläche lässt den Zeiger ruhig.

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Langsamer

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Schneller

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = USB-Anschlüsse

# Words behind the i of that card.
settings-keyboard-usb-about = Wenn ein USB-Anschluss nicht startet, schaltet Trindle ihn einmal aus und wieder ein, um es erneut zu versuchen, und nie öfter als einmal pro Anschluss.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error }; und seine Markierung konnte nicht wieder entfernt werden: { $undo }



# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Bildschirmfoto und Liste der Tastenkürzel

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Bildschirmfoto aufnehmen

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Die Liste der Tastenkürzel zeigen

# Small words under each of the two shortcuts.
settings-keyboard-shortcuts-about = Gib die Tasten ein, oder nichts für keine.

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Jetzt ein Bildschirmfoto aufnehmen

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Die Liste jetzt zeigen

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Die Arbeitsoberfläche neu starten

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Den Ring und die Hilfsteile neu starten

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Alles neu starten

# Small words under each of the two restart shortcuts. The example is keys a person types.
settings-keyboard-restart-about = Gib die Tasten ein, etwa Strg+Alt+Escape, oder nichts für keine.

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Tasten für { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Jetzt neu starten. Offene Fenster bleiben, wie sie sind.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Alles neu starten. Fragt vorher nach.

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Neu gestartet

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } wird schon von einem anderen Tastenkürzel verwendet.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Nicht gespeichert: { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Ein Problem melden

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Jetzt ein Problem melden

