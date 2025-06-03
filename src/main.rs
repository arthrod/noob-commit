use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionFunctionCall, ChatCompletionFunctions, ChatCompletionRequestMessage,
        CreateChatCompletionRequestArgs, FunctionCall, Role,
    },
};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::{error, info};
use question::{Answer, Question};
use rand::seq::SliceRandom;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use serde_json::json;
use spinners::{Spinner, Spinners};
use noob_commit::Commit;
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
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

    #[arg(short, long, help = "⚡ YOLO mode - just commit everything (living dangerously)")]
    force: bool,

    #[arg(
        long = "ok-to-send-env",
        help = "🔓 Include .env files (for when you want to leak your API keys like a pro)"
    )]
    ok_to_send_env: bool,

    #[arg(
        long = "no-push",
        help = "📦 Commit but don't push (for commitment-phobic developers)"
    )]
    no_push: bool,

    #[arg(
        long = "max-tokens",
        help = "🤖 How much the AI can ramble (higher = more verbose commits)",
        default_value = "2000"
    )]
    max_tokens: u16,

    #[arg(
        long = "model",
        help = "🧠 Pick your AI overlord (gpt-4.1-mini is cheap and good enough)",
        default_value = "gpt-4.1-mini"
    )]
    model: String,

    #[arg(
        long = "setup-alias",
        help = "🛠️ Setup 'nc' alias for easy access"
    )]
    setup_alias: bool,
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
        },
        "bash" => {
            let mut path = env::var("HOME")?;
            path.push_str("/.bashrc");
            path
        },
        "fish" => {
            let mut path = env::var("HOME")?;
            path.push_str("/.config/fish/config.fish");
            path
        },
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
    println!("💡 Restart your terminal or run 'source {}' to use 'nc' command", config_file);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
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

    let api_token = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        error!("🔑 Oops! You forgot to set OPENAI_API_KEY. Even noobs need API keys!\n💡 Get one at https://platform.openai.com/api-keys");
        std::process::exit(1);
    });

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

    // Auto-add files, but exclude .env unless explicitly allowed
    if !cli.ok_to_send_env {
        // First add all files
        let _add_output = Command::new("git")
            .arg("add")
            .arg(".")
            .output()
            .expect("Failed to add files");
        
        // Then unstage .env files if they exist
        let env_files = [".env", ".env.local", ".env.development", ".env.production", ".env.test"];
        for env_file in &env_files {
            if Path::new(env_file).exists() {
                let _unstage = Command::new("git")
                    .arg("reset")
                    .arg("HEAD")
                    .arg(env_file)
                    .output();
            }
        }
    } else {
        // Add all files including .env
        let _add_output = Command::new("git")
            .arg("add")
            .arg(".")
            .output()
            .expect("Failed to add files");
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
    let output = str::from_utf8(&output).unwrap();

    if !cli.dry_run {
        info!("Loading Data...");
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

        let spinner = vs.choose(&mut rand::thread_rng()).unwrap().clone();

        Some(Spinner::new(spinner, "Analyzing Codebase...".into()))
    } else {
        None
    };

    let mut generator = SchemaGenerator::new(SchemaSettings::openapi3().with(|settings| {
        settings.inline_subschemas = true;
    }));

    let commit_schema = generator.subschema_for::<Commit>().into_object();

    let completion = client
        .chat()
        .create(
            CreateChatCompletionRequestArgs::default()
                .messages(vec![
                    ChatCompletionRequestMessage {
                        role: Role::System,
                        content: Some(
                            "You are an experienced programmer who writes great commit messages."
                                .to_string(),
                        ),
                        ..Default::default()
                    },
                    ChatCompletionRequestMessage {
                        role: Role::Assistant,
                        content: Some("".to_string()),
                        function_call: Some(FunctionCall {
                            arguments: "{}".to_string(),
                            name: "get_diff".to_string(),
                        }),
                        ..Default::default()
                    },
                    ChatCompletionRequestMessage {
                        role: Role::Function,
                        content: Some(output.to_string()),
                        name: Some("get_diff".to_string()),
                        ..Default::default()
                    },
                ])
                .functions(vec![
                    ChatCompletionFunctions {
                        name: "get_diff".to_string(),
                        description: Some(
                            "Returns the output of `git diff HEAD` as a string.".to_string(),
                        ),
                        parameters: Some(json!({
                            "type": "object",
                            "properties": {}
                        })),
                    },
                    ChatCompletionFunctions {
                        name: "commit".to_string(),
                        description: Some(
                            "Creates a commit with the given title and a description.".to_string(),
                        ),
                        parameters: Some(serde_json::to_value(commit_schema).unwrap()),
                    },
                ])
                .function_call(ChatCompletionFunctionCall::Object(
                    json!({ "name": "commit" }),
                ))
                .model(&cli.model)
                .temperature(0.0)
                .max_tokens(cli.max_tokens)
                .build()
                .unwrap(),
        )
        .await
        .expect("Couldn't complete prompt.");

    if sp.is_some() {
        sp.unwrap().stop_with_message("Finished Analyzing!".into());
    }

    let commit_data = &completion.choices[0].message.function_call;
    let commit_msg = serde_json::from_str::<Commit>(&commit_data.as_ref().unwrap().arguments)
        .expect("Couldn't parse model response.")
        .to_string();

    if cli.dry_run {
        info!("{}", commit_msg);
        return Ok(());
    } else {
        info!(
            "Proposed Commit:\n------------------------------\n{}\n------------------------------",
            commit_msg
        );

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
            info!("Committing Message...");
        }
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

    info!("{}", str::from_utf8(&commit_output.stdout).unwrap());

    // Push to remote if not disabled
    if !cli.no_push {
        info!("Pushing to remote...");
        let push_output = Command::new("git")
            .arg("push")
            .output()
            .expect("Failed to push to remote");
        
        if push_output.status.success() {
            info!("🚀 Pushed to remote! Your code is now bothering other developers.");
        } else {
            let stderr = str::from_utf8(&push_output.stderr).unwrap();
            error!("😬 Push failed: {}\n💡 Maybe someone else pushed first? Try 'git pull' and run me again.", stderr);
        }
    }

    Ok(())
}
