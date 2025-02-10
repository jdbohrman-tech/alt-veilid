use super::*;
use serde::*;
use std::path::Path;

use validator::{Validate, ValidationError, ValidationErrors};

const PREDEFINED_CONFIG: &str = include_str!("predefined_config.yml");
const DEFAULT_CONFIG: &str = include_str!("default_config.yml");

#[derive(Debug, ThisError)]
pub enum ConfigError {
    #[error("io error: {0}")]
    IoError(std::io::Error),
    #[error("parse error: {0}: {1}")]
    ParseError(String, serde_yaml::Error),
    #[error("validate error: {0}")]
    ValidateError(String),
    #[error("no configuration files specified")]
    NoConfigFiles,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Profile {
    #[validate(length(min = 1), nested)]
    pub instances: Vec<Instance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Instance {
    Machine { machine: WeightedList<String> },
    Template { template: WeightedList<String> },
}

impl Validate for Instance {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Instance::Machine { machine } => machine.validate()?,
            Instance::Template { template } => template.validate()?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Machine {
    #[serde(flatten)]
    #[validate(nested)]
    pub location: MachineLocation,
    #[serde(default)]
    pub disable_capabilities: Vec<String>,
    #[serde(default)]
    pub bootstrap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MachineLocation {
    Network {
        network: String,
        #[serde(default)]
        address4: Option<Ipv4Addr>,
        #[serde(default)]
        address6: Option<Ipv6Addr>,
    },
}

impl Validate for MachineLocation {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        match self {
            MachineLocation::Network {
                network: _,
                address4,
                address6,
            } => {
                if address4.is_none() && address6.is_none() {
                    errors.add(
                        "MachineLocation",
                        ValidationError::new("badaddr")
                            .with_message("machine must have at least one address".into()),
                    );
                }
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Template {
    #[serde(flatten)]
    #[validate(nested)]
    pub location: TemplateLocation,
    #[serde(flatten)]
    #[validate(nested)]
    pub limits: TemplateLimits,
    #[serde(default)]
    #[validate(custom(function = "validate_disable_capabilities"))]
    pub disable_capabilities: Vec<String>,
}

fn validate_disable_capabilities(disable_capabilities: &[String]) -> Result<(), ValidationError> {
    if disable_capabilities.contains(&("".to_string())) {
        return Err(ValidationError::new("badcap").with_message("empty disabled capability".into()));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_template_limits"))]
pub struct TemplateLimits {
    /// maximum number of machines this template will generate
    #[validate(nested)]
    #[serde(default)]
    pub machine_count: Option<WeightedList<usize>>,
    #[validate(nested)]
    #[serde(default)]
    pub machines_per_network: Option<WeightedList<usize>>,
}

fn validate_template_limits(limits: &TemplateLimits) -> Result<(), ValidationError> {
    let mut has_at_least_one_limit = false;
    if let Some(machine_count) = &limits.machine_count {
        machine_count.try_for_each(|x| {
            if *x == 0 {
                return Err(ValidationError::new("badcount")
                    .with_message("template limits has zero machine count".into()));
            }
            Ok(())
        })?;
        has_at_least_one_limit = true;
    }
    if let Some(machines_per_network) = &limits.machines_per_network {
        machines_per_network.try_for_each(|x| {
            if *x == 0 {
                return Err(ValidationError::new("badcount")
                    .with_message("template limits has zero machines per network count".into()));
            }
            Ok(())
        })?;
        has_at_least_one_limit = true;
    }

    if !has_at_least_one_limit {
        return Err(ValidationError::new("nolimit")
            .with_message("template can not be unlimited per network".into()));
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateLocation {
    Network { network: WeightedList<String> },
    Blueprint { blueprint: WeightedList<String> },
}

impl Validate for TemplateLocation {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            TemplateLocation::Network { network } => network.validate()?,
            TemplateLocation::Blueprint { blueprint } => blueprint.validate()?,
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_network"))]
pub struct Network {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    #[validate(nested)]
    pub ipv4: Option<NetworkIpv4>,
    #[serde(default)]
    #[validate(nested)]
    pub ipv6: Option<NetworkIpv6>,
}

fn validate_network(network: &Network) -> Result<(), ValidationError> {
    if network.ipv4.is_none() && network.ipv6.is_none() {
        return Err(ValidationError::new("badaddr")
            .with_message("network must support at least one address type".into()));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NetworkIpv4 {
    #[validate(length(min = 1))]
    pub allocation: String,
    #[serde(default)]
    #[validate(nested)]
    pub gateway: Option<NetworkGateway>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NetworkIpv6 {
    #[validate(length(min = 1))]
    pub allocation: String,
    #[serde(default)]
    #[validate(nested)]
    pub gateway: Option<NetworkGateway>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NetworkGateway {
    pub translation: Translation,
    pub upnp: bool,
    #[validate(length(min = 1))]
    pub network: Option<String>,
}

////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_blueprint"))]
pub struct Blueprint {
    #[serde(default)]
    #[validate(nested)]
    pub model: Option<WeightedList<String>>,
    #[serde(flatten)]
    #[validate(nested)]
    pub limits: BlueprintLimits,
    #[serde(default)]
    #[validate(nested)]
    pub ipv4: Option<BlueprintIpv4>,
    #[serde(default)]
    #[validate(nested)]
    pub ipv6: Option<BlueprintIpv6>,
}

fn validate_blueprint(blueprint: &Blueprint) -> Result<(), ValidationError> {
    if blueprint.ipv4.is_none() && blueprint.ipv6.is_none() {
        return Err(ValidationError::new("badaddr")
            .with_message("blueprint must support at least one address type".into()));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_blueprint_limits"))]
pub struct BlueprintLimits {
    /// maximum number of networks this blueprint will generate
    #[validate(nested)]
    #[serde(default)]
    pub network_count: Option<WeightedList<usize>>,
}

fn validate_blueprint_limits(limits: &BlueprintLimits) -> Result<(), ValidationError> {
    if let Some(network_count) = &limits.network_count {
        network_count.try_for_each(|x| {
            if *x == 0 {
                return Err(ValidationError::new("badcount")
                    .with_message("blueprint limits has zero network count".into()));
            }
            Ok(())
        })?;
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlueprintLocation {
    Allocation {
        allocation: WeightedList<String>,
    },
    Network {
        #[serde(default)]
        network: Option<WeightedList<String>>,
    },
}

impl Validate for BlueprintLocation {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            BlueprintLocation::Allocation { allocation } => allocation.validate()?,
            BlueprintLocation::Network { network } => {
                if let Some(network) = network {
                    network.validate()?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_blueprint_ipv4"))]
pub struct BlueprintIpv4 {
    #[serde(flatten)]
    #[validate(nested)]
    pub location: BlueprintLocation,
    #[validate(nested)]
    pub prefix: WeightedList<u8>,
    #[serde(default)]
    #[validate(nested)]
    pub gateway: Option<BlueprintGateway>,
}

fn validate_blueprint_ipv4(blueprint_ipv4: &BlueprintIpv4) -> Result<(), ValidationError> {
    blueprint_ipv4.prefix.try_for_each(|x| {
        if *x > 32 {
            return Err(ValidationError::new("badprefix")
                .with_message("ipv4 blueprint prefix too long".into()));
        }
        Ok(())
    })?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_blueprint_ipv6"))]
pub struct BlueprintIpv6 {
    #[serde(flatten)]
    #[validate(nested)]
    pub location: BlueprintLocation,
    #[validate(nested)]
    pub prefix: WeightedList<u8>,
    #[serde(default)]
    #[validate(nested)]
    pub gateway: Option<BlueprintGateway>,
}

fn validate_blueprint_ipv6(blueprint_ipv6: &BlueprintIpv6) -> Result<(), ValidationError> {
    blueprint_ipv6.prefix.try_for_each(|x| {
        if *x > 128 {
            return Err(ValidationError::new("badprefix")
                .with_message("ipv6 blueprint prefix too long".into()));
        }
        Ok(())
    })?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BlueprintGateway {
    #[validate(nested)]
    pub translation: WeightedList<Translation>,
    #[validate(range(min = 0.0, max = 1.0))]
    pub upnp: Probability,
    #[serde(default, flatten)]
    #[validate(nested)]
    pub location: Option<TemplateLocation>,
}

////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Scope4 {
    #[validate(length(min = 1))]
    pub scope4: Vec<Ipv4Net>,
    #[serde(default)]
    pub pool4: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Scope6 {
    #[validate(length(min = 1))]
    pub scope6: Vec<Ipv6Net>,
    #[serde(default)]
    pub pool6: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_distance"))]
pub struct Distance {
    pub min: f32,
    pub max: f32,
}

fn validate_distance(distance: &Distance) -> Result<(), ValidationError> {
    if distance.min < 0.0 {
        return Err(ValidationError::new("baddist")
            .with_message("distance minimum must not be negative".into()));
    }
    if distance.max < distance.min {
        return Err(ValidationError::new("baddist")
            .with_message("distance maximum must not be less than the minimum".into()));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
#[validate(schema(function = "validate_distribution"))]
pub struct Distribution {
    pub mean: f32,
    pub sigma: f32,
    pub skew: f32,
    pub min: f32,
    pub max: f32,
}

fn validate_distribution(distribution: &Distribution) -> Result<(), ValidationError> {
    if distribution.mean < 0.0 {
        return Err(ValidationError::new("baddistrib")
            .with_message("distribution mean must not be negative".into()));
    }
    if distribution.sigma < 0.0 {
        return Err(ValidationError::new("baddistrib")
            .with_message("distribution sigma must not be negative".into()));
    }
    if distribution.max < distribution.min {
        return Err(ValidationError::new("baddistrib")
            .with_message("distribution maximum must not be less than the minimum".into()));
    }
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Translation {
    None,
    PortRestricted,
    AddressRestricted,
    Symmetric,
}

impl Default for Translation {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Model {
    #[validate(nested)]
    pub latency: Distribution,
    #[serde(default)]
    #[validate(nested)]
    pub distance: Option<Distance>,
    #[serde(default)]
    #[validate(range(min = 0.0, max = 1.0))]
    pub loss: Probability,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Allocation {
    #[serde(flatten)]
    #[validate(nested)]
    pub scope4: Option<Scope4>,
    #[serde(flatten)]
    #[validate(nested)]
    pub scope6: Option<Scope6>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub default_network: Option<String>,
    #[serde(default)]
    pub default_model: Option<String>,
    #[serde(default)]
    pub default_pool: Option<String>,
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
    #[serde(default)]
    pub machines: HashMap<String, Machine>,
    #[serde(default)]
    pub templates: HashMap<String, Template>,
    #[serde(default)]
    pub networks: HashMap<String, Network>,
    #[serde(default)]
    pub blueprints: HashMap<String, Blueprint>,
    #[serde(default)]
    pub allocations: HashMap<String, Allocation>,
    #[serde(default)]
    pub models: HashMap<String, Model>,
}

impl Validate for Config {
    fn validate(&self) -> Result<(), ValidationErrors> {
        // Validate config
        let mut errors = ValidationErrors::new();

        if let Some(default_network) = self.default_network.as_ref() {
            if default_network.is_empty() {
                errors.add(
                    "default_network",
                    ValidationError::new("badlen").with_message(
                        "Config must have non-empty default network if specified".into(),
                    ),
                );
            }
        }

        if let Some(default_model) = self.default_model.as_ref() {
            if default_model.is_empty() {
                errors.add(
                    "default_model",
                    ValidationError::new("badlen").with_message(
                        "Config must have non-empty default model if specified".into(),
                    ),
                );
            }
        }

        if let Some(default_pool) = self.default_pool.as_ref() {
            if default_pool.is_empty() {
                errors.add(
                    "default_pool",
                    ValidationError::new("badlen").with_message(
                        "Config must have non-empty default pool if specified".into(),
                    ),
                );
            }
        }

        errors.merge_self("profiles", validate_hash_map(&self.profiles));
        errors.merge_self("machines", validate_hash_map(&self.machines));
        errors.merge_self("templates", validate_hash_map(&self.templates));
        errors.merge_self("networks", validate_hash_map(&self.networks));
        errors.merge_self("blueprints", validate_hash_map(&self.blueprints));
        errors.merge_self("allocation", validate_hash_map(&self.allocations));
        errors.merge_self("models", validate_hash_map(&self.models));

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

fn expand_validation_errors(errors: ValidationErrors) -> String {
    let mut out = String::new();
    let errors = errors.into_errors();
    let mut keys: Vec<&str> = errors.keys().copied().collect();
    keys.sort();
    for k in keys {
        let v = errors.get(k).unwrap();
        let v_out = match v.clone() {
            validator::ValidationErrorsKind::Struct(validation_errors) => {
                expand_validation_errors(*validation_errors)
            }
            validator::ValidationErrorsKind::List(btree_map) => {
                let mut l_out = String::new();
                for (_, v) in btree_map {
                    l_out += &expand_validation_errors(*v);
                }
                l_out
            }
            validator::ValidationErrorsKind::Field(vec) => {
                let mut v_out = String::new();
                for v in vec {
                    v_out += &format!("{}\n", v);
                }
                v_out
            }
        };
        let v_out = indent::indent_all_by(4, v_out);

        out += &format!("{k}:\n{v_out}\n");
    }
    out
}

fn map_validation_error<S: AsRef<str>>(
    name: S,
) -> impl FnOnce(validator::ValidationErrors) -> ConfigError {
    let name = name.as_ref().to_string();
    move |errors| {
        ConfigError::ValidateError(format!("{name}: {}", expand_validation_errors(errors)))
    }
}

impl Config {
    pub fn new<P: AsRef<Path>>(
        config_files: &[P],
        no_predefined_config: bool,
    ) -> Result<Self, ConfigError> {
        let mut out = Self::default();

        if !no_predefined_config {
            out = load_predefined_config()?;
            out.validate()
                .map_err(map_validation_error("<predefined config>"))?;

            // Load default config file
            if config_files.is_empty() {
                let cfg: Self = load_default_config()?;
                cfg.validate()
                    .map_err(map_validation_error("<default config>"))?;

                out = out.combine(cfg)?;
            }
        } else {
            // There must be config files specified to use this option
            if config_files.is_empty() {
                return Err(ConfigError::NoConfigFiles);
            }
        }

        // Load specified config files
        for config_file in config_files {
            let cfg: Self = load_config_file(config_file)?;
            cfg.validate().map_err(map_validation_error(format!(
                "{}",
                config_file.as_ref().to_string_lossy()
            )))?;

            out = out.combine(cfg)?;
        }

        Ok(out)
    }

    pub fn combine(self, other: Self) -> Result<Self, ConfigError> {
        let out = Config {
            seed: other.seed.or(self.seed),
            default_network: other.default_network.or(self.default_network),
            default_model: other.default_model.or(self.default_model),
            default_pool: other.default_pool.or(self.default_pool),
            profiles: self.profiles.into_iter().chain(other.profiles).collect(),
            machines: self.machines.into_iter().chain(other.machines).collect(),
            templates: self.templates.into_iter().chain(other.templates).collect(),
            networks: self.networks.into_iter().chain(other.networks).collect(),
            blueprints: self
                .blueprints
                .into_iter()
                .chain(other.blueprints)
                .collect(),
            allocations: self
                .allocations
                .into_iter()
                .chain(other.allocations)
                .collect(),
            models: self.models.into_iter().chain(other.models).collect(),
        };

        // Validate config (should never fail if combine inputs also validated)
        out.validate().map_err(map_validation_error("<combined>"))?;
        Ok(out)
    }
}

fn validate_hash_map<T: Validate>(value: &HashMap<String, T>) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();
    for (n, x) in value.values().enumerate() {
        errors.merge_self(format!("[{n}]").to_static_str(), x.validate());
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}

fn load_predefined_config() -> Result<Config, ConfigError> {
    serde_yaml::from_str(PREDEFINED_CONFIG)
        .map_err(|x| ConfigError::ParseError("<predefined config>".to_string(), x))
}

fn load_default_config() -> Result<Config, ConfigError> {
    serde_yaml::from_str(DEFAULT_CONFIG)
        .map_err(|x| ConfigError::ParseError("<default config>".to_string(), x))
}

fn load_config_file<P: AsRef<Path>>(config_file: P) -> Result<Config, ConfigError> {
    let rdr = std::fs::File::open(&config_file).map_err(ConfigError::IoError)?;
    serde_yaml::from_reader(rdr)
        .map_err(|x| ConfigError::ParseError(config_file.as_ref().to_string_lossy().to_string(), x))
}
