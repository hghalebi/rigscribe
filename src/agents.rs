use crate::error::{map_provider_error, Result, ScribeError};
use crate::types::{Artifact, Intent, Specification};
use rig::agent::Agent;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::{CompletionModel, Prompt};
use rig::providers::gemini::Client;

pub type LlmAgent = Agent<rig::providers::gemini::completion::CompletionModel>;
const MODEL: &str = "gemini-2.0-flash-lite";

pub struct Chief {
    architect: LlmAgent,
    builder: LlmAgent,
    chief: LlmAgent,
}

impl Chief {
    pub fn from_env() -> Result<Self> {
        require_env("GEMINI_API_KEY")?;
        let client = Client::from_env();
        let architect = client
            .agent(MODEL)
            .preamble(
                "
                Role: Senior Solution Architect\n\
                Task: Extract constraints and risks\n\
                Output: A short bullet list, no prose
                ",
            )
            .build();

        let builder = client
            .agent(MODEL)
            .preamble(
                "
                Role: Prompt Engineer\n\
                Task: Write a system prompt that follows the goal and constraints exactly\n\
                Output: Only the system prompt text
                ",
            )
            .build();
        let chief = client
            .agent(MODEL)
            .preamble(
                "
                Role: Chief Prompt Officer\n\
                Task: Review the draft for safety, clarity, and constraint compliance\n\
                Output: Only the final system prompt text
                ",
            )
            .build();
        Ok(Self {
            architect,
            builder,
            chief,
        })
    }

    pub async fn plan(&self, intent: &Intent) -> Result<Specification> {
        let constraints = self
            .architect
            .prompt(&intent.text)
            .await
            .map_err(map_provider_error)?;

        Ok(Specification {
            goal: intent.text.clone(),
            constraints,
        })
    }

    pub async fn build(&self, spec: &Specification) -> Result<String> {
        let input = format!(
            "Goal:\n{}\n\nConstrains:\n{}\n",
            spec.goal, spec.constraints
        );
        self.builder
            .prompt(&input)
            .await
            .map_err(map_provider_error)
    }
    pub async fn review(&self, spec: &Specification, draft: &str) -> Result<Artifact> {
        let input = format!(
            "
            refactorer rebust following prompte:
            Goal:\n{}\n\nConstraints:\n{}\n\nDraft:\n{}\n\n\
            Instruction: Be highly descriptive and use all best practice and if needed use websearch. Return only final system prompt without any addition text final prompt and without asking additional question. \n",
            spec.goal, spec.constraints, draft
        );
        let final_prompt = self
            .chief
            .prompt(&input)
            .await
            .map_err(map_provider_error)?;
        Ok(Artifact::new(final_prompt, "Chief Prompt Officer"))
    }
}

pub fn require_env(name: &str) -> Result<()> {
    if std::env::var(name).is_ok() {
        Ok(())
    } else {
        Err(ScribeError::Config(format!("{name} is missing!")))
    }
}
