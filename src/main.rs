use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, ChatCompletionTool, ChatCompletionToolType,
        CreateChatCompletionRequestArgs, FunctionObject,
    },
};
use chrono::Local;
use clap::Parser;
use tiktoken_rs::cl100k_base;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::{error, info};
use noob_commit::CommitAdvice;
use question::{Answer, Question};
use rand::prelude::*;
use schemars::generate::SchemaSettings;
use schemars::SchemaGenerator;
use spinners::{Spinner, Spinners};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
    str,
};

#[derive(Parser)]
#[command(version)]
#[command(name = "Noob Commit")]
#[command(author = "Noob Commit Contributors")]
#[command(about = "🤡 For devs who code like ninjas but commit like toddlers\n\nTired of writing 'fix stuff' and 'idk it works now' commits?\nThis tool auto-adds files, asks AI to write proper commits, and pushes for you.\nBecause we're great at coding but terrible at git.", long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[arg(
        short = 'd',
        long = "dry-run",
        help = "🔍 Just show me what commit message you'd create (for anxious devs)"
    )]
    dry_run: bool,

    #[arg(
        short,
        long,
        help = "✏️ Let me edit the AI's work (because sometimes AI is also bad at git)"
    )]
    review: bool,

    #[arg(
        short,
        long,
        help = "⚡ YOLO mode - just commit everything (living dangerously)"
    )]
    force: bool,

    #[arg(
        short = 'e',
        long = "ok-to-send-env",
        help = "🔓 Include .env files (for when you want to leak your API keys like a pro)"
    )]
    ok_to_send_env: bool,

    #[arg(
        short = 'p',
        long = "no-push",
        help = "📦 Commit but don't push (for commitment-phobic developers)"
    )]
    no_push: bool,

    #[arg(
        short = 't',
        long = "max-tokens",
        help = "🤖 How much the AI can ramble (higher = more verbose commits)",
        default_value = "2000"
    )]
    max_tokens: u16,

    #[arg(
        short = 'i',
        long = "max-input-chars",
        help = "✂️ Maximum characters of git diff to send to AI (0 = unlimited)",
        default_value = "50000"
    )]
    max_input_chars: usize,

    #[arg(
        short = 'm',
        long = "model",
        help = "🧠 Pick your AI overlord (gpt-4.1-nano is fast and efficient)",
        default_value = "gpt-4.1-nano"
    )]
    model: String,

    #[arg(
        short = 's',
        long = "setup-alias",
        help = "🛠️ Setup 'nc' alias for easy access"
    )]
    setup_alias: bool,

    #[arg(
        short = 'M',
        long = "yes-to-modules",
        help = "📦 Include dependency folders (node_modules, venv, etc) - WARNING: This will make your repo HUGE!"
    )]
    yes_to_modules: bool,

    #[arg(
        short = 'c',
        long = "yes-to-crap",
        help = "🗑️ Include cache/build artifacts (__pycache__, .DS_Store, etc) - Not recommended!"
    )]
    yes_to_crap: bool,

    #[arg(
        short = 'b',
        long = "br-huehuehue",
        help = "🇧🇷 Output advice in Brazilian Portuguese with extra humor"
    )]
    br_huehuehue: bool,

    #[arg(
        short = 'a',
        long = "no-f-ads",
        help = "🙊 Disable the silly post-commit tagline",
        default_value_t = false
    )]
    no_f_ads: bool,

    #[arg(
        short = 'u',
        long = "update",
        help = "🚀 Update noob-commit to the latest version"
    )]
    update: bool,
}

fn setup_alias() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤡 Setting up 'nc' alias for noob-commit...");

    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let shell_name = Path::new(&shell).file_name().unwrap().to_str().unwrap();

    let config_file = match shell_name {
        "zsh" => {
            let mut path = env::var("HOME")?;
            path.push_str("/.zshrc");
            path
        }
        "bash" => {
            let mut path = env::var("HOME")?;
            path.push_str("/.bashrc");
            path
        }
        "fish" => {
            let mut path = env::var("HOME")?;
            path.push_str("/.config/fish/config.fish");
            path
        }
        _ => {
            println!("⚠️  Unknown shell: {}. Please manually add 'alias nc=noob-commit' to your shell config.", shell_name);
            return Ok(());
        }
    };

    let alias_line = "alias nc='noob-commit'";

    // Check if alias already exists
    if let Ok(content) = fs::read_to_string(&config_file) {
        if content.contains("alias nc") || content.contains("nc='noob-commit'") {
            println!("✅ 'nc' alias already exists!");
            return Ok(());
        }
    }

    // Add alias to config file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config_file)?;

    writeln!(file, "\n# Added by noob-commit")?;
    writeln!(file, "{}", alias_line)?;

    println!("✅ Added 'nc' alias to {}", config_file);
    println!(
        "💡 Restart your terminal or run 'source {}' to use 'nc' command",
        config_file
    );

    Ok(())
}

fn load_api_key() -> Result<String, String> {
    // First, check environment variable
    if let Ok(key) = env::var("OPENAI_API_KEY") {
        if !key.is_empty() {
            return Ok(key);
        }
    }

    // If not in environment, try to load from .env file
    if let Ok(env_content) = fs::read_to_string(".env") {
        for line in env_content.lines() {
            let line = line.trim();
            if line.starts_with("OPENAI_API_KEY=") {
                let key = line[15..].trim().trim_matches('"').trim_matches('\'');
                if !key.is_empty() {
                    return Ok(key.to_string());
                }
            }
        }
    }

    Err("🔑 Oops! You forgot to set OPENAI_API_KEY. Even noobs need API keys!\n💡 Get one at https://platform.openai.com/api-keys".to_string())
}

fn is_security_file(file_path: &str) -> bool {
    let filename = Path::new(file_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("");

    // Block .env files anywhere in the directory structure
    if filename == ".env" || filename.starts_with(".env.") {
        return true;
    }

    // Block other security files
    matches!(
        filename,
        ".env.local"
            | ".env.production"
            | ".env.development"
            | ".env.test"
            | ".env.staging"
            | ".npmrc"
            | ".pypirc"
            | "credentials"
            | "secrets.yml"
            | "secrets.yaml"
            | "id_rsa"
            | "id_ed25519"
            | "id_ecdsa"
            | "id_dsa"
    ) || (filename.starts_with(".env.") && filename.ends_with(".local"))
}

fn is_module_directory(path: &str) -> bool {
    // Check if any part of the path contains common module/dependency directories
    let parts: Vec<&str> = path.split('/').collect();
    for part in parts {
        if matches!(
            part,
            "node_modules" |
            "venv" |
            ".venv" |
            "env" |
            "virtualenv" |
            ".virtualenv" |
            "vendor" |
            "bower_components" |
            "jspm_packages" |
            ".npm" |
            ".yarn" |
            ".pnpm-store" |
            "pip-wheel-metadata" |
            ".tox" |
            ".nox" |
            ".hypothesis" |
            ".pytest_cache" |
            "htmlcov" |
            ".coverage" |
            "target" |  // Rust
            "Pods" |    // iOS
            ".gradle" | // Android
            "build" |   // Various build systems
            "dist" // Distribution folders
        ) {
            return true;
        }
    }
    false
}

fn is_crap_file(path: &str) -> bool {
    // Check if the path contains cache/build artifacts
    let filename = Path::new(path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("");

    // Check exact filenames
    if matches!(
        filename,
        ".DS_Store" | "Thumbs.db" | "desktop.ini" | ".gitkeep" | ".keep"
    ) {
        return true;
    }

    // Check if it's an editor temp file
    if filename.ends_with(".swp") || filename.ends_with(".swo") || filename.ends_with(".swn") {
        return true;
    }

    // Check if it's a backup/temp file
    if filename.ends_with(".log")
        || filename.ends_with(".tmp")
        || filename.ends_with(".temp")
        || filename.ends_with(".cache")
        || filename.ends_with(".bak")
        || filename.ends_with(".backup")
        || filename.ends_with(".old")
        || filename.ends_with(".orig")
        || filename.ends_with("~")
    {
        return true;
    }

    // Check file extensions more carefully
    // We want to filter actual compiled/generated files, not files that happen to contain
    // these strings in their name (e.g., "demo.pyc" as a regular file name)
    if let Some(ext) = Path::new(filename).extension().and_then(|e| e.to_str()) {
        // Only filter if it's ACTUALLY a compiled/generated file extension
        match ext {
            // Python compiled
            "pyc" | "pyo" | "pyd" => {
                // Don't filter if it's clearly not a Python bytecode file
                // Real Python bytecode files are binary and typically in __pycache__
                // or have names like script.cpython-39.pyc
                if path.contains("__pycache__") || filename.contains("cpython") {
                    return true;
                }
                // Also filter if it's in a directory that suggests it's compiled
                if path.contains("/build/")
                    || path.contains("/dist/")
                    || path.contains("/.eggs/")
                    || path.contains("/wheelhouse/")
                {
                    return true;
                }
                // For standalone .pyc files, we'll be conservative and filter them
                // unless they're in a source directory (which would be unusual)
                if !path.contains("/src/") && !path.contains("/docs/") {
                    return true;
                }
                return false;
            }
            // Native libraries - be careful not to filter legitimate shared libraries
            "so" | "dylib" | "dll" => {
                // Filter if it looks like a build artifact
                return path.contains("/build/")
                    || path.contains("/dist/")
                    || path.contains("/target/")
                    || path.contains("/.libs/");
            }
            // Java
            "class" => return true, // Java compiled files
            "jar" => {
                // JAR files in dependencies folders should be filtered
                return path.contains("/lib/")
                    || path.contains("/libs/")
                    || path.contains("/vendor/")
                    || path.contains("/dependencies/");
            }
            // C/C++ objects
            "o" | "a" => {
                // Object files and archives are typically build artifacts
                return true;
            }
            // Windows executables
            "exe" => {
                // Filter if in common build directories
                return path.contains("/build/")
                    || path.contains("/dist/")
                    || path.contains("/target/")
                    || path.contains("/bin/")
                    || path.contains("/Debug/")
                    || path.contains("/Release/");
            }
            // Debug databases
            "idb" | "pdb" => return true,
            // Other build artifacts
            "sage" => return true,
            _ => {}
        }
    }

    // Special handling for egg-info (it's a directory suffix, not a file extension)
    if filename.ends_with(".egg-info") || path.contains(".egg-info/") {
        return true;
    }

    // Check if path contains cache directories
    path.contains("/__pycache__/")
        || path.contains("/.pytest_cache/")
        || path.contains("/.mypy_cache/")
        || path.contains("/.ruff_cache/")
        || path.contains("/.sass-cache/")
        || path.contains("/.cache/")
        || path.contains("/.parcel-cache/")
        || path.contains("/.next/")
        || path.contains("/.nuxt/")
        || path.contains("/.docusaurus/")
        || path.contains("/.serverless/")
        || path.contains("/.fusebox/")
        || path.contains("/.dynamodb/")
        || path.contains("/.tern-port")
        || path.contains("/.yarn-integrity")
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .format(|buf, record| {
            use std::io::Write;
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            
            let level_icon = match record.level() {
                log::Level::Error => "❌",
                log::Level::Warn => "⚠️ ",
                log::Level::Info => "✨",
                log::Level::Debug => "🔍",
                log::Level::Trace => "📝",
            };
            
            writeln!(buf, "{} {} {}", level_icon, ts, record.args())
        })
        .filter_level(cli.verbose.log_level_filter())
        .init();

    // Handle alias setup
    if cli.setup_alias {
        match setup_alias() {
            Ok(_) => return Ok(()),
            Err(e) => {
                error!("Failed to setup alias: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Handle update
    if cli.update {
        info!("🚀 Updating noob-commit to the latest version...");
        
        let update_output = Command::new("cargo")
            .args(&["install", "noob-commit", "--force"])
            .output()
            .expect("Failed to run cargo install");
        
        if update_output.status.success() {
            info!("✅ Successfully updated noob-commit!");
            info!("🎉 You're now running the latest version!");
            return Ok(());
        } else {
            let stderr = str::from_utf8(&update_output.stderr).unwrap();
            error!("😬 Failed to update: {}", stderr);
            error!("💡 Try running: cargo install noob-commit --force");
            std::process::exit(1);
        }
    }

    let api_token = match load_api_key() {
        Ok(key) => key,
        Err(msg) => {
            error!("{}", msg);
            std::process::exit(1);
        }
    };

    // Check if we're in a git repo first
    let is_repo = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect("Failed to check if this is a git repository.")
        .stdout;

    if str::from_utf8(&is_repo).unwrap().trim() != "true" {
        error!("🙈 This isn't a git repo! Run 'git init' first, or cd into your project folder.\n💡 Even noobs need to be in the right directory!");
        std::process::exit(1);
    }

    // Auto-add files, but exclude security files unless explicitly allowed
    let _add_output = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to add files");

    // Get list of all files in the repository
    let all_files_output = Command::new("git")
        .arg("ls-files")
        .arg("--cached")
        .output()
        .expect("Failed to list git files");

    let all_files = str::from_utf8(&all_files_output.stdout).unwrap();
    let mut unstaged_security = false;
    let mut unstaged_modules = false;
    let mut unstaged_crap = false;

    for file_path in all_files.lines() {
        let mut should_unstage = false;
        let mut reason = "";

        // Check security files
        if !cli.ok_to_send_env && is_security_file(file_path) {
            should_unstage = true;
            reason = "security file";
            unstaged_security = true;
        }

        // Check module directories
        if !cli.yes_to_modules && is_module_directory(file_path) {
            should_unstage = true;
            reason = "dependency/module folder";
            unstaged_modules = true;
        }

        // Check crap files
        if !cli.yes_to_crap && is_crap_file(file_path) {
            should_unstage = true;
            reason = "cache/build artifact";
            unstaged_crap = true;
        }

        if should_unstage {
            let flag_hint = match reason {
                "security file" => "--ok-to-send-env",
                "dependency/module folder" => "--yes-to-modules",
                _ => "--yes-to-crap"
            };
            info!("Excluding {}: {} (use {} to include)", reason, file_path, flag_hint);

            let unstage_result = Command::new("git")
                .arg("reset")
                .arg("HEAD")
                .arg(file_path)
                .output();

            if let Err(e) = unstage_result {
                error!("⚠️  Failed to unstage {}: {}", file_path, e);
            }
        }
    }

    // Show summary messages
    if unstaged_security || unstaged_modules || unstaged_crap {
        println!("\n{}", "─".repeat(60));
        if unstaged_security {
            println!("🔒 Protected security files");
            println!("   → Use --ok-to-send-env to include (not recommended)");
        }
        if unstaged_modules {
            println!("📦 Excluded dependency folders");
            println!("   → Use --yes-to-modules to include (large files)");
        }
        if unstaged_crap {
            println!("🗑️  Excluded build artifacts");
            println!("   → Use --yes-to-crap to include (not recommended)");
        }
        println!("{}", "─".repeat(60));
    }

    let git_staged_cmd = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .output()
        .expect("Couldn't find diff.")
        .stdout;

    let git_staged_cmd = str::from_utf8(&git_staged_cmd).unwrap();

    if git_staged_cmd.is_empty() {
        error!("🤷 Nothing to commit! Did you actually write any code?\n💡 If you did, something went wrong with auto-adding files.");
        std::process::exit(1);
    }

    let client = async_openai::Client::with_config(OpenAIConfig::new().with_api_key(api_token));

    let output = Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .output()
        .expect("Couldn't find diff.")
        .stdout;
    let mut output = str::from_utf8(&output).unwrap().to_string();
    
    // Count tokens and optionally trim the git diff
    let bpe = cl100k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(&output);
    let token_count = tokens.len();
    
    if cli.verbose.log_level().is_some() {
        info!("Git diff: {} characters, {} tokens", output.len(), token_count);
    }
    
    // Trim the git diff if it exceeds max_input_chars
    if cli.max_input_chars > 0 && output.len() > cli.max_input_chars {
        if cli.verbose.log_level().is_some() {
            info!("✂️  Trimming git diff from {} to {} characters", output.len(), cli.max_input_chars);
        }
        output.truncate(cli.max_input_chars);
        output.push_str("\n... (diff truncated due to size limit)");
        
        // Recount tokens after truncation
        let new_tokens = bpe.encode_with_special_tokens(&output);
        if cli.verbose.log_level().is_some() {
            info!("After truncation: {} characters, {} tokens", output.len(), new_tokens.len());
        }
    }

    if !cli.dry_run && cli.verbose.is_silent() {
        println!("\n🤖 Analyzing your changes...");
    }

    let sp: Option<Spinner> = if !cli.dry_run && cli.verbose.is_silent() {
        let vs = [
            Spinners::Earth,
            Spinners::Aesthetic,
            Spinners::Hearts,
            Spinners::BoxBounce,
            Spinners::BoxBounce2,
            Spinners::BouncingBar,
            Spinners::Christmas,
            Spinners::Clock,
            Spinners::FingerDance,
            Spinners::FistBump,
            Spinners::Flip,
            Spinners::Layer,
            Spinners::Line,
            Spinners::Material,
            Spinners::Mindblown,
            Spinners::Monkey,
            Spinners::Noise,
            Spinners::Point,
            Spinners::Pong,
            Spinners::Runner,
            Spinners::SoccerHeader,
            Spinners::Speaker,
            Spinners::SquareCorners,
            Spinners::Triangle,
        ];

        let mut rng = rand::rng();
        let spinner = vs.choose(&mut rng).unwrap().clone();

        Some(Spinner::new(spinner, "AI is thinking...".into()))
    } else {
        None
    };

    let settings = SchemaSettings::openapi3().with(|s| {
        s.inline_subschemas = true;
    });
    let mut generator = SchemaGenerator::new(settings);

    let commit_schema = generator.subschema_for::<CommitAdvice>();

    let mut system_prompt = "You are an experienced programmer who writes great commit messages. Analyze the git diff and call the commit function with exactly these fields: 'message' (string for the developer) and 'commit' object containing 'title' (string) and 'description' (string). IMPORTANT: Use each field name exactly once - no duplicates. 

CRITICAL SECURITY CHECK: Carefully scan the diff for actual API keys, tokens, passwords, or secrets (not just variable names or comments). Look for patterns like:
- OpenAI API keys (sk-...)
- AWS keys (AKIA...)
- JWT tokens
- Database passwords
- Private keys
- Auth tokens

If you detect ACTUAL secrets (not just references), respond with: 'CRITICAL: API KEY/SECRET DETECTED in file [filename] - DO NOT COMMIT! The secret appears to be: [type of secret]' in the message field.".to_string();
    if !cli.no_f_ads {
        system_prompt.push_str(" Always append 'One more noob commit by arthrod/noob-commit 🤡' to the end of the commit description.");
    }
    if cli.br_huehuehue {
        system_prompt.push_str(" Respond in Brazilian Portuguese with a playful tone and add 'huehuehue' when it makes sense.");
    }

    let completion = client
        .chat()
        .create(
            CreateChatCompletionRequestArgs::default()
                .messages(vec![
                    ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                        content: ChatCompletionRequestSystemMessageContent::Text(system_prompt),
                        name: None,
                    }),
                    ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                        content: ChatCompletionRequestUserMessageContent::Text(format!(
                            "Here's the git diff:\n{}",
                            output
                        )),
                        name: None,
                    }),
                ])
                .tools(vec![ChatCompletionTool {
                    r#type: ChatCompletionToolType::Function,
                    function: FunctionObject {
                        name: "commit".to_string(),
                        description: Some(
                            "Returns a message for the developer and a structured commit."
                                .to_string(),
                        ),
                        parameters: Some(serde_json::to_value(commit_schema).unwrap()),
                        strict: Some(false),
                    },
                }])
                .tool_choice("commit".to_string())
                .model(&cli.model)
                .temperature(0.0)
                .max_tokens(cli.max_tokens)
                .build()
                .unwrap(),
        )
        .await
        .expect("Couldn't complete prompt.");

    if sp.is_some() {
        sp.unwrap().stop_with_message("✅ Analysis complete!".into());
    }

    let tool_calls = &completion.choices[0].message.tool_calls;
    let (noob_msg, commit_msg) = if let Some(tool_calls) = tool_calls {
        if let Some(tool_call) = tool_calls.first() {
            let advice: CommitAdvice = match serde_json::from_str(&tool_call.function.arguments) {
                Ok(advice) => advice,
                Err(e) => {
                    error!("Failed to parse AI response: {}", e);
                    error!("Raw response: {}", tool_call.function.arguments);
                    std::process::exit(1);
                }
            };
            (advice.message, advice.commit.to_string())
        } else {
            error!("No tool calls in response");
            std::process::exit(1);
        }
    } else {
        error!("No tool calls in response");
        std::process::exit(1);
    };

    println!("\n{}", "═".repeat(60));
    println!("📝 PROPOSED COMMIT MESSAGE");
    println!("{}", "─".repeat(60));
    println!("{}", commit_msg);
    println!("{}", "─".repeat(60));
    println!("💬 AI FEEDBACK: {}", noob_msg);
    println!("{}", "═".repeat(60));
    
    if cli.dry_run {
        return Ok(());
    }

    if !cli.force {
            let answer = Question::new("Do you want to continue? (Y/n)")
                .yes_no()
                .until_acceptable()
                .default(Answer::YES)
                .ask()
                .expect("Couldn't ask question.");

            if answer == Answer::NO {
                error!("😅 Chickened out? That's okay, even I would be scared of my own commits sometimes.");
                std::process::exit(1);
            }
            println!("\n🚀 Creating commit...");
        }

    let mut ps_commit = Command::new("git")
        .arg("commit")
        .args(if cli.review { vec!["-e"] } else { vec![] })
        .arg("-F")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = ps_commit.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(commit_msg.as_bytes())
            .expect("Failed to write to stdin");
    });

    let commit_output = ps_commit
        .wait_with_output()
        .expect("There was an error when creating the commit.");

    if commit_output.status.success() {
        println!("✅ Commit created successfully!");
    } else {
        error!("Failed to create commit: {}", str::from_utf8(&commit_output.stderr).unwrap());
        std::process::exit(1);
    }

    // Push to remote if not disabled
    if !cli.no_push {
        print!("🌐 Pushing to remote...");
        io::stdout().flush().unwrap();
        let push_output = Command::new("git")
            .arg("push")
            .output()
            .expect("Failed to push to remote");

        if push_output.status.success() {
            println!(" ✅ Successfully pushed!");
        } else {
            println!(" ❌ Push failed");
            let stderr = str::from_utf8(&push_output.stderr).unwrap();
            error!("Error details: {}", stderr);
            println!("💡 Tip: Try 'git pull' first, then run noob-commit again");
        }
    }

    if !cli.no_f_ads {
        println!("\n🤡 One more noob commit by arthrod/noob-commit");
    }

    Ok(())
}
