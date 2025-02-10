use super::*;

#[derive(Debug)]
struct ProfileStateFields {
    next_instance_index: usize,
}

#[derive(Debug)]
struct ProfileStateImmutable {
    id: ProfileStateId,
    name: String,
    def: config::Profile,
}

#[derive(Debug, Clone)]
pub struct ProfileState {
    immutable: Arc<ProfileStateImmutable>,
    fields: Arc<ProfileStateFields>,
}

pub type ProfileStateId = StateId<ProfileState>;

impl ProfileState {
    pub fn new(id: ProfileStateId, name: String, def: config::Profile) -> Self {
        Self {
            immutable: Arc::new(ProfileStateImmutable { id, name, def }),
            fields: Arc::new(ProfileStateFields {
                next_instance_index: 0,
            }),
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub fn next_instance(&mut self) -> Option<config::Instance> {
        let instance_index = {
            let instance_index = self.fields.next_instance_index;
            if instance_index >= self.immutable.def.instances.len() {
                return None;
            }
            self.fields = Arc::new(ProfileStateFields {
                next_instance_index: instance_index + 1,
            });
            instance_index
        };
        Some(self.immutable.def.instances[instance_index].clone())
    }
}

impl State for ProfileState {
    fn id(&self) -> StateId<Self> {
        self.immutable.id
    }

    fn name(&self) -> Option<String> {
        Some(self.immutable.name.clone())
    }
}
