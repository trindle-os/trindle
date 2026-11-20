# machine translated (Japanese), not yet reviewed by a person.

# Settings: the AI programs page (src/settings/panels/ai_programs.rs).

## The AI programs page (src/settings/panels/ai_programs.rs)

# Name of the AI programs page, in the page list, as its heading and on the card listing the programs.
settings-ai-title = AIプログラム
# Words people may search for to find the AI programs page, separated by spaces.
settings-ai-keywords = ai programs claude codex cursor gemini aider agent fence sandbox private folders share ssh keys token sudo
# Help line at the foot of the AI programs page.
settings-ai-help = AIコーディングプログラムは、個人のファイルを隠すフェンスの中で動きます。必要なフォルダだけを共有してください。
# Headline of the AI programs page.
settings-ai-note = ふだん使うかぎり、これでAIプログラムを個人のファイルから遠ざけられます。たとえば、だまされたプログラムが銀行の明細を集めて外へ送ることを防ぎます。この保護には限界があり、保証ではありません。プログラムが執拗に試みた場合、コンピューターのソフトウェアに欠陥がある場合、要求を承認したりフォルダを共有したりした場合は、すり抜けることがあります。
# Added after the words behind the i of its card.
settings-ai-inside = インストールしたプログラムやスクリプトなど、AIプログラムが起動したものはすべて、一緒にフェンスの中に残ります。
# Red words when a change to the fence could not be made.
settings-ai-not-changed = 変更できませんでした。
# Said beside a setting on the AI programs page once it is saved.
settings-ai-saved = 保存しました
# Title of the window for choosing a folder to share with AI programs.
settings-ai-choose-folder = AIプログラムと共有するフォルダを選んでください
# Red words when the key for AI programs to reach computers by ssh could not be made.
settings-ai-key-not-made = キーを作れませんでした。
# Label of the switch that limits the network of AI programs, and heading of its line.
settings-ai-limit-net = 下のサイトだけを許可
# Tip of the button that removes a site from the list AI programs may reach.
settings-ai-stop-site = AIプログラムがこのサイトに接続できないようにする
# Example in the box for typing a site AI programs may reach.
settings-ai-eg-site = api.anthropic.com
# Tip of the button that adds a site AI programs may reach.
settings-ai-let-site = AIプログラムがこのサイトに接続できるようにする
# Heading of the line for adding a site.
settings-ai-add-site = サイトを追加
# Added after the words behind the i of its card.
settings-ai-add-site-sub = 名前を入力してください。*.example.com と入力すると、その下のすべてのサイトになります。
# Heading of the card that limits the network of AI programs.
settings-ai-network = ネットワーク
# Help text of the Network card (the information button).
settings-ai-network-about = オフのときは、AIプログラムはあなたと同じようにネットワークを使います。オンのときは、下のサイトにだけ、安全な (https) 接続で届きます。フェンスには専用のネットワークはありません。プログラムのツールは、渡されたプロキシ設定を使う必要があります。AIサービスのサイトと、インストールに必要なパッケージの入手先を追加してください。
# Label of the switch that keeps AI programs inside the fence, and heading of its line.
settings-ai-keep-inside = AIプログラムをフェンスの中に留める
# Under the fence switch when it is off.
settings-ai-fence-off = フェンスはオフです。AIプログラムは、あなたに見えるものをすべて見られます。
# Heading of the card with the fence switch.
settings-ai-the-fence = フェンス
# Help text of The fence card (the information button).
settings-ai-fence-about = AIプログラムは、システムを見たり使ったりできます。ホームフォルダの代わりに、空の状態から始まり、実行のあいだ残る専用フォルダが渡されます。その中に現れるのは、あなたが共有したフォルダだけです。ほかのプログラム、セッション、キー、パスワードは見えません。このリストにある名前以外の方法で起動したプログラムは、フェンスの中に入りません。
# Label of the switch that fences one AI program: its name is { $name }.
settings-ai-fence-name = { $name } をフェンスに入れる
# Tip of the button that takes one program off the list: its name is { $name }.
settings-ai-take-off = { $name } をリストから外す
# Label of the switch for using a program's existing settings folder: its name is { $name }.
settings-ai-own-for = { $name } に既存の設定フォルダを使う
# Heading of the line with the switch for using a program's existing settings folder.
settings-ai-own = 既存の設定フォルダを使う
# Added after the words behind the i of its card.
settings-ai-own-sub = このフォルダには、プログラムの過去の会話が入っています。
# Example in the box for typing the name of an AI program.
settings-ai-eg-program = claude
# Tip of the button that adds an AI program, and heading of its line.
settings-ai-add-program = プログラムを追加
# Help text of the AI programs card (the information button).
settings-ai-programs-about = これらのプログラムは、フェンスの中で起動します。「既存の設定フォルダを使う」がオフのときは、新しい専用フォルダで始まります。各プログラムは、フェンスの中で個別にログインします。あなた自身のログインは共有されません。
# Heading of the line shown when no folder is shared with AI programs.
settings-ai-nothing-shared = 共有しているものはありません
# Label of the switch that lets AI programs only read a shared folder.
settings-ai-only-read = このフォルダを読むだけにする
# Tip of the button that stops sharing a folder with AI programs.
settings-ai-make-private = このフォルダをまた非公開にする
# Under a shared folder's path, beside the switch that makes it read only.
settings-ai-only-read-short = 読むだけにする
# Tip of the button that shares a folder, and heading of its line.
settings-ai-share-folder = フォルダを共有
# Added after the words behind the i of its card.
settings-ai-share-folder-sub = 同じ場所に表示されます。AIプログラムは読み書きできます。
# Heading of the card listing folders shared with AI programs.
settings-ai-shared = AIプログラムと共有
# Help text of the shared folders card (the information button).
settings-ai-shared-about = ホームフォルダのほかのフォルダは、AIプログラムから見えないままです。隠しフォルダや、キー、パスワード、メールが入ったフォルダは共有できません。ここで共有したフォルダは、次にプログラムを起動したときから使えます。
# Tip of the button that removes a computer from the list AI programs may reach by ssh.
settings-ai-stop-computer = AIプログラムがこのコンピューターに接続できないようにする
# Example in the box for typing a computer AI programs may reach by ssh.
settings-ai-eg-computer = nas.local
# Tip of the button that adds a computer AI programs may reach by ssh.
settings-ai-let-computer = AIプログラムがこのコンピューターに接続できるようにする
# Heading of the line for adding a computer.
settings-ai-add-computer = コンピューターを追加
# Note showing the public key to put on each computer: the key is { $key }.
settings-ai-put-key = このキーを各コンピューターに置いてください: { $key }
# Tip of the button that makes a key for AI programs.
settings-ai-make-key = キーを作る
# Heading of the line for making a key for AI programs.
settings-ai-make-key-for = AIプログラム用のキーを作る
# Added after the words behind the i of its card.
settings-ai-make-key-sub = AIプログラムから見えるフォルダの外に保管される、別のキーです。
# Heading of the card listing computers AI programs may reach by ssh.
settings-ai-ssh = ssh で接続できるコンピューター
# Help text of the ssh card (the information button).
settings-ai-ssh-about = フェンスの中では、ssh は Trindle を通して接続します。Trindle は専用のキーを使い、ここに載せたコンピューターにだけ使います。AIプログラムにあなた自身のキーは見えません。
# Tip of the button that stops giving AI programs a key.
settings-ai-stop-key = AIプログラムにこのキーを渡さない
# Example in the box for typing a key to give AI programs: a name, an equals sign and the value.
settings-ai-eg-key = NAME=value
# Tip of the button that gives AI programs a key.
settings-ai-give-key = AIプログラムにこのキーを渡す
# Heading of the line for adding a key.
settings-ai-add-key = キーを追加
# Heading of the card listing keys given to AI programs.
settings-ai-keys = AIプログラムに渡したキー
# Help text of the keys card (the information button).
settings-ai-keys-about = セッションにあるキーやトークンは、AIプログラムには渡りません。渡るのは、ここで入力したキーだけです。
# Under the AI programs card, greyed, when none of the listed AI programs is on this computer.
settings-ai-none-installed = このコンピューターにAIプログラムはありません。
# Under the box, when the program typed in to add is not on this computer.
settings-ai-not-installed = そのプログラムはこのコンピューターにありません。
