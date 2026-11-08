
# machine translated (German), not yet reviewed by a person.

# What a device runs, shown in a device's small line, when it is an iPhone or iPad.
settings-tailscale-system-ios = iPhone oder iPad

# In a device's small line: the device is on the network now.
settings-tailscale-online = jetzt online

# In a device's small line: the device has never been seen.
settings-tailscale-not-seen = noch nicht gesehen

# In a device's small line: when the device was last online; the date is already written.
settings-tailscale-last-online = zuletzt online { $day }

# Shown beside the Use Tailscale switch while the password is asked for, when Tailscale is already installed.
# SAFETY: needs a human review before it ships
settings-tailscale-waiting = Warte auf das Passwort...

# Shown beside the Use Tailscale switch while the password is asked for, before Tailscale is installed.
# SAFETY: needs a human review before it ships
settings-tailscale-waiting-install = Warte auf das Passwort, dann wird Tailscale installiert...

# Shown beside the switch after Tailscale is turned on.
settings-tailscale-done-on = Tailscale an

# Shown beside the switch after Tailscale is turned off.
settings-tailscale-done-off = Tailscale aus

# Shown under the Sign in line once the browser is being opened.
settings-tailscale-opening = Die Anmeldeseite wird in deinem Browser geöffnet...

# Red words under the Sign in line when Tailscale could not be started.
settings-tailscale-not-started = Tailscale konnte nicht gestartet werden.

# Red words under the Sign out line when signing out did not work.
settings-tailscale-sign-out-failed = Tailscale ließ sich nicht abmelden.

# Headline of the page while Tailscale is being asked how it is.
settings-tailscale-top-asking = Tailscale wird gefragt…

# Headline of the page when Tailscale is off and not installed.
settings-tailscale-top-not-installed = Tailscale ist nicht installiert. Schalte es unten ein, um es zu installieren.

# Headline of the page when Tailscale is installed but off.
settings-tailscale-top-off = Tailscale ist aus und braucht deshalb keinen Speicher.

# Headline of the page when Tailscale is on but does not answer.
settings-tailscale-top-silent = Tailscale antwortet nicht. Vielleicht startet es noch.

# Headline of the page when connected: the name of the private network and of this computer.
settings-tailscale-top-connected = Verbunden mit { $network } als { $name }.

# Name used in the headline for the private network when it has no name of its own.
settings-tailscale-network-fallback = Tailscale

# Headline of the page when Tailscale is on and has to be signed in to.
settings-tailscale-top-sign-in = Tailscale ist an. Melde dich unten an, um deinem privaten Netzwerk beizutreten.

# Headline of the page for any other state; the state is already written in lower case.
settings-tailscale-top-state = Tailscale ist { $state }.

# Name of the page in the list of settings pages.
settings-tailscale-title = Tailscale

# Words the settings search looks for on this page (not shown).
settings-tailscale-keywords = tailscale vpn privates netzwerk geräte fern tailnet

# Help text at the top of the page.
settings-tailscale-help = Tailscale verbindet deine eigenen Computer und Telefone zu einem privaten Netzwerk, wo auch immer sie sind. Bis du es einschaltest, ist nichts davon installiert, und solange es aus ist, braucht es keinen Speicher.

# Small line under the Use Tailscale switch before Tailscale is installed.
settings-tailscale-switch-sub-first = Installiert es aus der eigenen Quelle von Tailscale.

# Small line under the Use Tailscale switch once Tailscale is installed.
settings-tailscale-switch-sub = Aus: angehalten, nicht mit dem Computer gestartet.

# Text of the switch that turns Tailscale on or off, and heading of its line.
settings-tailscale-switch = Tailscale verwenden

# Help text (i button) of the Tailscale card when Tailscale is off or has not answered.
settings-tailscale-about-plain = Beim ersten Einschalten wird Tailscale aus der eigenen Softwarequelle von Tailscale installiert und dann gestartet. Sein Hintergrunddienst braucht im Betrieb Speicher und läuft deshalb nur, wenn dies eingeschaltet ist. Solange es aus ist, ist Tailscale angehalten und startet nicht mit dem Computer. Die Firewall lässt deine eigenen Tailscale-Geräte herein und hält alles andere wie bisher draußen.

# Help text (i button) of the Tailscale card when Tailscale is on but not yet connected.
settings-tailscale-about-sign-in = { settings-tailscale-about-plain } Beim Anmelden öffnet sich die Anmeldeseite von Tailscale in deinem Browser. Verwende auf jedem deiner Geräte dasselbe Konto.

# Help text (i button) of the Tailscale card when connected.
settings-tailscale-about-connected = { settings-tailscale-about-plain } Beim Abmelden ist dieser Computer nicht mehr in deinem privaten Netzwerk, bis du dich wieder anmeldest.

# Line in the devices card when there are no other devices.
settings-tailscale-no-devices = Noch keine anderen Geräte

# Small line under "No other devices yet".
settings-tailscale-no-devices-sub = Melde dich auf einem anderen mit demselben Konto an.

# Shown on the devices card while Tailscale is off.
settings-tailscale-waits-off = Schalte Tailscale ein, um deine Geräte zu sehen.

# Shown on the devices card while Tailscale has not answered.
settings-tailscale-waits-looking = Deine Geräte werden gesucht…

# Shown on the devices card when Tailscale needs signing in to.
settings-tailscale-waits-sign-in = Melde dich an, um deine Geräte zu sehen.

# Shown on the devices card when Tailscale is on but not connected.
settings-tailscale-waits-connect = Verbinde dich, um deine Geräte zu sehen.

# Small line under this computer when it has no address yet.
settings-tailscale-no-address = Noch keine Adresse.

# Small line under this computer: its address on the private network.
settings-tailscale-address = { $address } in deinem privaten Netzwerk.

# Name of this computer's line when Tailscale gives it no name.
settings-tailscale-this-computer = Dieser Computer

# Heading of the line that signs in to Tailscale, and tip of its round button.
settings-tailscale-sign-in = Anmelden

# Small line under the Sign in line.
settings-tailscale-sign-in-sub = Öffnet die Anmeldeseite in deinem Browser.

# Heading of the line that joins the private network again, and tip of its round button.
settings-tailscale-connect = Verbinden

# Small line under the Connect line.
settings-tailscale-connect-sub = Tritt deinem privaten Netzwerk wieder bei.

# Heading of the line that says the firewall lets the person's own devices in.
settings-tailscale-firewall = Deine eigenen Geräte kommen herein

# Small line under that heading.
settings-tailscale-firewall-sub = Alles andere bleibt draußen, wie bisher.

# Heading of the line that signs out, and tip of its button.
settings-tailscale-sign-out = Abmelden

# Small line under the Sign out line.
settings-tailscale-sign-out-sub = Verlässt dein privates Netzwerk.

# Heading of the card with the Tailscale switch.
settings-tailscale-card = Tailscale

# Heading of the card that lists the person's devices.
settings-tailscale-devices = Deine Geräte

# Help text (i button) of the Your devices card.
settings-tailscale-devices-about = Jedes in deinem privaten Netzwerk angemeldete Gerät, mit seiner Adresse darin, dem, was es ausführt, und wann es zuletzt online war. Um eines hinzuzufügen, installiere Tailscale auf deinem Telefon oder einem anderen Computer und melde dich mit demselben Konto an.
