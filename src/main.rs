use cloud_play::CloudProvider;
use cloud_play::providers::aliyun::AliyunProvider;

fn main() {
    let aliyun_provider = AliyunProvider;
    aliyun_provider.list_spot_instances();
}

