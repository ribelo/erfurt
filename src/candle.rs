use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: Option<f64>,
    pub time: DateTime<Utc>,
    pub id: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Candles {
    pub id: String,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Option<Vec<f64>>,
    pub time: Vec<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct CandlesIterator {
    candles: Candles,
    idx: usize,
}

pub trait CandleExt {
    fn open(&self) -> f64;
    fn high(&self) -> f64;
    fn low(&self) -> f64;
    fn close(&self) -> f64;
    fn volume(&self) -> Option<f64>;
    fn time(&self) -> DateTime<Utc>;
}

impl CandleExt for Candle {
    fn open(&self) -> f64 {
        self.open
    }

    fn high(&self) -> f64 {
        self.high
    }

    fn low(&self) -> f64 {
        self.low
    }

    fn close(&self) -> f64 {
        self.close
    }

    fn volume(&self) -> Option<f64> {
        self.volume
    }

    fn time(&self) -> DateTime<Utc> {
        self.time
    }
}

pub trait CandlesExt {
    fn get(&self, index: usize) -> Option<Candle>;
    fn open(&self) -> &Vec<f64>;
    fn high(&self) -> &Vec<f64>;
    fn low(&self) -> &Vec<f64>;
    fn close(&self) -> &Vec<f64>;
    fn volume(&self) -> &Option<Vec<f64>>;
    fn time(&self) -> &Vec<DateTime<Utc>>;
    fn last(&self) -> Option<Candle>;
    fn take_last(&self, n: usize) -> Option<Candles>;
}

impl CandlesExt for Candles {
    fn get(&self, index: usize) -> Option<Candle> {
        if index < self.time.len() {
            let symbol = &self.id;
            let open = self.open[index];
            let high = self.high[index];
            let low = self.low[index];
            let close = self.close[index];
            let volume = self.volume.as_ref().map(|x| x[index]);
            let time = self.time[index];

            Some(Candle {
                id: symbol.clone(),
                open,
                high,
                low,
                close,
                volume,
                time,
            })
        } else {
            None
        }
    }

    fn last(&self) -> Option<Candle> {
        self.time.last().map(|time| Candle {
            id: self.id.clone(),
            open: *self.open.last().unwrap(),
            high: *self.high.last().unwrap(),
            low: *self.low.last().unwrap(),
            close: *self.close.last().unwrap(),
            volume: self.volume.as_ref().map(|xs| *xs.last().unwrap()),
            time: *time,
        })
    }

    #[inline]
    fn open(&self) -> &Vec<f64> {
        &self.open
    }

    #[inline]
    fn high(&self) -> &Vec<f64> {
        &self.high
    }

    #[inline]
    fn low(&self) -> &Vec<f64> {
        &self.low
    }

    #[inline]
    fn close(&self) -> &Vec<f64> {
        &self.close
    }

    #[inline]
    fn volume(&self) -> &Option<Vec<f64>> {
        &self.volume
    }

    #[inline]
    fn time(&self) -> &Vec<DateTime<Utc>> {
        &self.time
    }

    fn take_last(&self, n: usize) -> Option<Candles> {
        let len = self.time.len();
        if len < n {
            None
        } else {
            Some(Candles {
                id: self.id.clone(),
                open: self.open[len - n..].to_vec(),
                high: self.high[len - n..].to_vec(),
                low: self.low[len - n..].to_vec(),
                close: self.close[len - n..].to_vec(),
                volume: self.volume.as_ref().map(|xs| xs[len - n..].to_vec()),
                time: self.time[len - n..].to_vec(),
            })
        }
    }
}

impl CandlesExt for &Candles {
    #[inline]
    fn get(&self, index: usize) -> Option<Candle> {
        self.to_owned().get(index)
    }

    #[inline]
    fn open(&self) -> &Vec<f64> {
        &self.open
    }

    #[inline]
    fn high(&self) -> &Vec<f64> {
        &self.high
    }

    #[inline]
    fn low(&self) -> &Vec<f64> {
        &self.low
    }

    #[inline]
    fn close(&self) -> &Vec<f64> {
        &self.close
    }

    #[inline]
    fn volume(&self) -> &Option<Vec<f64>> {
        &self.volume
    }

    #[inline]
    fn time(&self) -> &Vec<DateTime<Utc>> {
        &self.time
    }

    #[inline]
    fn last(&self) -> Option<Candle> {
        self.to_owned().last()
    }

    fn take_last(&self, n: usize) -> Option<Candles> {
        let len = self.time.len();
        if len < n {
            None
        } else {
            Some(Candles {
                id: self.id.clone(),
                open: self.open[len - n..].to_vec(),
                high: self.high[len - n..].to_vec(),
                low: self.low[len - n..].to_vec(),
                close: self.close[len - n..].to_vec(),
                volume: self.volume.as_ref().map(|xs| xs[len - n..].to_vec()),
                time: self.time[len - n..].to_vec(),
            })
        }
    }
}

impl Candles {
    pub fn is_empty(&self) -> bool {
        self.time.is_empty()
    }
    pub fn push(
        &mut self,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: Option<f64>,
        time: DateTime<Utc>,
    ) {
        self.open.push(open);
        self.high.push(high);
        self.low.push(low);
        self.close.push(close);
        if let Some(value) = volume {
            self.volume.as_mut().unwrap().push(value);
        };
        self.time.push(time);
    }
    pub fn iter(&self) -> CandlesIterator {
        CandlesIterator {
            candles: self.clone(),
            idx: 0,
        }
    }
}

impl IntoIterator for Candles {
    type Item = Candle;

    type IntoIter = CandlesIterator;

    fn into_iter(self) -> Self::IntoIter {
        CandlesIterator {
            candles: self,
            idx: 0,
        }
    }
}

impl Iterator for CandlesIterator {
    type Item = Candle;
    fn next(&mut self) -> Option<Self::Item> {
        let candle = self.candles.get(self.idx);
        self.idx += 1;
        candle
    }
}
