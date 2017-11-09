use std::collections::HashMap;
use std::io;
use std::str::FromStr;
use csv;

#[derive(Clone, Debug)]
pub struct ZipCodes {
    data: HashMap<String, Region>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Region {
    country: String,
    zip: String,
    place_name: String,
    lat: f64,
    lng: f64,
}

impl ZipCodes {
    pub fn load_from<R: io::Read>(stream: R) -> Result<Self, ()> {
        let mut ret = ZipCodes { data: Default::default() };
        let mut data = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(stream);

        for result in data.records() {
            let row = try!(result.map_err(|_| ()));
            let code = try!(Region::from_row(row));

            ret.data.insert(code.zip.clone(), code);
        }

        Ok(ret)
    }

    pub fn find<T: AsRef<str>>(&self, zip: T) -> Option<&Region> {
        self.data.get(zip.as_ref())
    }
}

impl Region {
    fn from_row(record: csv::StringRecord) -> Result<Self, ()> {
        Ok(Region {
            country: try!(extract(&record, 0)),
            zip: try!(extract(&record, 1)),
            place_name: try!(extract(&record, 2)),
            lat: try!(extract(&record, 9)),
            lng: try!(extract(&record, 10)),
        })
    }
}

fn extract<T>(record: &csv::StringRecord, i: usize) -> Result<T, ()>
where
    T: FromStr,
{
    record.get(i).and_then(|s| T::from_str(s).ok()).ok_or(())
}
