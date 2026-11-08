# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# What a device runs, shown in a device's small line, when it is an iPhone or iPad.
settings-tailscale-system-ios = iPhone ou iPad

# In a device's small line: the device is on the network now.
settings-tailscale-online = online agora

# In a device's small line: the device has never been seen.
settings-tailscale-not-seen = ainda não visto

# In a device's small line: when the device was last online; the date is already written.
settings-tailscale-last-online = visto online por último { $day }

# Shown beside the Use Tailscale switch while the password is asked for, when Tailscale is already installed.
# SAFETY: needs a human review before it ships
settings-tailscale-waiting = Aguardando a senha...

# Shown beside the Use Tailscale switch while the password is asked for, before Tailscale is installed.
# SAFETY: needs a human review before it ships
settings-tailscale-waiting-install = Aguardando a senha, depois instalando o Tailscale...

# Shown beside the switch after Tailscale is turned on.
settings-tailscale-done-on = Tailscale ligado

# Shown beside the switch after Tailscale is turned off.
settings-tailscale-done-off = Tailscale desligado

# Shown under the Sign in line once the browser is being opened.
settings-tailscale-opening = Abrindo a página de login no navegador...

# Red words under the Sign in line when Tailscale could not be started.
settings-tailscale-not-started = Não foi possível iniciar o Tailscale.

# Red words under the Sign out line when signing out did not work.
settings-tailscale-sign-out-failed = O Tailscale não saiu da conta.

# Headline of the page while Tailscale is being asked how it is.
settings-tailscale-top-asking = Perguntando ao Tailscale…

# Headline of the page when Tailscale is off and not installed.
settings-tailscale-top-not-installed = O Tailscale não está instalado. Ligue-o abaixo para instalá-lo.

# Headline of the page when Tailscale is installed but off.
settings-tailscale-top-off = O Tailscale está desligado, então não usa memória.

# Headline of the page when Tailscale is on but does not answer.
settings-tailscale-top-silent = O Tailscale não está respondendo. Pode ainda estar iniciando.

# Headline of the page when connected: the name of the private network and of this computer.
settings-tailscale-top-connected = Conectado a { $network } como { $name }.

# Name used in the headline for the private network when it has no name of its own.
settings-tailscale-network-fallback = Tailscale

# Headline of the page when Tailscale is on and has to be signed in to.
settings-tailscale-top-sign-in = O Tailscale está ligado. Entre abaixo para se juntar à sua rede privada.

# Headline of the page for any other state; the state is already written in lower case.
settings-tailscale-top-state = O Tailscale está { $state }.

# Name of the page in the list of settings pages.
settings-tailscale-title = Tailscale

# Words the settings search looks for on this page (not shown).
settings-tailscale-keywords = tailscale vpn rede privada dispositivos remoto tailnet

# Help text at the top of the page.
settings-tailscale-help = O Tailscale junta seus próprios computadores e celulares em uma só rede privada, onde quer que cada um esteja. Nada dele é instalado até você ligá-lo, e, enquanto está desligado, não usa memória.

# Small line under the Use Tailscale switch before Tailscale is installed.
settings-tailscale-switch-sub-first = Instala a partir da fonte do próprio Tailscale.

# Small line under the Use Tailscale switch once Tailscale is installed.
settings-tailscale-switch-sub = Desligado: parado, não inicia com o computador.

# Text of the switch that turns Tailscale on or off, and heading of its line.
settings-tailscale-switch = Usar o Tailscale

# Help text (i button) of the Tailscale card when Tailscale is off or has not answered.
settings-tailscale-about-plain = Na primeira vez que é ligado, o Tailscale é instalado a partir da fonte de software do próprio Tailscale e depois iniciado. Seu serviço em segundo plano usa memória enquanto ligado, então só roda quando isto está ligado. Enquanto isto está desligado, o Tailscale fica parado e não inicia com o computador. O firewall deixa entrar seus próprios dispositivos Tailscale e mantém todo o resto fora, como antes.

# Help text (i button) of the Tailscale card when Tailscale is on but not yet connected.
settings-tailscale-about-sign-in = { settings-tailscale-about-plain } Entrar abre a página de login do Tailscale no navegador. Use a mesma conta em cada um dos seus dispositivos.

# Help text (i button) of the Tailscale card when connected.
settings-tailscale-about-connected = { settings-tailscale-about-plain } Sair da conta tira este computador da sua rede privada até você entrar de novo.

# Line in the devices card when there are no other devices.
settings-tailscale-no-devices = Nenhum outro dispositivo ainda

# Small line under "No other devices yet".
settings-tailscale-no-devices-sub = Entre em outro com a mesma conta.

# Shown on the devices card while Tailscale is off.
settings-tailscale-waits-off = Ligue o Tailscale para ver seus dispositivos.

# Shown on the devices card while Tailscale has not answered.
settings-tailscale-waits-looking = Procurando seus dispositivos…

# Shown on the devices card when Tailscale needs signing in to.
settings-tailscale-waits-sign-in = Entre para ver seus dispositivos.

# Shown on the devices card when Tailscale is on but not connected.
settings-tailscale-waits-connect = Conecte-se para ver seus dispositivos.

# Small line under this computer when it has no address yet.
settings-tailscale-no-address = Ainda sem endereço.

# Small line under this computer: its address on the private network.
settings-tailscale-address = { $address } na sua rede privada.

# Name of this computer's line when Tailscale gives it no name.
settings-tailscale-this-computer = Este computador

# Heading of the line that signs in to Tailscale, and tip of its round button.
settings-tailscale-sign-in = Entrar

# Small line under the Sign in line.
settings-tailscale-sign-in-sub = Abre a página de login no navegador.

# Heading of the line that joins the private network again, and tip of its round button.
settings-tailscale-connect = Conectar

# Small line under the Connect line.
settings-tailscale-connect-sub = Junta-se de novo à sua rede privada.

# Heading of the line that says the firewall lets the person's own devices in.
settings-tailscale-firewall = Seus próprios dispositivos entram

# Small line under that heading.
settings-tailscale-firewall-sub = Todo o resto fica de fora, como antes.

# Heading of the line that signs out, and tip of its button.
settings-tailscale-sign-out = Sair da conta

# Small line under the Sign out line.
settings-tailscale-sign-out-sub = Deixa a sua rede privada.

# Heading of the card with the Tailscale switch.
settings-tailscale-card = Tailscale

# Heading of the card that lists the person's devices.
settings-tailscale-devices = Seus dispositivos

# Help text (i button) of the Your devices card.
settings-tailscale-devices-about = Todo dispositivo conectado à sua rede privada, com seu endereço nela, o que executa e quando esteve online pela última vez. Para adicionar um, instale o Tailscale no seu celular ou em outro computador e entre com a mesma conta.
