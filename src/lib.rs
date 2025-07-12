/// This library takes a batch of data and aggregate it into a small portion
/// of data using timestamp's.
pub struct Squisher {
    __maxed: bool,
    __last_hour: usize,

    data: Vec<f32>,
    interval_seconds: f64,
    limit: usize,
}

impl Squisher {
    /// `last_hours` - The maximum size of the timeseries (in hours).
    /// `interval` - The interval for the data to be aggregated (in hours).
    ///
    /// @example: for a timeseries 24 hours with 1 hour of intervale set
    /// the `last_hours` to 24 and the `interval` to 1.
    pub fn new(last_hours: usize, interval: f32) -> Self {
        Self {
            __maxed: false,
            __last_hour: 0,

            data: vec![0f32; last_hours],
            interval_seconds: (interval * 3600f32) as f64,
            limit: last_hours - 1,
        }
    }

    /// `value` - The number that will be aggregated by your initial setup.
    /// `timestamp` - A UNIX timestamp that will be used to match into the timeseries stored data.
    pub fn compute(&mut self, value: f32, timestamp: u128) -> () {
        let hour = ((timestamp as f64 / self.interval_seconds) % self.limit as f64) as usize;
        if hour == 0 && self.__last_hour != hour {
            self.__maxed = true;
        }

        // @NOTE: when reach the size limit we need to shift the entire data
        // and start to accumulate on the last index until the hour changes, then
        // we do it again and so on.
        if !self.__maxed {
            self.data[hour] += value;
        } else {
            let index = self.limit;
            if self.__last_hour == hour {
                self.data[index] += value;
            } else {
                self.data.drain(0..1);
                self.data.push(0f32);
                self.data[index] += value;
            }
        }

        self.__last_hour = hour
    }

    pub fn ts(&self) -> Vec<f32> {
        self.data.clone()
    }
}
