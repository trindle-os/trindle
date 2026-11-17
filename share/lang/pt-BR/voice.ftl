
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Said under the ring when a radio is switched on by speaking; the radio's name comes first.
voice-radio-on = { $name } ligado

# Said under the ring when a radio is switched off by speaking; the radio's name comes first.
voice-radio-off = { $name } desligado

# Said under the ring after "brighter" turns the screen up.
voice-brighter = Mais brilho

# Said under the ring after "dimmer" turns the screen down.
voice-dimmer = Menos brilho

# Said under the ring when the request is to let the screen rest again.
voice-keep-awake-off = A tela pode apagar de novo

# Said under the ring when the screen is kept awake.
voice-keep-awake-on = Mantendo a tela ligada

# Said under the ring when the request is to look for updates.
voice-updates = Procurando atualizações

# Said under the ring when nothing was heard at all.
voice-nothing-heard = O Trindle não conseguiu ouvir você. Toque no meio do anel e fale.

# Said under the ring when the volume goes up.
voice-louder = Mais alto

# Said under the ring when the volume goes down.
voice-quieter = Mais baixo

# Said under the ring when the sound is switched back on.
voice-sound-on = Som ligado

# Said under the ring when the sound is muted.
voice-muted = Sem som

# Said under the ring when the wallpaper changes to the next one.
voice-next-wallpaper = Próximo papel de parede

# Said under the ring after a screenshot is taken; Pictures is the folder's name.
voice-screenshot-saved = Captura de tela salva em Pictures

# Said under the ring when the request is to show every app.
voice-all-apps = Todos os aplicativos

# Said under the ring before shutting down, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-power-off = Desligar? Clique para confirmar

# Said under the ring before restarting, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-reboot = Reiniciar? Clique para confirmar

# Said under the ring before logging out, which waits for a click to confirm.
# SAFETY: needs a human review before it ships
voice-confirm-logout = Sair da sessão? Clique para confirmar

# Said under the ring when the request is to find an app in Apps; the words the person said for it come in.
voice-looking-for-app = Procurando { $what } em Aplicativos

# Said under the ring when the request opens Apps (the store).
voice-apps = Aplicativos

# Said under the ring when the request opens the disks page of Settings; Drives is the page's name.
voice-drives = Unidades

# Said under the ring when the request opens Settings.
voice-settings = Configurações

# Said under the ring when the request closes the window in front.
voice-closing-window = Fechando a janela

# Said under the ring when an open app is closed; the app's name comes in.
voice-closing-app = Fechando { $name }

# Said under the ring when no app has the name the person said; the name they said comes in.
voice-no-app = Nenhum aplicativo chamado { $name }

# Said under the ring when a window is brought to the front; the app's name comes in.
voice-showing-app = Mostrando { $name }

# Said under the ring when an app is opened; the app's name comes in.
voice-opening-app = Abrindo { $name }

# Said under the ring when the request was heard but not understood; what was heard comes in.
voice-heard-unknown = O Trindle não reconheceu o comando “{ $heard }”

# Said in the ring when voice has not been added yet.
voice-optional = A voz é um extra opcional. Adicione-a em Configurações, na página Anel.

# Said in the ring when voice has been added but is switched off.
voice-switched-off = A voz está desligada. Ligue-a em Configurações, na página Anel.

# Said in the ring when the microphone cannot be opened; the reason comes after it.
voice-mic-failed = O Trindle não conseguiu acessar o microfone: { $why }

# Said in the ring when the microphone's recorder would not start.
voice-mic-not-started = O Trindle não conseguiu acessar o microfone porque o programa de gravação não iniciou

# Said in the ring when listening stopped and nothing was recorded.
voice-nothing-recorded = Nada foi gravado.

# Said in the ring when the recording had no speech in it.
voice-nothing-heard-after = O Trindle não conseguiu ouvir você. Toque no meio do anel e fale.

# Said in the ring when the speech engine failed and gave no reason.
voice-whisper-failed = O Whisper falhou

# Progress line while the build tools are added for the build; the names of the packages come in.
voice-installing-tools = Instalando ferramentas de compilação temporariamente: { $names }

# Progress line while whisper.cpp is built; its version comes in.
voice-building = Compilando o whisper.cpp { $version } para o controle por voz. Isso leva um ou dois minutos.

# Progress line while the speech model is downloaded (148 MB).
voice-fetching-model = Baixando o modelo de fala (148 MB)

# Progress line while the build tools are taken away again.
voice-removing-tools = Removendo as ferramentas de compilação agora

# Said once voice is added and ready to use.
voice-ready = O controle por voz está pronto. Abra o anel, toque no meio e fale.

# Said once voice is removed.
voice-removed = Controle por voz removido.

# Shown in the terminal window that stays open, asking for Enter to close it.
voice-press-enter = Pressione Enter para fechar.

# Shown when the command is run with neither install nor remove.
voice-usage = uso: trindle-voice-setup install | remove

# Said when a step of the voice set-up fails. { $program } is the program that was run, with its first word.
voice-step-failed = { $program } não funcionou.

# Said when the whisper.cpp download isn't the one Trindle checks for, so nothing is built from it.
voice-whisper-fingerprint = O download do whisper.cpp não é o que o Trindle esperava, então não foi compilado.

# Said when the speech model download isn't the one Trindle checks for, so it isn't installed.
voice-model-fingerprint = O download do modelo de fala não é o que o Trindle esperava, então não foi instalado.
# The reason after "Trindle couldn't access the microphone:" when the session has no private folder to keep a recording in.
voice-no-private-folder = nenhuma pasta privada está disponível.
