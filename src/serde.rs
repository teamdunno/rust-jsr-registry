use std::{collections::HashMap, fmt, marker::PhantomData};

use chrono::{DateTime, Utc};
use semver::Version;
use serde::{de::{MapAccess, Visitor, Error}, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    name: String,
    #[serde(with = "rfc3339")]
    timestamp: DateTime<Utc>,
}


pub mod rfc3339 {
    use super::*;

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.fZ";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        Serializer::serialize_str(serializer, &s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT)
            .map(|dt| dt.with_timezone(&Utc)) // explicitly convert to UTC
            .map_err(serde::de::Error::custom)
    }
}

/// Wrapper for [HashMap] in [TimeInfo]
#[derive(Debug, Clone)]
pub struct VersionDateTimeMap(HashMap<Version, DateTime<Utc>>);

impl Serialize for VersionDateTimeMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        
        for (version, datetime) in &self.0 {
            // Convert Version to string for serialization
            let version_str = version.to_string();
            
            // Use chrono's RFC3339 formatter for the datetime
            let datetime_str = datetime.to_rfc3339();
            
            map.serialize_entry(&version_str, &datetime_str)?;
        }
        
        map.end()
    }
}

impl<'de> Deserialize<'de> for VersionDateTimeMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VersionDateTimeMapVisitor(PhantomData<fn() -> VersionDateTimeMap>);
        
        impl VersionDateTimeMapVisitor {
            fn new() -> Self {
                VersionDateTimeMapVisitor(PhantomData)
            }
        }
        
        impl<'de> Visitor<'de> for VersionDateTimeMapVisitor {
            type Value = VersionDateTimeMap;
            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of version strings to RFC3339 datetime strings")
            }
            
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));
                
                while let Some((version_str, datetime_str)) = access.next_entry::<String, String>()? {
                    // Parse the version string
                    let version = Version::parse(&version_str)
                        .map_err(|e| M::Error::custom(format!("Invalid version: {}", e)))?;
                    
                    // Parse the datetime string
                    let datetime = DateTime::parse_from_rfc3339(&datetime_str)
                        .map_err(|e| M::Error::custom(format!("Invalid datetime: {}", e)))?
                        .with_timezone(&Utc);
                    
                    map.insert(version, datetime);
                }
                
                Ok(VersionDateTimeMap(map))
            }
        }
        
        deserializer.deserialize_map(VersionDateTimeMapVisitor::new())
    }
}

impl VersionDateTimeMap {
    pub fn new() -> Self {
        VersionDateTimeMap(HashMap::new())
    }
    pub fn inner(self) -> HashMap<Version, DateTime<Utc>> {
        return self.0;
    }
    pub fn inner_ref(&self) -> &HashMap<Version, DateTime<Utc>> {
        return &self.0;
    }
    
}


#[derive(Debug, Clone)]
pub struct TimeInfo {
    /// Package publicly created timestamp, for the first time
    pub created: DateTime<Utc>,
    /// Package modified on timestamp
    pub modified: DateTime<Utc>,
    /// List of package versions published at timestamp
    pub versions: VersionDateTimeMap,
}

impl Serialize for TimeInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;
        
        let mut map = serializer.serialize_map(None)?;
        
        // Add created and modified fields
        map.serialize_entry("created", &self.created.to_rfc3339())?;
        map.serialize_entry("modified", &self.modified.to_rfc3339())?;
        
        // Add version entries
        for (version, datetime) in self.versions.0.iter() {
            map.serialize_entry(&version.to_string(), &datetime.to_rfc3339())?;
        }
        
        map.end()
    }
}
impl<'de> Deserialize<'de> for TimeInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // We'll use a helper struct to parse the JSON
        #[derive(Deserialize)]
        struct TimeInfoHelper {
            created: String,
            modified: String,
            #[serde(flatten)]
            other_fields: HashMap<String, String>,
        }
        
        let helper = TimeInfoHelper::deserialize(deserializer)?;
        
        // Parse created and modified timestamps
        let created = DateTime::parse_from_rfc3339(&helper.created)
            .map_err(Error::custom)?
            .with_timezone(&Utc);
            
        let modified = DateTime::parse_from_rfc3339(&helper.modified)
            .map_err(Error::custom)?
            .with_timezone(&Utc);
        
        // Build versions map
        let mut versions_map = HashMap::new();
        for (key, val) in helper.other_fields {
            // Skip any non-version keys
            if let Ok(version) = Version::parse(&key) {
                if let Ok(datetime) = DateTime::parse_from_rfc3339(&val) {
                    versions_map.insert(version, datetime.with_timezone(&Utc));
                }
            }
        }
        
        Ok(TimeInfo {
            created,
            modified,
            versions: VersionDateTimeMap(versions_map),
        })
    }
}

pub mod url {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;
    use url::Url;

    pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(url.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Url::from_str(&s).map_err(serde::de::Error::custom)
    }
}