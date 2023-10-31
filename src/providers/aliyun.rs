use crate::CloudProvider;

pub struct AliyunProvider;

impl CloudProvider for AliyunProvider {
    fn list_spot_instances(&self) {
        // to do 
    }

    fn buy_spot_instance(&self, instance_id: &str) {
        println!("{:?}",instance_id)
        // to do 
    }
}


