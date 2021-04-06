use std::{env, thread, time::Duration};
use log::info;
use rumqtt::{MqttClient, MqttOptions, SecurityOptions,QoS};

use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

/// http服务配置
#[derive(Deserialize, Serialize, Debug)]
struct Service {
    port: Option<i32>,
    mongo: Option<String>,
}

/// 消息队列配置
#[derive(Deserialize, Serialize, Debug)]
struct Rabbitmq {
    server: Option<String>,
    port: Option<u16>,
    topic: Option<String>,
    clientid: Option<String>,
    sleep: Option<i32>,
    user: Option<String>,
    password: Option<String>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    service: Service,
    rabbitmq: Rabbitmq,
}

impl Config {
    /// 从文件中加载配置内容
    pub fn load(file_path: &str) -> Config {
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e),
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };
        let config: Config = toml::from_str(&str_val).unwrap();
        return config;
    }

    /// 返回配置的端口如果没有配置默认返回10086
    pub fn port(&self) -> i32 {
        match self.service.port {
            Some(port) => port,
            None => 10086,
        }
    }

    pub fn mongo(&self) -> &str {
        match &self.service.mongo {
            Some(mongo) => &mongo,
            None => "mongodb://localhost:27017",
        }
    }

    pub fn mq_host(&self) -> &str {
        match &self.rabbitmq.server {
            Some(server) => &server,
            None => "127.0.0.1",
        }
    }

    pub fn mq_port(&self) -> u16 {
        match &self.rabbitmq.port {
            Some(port) => *port,
            None => 1883,
        }
    }

    pub fn mq_topic(&self) -> &str {
        match &self.rabbitmq.topic {
            Some(val) => &val,
            None => "topic.ylz",
        }
    }

    pub fn mq_clientid(&self) -> &str {
        match &self.rabbitmq.clientid {
            Some(val) => &val,
            None => "ylz_log_viewer",
        }
    }

    pub fn mq_sleep(&self) -> i32 {
        match &self.rabbitmq.sleep {
            Some(val) => *val,
            None => 200,
        }
    }

    pub fn mq_user(&self) -> &str {
        match &self.rabbitmq.user {
            Some(val) => &val,
            None => "ylzyx",
        }
    }

    pub fn mq_password(&self) -> &str {
        match &self.rabbitmq.password {
            Some(val) => &val,
            None => "ylzyx",
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("加载日志参数个数错误！");
    }

    let send_cmd = args[1].clone();

    let config = Config::load("config.toml");

    let topic = config.mq_topic();
    let mqtt_options = MqttOptions::new(
      config.mq_clientid(),
      config.mq_host(),
      config.mq_port())
    .set_security_opts(SecurityOptions::UsernamePassword(
        config.mq_user().to_string(),
        config.mq_password().to_string()));
    let (mut mqtt_client, _) = MqttClient::start(mqtt_options).unwrap();
    let sleep_time = Duration::from_millis(500);
    thread::sleep(sleep_time);
      
    mqtt_client.subscribe(topic, QoS::AtLeastOnce).unwrap();
    thread::sleep(sleep_time);

    //let cmd = "{\"Type\":\"6\",\"Data\":{\"ClientId\":\"222\",\"ClientType\":\"0\",\"DateTime\":\"2021-04-01\"}}";
    info!("发送指令：{}", send_cmd);
    mqtt_client.publish(topic, QoS::AtLeastOnce, false, send_cmd).unwrap();
    let sleep_time = Duration::from_millis(500);
    thread::sleep(sleep_time);
}