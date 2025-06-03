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
2. **Filters out .env files** - saves you from the shame of leaked API keys  
3. **Generates intelligent commit messages** using AI - no more "update stuff"
4. **Pushes to remote** - because why not finish the job?

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

# Just commit, don't push
noob-commit --no-push

# Use a different AI model
noob-commit --model gpt-4

# Let me edit the AI's commit message
noob-commit --review
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

### Features 🔥

- 🤖 **AI-powered commit messages** - Actually descriptive commits
- 🛡️ **Auto .env protection** - Saves you from security nightmares  
- ⚡ **One command workflow** - Add, commit, push in one go
- 🎭 **Self-deprecating humor** - Because we're all noobs sometimes
- 🔧 **Highly configurable** - But works great out of the box
- 🚨 **Noob-friendly errors** - Helpful messages when things go wrong

### Configuration 🛠️

All the knobs you might want to turn:

| Flag | Description | Default |
|------|-------------|---------|
| `--model` | AI model to use | `gpt-4.1-mini` |
| `--max-tokens` | How much the AI can ramble | `2000` |
| `--dry-run` | Just show what would happen | `false` |
| `--force` | Skip confirmations | `false` |
| `--review` | Edit AI's message | `false` |
| `--ok-to-send-env` | Include .env files | `false` |
| `--no-push` | Don't push to remote | `false` |

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
2. **Filtra arquivos .env** - te salva da vergonha de vazar chaves de API
3. **Gera mensagens de commit inteligentes** usando IA - chega de "update bagui"
4. **Faz push pro remoto** - porque por que não terminar o serviço?

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
```

### Recursos 🔥

- 🤖 **Mensagens de commit com IA** - Commits realmente descritivos
- 🛡️ **Proteção automática de .env** - Te salva de pesadelos de segurança
- ⚡ **Fluxo de um comando só** - Add, commit, push de uma vez
- 🎭 **Humor autodepreciativo** - Porque todos somos noobs às vezes
- 🔧 **Altamente configurável** - Mas funciona bem direto da caixa
- 🚨 **Erros amigáveis para noobs** - Mensagens úteis quando dá ruim

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