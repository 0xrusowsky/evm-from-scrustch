use crate::types::{hex_string_to_address, hex_string_to_bytes, hex_string_to_bytes_vec, Bytes, Bytes32, Address};
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone)]
pub struct Log {
    address: Address,
    data: Bytes,
    topic1: Option<Bytes32>,
    topic2: Option<Bytes32>,
    topic3: Option<Bytes32>,
    topic4: Option<Bytes32>,
    topic_count: usize,
}

impl Log {
    pub fn new(address:Address, data: Bytes) -> Self {
        Self {
            address,
            data,
            topic1: None,
            topic2: None,
            topic3: None,
            topic4: None,
            topic_count: 0,
        }
    }

    pub fn default() -> Self {
        Self {
            address: Address::zero(),
            data: Bytes::new(),
            topic1: None,
            topic2: None,
            topic3: None,
            topic4: None,
            topic_count: 0,
        }
    }

    pub fn add_topic(&mut self, topic: Bytes32) {
        match self.topic_count {
            0 => self.topic1 = Some(topic),
            1 => self.topic2 = Some(topic),
            2 => self.topic3 = Some(topic),
            3 => self.topic4 = Some(topic),
            _ => panic!("Too many topics"),
        }

        self.topic_count += 1;
    }

    pub fn add_topics(&mut self, topics: Vec<Bytes32>) {
        for topic in topics {
            self.add_topic(topic);
        }
    }

    pub fn add_data(&mut self, data: Bytes) {
        self.data = data;
    }

    // Getters

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn data(&self) -> Bytes {
        self.data.clone()
    }

    pub fn topic1(&self) -> Option<Bytes32> {
        self.topic1.clone()
    }

    pub fn topic2(&self) -> Option<Bytes32> {
        self.topic2.clone()
    }

    pub fn topic3(&self) -> Option<Bytes32> {
        self.topic3.clone()
    }

    pub fn topic4(&self) -> Option<Bytes32> {
        self.topic4.clone()
    }

    pub fn topic_count(&self) -> usize {
        self.topic_count
    }

    // Parsing
    pub fn from_json(json_log: &JsonLog) -> Result<Self, String> {
        let topics = json_log.topics.clone();
        let topics: Vec<Option<Bytes32>> = topics.into_iter()
            .map(|bytes| Some(Bytes32::from_bytes(bytes)))
            .collect();

        // Handling possible absence of topics
        let topic1 = topics.get(0).cloned().flatten();
        let topic2 = topics.get(1).cloned().flatten();
        let topic3 = topics.get(2).cloned().flatten();
        let topic4 = topics.get(3).cloned().flatten();

        Ok(Log {
            address: json_log.address,
            data: json_log.data.clone(),
            topic1,
            topic2,
            topic3,
            topic4,
            topic_count: topics.len(),
        })
    }
}

// Aux struct for deserializing logs from JSON
#[derive(Deserialize, Debug)]
pub struct JsonLog {
    #[serde(deserialize_with = "hex_string_to_address")]
    address: Address,
    #[serde(deserialize_with = "hex_string_to_bytes")]
    data: Bytes,
    #[serde(deserialize_with = "hex_string_to_bytes_vec")]
    topics: Vec<Bytes>,
}