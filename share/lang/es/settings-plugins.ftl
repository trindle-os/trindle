
# machine translated (Spanish), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Tus complementos

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = complementos plugins extensiones añadidos instalados eliminar ajustes

# SAFETY: needs a human review before it ships
# Help line at the foot of the Your plugins page.
settings-plugins-help = Los complementos amplían Trindle. Un complemento puede añadir un icono al anillo, ajustes aquí, notificaciones o algo que arranca al iniciar sesión. Cada complemento se ejecuta aislado y no puede usar nada hasta que lo permitas en Permisos. El botón junto al título de la página abre Permisos. Para eliminar un complemento, pulsa dos veces la papelera de su tarjeta.

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Trindle no pudo abrir la ventana que pregunta qué puede usar el complemento.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Versión { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Versión { $v }. { $settled }

# SAFETY: needs a human review before it ships
# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } no pide nada, así que se ejecuta aislado y sin nada permitido.

# SAFETY: needs a human review before it ships
# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } pide { $list }. Has permitido { $allowed } de { $asks }. Se ejecuta aislado, solo con lo que permitas.
        [many] { $name } pide { $list }. Has permitido { $allowed } de { $asks }. Se ejecuta aislado, solo con lo que permitas.
       *[other] { $name } pide { $list }. Has permitido { $allowed } de { $asks }. Se ejecuta aislado, solo con lo que permitas.
    }

# Heading of the line about a plugin on its card.
settings-plugins-about-it = Acerca de

# SAFETY: needs a human review before it ships
# Tip of the button that changes what a plugin may use.
settings-plugins-change = Cambiar lo que puede usar

# SAFETY: needs a human review before it ships
# Tip of the button that removes a plugin.
settings-plugins-remove = Eliminar este complemento

# SAFETY: needs a human review before it ships
# Red words when a plugin could not be removed.
settings-plugins-not-removed = No se pudo eliminar.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Escribe aquí

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } no tiene ajustes propios.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } no tiene ajustes que mostrar por ahora.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] No hay complementos instalados.
        [one] Hay un complemento instalado: { $list }.
        [many] Hay { $n } de complementos instalados: { $list }.
       *[other] Hay { $n } complementos instalados: { $list }.
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = Complementos

# Heading of the line shown when no plugin is installed.
settings-plugins-none = Ninguno instalado

# SAFETY: needs a human review before it ships
# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = Abrir Permisos

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = Preguntando a { $name }…

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } no responde. Puede que su servicio esté detenido.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Guardado.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } no aceptó eso.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } no pudo hacerlo.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } no pudo hacerlo: { $why }.
