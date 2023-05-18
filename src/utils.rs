use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

/// A Discord snowflake ID
/// This information is unique to every Discord user
/// Reference:
/// https://discord.com/developers/docs/reference#snowflakes
///
/// Example:
/// ```rs
/// let snowflake = Snowflake::from_bits(782164174821523467);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Snowflake {
    /// The timestamp of the ID
    /// 42 bits number based
    pub timestamp: DateTime<Utc>,
    /// The internal worker ID
    /// 5 bits
    internal_worker_id: u8,
    /// The internal process ID
    /// 5 bits
    internal_process_id: u8,
    /// Incremental for every ID generated on that process
    /// 12 bits
    increment: u16,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SnowflakeError {
    InvalidBits,
}

impl Snowflake {
    pub fn from_bits(snowflake: u64) -> Result<Self, SnowflakeError> {
        let timestamp_ms = (snowflake >> 22) + 1420070400000;
        let internal_worker_id = (snowflake & 0x3E0000) >> 17;
        let internal_process_id = (snowflake & 0x1F000) >> 12;
        let increment = snowflake & 0xFFF;

        let naive_time = NaiveDateTime::from_timestamp_millis(timestamp_ms as i64);
        if naive_time.is_none() {
            return Err(SnowflakeError::InvalidBits);
        }

        Ok(Self {
            timestamp: DateTime::<Utc>::from_utc(naive_time.unwrap(), Utc),
            internal_worker_id: internal_worker_id as u8,
            internal_process_id: internal_process_id as u8,
            increment: increment as u16,
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_snowflake() {
        let snowflake = Snowflake::from_bits(782164174821523467).unwrap();
        assert_eq!(snowflake.timestamp.timestamp_millis(), 1606552871185);
    }
}