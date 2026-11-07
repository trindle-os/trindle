# Security

## Supported versions

Only the latest release is looked after. Please update before reporting.

| Version | Supported |
| --- | --- |
| Latest release | Yes |
| Anything older | No |

## Reporting a problem

Please do not open a public issue for a security problem.

1. **First choice:** use GitHub's private reporting. On the repository, open the **Security** tab and choose **Report a vulnerability**. Only the Trindle project sees it.
2. **Otherwise:** write to [distro@trindle.org](mailto:distro@trindle.org) with the word "Security" in the subject.

Please say what you found, how to make it happen, the Trindle version and the kind of computer you were using. Do not include real passwords or personal files.

## What to expect

Trindle is a small project, so there is no promise of a fixed reply time. Each report is read, and you will hear back whether it is a fault, what is being done about it and when a fixed version is out. Reporters are thanked in the changelog unless they would rather not be named.

## How Trindle keeps itself safe

The design is described in [README.md](README.md): only `trindle-admin` runs as root and it checks every argument, programs and plugins run in their own sealed boxes, and nothing is sent anywhere without being asked, apart from the update checks that the Check for updates switch in Settings, About this computer, turns off (Debian's daily security fixes and the blocker's daily list refresh included).
