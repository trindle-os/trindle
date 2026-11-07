# Settings, Tailscale page: a private network between the person's own devices.

## Tailscale page (src/settings/panels/tailscale.rs)

# What a device runs, shown in a device's small line, when it is an iPhone or iPad.
settings-tailscale-system-ios = iPhone or iPad
# In a device's small line: the device is on the network now.
settings-tailscale-online = online now
# In a device's small line: the device has never been seen.
settings-tailscale-not-seen = not seen yet
# In a device's small line: when the device was last online; the date is already written.
settings-tailscale-last-online = last online { $day }
# Shown beside the Use Tailscale switch while the password is asked for, when Tailscale is already installed.
settings-tailscale-waiting = Waiting for the password...
# Shown beside the Use Tailscale switch while the password is asked for, before Tailscale is installed.
settings-tailscale-waiting-install = Waiting for the password, then installing Tailscale...
# Shown beside the switch after Tailscale is turned on.
settings-tailscale-done-on = Tailscale on
# Shown beside the switch after Tailscale is turned off.
settings-tailscale-done-off = Tailscale off
# Shown under the Sign in line once the browser is being opened.
settings-tailscale-opening = Opening the sign-in page in your browser...
# Red words under the Sign in line when Tailscale could not be started.
settings-tailscale-not-started = Tailscale could not be started.
# Red words under the Sign out line when signing out did not work.
settings-tailscale-sign-out-failed = Tailscale would not sign out.
# Headline of the page while Tailscale is being asked how it is.
settings-tailscale-top-asking = Asking Tailscale…
# Headline of the page when Tailscale is off and not installed.
settings-tailscale-top-not-installed = Tailscale is not installed. Switch it on below to install it.
# Headline of the page when Tailscale is installed but off.
settings-tailscale-top-off = Tailscale is off, so it uses no memory.
# Headline of the page when Tailscale is on but does not answer.
settings-tailscale-top-silent = Tailscale is not answering. It may still be starting.
# Headline of the page when connected: the name of the private network and of this computer.
settings-tailscale-top-connected = Connected to { $network } as { $name }.
# Name used in the headline for the private network when it has no name of its own.
settings-tailscale-network-fallback = Tailscale
# Headline of the page when Tailscale is on and has to be signed in to.
settings-tailscale-top-sign-in = Tailscale is on. Sign in below to join your private network.
# Headline of the page for any other state; the state is already written in lower case.
settings-tailscale-top-state = Tailscale is { $state }.
# Name of the page in the list of settings pages.
settings-tailscale-title = Tailscale
# Words the settings search looks for on this page (not shown).
settings-tailscale-keywords = tailscale vpn private network devices remote tailnet
# Help text at the top of the page.
settings-tailscale-help = Tailscale joins your own computers and phones into one private network, wherever each one is. Nothing of it is installed until you switch it on, and while it is off it uses no memory.
# Small line under the Use Tailscale switch before Tailscale is installed.
settings-tailscale-switch-sub-first = Installs it from Tailscale's own source.
# Small line under the Use Tailscale switch once Tailscale is installed.
settings-tailscale-switch-sub = Off: stopped, not started with the computer.
# Text of the switch that turns Tailscale on or off, and heading of its line.
settings-tailscale-switch = Use Tailscale
# Help text (i button) of the Tailscale card when Tailscale is off or has not answered.
settings-tailscale-about-plain = The first time it is switched on, Tailscale is installed from Tailscale's own software source, then started. Its background service uses memory while on, so it only runs when this is on. While this is off, Tailscale is stopped and does not start with the computer. The firewall lets your own Tailscale devices in, and keeps everything else out as before.
# Help text (i button) of the Tailscale card when Tailscale is on but not yet connected.
settings-tailscale-about-sign-in = { settings-tailscale-about-plain } Signing in opens Tailscale's sign-in page in your browser. Use the same account on each of your devices.
# Help text (i button) of the Tailscale card when connected.
settings-tailscale-about-connected = { settings-tailscale-about-plain } Signing out takes this computer off your private network until you sign in again.
# Line in the devices card when there are no other devices.
settings-tailscale-no-devices = No other devices yet
# Small line under "No other devices yet".
settings-tailscale-no-devices-sub = Sign in on another with the same account.
# Shown on the devices card while Tailscale is off.
settings-tailscale-waits-off = Switch on Tailscale to see your devices.
# Shown on the devices card while Tailscale has not answered.
settings-tailscale-waits-looking = Looking for your devices…
# Shown on the devices card when Tailscale needs signing in to.
settings-tailscale-waits-sign-in = Sign in to see your devices.
# Shown on the devices card when Tailscale is on but not connected.
settings-tailscale-waits-connect = Connect to see your devices.
# Small line under this computer when it has no address yet.
settings-tailscale-no-address = No address yet.
# Small line under this computer: its address on the private network.
settings-tailscale-address = { $address } on your private network.
# Name of this computer's line when Tailscale gives it no name.
settings-tailscale-this-computer = This computer
# Heading of the line that signs in to Tailscale, and tip of its round button.
settings-tailscale-sign-in = Sign in
# Small line under the Sign in line.
settings-tailscale-sign-in-sub = Opens the sign-in page in your browser.
# Heading of the line that joins the private network again, and tip of its round button.
settings-tailscale-connect = Connect
# Small line under the Connect line.
settings-tailscale-connect-sub = Joins your private network again.
# Heading of the line that says the firewall lets the person's own devices in.
settings-tailscale-firewall = Your own devices get in
# Small line under that heading.
settings-tailscale-firewall-sub = Everything else is kept out, as before.
# Heading of the line that signs out, and tip of its button.
settings-tailscale-sign-out = Sign out
# Small line under the Sign out line.
settings-tailscale-sign-out-sub = Leaves your private network.
# Heading of the card with the Tailscale switch.
settings-tailscale-card = Tailscale
# Heading of the card that lists the person's devices.
settings-tailscale-devices = Your devices
# Help text (i button) of the Your devices card.
settings-tailscale-devices-about = Every device signed in to your private network, with its address on it, what it runs and when it was last online. To add one, install Tailscale on your phone or another computer and sign in with the same account.
