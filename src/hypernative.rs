use reqwest;
use std::env;
use serde::{Serialize, Deserialize, de};
use chrono::{Duration, Utc};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError(String);

impl CustomError {
    fn new(message: &str) -> Self {
        CustomError(message.to_string())
    }
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


// -- API AUTHENTICATION --------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct AccessToken {
    token: String,
    lifespan_in_seconds: i64,
    expires_at: chrono::DateTime<Utc>,
}

impl AccessToken {
    pub fn new(lifespan_in_seconds: i64) -> Self {
        let expires_at = Utc::now() - Duration::seconds(1);
        AccessToken { token: String::new(), lifespan_in_seconds, expires_at }
    }

    fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    fn refresh(&mut self, new_token: String) {
        self.token = new_token;
        self.expires_at = Utc::now() + Duration::seconds(self.lifespan_in_seconds);
    }
}

#[derive(Serialize, Debug)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    token: String
}

pub async fn refresh_bearer_token(access_token: &mut AccessToken) -> Result<(), Box<dyn Error + Send>> {
    let body = LoginRequest{
        email: env::var("HYPERNATIVE_USERNAME").expect("HYPERNATIVE_USERNAME not found in the .env file."),
        password: env::var("HYPERNATIVE_PASSWORD").expect("HYPERNATIVE_PASSWORD not found in the .env file."),
    };
    let body = serde_json::to_string(&body).expect("Failed to serialize RequestBody");
    println!("{:#?}", body);
    
    let response = reqwest::Client::new()
        .post("https://api.hypernative.xyz/auth/login")
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|err| Box::new(err) as Box<dyn Error + Send>)?;

    println!("{:#?}", response);
    if response.status().is_success() {
        println!("Authenticated successfully as: {}", "wartull@olympusdao.finance");
        let json_response = response.text().await.map_err(|err| Box::new(err) as Box<dyn Error + Send>)?;
        let parsed_response: RequestResponse = serde_json::from_str(&json_response).unwrap();
        match parsed_response.data {
            ResponseData::Login(login_data) => {
                access_token.refresh(login_data.token);
                Ok(())
            },
            _ => Err(Box::new(CustomError::new("Unexpected API Response. Unable to update Hypernative auth token!".into())))
        }
    } else {
        eprintln!("Request failed! Status: {}", response.status());
        return Err(Box::new(CustomError::new("Unable to authenticate in the Hypernative platform!".into())));
    }
}


// -- CUSTOM AGENT LOGIC --------------------------------------------------------------------------

#[derive(Serialize, Debug)]
pub struct RequestBody {
    #[serde(rename = "agentType")]
    agent_type: String,
    #[serde(rename = "agentName")]
    agent_name: String,
    severity: Severity,
    #[serde(rename = "muteDuration")]
    mute_duration: i64,
    state: State,
    rule: Rule,
    #[serde(rename = "channelsConfigurations")]
    channels_configurations: Vec<ChannelConfiguration>,
    #[serde(rename = "remindersConfigurations")]
    reminders_configurations: Vec<ChannelConfiguration>,
    delay: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    chain: String,
    input: Vec<String>,
    #[serde(rename = "funcSig")]
    func_sig: String,
    #[serde(rename = "fileName")]
    file_name: String,
    operands: Vec<String>,
    operator: String,
    #[serde(rename = "ruleString")]
    rule_string: String,
    #[serde(rename = "outputIndex")]
    output_index: i64,
    #[serde(rename = "inputDataType")]
    input_data_type: Vec<String>,
    #[serde(rename = "outputDataType")]
    output_data_type: Vec<String>,
    #[serde(rename = "contractAddress")]
    contract_address: String,
    #[serde(rename = "contractAddressAlias")]
    contract_address_alias: String,
    #[serde(rename = "contractFunctionObject")]
    contract_function_object: ContractFunction,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContractFunction {
    name: String,
    #[serde(rename = "type")]
    function_type: String,
    inputs: Vec<FunctionInput>,
    outputs: Vec<FunctionOutput>,
    #[serde(rename = "stateMutability")]
    state_mutability: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FunctionInput {
    name: String,
    #[serde(rename = "type")]
    input_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FunctionOutput {
    name: String,
    #[serde(rename = "type")]
    output_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelConfiguration {
    #[serde(rename = "channelType")]
    channel_type: ChannelType,
    configuration: Configuration,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Configuration {
    #[serde(rename = "email")]
    Email(Vec<String>),
    #[serde(rename = "url")]
    Webhook(Vec<String>),
}

#[derive(Debug)]
pub enum ChannelType {
    Email,
    Discord,
    Slack,
}

impl Serialize for ChannelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        let channel_type_str = match self {
            ChannelType::Email => "Email",
            ChannelType::Discord => "Discord",
            ChannelType::Slack => "Slack",
        };
        serializer.serialize_str(channel_type_str)
    }
}

impl<'de> Deserialize<'de> for ChannelType {
    fn deserialize<D>(deserializer: D) -> Result<ChannelType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ChannelTypeVisitor;

        impl<'de> de::Visitor<'de> for ChannelTypeVisitor {
            type Value = ChannelType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing ChannelType")
            }

            fn visit_str<E>(self, value: &str) -> Result<ChannelType, E>
            where
                E: de::Error,
            {
                match value {
                    "Email" => Ok(ChannelType::Email),
                    "Discord" => Ok(ChannelType::Discord),
                    "Slack" => Ok(ChannelType::Slack),
                    _ => Err(de::Error::unknown_variant(value, &["Email", "Discord", "Slack"])),
                }
            }
        }

        deserializer.deserialize_str(ChannelTypeVisitor)
    }
}


impl ChannelConfiguration {
    pub fn new_email(email: Vec<String>) -> Self {
        ChannelConfiguration {
            channel_type: ChannelType::Email,
            configuration: Configuration::Email(email),
        }
    }

    pub fn new_discord(url: Vec<String>) -> Self {
        ChannelConfiguration {
            channel_type: ChannelType::Discord,
            configuration: Configuration::Webhook(url),
        }
    }

    pub fn new_slack(url: Vec<String>) -> Self {
        ChannelConfiguration {
            channel_type: ChannelType::Slack,
            configuration: Configuration::Webhook(url),
        }
    }
}

#[derive(Debug)]
enum Severity {
    High,
    Medium,
}

impl Serialize for Severity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        let severity_type_str = match self {
            Severity::High => "High",
            Severity::Medium => "Medium",
        };
        serializer.serialize_str(severity_type_str)
    }
}

#[derive(Debug)]
enum State {
    Enabled,
    Disabled,
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        let state_type_str = match self {
            State::Enabled => "enabled",
            State::Disabled => "disabled",
        };
        serializer.serialize_str(state_type_str)
    }
}

fn build_request_body(
    input: Vec<String>,
    threshold: i64,
    email: Option<String>, 
    webhook: Option<String>
) -> Result<RequestBody, Box<dyn Error + Send>> {
    let cooler = input[0].clone();
    let loan = input[1].clone();

    let rule = Rule {
        chain: String::from("ethereum"),
        input: input,
        func_sig: String::from("timeToExpiry(address cooler_, uint256 id_)"),
        file_name: String::from(""),
        operands: vec![threshold.to_string()],
        operator: String::from("lt"),
        rule_string: format!("Cooler ({cooler}) Loan (id: {loan}) expires in less than {} days", threshold/(24*3600)),
        output_index: 0,
        input_data_type: vec![String::from("address"), String::from("uint256")],
        output_data_type: vec![String::from("uint256")],
        contract_address: String::from("0xA00F4b7c57a4995796D6E2ae4A6D5dEc8a557367"),
        contract_address_alias: String::from("Cooler Monitoring"),
        contract_function_object: ContractFunction {
            name: String::from("timeToExpiry"),
            function_type: String::from("function"),
            inputs: vec![
                FunctionInput {
                    name: String::from("cooler_"),
                    input_type: String::from("address"),
                },
                FunctionInput {
                    name: String::from("id_"),
                    input_type: String::from("uint256"),
                },
            ],
            outputs: vec![FunctionOutput {
                name: String::from("timeLeft"),
                output_type: String::from("uint256"),
            }],
            state_mutability: String::from("view"),
        }
    };

    let mut channel_configurations: Vec<ChannelConfiguration> = Vec::new(); 
    if let Some(email) = email {
        channel_configurations.push(
            ChannelConfiguration::new_email(vec![email])
        );
    }
    if let Some(url) = webhook {
        channel_configurations.push(
            ChannelConfiguration::new_discord(vec![url])
        );
    }

    let request_body = RequestBody {
        agent_type: String::from("genericNodeQuery"),
        agent_name: String::from("COOLER ALERTS"),
        severity: Severity::Medium,
        mute_duration: 0,
        state: State::Disabled,
        rule,
        channels_configurations: channel_configurations,
        reminders_configurations: vec![],
        delay: 50000,
    };

    Ok(request_body)
}

pub async fn create_custom_agent(
    access_token: &mut AccessToken,
    input: Vec<String>,
    threshold: i64,
    email: Option<String>, 
    webhook: Option<String>
) -> Result<Option<CustomAgentResponse>, Box<dyn Error + Send>> {
    // Ensure token hasn't expired
    if access_token.is_expired() { refresh_bearer_token(access_token).await?; }

    match build_request_body(input, threshold, email, webhook) {
        Ok(body) => {
            let json_body = serde_json::to_string(&body).expect("Failed to serialize RequestBody");
            let response = reqwest::Client::new()
                .post("https://api.hypernative.xyz/custom-agents")
                .header("accept", "application/json")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", &access_token.token))
                .body(json_body)
                .send()
                .await
                .map_err(|err| Box::new(err) as Box<dyn Error + Send>)?;

            if response.status().is_success() {
                println!("Request successful! Status: {}", response.status());
                let json_response = response.text().await
                .map_err(|err| Box::new(err) as Box<dyn Error + Send>)?;
                let parsed_response: RequestResponse = serde_json::from_str(&json_response)
                .map_err(|err| Box::new(err) as Box<dyn Error + Send>)?;
                match parsed_response.data {
                    ResponseData::CustomAgent(custom_agent_data) => Ok(Some(custom_agent_data)),
                    ResponseData::Login(login_data) => {
                        Ok(None)
                    }
                }
            } else if response.status().is_server_error() {
                eprintln!("Request failed! Status: {}", response.status());
                return Ok(None);
            } else {
                eprintln!("Request failed! Status: {}", response.status());
                return Err(Box::new(CustomError::new("Request failed!".into())));
            }
        }
        Err(err) => {
            eprintln!("Error: {:#?}", err);
            return Err(err)?;
        }
     }
}



#[derive(Deserialize, Debug)]
pub struct RequestResponse {
    data: ResponseData,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseData {
    Login(LoginResponse),
    CustomAgent(CustomAgentResponse),
}

#[derive(Deserialize, Debug)]
pub struct CustomAgentResponse {
    id: i64,
    rule: Rule,
    #[serde(rename = "alertPolicies")]
    alert_policies: Vec<AlertPolicy>,
}

#[derive(Deserialize, Debug)]
pub struct AlertPolicy {
    #[serde(rename = "channelsConfigurations")]
    channels_configurations: Vec<ChannelConfiguration>,
}

impl CustomAgentResponse {
    pub fn agent_id(&self) -> &i64 {
        &self.id
    }

    pub fn agent_rules(&self) -> &Rule {
        &self.rule
    }

    pub fn alert_policies(&self) -> &Vec<AlertPolicy> {
        &self.alert_policies
    }
}


pub async fn delete_custom_agent(
    access_token: &mut AccessToken,
    id: &i64
) -> Result<(), Box<dyn Error + Send>> {
    // Ensure token hasn't expired
    if access_token.is_expired() { refresh_bearer_token(access_token).await?; }

    let response = reqwest::Client::new()
        .delete(format!("https://api.hypernative.xyz/custom-agents/{id}"))
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", &access_token.token))
        .send()
        .await
        .map_err(|err| Box::new(err) as Box<dyn Error + Send>)?;;

    if response.status().is_success() {
        println!("Request successful! Status: {}", response.status());
        println!("Agent deleted!");
    } else {
        eprintln!("Request failed! Status: {}", response.status());
    }

    Ok(())
}
