#[derive(Debug, Clone)]
pub struct Observation {
    pub source_id: String,
    pub timestamp: u64,
    pub x: f64,
    pub y: f64,
    pub confidence: f64,
    pub payload: ObservationPayload,
}

#[derive(Debug, Clone)]
pub enum ObservationPayload {
    SurvivorSignal {
        strength: f64,
    },

    WaterReading {
        surface_temp_c: f64,
        turbidity: f64,
        spectral_anomaly: f64,
    },

    Weather {
        wind_speed: f64,
        humidity: f64,
    },

    Wifi {
        network: String,
        strength: f64,
    },

    PowerGrid {
        continuity: bool,
        voltage: bool,
    },

    HumanFieldReport {
        category: String,
        what_changed: String,
    },
}
