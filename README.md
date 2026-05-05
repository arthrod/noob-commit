# 🤡 noob-commit

*For developers who code like ninjas but commit like toddlers*

[🇧🇷 Português](#português) | [🇺🇸 English](#english)

---

## English

### The Problem 😅

How many times have you:
- Written commit messages like "fix stuff" or "idk it works now"?
- Forgotten to run `git add` before committing?
- Rushed to get a new OpenAI API key because you accidentally committed it?
- Paid that shameful OpenAI bill after pushing your keys to GitHub?

**No more!** This tool is for developers who are amazing at coding but absolutely terrible at git.

### What It Does 🎯

**noob-commit** automatically:
1. **Adds all your files** (`git add .`) - because you always forget
2. **Smart filtering** - protects you from committing:
   - 🔐 Security files (.env, credentials, secrets)
   - 📦 Dependency folders (node_modules, venv, target)
   - 🗑️ Build artifacts (__pycache__, .DS_Store, *.pyc)
3. **Generates intelligent commit messages** using AI - no more "update stuff"
4. **Pushes to remote** - because why not finish the job?
5. **Loads API keys smartly** - from environment or .env file

### Installation 🚀

```bash
cargo install noob-commit
```

Or build from source:
```bash
git clone https://github.com/arthrod/noob-commit
cd noob-commit
cargo build --release
```

### Setup ⚡

1. Get an OpenAI API key at https://platform.openai.com/api-keys
2. Set your environment variable:
   ```bash
   export OPENAI_API_KEY="your-key-here"
   ```
3. (Optional) Setup the `nc` alias for ultimate laziness:
   ```bash
   noob-commit --setup-alias
   ```

### Usage 🎮

**Basic usage** (does everything for you):
```bash
noob-commit
# or if you set up the alias:
nc
```

**Common options**:
```bash
# See what it would commit without actually doing it
noob-commit --dry-run

# YOLO mode - no confirmations asked
noob-commit --force

# Include .env files (living dangerously)
noob-commit --ok-to-send-env

# Include dependency folders (WARNING: huge repo!)
noob-commit --yes-to-modules

# Include build artifacts (not recommended)
noob-commit --yes-to-crap

# Output the AI advice in Brazilian Portuguese
noob-commit -b

# Just commit, don't push
noob-commit --no-push

# Use a different AI model
noob-commit --model gpt-4

# Let me edit the AI's commit message
noob-commit --review

# Limit the git diff size sent to AI (useful for huge diffs)
noob-commit --max-input-chars 10000

# Send full diff without any truncation
noob-commit --max-input-chars 0

# Update to the latest version
noob-commit --update
```

### Examples 💡

**The lazy developer's workflow:**
```bash
# Write amazing code
vim src/main.rs

# Commit like a pro with zero effort
nc
```

**The anxious developer's workflow:**
```bash
# Check what would happen first
nc --dry-run

# Looks good, now do it for real
nc
```

**The perfectionist's workflow:**
```bash
# Let me review the AI's work
nc --review
```

**Working on a messy project:**
```bash
# You have node_modules, __pycache__, .env files everywhere
# noob-commit filters them all automatically!
nc

# Output:
# 🛡️  Protecting security file: .env
# 📦 Unstaged dependency folders: node_modules/
# 🗑️  Unstaged cache/build artifacts: __pycache__/
# ✅ Generated perfect commit message for your actual code changes
```

**The "I know what I'm doing" workflow:**
```bash
# Include EVERYTHING (use with caution!)
nc --ok-to-send-env --yes-to-modules --yes-to-crap

# Get advice in Portuguese (for BR devs)
nc -b
```

### Features 🔥

- 🤖 **AI-powered commit messages** - Actually descriptive commits
- 🛡️ **Smart security filtering** - Protects .env, credentials, secrets, SSH keys
- 📦 **Dependency folder filtering** - Keeps node_modules, venv, vendor out
- 🗑️ **Build artifact filtering** - No more __pycache__, .DS_Store, *.pyc
- ✂️ **Input size limiting** - Truncate huge diffs to save API costs
- ⚡ **One command workflow** - Add, commit, push in one go
- 🔑 **Flexible API key loading** - From environment or .env file
- 🎭 **Self-deprecating humor** - Because we're all noobs sometimes
- 🔧 **Highly configurable** - But works great out of the box
- 🚨 **Noob-friendly errors** - Helpful messages when things go wrong
- 🚀 **Modern async implementation** - Fast and efficient

### Configuration 🛠️

All the knobs you might want to turn:

| Flag | Description | Default |
|------|-------------|---------|
| `-m, --model` | 🧠 AI model to use | `gpt-4.1-mini` |
| `-t, --max-tokens` | 🤖 How much the AI can ramble (output tokens) | `2000` |
| `-i, --max-input-chars` | ✂️ Maximum characters of git diff to send to AI (0 = unlimited) | `50000` |
| `-d, --dry-run` | 🔍 Just show what would happen | `false` |
| `-f, --force` | ⚡ Skip confirmations (YOLO mode) | `false` |
| `-r, --review` | ✏️ Edit AI's message before committing | `false` |
| `-e, --ok-to-send-env` | 🔓 Include .env files (dangerous!) | `false` |
| `-M, --yes-to-modules` | 📦 Include dependency folders (huge repo!) | `false` |
| `-c, --yes-to-crap` | 🗑️ Include build artifacts | `false` |
| `-b, --br-huehuehue` | 🇧🇷 Output advice in Brazilian Portuguese | `false` |
| `-a, --no-f-ads` | 🙊 Disable the silly post-commit tagline | `false` |
| `-p, --no-push` | 📦 Commit but don't push | `false` |
| `-s, --setup-alias` | 🛠️ Setup 'nc' alias | - |
| `-u, --update` | 🚀 Update noob-commit to the latest version | - |
| `-v, --verbose` | 📢 Increase verbosity (can be used multiple times) | - |

### What Gets Filtered? 🚫

**Security Files** (use `--ok-to-send-env` to include):
- `.env`, `.env.local`, `.env.production`, etc.
- `.npmrc`, `.pypirc`
- `credentials`, `secrets.yml`, `secrets.yaml`
- SSH keys: `id_rsa`, `id_ed25519`, etc.

**Dependency Folders** (use `--yes-to-modules` to include):
- `node_modules/`, `venv/`, `.venv/`, `vendor/`
- `bower_components/`, `jspm_packages/`
- `target/` (Rust), `Pods/` (iOS), `.gradle/` (Android)
- `build/`, `dist/` (various build systems)

**Build Artifacts** (use `--yes-to-crap` to include):
- `__pycache__/`, `*.pyc`, `*.pyo`
- `.DS_Store`, `Thumbs.db`, `desktop.ini`
- `*.swp`, `*.swo`, `*.swn` (editor files)
- `*.log`, `*.tmp`, `*.cache`, `*.bak`
- Compiled files: `*.o`, `*.a`, `*.class`, `*.so`, `*.dll`

### Contributing 🤝

Found a bug? Want to add a feature? PRs welcome! Just remember:
- Keep the humor level high
- Keep the noob-friendliness higher
- Write tests (we're not complete noobs)

---

## Português

### O Problema 😅

Quantas vezes você:
- Escreveu commits tipo "arrumei uns bagui" ou "sei lá, agora funciona"?
- Esqueceu de rodar `git add` antes do commit?
- Correu atrás de uma nova chave da OpenAI porque commitou ela sem querer?
- Pagou aquela conta vergonhosa da OpenAI depois de empurrar suas chaves pro GitHub?

**Nunca mais!** Esta ferramenta é para devs que são ninjas no código mas crianças no git.

### O Que Faz 🎯

**noob-commit** automaticamente:
1. **Adiciona todos os arquivos** (`git add .`) - porque você sempre esquece
2. **Filtragem inteligente** - te protege de commitar:
   - 🔐 Arquivos de segurança (.env, credentials, secrets)
   - 📦 Pastas de dependências (node_modules, venv, target)
   - 🗑️ Artefatos de build (__pycache__, .DS_Store, *.pyc)
3. **Gera mensagens de commit inteligentes** usando IA - chega de "update bagui"
4. **Faz push pro remoto** - porque por que não terminar o serviço?
5. **Carrega chaves de API** - do ambiente ou arquivo .env

### Instalação 🚀

```bash
cargo install noob-commit
```

Ou compile do código:
```bash
git clone https://github.com/arthrod/noob-commit
cd noob-commit
cargo build --release
```

### Configuração ⚡

1. Pegue uma chave da OpenAI em https://platform.openai.com/api-keys
2. Configure sua variável de ambiente:
   ```bash
   export OPENAI_API_KEY="sua-chave-aqui"
   ```
3. (Opcional) Configure o alias `nc` para preguiça máxima:
   ```bash
   noob-commit --setup-alias
   ```

### Uso 🎮

**Uso básico** (faz tudo pra você):
```bash
noob-commit
# ou se configurou o alias:
nc
```

**Opções comuns**:
```bash
# Ver o que commitaria sem fazer de verdade
noob-commit --dry-run

# Modo YOLO - sem confirmações
noob-commit --force

# Incluir arquivos .env (vivendo perigosamente)
noob-commit --ok-to-send-env

# Só commitar, não fazer push
noob-commit --no-push

# Usar um modelo de IA diferente
noob-commit --model gpt-4

# Deixa eu editar a mensagem da IA
noob-commit --review

# Limitar o tamanho do diff enviado pra IA (útil para diffs gigantes)
noob-commit --max-input-chars 10000

# Enviar o diff completo sem truncar
noob-commit --max-input-chars 0
```

### Recursos 🔥

- 🤖 **Mensagens de commit com IA** - Commits realmente descritivos
- 🛡️ **Filtragem inteligente de segurança** - Protege .env, credentials, secrets, chaves SSH
- 📦 **Filtragem de pastas de dependências** - Mantém node_modules, venv, vendor fora
- 🗑️ **Filtragem de artefatos de build** - Sem mais __pycache__, .DS_Store, *.pyc
- ✂️ **Limitação de tamanho de entrada** - Trunca diffs gigantes para economizar na API
- ⚡ **Fluxo de um comando só** - Add, commit, push de uma vez
- 🔑 **Carregamento flexível de API key** - Do ambiente ou arquivo .env
- 🎭 **Humor autodepreciativo** - Porque todos somos noobs às vezes
- 🔧 **Altamente configurável** - Mas funciona bem direto da caixa
- 🚨 **Erros amigáveis para noobs** - Mensagens úteis quando dá ruim
- 🚀 **Implementação async moderna** - Rápida e eficiente

---

## Acknowledgments 🙏

This project is built upon the excellent foundation of [auto-commit](https://github.com/m1guelpf/auto-commit) by [Miguel Piedrafita](https://github.com/m1guelpf). 

The original auto-commit was a brilliant tool for AI-powered commit messages. noob-commit extends it with:
- Auto-adding files (because we always forget `git add`)
- Smart .env file filtering (security first!)
- Auto-pushing (complete the workflow!)
- Self-deprecating humor (because coding is hard enough)
- Noob-friendly error messages (we've all been there)

Huge thanks to Miguel for creating the original tool and providing such a solid foundation! 🎉

---

## Made with ❤️ by [Neurotic Coder](https://github.com/arthrod)
## Assisted by Beloved Claude

*Stop being a noob at git. Be a noob with style! 🎭*
