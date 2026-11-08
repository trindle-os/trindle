# machine translated (Russian), not yet reviewed by a person.

# What a device runs, shown in a device's small line, when it is an iPhone or iPad.
settings-tailscale-system-ios = iPhone или iPad

# In a device's small line: the device is on the network now.
settings-tailscale-online = сейчас в сети

# In a device's small line: the device has never been seen.
settings-tailscale-not-seen = ещё не появлялось

# In a device's small line: when the device was last online; the date is already written.
settings-tailscale-last-online = последний раз в сети: { $day }

# Shown beside the Use Tailscale switch while the password is asked for, when Tailscale is already installed.
# SAFETY: needs a human review before it ships
settings-tailscale-waiting = Ожидание пароля...

# Shown beside the Use Tailscale switch while the password is asked for, before Tailscale is installed.
# SAFETY: needs a human review before it ships
settings-tailscale-waiting-install = Ожидание пароля, затем установка Tailscale...

# Shown beside the switch after Tailscale is turned on.
settings-tailscale-done-on = Tailscale включён

# Shown beside the switch after Tailscale is turned off.
settings-tailscale-done-off = Tailscale выключен

# Shown under the Sign in line once the browser is being opened.
settings-tailscale-opening = Страница входа открывается в браузере...

# Red words under the Sign in line when Tailscale could not be started.
settings-tailscale-not-started = Не удалось запустить Tailscale.

# Red words under the Sign out line when signing out did not work.
settings-tailscale-sign-out-failed = Не удалось выйти из Tailscale.

# Headline of the page while Tailscale is being asked how it is.
settings-tailscale-top-asking = Запрос к Tailscale…

# Headline of the page when Tailscale is off and not installed.
settings-tailscale-top-not-installed = Tailscale не установлен. Включите его ниже, чтобы установить.

# Headline of the page when Tailscale is installed but off.
settings-tailscale-top-off = Tailscale выключен, поэтому не занимает память.

# Headline of the page when Tailscale is on but does not answer.
settings-tailscale-top-silent = Tailscale не отвечает. Возможно, он ещё запускается.

# Headline of the page when connected: the name of the private network and of this computer.
settings-tailscale-top-connected = Подключено к сети { $network } как { $name }.

# Name used in the headline for the private network when it has no name of its own.
settings-tailscale-network-fallback = Tailscale

# Headline of the page when Tailscale is on and has to be signed in to.
settings-tailscale-top-sign-in = Tailscale включён. Войдите ниже, чтобы присоединиться к своей частной сети.

# Headline of the page for any other state; the state is already written in lower case.
settings-tailscale-top-state = Состояние Tailscale: { $state }.

# Name of the page in the list of settings pages.
settings-tailscale-title = Tailscale

# Words the settings search looks for on this page (not shown).
settings-tailscale-keywords = tailscale vpn частная сеть устройства удалённый доступ tailnet

# Help text at the top of the page.
settings-tailscale-help = Tailscale объединяет ваши компьютеры и телефоны в одну частную сеть, где бы они ни находились. Пока вы его не включите, ничего не устанавливается, а пока он выключен, он не занимает память.

# Small line under the Use Tailscale switch before Tailscale is installed.
settings-tailscale-switch-sub-first = Устанавливает его из собственного источника Tailscale.

# Small line under the Use Tailscale switch once Tailscale is installed.
settings-tailscale-switch-sub = Выкл.: остановлен, не запускается вместе с компьютером.

# Text of the switch that turns Tailscale on or off, and heading of its line.
settings-tailscale-switch = Использовать Tailscale

# Help text (i button) of the Tailscale card when Tailscale is off or has not answered.
# SAFETY: needs a human review before it ships
settings-tailscale-about-plain = При первом включении Tailscale устанавливается из собственного источника программ Tailscale и запускается. Его фоновая служба занимает память, пока он включён, поэтому работает только при включённом переключателе. Пока он выключен, Tailscale остановлен и не запускается вместе с компьютером. Брандмауэр пропускает ваши собственные устройства Tailscale и, как и раньше, не пускает всех остальных.

# Help text (i button) of the Tailscale card when Tailscale is on but not yet connected.
settings-tailscale-about-sign-in = { settings-tailscale-about-plain } При входе в браузере откроется страница входа Tailscale. Используйте одну и ту же учётную запись на всех своих устройствах.

# Help text (i button) of the Tailscale card when connected.
settings-tailscale-about-connected = { settings-tailscale-about-plain } После выхода этот компьютер покинет вашу частную сеть, пока вы не войдёте снова.

# Line in the devices card when there are no other devices.
settings-tailscale-no-devices = Других устройств пока нет

# Small line under "No other devices yet".
settings-tailscale-no-devices-sub = Войдите на другом устройстве с той же учётной записью.

# Shown on the devices card while Tailscale is off.
settings-tailscale-waits-off = Включите Tailscale, чтобы увидеть свои устройства.

# Shown on the devices card while Tailscale has not answered.
settings-tailscale-waits-looking = Поиск ваших устройств…

# Shown on the devices card when Tailscale needs signing in to.
settings-tailscale-waits-sign-in = Войдите, чтобы увидеть свои устройства.

# Shown on the devices card when Tailscale is on but not connected.
settings-tailscale-waits-connect = Подключитесь, чтобы увидеть свои устройства.

# Small line under this computer when it has no address yet.
settings-tailscale-no-address = Адреса пока нет.

# Small line under this computer: its address on the private network.
settings-tailscale-address = { $address } в вашей частной сети.

# Name of this computer's line when Tailscale gives it no name.
settings-tailscale-this-computer = Этот компьютер

# Heading of the line that signs in to Tailscale, and tip of its round button.
settings-tailscale-sign-in = Войти

# Small line under the Sign in line.
settings-tailscale-sign-in-sub = Открывает страницу входа в браузере.

# Heading of the line that joins the private network again, and tip of its round button.
settings-tailscale-connect = Подключиться

# Small line under the Connect line.
settings-tailscale-connect-sub = Снова присоединяет к вашей частной сети.

# Heading of the line that says the firewall lets the person's own devices in.
# SAFETY: needs a human review before it ships
settings-tailscale-firewall = Ваши устройства могут подключаться

# Small line under that heading.
# SAFETY: needs a human review before it ships
settings-tailscale-firewall-sub = Всех остальных, как и раньше, не пускают.

# Heading of the line that signs out, and tip of its button.
settings-tailscale-sign-out = Выйти

# Small line under the Sign out line.
settings-tailscale-sign-out-sub = Покидает вашу частную сеть.

# Heading of the card with the Tailscale switch.
settings-tailscale-card = Tailscale

# Heading of the card that lists the person's devices.
settings-tailscale-devices = Ваши устройства

# Help text (i button) of the Your devices card.
settings-tailscale-devices-about = Все устройства, вошедшие в вашу частную сеть, с их адресами в ней, системами и временем последнего появления в сети. Чтобы добавить устройство, установите Tailscale на телефон или другой компьютер и войдите с той же учётной записью.
