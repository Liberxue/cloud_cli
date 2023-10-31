pub trait CloudProvider {
    fn list_spot_instances(&self);
    fn buy_spot_instance(&self, instance_id: &str);
}

pub mod providers;

