pub mod logger;
pub mod publisher;
pub mod subscriber;
pub mod message;
mod utils;

pub struct Node
{
    name : String,
    logger : logger::Logger,
    id_ : usize
}

impl Node {
    pub fn new(node_name : String)->Self
    {
        Self { name: node_name.clone(), logger: logger::Logger::new(node_name) , id_ : 0}
    }

    pub fn create_publisher<T : message::FLSMsg>(&mut self, topic_name : String)->publisher::Publisher<T>
    {
        let publisher = publisher::Publisher::<T>::new(self.name.clone(), topic_name, self.id_);

        self.id_ += 1;

        publisher
    }
    pub fn create_subscriber<T : message::FLSMsg>(&self, topic_name : String)->subscriber::Subscriber<T>
    {
        let subscriber = subscriber::Subscriber::<T>::new(topic_name);

        subscriber
    }

    pub fn log_info(&self, content : String)
    {
        self.logger.info(content);
    }

    pub fn log_error(&self, content : String)
    {
        self.logger.error(content);
    }
}