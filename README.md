# WTF - Command Typo Fixer

A cross-platform CLI tool that fixes typos in your previous shell commands. When you type a command wrong, just run `wtf` and it will suggest corrections!

## ✨ Features

- 🔍 **Smart Detection**: Detects typos in your last command automatically
- 🎯 **Fuzzy Matching**: Catches similar typos using Jaro-Winkler algorithm
- 🤖 **AI-Powered Fixing**: Use Google Gemini for intelligent command correction
- ⚡ **Auto-Mode**: Enable auto-run mode to skip confirmation prompts
- 🌍 **Cross-Platform**: Windows (PowerShell), Linux (Bash/Zsh/Fish), macOS
- 📦 **300+ Built-in Typos**: Pre-configured for npm, git, cargo, docker, python, kubernetes, and more!
- 🎨 **Beautiful Output**: Colored terminal output with interactive selection
- 🚀 **Fast & Lightweight**: Built in Rust for maximum performance
- 💾 **Custom Typos**: Add your own typos and fixes
- 🔧 **Modular Design**: Clean, organized codebase split into modules

## 📦 Installation

### APT (Debian/Ubuntu) - Recommended

```bash
# Add Mewisme APT repository
curl -fsSL https://apt.mewis.me/mewisme.asc | sudo tee /etc/apt/keyrings/mewisme.gpg >/dev/null
echo "deb [signed-by=/etc/apt/keyrings/mewisme.gpg] https://apt.mewis.me stable main" | sudo tee /etc/apt/sources.list.d/mewisme.list

# Install WTF
sudo apt update
sudo apt install wtf

# Ready to use!
wtf --version
```

### Build from Source

```bash
# 1. Build from source
cargo build --release

# 2. First-time setup (automatic prompt)
cd target/release
./wtf

# On first run, you'll see:
# 🎉 Welcome to WTF - Command Typo Fixer!
# 
# Would you like to install WTF globally to your PATH?
# This will allow you to run 'wtf' from anywhere.
# 
# Install globally? [Y/n]:

# 3. Or install manually
./wtf install    # or short: ./wtf i

# 4. Restart terminal and use from anywhere!
wtf --version
```

**First-time experience:**
- ✅ Automatic prompt on first run
- ✅ One-time only (won't ask again)
- ✅ Skip by pressing 'n'
- ✅ Manual install anytime with `wtf install`

### Uninstall

```bash
wtf uninstall    # or short: wtf u
```


## 🚀 Usage

### 🤖 AI-Powered Fixing (Google Gemini)

Use Google Gemini 2.0 Flash for intelligent command fixing:

```bash
# 1. Get API key: https://aistudio.google.com/app/apikey

# 2. Set it
wtf set-api-key your-google-key-here

# Or use environment variable
export GOOGLE_API_KEY="your-key-here"  # Linux/macOS
$env:GOOGLE_API_KEY = "your-key-here"  # Windows

# 3. Use AI to fix any command
npm onstall express
wtf --ai

# Output:
# 🤖 Asking Google Gemini to fix the command...
# 🤖 AI suggestion: npm install express
# Run this command? [Y/n]:
```

**Benefits:**
- ✅ Unlimited command fixing (not limited to 300+ built-in)
- ✅ Context-aware corrections
- ✅ Works with any command
- ✅ Automatic fallback to built-in if AI fails


### Basic Usage - Fix Previous Command

When you make a typo in a command:

```powershell
# Windows PowerShell
PS> npm onstall express
# Command fails...

PS> wtf
Previous command:
  npm onstall express

[1] Suggested fix: npm install express (npm typo)

Select a fix [1-1] (or 'n' to cancel): 1
Running: npm install express
# Command executes!
```

### Auto-run Mode

Skip confirmation and run the first suggestion automatically:

```bash
wtf -y
# or
wtf --yes
```

### Debug Mode

See what command was detected:

```bash
wtf -d
# or
wtf --debug
```

## 🔧 Custom Typo Management

### Add a Custom Typo

Add your own typos and fixes:

```bash
wtf add "npm i" "npm install"
wtf add "gti" "git"
wtf add "dokcer" "docker"
```

When a typo already exists in the built-in database, it will be added to your custom list for priority matching.

### Save Last Command as Custom Typo

Made a typo that's not recognized? Save it directly:

```bash
# Type a wrong command
PS> my_custom_command_typo

# Save it with the correct version
PS> wtf save "my_custom_command"
```

### List Custom Typos

View all your custom typos:

```bash
wtf list
```

Output:
```
Custom Typos:

[1] npm i → npm install
[2] gti → git
[3] dokcer → docker

3 custom typo(s)
```

### Remove a Custom Typo

Remove a typo from your custom list:

```bash
wtf remove "npm i"
```

### Clear All Custom Typos

Clear your entire custom typo list:

```bash
wtf clear
```

### Show Config Location

Find where your custom typos are stored:

```bash
wtf config
```

## 📚 Built-in Typos (300+)

### NPM (20+ typos)
- `npm onstall` → `npm install`
- `npm isntall` → `npm install`
- `npm statr` → `npm start`
- `npm tset` → `npm test`
- `npm biuld` → `npm build`
- And more...

### Git (50+ typos)
- `git comit` → `git commit`
- `git pussh` → `git push`
- `git statsu` → `git status`
- `git checkotu` → `git checkout`
- `git branh` → `git branch`
- `git meger` → `git merge`
- And more...

### Cargo (15+ typos)
- `cargo biuld` → `cargo build`
- `cargo rnu` → `cargo run`
- `cargo tset` → `cargo test`
- `cargo chekc` → `cargo check`
- `cargo cilppy` → `cargo clippy`
- And more...

### Docker (15+ typos)
- `dokcer` → `docker`
- `docker rnu` → `docker run`
- `docker biuld` → `docker build`
- `docker imgaes` → `docker images`
- `docker-compoes` → `docker-compose`
- And more...

### Kubernetes (10+ typos)
- `kubeclt` → `kubectl`
- `kubectl aplply` → `kubectl apply`
- `kubectl delte` → `kubectl delete`
- `kubectl desribe` → `kubectl describe`
- And more...

### Python/Pip (10+ typos)
- `pyhton` → `python`
- `pip isntall` → `pip install`
- `pip unintsall` → `pip uninstall`
- `pip freez` → `pip freeze`
- And more...

### Linux/Unix Commands (100+ typos)
- `sl` → `ls` (classic!)
- `cd..` → `cd ..`
- `grpe` → `grep`
- `gti` → `git`
- `claer` → `clear`
- `sduo` → `sudo`
- `mkdri` → `mkdir`
- `tuch` → `touch`
- `wegt` → `wget`
- `culr` → `curl`
- And 90+ more...

### Other Tools
- Yarn, PNPM
- Terraform
- AWS CLI
- Make/CMake
- Rustc/Rustup
- And more...

**Plus fuzzy matching for any similar command!**

## 🏗️ Architecture

The project is now modular and well-organized:

```
src/
├── main.rs          # CLI interface and command routing
├── history.rs       # Shell history reading (PowerShell, Bash, Zsh, Fish)
├── corrections.rs   # Typo detection and fuzzy matching logic
├── commands.rs      # Built-in typo database (300+ typos)
├── ui.rs           # User interface and colored output
├── executor.rs     # Command execution
└── config.rs       # User configuration and custom typos
```

## 🔄 How It Works

1. **Reads Shell History**: Automatically detects your shell and reads history
2. **Extracts Last Command**: Gets the command before `wtf`
3. **Checks Custom Typos**: Your custom fixes have priority
4. **Checks Built-in Database**: Matches against 300+ known typos
5. **Fuzzy Matching**: Uses Jaro-Winkler similarity (85%+ threshold)
6. **Presents Suggestions**: Shows up to 5 suggestions with confidence scores
7. **Executes Fix**: Runs your selected correction

## 🌐 Shell Support

| Shell | Windows | Linux | macOS |
|-------|---------|-------|-------|
| PowerShell | ✅ | ✅ | ✅ |
| Bash | ❌ | ✅ | ✅ |
| Zsh | ❌ | ✅ | ✅ |
| Fish | ❌ | ✅ | ✅ |

## 📝 Examples

### Example 1: Basic Fix
```powershell
PS> npm onstall lodash
# Error...

PS> wtf
Previous command:
  npm onstall lodash

[1] Suggested fix: npm install lodash (npm typo)

Select a fix [1-1] (or 'n' to cancel): 
Running: npm install lodash
```

### Example 2: Multiple Suggestions
```bash
$ gti status
# Error...

$ wtf
Previous command:
  gti status

[1] Suggested fix: git status (git typo)
[2] Suggested fix: git status (similar to 'git')

Select a fix [1-2] (or 'n' to cancel): 1
Running: git status
```

### Example 3: Add Custom Typo
```bash
$ wtf add "deploy-prod" "npm run deploy:production"
✓ Added: deploy-prod → npm run deploy:production

$ deploy-prod
# Error...

$ wtf
Previous command:
  deploy-prod

[1] Suggested fix: npm run deploy:production (custom fix)

Select a fix [1-1] (or 'n' to cancel): 
Running: npm run deploy:production
```

### Example 4: Save Unknown Typo
```bash
$ complicated_commmand_with_typo
# Error...

$ wtf save "complicated_command_with_correct_spelling"
✓ Added: complicated_commmand_with_typo → complicated_command_with_correct_spelling

Now you can use 'wtf' to fix this typo in the future!
```

### ⚡ Auto-Mode (Always Run First Suggestion)

Enable auto-mode to always run the first suggestion without confirmation:

```bash
# Enable auto-mode
wtf auto-mode true
✓ Auto-mode enabled!

wtf will now automatically run the first suggestion without prompting.

This is equivalent to always using 'wtf -y'

# Now wtf works like wtf -y automatically
npm onstall express
wtf
# Immediately runs: npm install express (no prompt!)

# Disable auto-mode
wtf auto-mode false
✓ Auto-mode disabled!

# Or toggle on/off
wtf toggle-auto
✓ Auto-mode toggled ON!
```

**Benefits:**
- ✅ No need to type `-y` every time
- ✅ Faster workflow for trusted typo fixes
- ✅ Still can override with manual selection when needed
- ✅ Saves keystrokes!

## 🎯 Command Reference

```bash
# Basic Usage
wtf                       # Fix the last command
wtf -y                    # Auto-run first suggestion (one-time)
wtf -d                    # Debug mode
wtf --ai                  # Use AI to fix command

# Custom Typo Management
wtf add <wrong> <correct> # Add custom typo
wtf save <correct>        # Save last command as typo
wtf list                  # List custom typos
wtf remove <wrong>        # Remove custom typo
wtf clear                 # Clear all custom typos

# Configuration
wtf config                # Show config file location
wtf set-api-key <key>     # Set Google AI API key
wtf auto-mode <true|false> # Enable/disable auto-run mode
wtf toggle-auto           # Toggle auto-mode on/off

# PATH Management
wtf install               # Add to PATH (short: i)
wtf i                     # Same as install
wtf uninstall             # Remove from PATH (short: u)
wtf u                     # Same as uninstall

# Info
wtf --help                # Show help
wtf --version             # Show version
```

## 💡 Pro Tips

1. **First-run setup**: On first use, wtf will ask if you want to install globally - press Y for convenience!
2. **Quick install**: After building, run `./wtf install` to add to PATH
3. **Enable auto-mode**: Run `wtf auto-mode true` to always auto-run first suggestion (no more `-y` needed!)
4. **Use `-y` for speed**: Or use `-y` flag for one-time auto-run without enabling auto-mode
5. **Build your library**: Add common typos you make with `wtf add`
6. **Use `wtf save`**: When wtf doesn't recognize a typo, save it immediately
7. **Custom shortcuts**: Add your own shortcuts (e.g., `wtf add "gs" "git status"`)
8. **AI mode**: Set up Google Gemini for unlimited command fixing

## 🔧 Configuration

Config file location:
- **Windows**: `%APPDATA%\wtf\config.json`
- **Linux/macOS**: `~/.config/wtf/config.json`

Format:
```json
{
  "custom_typos": [
    ["npm i", "npm install"],
    ["gti", "git"],
    ["deploy-prod", "npm run deploy:production"]
  ],
  "first_run_complete": true,
  "auto_mode": false,
  "google_api_key": "your-api-key-here"
}
```

**Fields:**
- `custom_typos`: Your custom typo definitions
- `first_run_complete`: Automatically set to `true` after first-time setup prompt
- `auto_mode`: Enable/disable auto-run mode (set via `wtf auto-mode` command)
- `google_api_key`: Google AI API key for AI-powered fixing (set via `wtf set-api-key` command)

## 🚀 Performance

- **Blazing fast**: Built in Rust with optimized release builds
- **Small binary**: ~2.6MB with Google Gemini AI included
- **Low memory**: Minimal memory footprint
- **Instant suggestions**: Sub-100ms response time (regular mode)
- **AI mode**: ~500ms (network latency)

## 🤝 Contributing

Want to add more typos? They're all in `src/commands.rs`!

1. Edit `get_common_fixes()` to add exact typos
2. Edit `get_common_commands()` for fuzzy matching
3. Submit a PR!

## 📄 License

MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Mew <<mauminh.nguyen@gmail.com>>

## 👤 Author

**Mew**
- Email: mauminh.nguyen@gmail.com
- GitHub: [@mewisme](https://github.com/mewisme)

## 🙏 Acknowledgments

Inspired by [thefuck](https://github.com/nvbn/thefuck) but rewritten in Rust with better performance, modular architecture, custom typo management, and a cleaner codebase.

---

**Made with 🦀 Rust by Mew | [GitHub](https://github.com/mewisme/wtf) | [License](LICENSE)**
